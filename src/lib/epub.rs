// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use error::{Error, Result, Source};
use token::Token;
use html::HtmlRenderer;
use book::{Book, compile_str};
use zipper::Zipper;
use templates::epub::*;
use templates::epub3;
use resource_handler;
use renderer::Renderer;
use parser::Parser;

use chrono;
use uuid;
use mustache::Template;

use std::io::Read;
use std::convert::{AsRef, AsMut};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::borrow::Cow;
use std::mem;
use mime_guess::guess_mime_type_opt;

/// Renderer for Epub
///
/// Uses part of the HTML renderer
pub struct EpubRenderer<'a> {
    toc: Vec<String>,
    html: HtmlRenderer<'a>,
    chapter_title: String,
}

impl<'a> EpubRenderer<'a> {
    /// Creates a new Epub renderer
    pub fn new(book: &'a Book) -> EpubRenderer<'a> {
        let mut html = HtmlRenderer::new(book);
        html.toc.numbered(true);
        html.handler.set_images_mapping(true);
        html.handler.set_base64(false);
        EpubRenderer {
            html: html,
            toc: vec![],
            chapter_title: String::new(),
        }
    }

    /// Render a book
    pub fn render_book(&mut self) -> Result<String> {
        for (i, filename) in self.html.book.filenames.iter().enumerate() {
            self.html.handler.add_link(filename.clone(), filenamer(i));
        }

        let mut zipper =
            Zipper::new(&self.html.book.options.get_path("crowbook.temp_dir").unwrap())?;

        // Write mimetype
        zipper.write("mimetype", b"application/epub+zip", true)?;

        // Write cover.xhtml (if needs be)
        if self.html.book.options.get_path("cover").is_ok() {
            zipper.write("cover.xhtml", &self.render_cover()?.as_bytes(), true)?;
        }

        // Write chapters
        let template_chapter =
            compile_str(self.html.book.get_template("epub.chapter.xhtml")?.as_ref(),
                        &self.html.book.source,
                        lformat!("could not compile template 'epub.chapter.xhtml'"))?;
        for (i, &(n, ref v)) in self.html.book.chapters.iter().enumerate() {
            self.html.chapter_config(i, n, filenamer(i));
            let chapter = self.render_chapter(v, &template_chapter)?;

            zipper.write(&filenamer(i), &chapter.as_bytes(), true)?;
        }
        self.html.source = Source::empty();

        // Render the CSS file and write it
        let template_css =
            compile_str(self.html.book.get_template("epub.css").unwrap().as_ref(),
                             &self.html.book.source,
                             lformat!("could not compile template 'epub.css'"))?;
        let data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
        .insert_bool(self.html.book.options.get_str("lang").unwrap(), true)
            .build();
        let mut res: Vec<u8> = vec![];
        template_css.render_data(&mut res, &data)?;
        let css = String::from_utf8_lossy(&res);
        zipper.write("stylesheet.css", css.as_bytes(), true)?;

        // Write titlepage
        zipper.write("title_page.xhtml",
                          &self.render_titlepage()?.as_bytes(),
                          true)?;

        // Write file for ibook (why?)
        zipper.write("META-INF/com.apple.ibooks.display-options.xml",
                          IBOOK.as_bytes(),
                          true)?;

        // Write container.xml
        zipper.write("META-INF/container.xml", CONTAINER.as_bytes(), true)?;

        // Write nav.xhtml
        zipper.write("nav.xhtml", &self.render_nav()?.as_bytes(), true)?;

        // Write content.opf
        zipper.write("content.opf", &self.render_opf()?.as_bytes(), true)?;

        // Write toc.ncx
        zipper.write("toc.ncx", &self.render_toc()?.as_bytes(), true)?;

        // Write all images (including cover)
        for (source, dest) in self.html.handler.images_mapping() {
            let mut f = File::open(source).map_err(|_| {
                Error::file_not_found(&self.html.source,
                                      lformat!("image or cover"),
                                      source.to_owned())
            })?;
            let mut content = vec![];
            f.read_to_end(&mut content).map_err(|e| {
                Error::render(&self.html.source,
                              lformat!("error while reading image file: {error}", error = e))
            })?;
            zipper.write(dest, &content, true)?;
        }

        // Write additional resources
        if let Ok(list) = self.html.book.options.get_paths_list("resources.files") {
            let base_path_files =
                self.html.book.options.get_path("resources.base_path.files").unwrap();
            let list = resource_handler::get_files(list, &base_path_files)?;
            let data_path =
                Path::new(self.html.book.options.get_relative_path("resources.out_path")?);
            for path in list {
                let abs_path = Path::new(&base_path_files).join(&path);
                let mut f = File::open(&abs_path).map_err(|_| {
                    Error::file_not_found(&self.html.book.source,
                                          lformat!("additional resource from resources.files"),
                                          abs_path.to_string_lossy().into_owned())
                })?;
                let mut content = vec![];
                f.read_to_end(&mut content)
                    .map_err(|e| {
                        Error::render(&self.html.book.source,
                                      lformat!("error while reading resource file: {error}", error = e))
                    })?;
                zipper.write(data_path.join(&path).to_str().unwrap(), &content, true)?;
            }
        }

        if let Ok(epub_file) = self.html.book.options.get_path("output.epub") {
            let res = zipper.generate_epub(self.html
                                           .book
                                           .options
                                           .get_str("crowbook.zip.command")
                                           .unwrap(),
                                           &epub_file)?;
            Ok(res)
        } else {
            Err(Error::render(&self.html.book.source,
                              lformat!("no output epub file specified in book config")))
        }
    }

    /// Render the titlepgae
    fn render_titlepage(&mut self) -> Result<String> {
        let epub3 = self.html.book.options.get_i32("epub.version").unwrap() == 3;
        let template = compile_str(if epub3 { epub3::TITLE } else { TITLE },
                                   &self.html.book.source,
                                   lformat!("could not compile template for title page"))?;
        let data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
        .build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!("generated HTML in titlepage was not utf-8 valid"),
            Ok(res) => Ok(res),
        }
    }

    /// Render toc.ncx
    fn render_toc(&mut self) -> Result<String> {
        let mut nav_points = String::new();

        for (n, ref title) in self.toc.iter().enumerate() {
            let filename = filenamer(n);
            let id = format!("navPoint-{}", n + 1);
            nav_points.push_str(&format!("\
    <navPoint id=\"{id}\">
      <navLabel>
        <text>{title}</text>
      </navLabel>
      <content src = \"{file}\" />
    </navPoint>\n",
                                         id = id,
                                         title = title,
                                         file = filename));
        }
        let template = compile_str(TOC,
                                   &self.html.book.source,
                                   lformat!("could not render template for toc.ncx"))?;
        let data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("nav_points", nav_points)
            .build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!(lformat!("generated HTML in toc.ncx was not valid utf-8")),
            Ok(res) => Ok(res),
        }
    }

    /// Render content.opf
    fn render_opf(&mut self) -> Result<String> {
        // Optional metadata
        let mut cover_xhtml = String::new();
        let mut optional = String::new();
        if let Ok(s) = self.html.book.options.get_str("description") {
            optional.push_str(&format!("<dc:description>{}</dc:description>\n", s));
        }
        if let Ok(s) = self.html.book.options.get_str("subject") {
            optional.push_str(&format!("<dc:subject>{}</dc:subject>\n", s));
        }
        if let Ok(ref s) = self.html.book.options.get_path("cover") {
            optional.push_str(&format!("<meta name = \"cover\" content = \"{}\" />\n",
                                       self.html
                                       .handler
                                       .map_image(&self.html.source, s.as_ref())?));
            cover_xhtml.push_str("<reference type=\"cover\" \
                                  title=\"Cover\" href=\"cover.xhtml\" />");
        }

        // date
        let date = chrono::UTC::now().format("%Y-%m-%dT%H:%M:%SZ");

        // uuid
        let uuid = uuid::Uuid::new_v4().urn().to_string();

        let mut items = String::new();
        let mut itemrefs = String::new();
        let mut coverref = String::new();
        if self.html.book.options.get("cover").is_ok() {
            items.push_str("<item id = \"cover_xhtml\" href = \"cover.xhtml\" media-type = \
                            \"application/xhtml+xml\" />\n");
            coverref.push_str("<itemref idref = \"cover_xhtml\" />");
        }
        for n in 0..self.toc.len() {
            let filename = filenamer(n);
            items.push_str(&format!("<item id = \"{}\" href = \"{}\" \
                                     media-type=\"application/xhtml+xml\" />\n",
                                    to_id(&filename),
                                    filename));
            itemrefs.push_str(&format!("<itemref idref=\"{}\" />\n", to_id(&filename)));
        }

        // put the images in the manifest too
        for image in self.html.handler.images_mapping().values() {
            let format = self.get_format(image);
            items.push_str(&format!("<item media-type = \"{}\" id = \"{}\" href = \"{}\" />\n",
                                    format,
                                    to_id(image),
                                    image));
        }

        // and additional files too
        if let Ok(list) = self.html.book.options.get_paths_list("resources.files") {
            let list = resource_handler::get_files(list,
                                                   &self.html
                                                   .book
                                                   .options
                                                   .get_path("resources.base_path.\
                                                              files")
                                                   .unwrap())?;
            let data_path =
                Path::new(self.html.book.options.get_relative_path("resources.out_path").unwrap());
            for path in list {
                let format = self.get_format(&path);
                let path = data_path.join(&path);
                let path_str = path.to_str().unwrap();
                items.push_str(&format!("<item media-type = \"{}\" id = \"{}\" href = \"{}\" \
                                         />\n",
                                        format,
                                        to_id(path_str),
                                        path_str));
            }
        }

        let epub3 = self.html.book.options.get_i32("epub.version").unwrap() == 3;
        let template = compile_str(if epub3 { epub3::OPF } else { OPF },
                                   &self.html.book.source,
                                   lformat!("could not compile template for content.opf"))?;
        let data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("optional", optional)
            .insert_str("items", items)
            .insert_str("itemrefs", itemrefs)
            .insert_str("date", date)
            .insert_str("uuid", uuid)
            .insert_str("cover_xhtml", cover_xhtml)
            .insert_str("coverref", coverref)
            .build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!(lformat!("generated HTML in content.opf was not valid utf-8")),
            Ok(res) => Ok(res),
        }
    }

    /// Render cover.xhtml
    fn render_cover(&mut self) -> Result<String> {
        if let Ok(cover) = self.html.book.options.get_path("cover") {
            // Check that cover can be found
            if fs::metadata(&cover).is_err() {
                return Err(Error::file_not_found(&self.html.book.source, lformat!("cover"), cover));

            }
            let epub3 = self.html.book.options.get_i32("epub.version").unwrap() == 3;
            let template = compile_str(if epub3 { epub3::COVER } else { COVER },
                                       &self.html.book.source,
                                       lformat!("could not compile template for \
                                                 cover.xhtml"))?;
            let data = self.html
                .book
                .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
                .insert_str("cover",
                            self.html
                            .handler
                            .map_image(&self.html.source, Cow::Owned(cover))?
                            .into_owned())
                .build();
            let mut res: Vec<u8> = vec![];
            template.render_data(&mut res, &data)?;
            match String::from_utf8(res) {
                Err(_) => panic!(lformat!("generated HTML for cover.xhtml was not utf-8 valid")),
                Ok(res) => Ok(res),
            }
        } else {
            panic!(lformat!("Why is this method called if cover is None???"));
        }
    }

    /// Render nav.xhtml
    fn render_nav(&mut self) -> Result<String> {
        let content = self.html.toc.render();

        let template = if self.html.book.options.get_i32("epub.version").unwrap() == 3 {
            epub3::NAV
        } else {
            NAV
        };
        let template = compile_str(template,
                                   &self.html.book.source,
                                   lformat!("could not compile template for nav.xhtml"))?;
        let data = self.html
                .book
                .get_metadata(|s| self.render_vec(&(Parser::new().parse_inline(s)?)))?
            .insert_str("content", content)
            .build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!(lformat!("generated HTML in nav.xhtml was not utf-8 valid")),
            Ok(res) => Ok(res),
        }
    }

    /// Render a chapter
    pub fn render_chapter(&mut self, v: &[Token], template: &Template) -> Result<String> {
        let mut content = String::new();

        for token in v {
            let res = self.render_token(&token)?;
            content.push_str(&res);
            self.html.render_side_notes(&mut content);
        }
        self.html.render_end_notes(&mut content);

        if self.chapter_title.is_empty() && self.html.current_numbering >= 1 {
            let number = self.html.current_chapter[0] + 1;
            self.chapter_title = self.html.book.get_chapter_header(number, "".to_owned(), |s| {
                    self.render_vec(&Parser::new().parse_inline(s)?)
                })?;
        }
        self.toc.push(self.chapter_title.clone());

        let data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("content", content)
            .insert_str("chapter_title",
                        mem::replace(&mut self.chapter_title, String::new()))
            .build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!(lformat!("generated HTML was not utf-8 valid")),
            Ok(res) => Ok(res),
        }
    }

    /// Renders the header section of the book, finding the title of the chapter
    fn find_title(&mut self, vec: &[Token]) -> Result<()> {
        if self.html.current_hide || self.html.current_numbering == 0 {
            if self.chapter_title.is_empty() {
                self.chapter_title = self.html.render_vec(vec)?;
            } else {
                self.html
                    .book
                    .logger
                    .warning(lformat!("EPUB ({source}): detected two chapter titles inside the \
                                       same markdown file, in a file where chapter titles are \
                                       not even rendered.",
                                      source = self.html.source));
            }
        } else {
            let res = self.html.book.get_chapter_header(self.html.current_chapter[0] + 1,
                                                        self.html.render_vec(vec)?,
                                                        |s| {
                                                            self.render_vec(&(Parser::new()
                                                                .parse_inline(s)?))
                                                        });
            let s = res.unwrap();
            if self.chapter_title.is_empty() {
                self.chapter_title = s;
            } else {
                self.html
                    .book
                    .logger
                    .warning(lformat!("EPUB ({source}): detected two chapters inside the same \
                                       markdown file.",
                                      source = self.html.source));
                self.html
                    .book
                    .logger
                    .warning(lformat!("EPUB ({source}): conflict between: {title1} and {title2}",
                                      source = self.html.source,
                                      title1 = self.chapter_title,
                                      title2 = s));
            }
        }
        Ok(())
    }

    // Get the format of an image file, based on its extension
    fn get_format(&self, s: &str) -> String {
        let opt = guess_mime_type_opt(s);
        match opt {
            Some(s) => s.to_string(),
            None => {
                self.html
                    .book
                    .logger
                    .error(lformat!("EPUB: could not guess the format of {file} based on \
                                     extension. Assuming png.",
                                    file = s));
                String::from("png")
            }
        }
    }

    /// Renders a token
    ///
    /// Used by render_token implementation of Renderer trait. Separate function
    /// because we need to be able to call it from other renderers.
    ///
    /// See http://lise-henry.github.io/articles/rust_inheritance.html
    #[doc(hidden)]
    pub fn static_render_token<T>(this: &mut T, token: &Token) -> Result<String>
    where T: AsMut<EpubRenderer<'a>>+AsRef<EpubRenderer<'a>> +
        AsMut<HtmlRenderer<'a>>+AsRef<HtmlRenderer<'a>> + Renderer
    {
        match *token {
            Token::Header(1, ref vec) => {
                {
                    let epub: &mut EpubRenderer = this.as_mut();
                    epub.find_title(vec)?;
                }
                HtmlRenderer::static_render_token(this, token)
            }
            Token::Footnote(ref vec) => {
                let epub3 = (this.as_ref() as &HtmlRenderer)
                    .book
                    .options
                    .get_i32("epub.version")
                    .unwrap() == 3;
                let inner_content = this.render_vec(vec)?;
                let html: &mut HtmlRenderer = this.as_mut();
                html.footnote_number += 1;
                let number = html.footnote_number;
                let note_number = format!("<p class = \"note-number\">
  <a href = \"#note-source-{}\">[{}]</a>
</p>\n",
                                          number,
                                          number);
                let inner = if epub3 {
                    format!("<aside epub:type = \"footnote\" id = \"note-dest-{}\">{}</aside>",
                            number,
                            inner_content)
                } else {
                    format!("<a id = \"note-dest-{}\" />{}", number, inner_content)
                };
                html.add_footnote(note_number, inner);

                Ok(format!("<a {} href = \"#note-dest-{}\"><sup id = \
                            \"note-source-{}\">{}</sup></a>",
                           if epub3 { "epub:type = \"noteref\"" } else { "" },
                           number,
                           number,
                           number))
            }
            _ => HtmlRenderer::static_render_token(this, token),
        }
    }
}


// generate an id compatible string, replacing / and . by _
fn to_id(s: &str) -> String {
    s.replace(".", "_").replace("/", "_")
}

/// Generate a file name given an int
fn filenamer(i: usize) -> String {
    format!("chapter_{:03}.xhtml", i)
}


derive_html!{EpubRenderer<'a>, EpubRenderer::static_render_token}
