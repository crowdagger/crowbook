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

use error::{Error,Result};
use token::Token;
use html::HtmlRenderer;
use book::Book;
use number::Number;
use zipper::Zipper;
use templates::epub::*;
use templates::epub3;

use mustache;
use chrono;
use uuid;

use std::io::{Read,Write};
use std::fs::File;
use std::path::Path;
use std::borrow::Cow;
use mime_guess::guess_mime_type_opt;

/// Renderer for Epub
///
/// Uses part of the HTML renderer
pub struct EpubRenderer<'a> {
    book: &'a Book,
    toc: Vec<String>,
    html: HtmlRenderer<'a>,
}

impl<'a> EpubRenderer<'a> {
    /// Creates a new Epub renderer
    pub fn new(book: &'a Book) -> EpubRenderer<'a> {
        let mut html = HtmlRenderer::new(book);
        html.toc.numbered(true);
        html.handler.set_images_mapping(true);
        EpubRenderer {
            book: book,
            html: html,
            toc: vec!(),
        }
    }

    /// Render a book
    pub fn render_book(&mut self) -> Result<String> {
        for (i, filename) in self.book.filenames.iter().enumerate() {
            self.html.handler.add_link(filename.clone(), filenamer(i));
        }

        let mut zipper = try!(Zipper::new(&self.book.options.get_path("temp_dir").unwrap()));
        
        // Write mimetype
        try!(zipper.write("mimetype", b"application/epub+zip", true));

        // Write chapters        
        for (i, &(n, ref v)) in self.book.chapters.iter().enumerate() {
            self.html.filename = filenamer(i);
            self.html.current_hide = false;
            let book_numbering = self.book.options.get_i32("numbering").unwrap();
            match n {
                Number::Unnumbered => self.html.current_numbering = 0,
                Number::Default => self.html.current_numbering = book_numbering,
                Number::Specified(n) => {
                    self.html.current_numbering = book_numbering;
                    self.html.current_chapter[0] = n - 1;
                },
                Number::Hidden => {
                    self.html.current_numbering = 0;
                    self.html.current_hide = true;
                }
            }
            let chapter = try!(self.render_chapter(v));

            try!(zipper.write(&filenamer(i), &chapter.as_bytes(), true));
        }
        
        // Render the CSS file and write it
        let template_css = mustache::compile_str(try!(self.book.get_template("epub.css")).as_ref());
        let data = self.book.get_mapbuilder("none")
            .insert_bool(self.book.options.get_str("lang").unwrap(), true)
            .build();
        let mut res:Vec<u8> = vec!();
        template_css.render_data(&mut res, &data);
        let css = String::from_utf8_lossy(&res);
        try!(zipper.write("stylesheet.css",
                          css.as_bytes(), true));

        // Write titlepage
        try!(zipper.write("title_page.xhtml", &try!(self.render_titlepage()).as_bytes(), true));

        // Write file for ibook (why?)
        try!(zipper.write("META-INF/com.apple.ibooks.display-options.xml", IBOOK.as_bytes(), true));

        // Write container.xml
        try!(zipper.write("META-INF/container.xml", CONTAINER.as_bytes(), true));

        // Write nav.xhtml
        try!(zipper.write("nav.xhtml", &try!(self.render_nav()).as_bytes(), true));

        // Write content.opf
        try!(zipper.write("content.opf", &try!(self.render_opf()).as_bytes(), true));

        // Write toc.ncx
        try!(zipper.write("toc.ncx", &try!(self.render_toc()).as_bytes(), true));

        // Write cover.xhtml (if needs be)
        if self.book.options.get_path("cover").is_ok() {
            try!(zipper.write("cover.xhtml", &try!(self.render_cover()).as_bytes(), true));
        }

        // Write all images (including cover)
        for (source, dest) in self.html.handler.images_mapping() {
            let mut f = try!(File::open(self.book.root.join(source)).map_err(|_| Error::FileNotFound(source.to_owned())));
            let mut content = vec!();
            try!(f.read_to_end(&mut content).map_err(|_| Error::Render("error while reading image file")));
            try!(zipper.write(dest, &content, true));
        }

        // Write additional resources
        if let Ok(list) = self.book.options.get_paths_list("resources.files") {
            let data_path = Path::new(try!(self.book.options.get_relative_path("resources.path")));
            for path in list{
                let mut f = try!(File::open(self.book.root.join(&path)).map_err(|_| Error::FileNotFound(path.clone())));
                let mut content = vec!();
                try!(f.read_to_end(&mut content).map_err(|_| Error::Render("error while reading resource file")));
                try!(zipper.write(data_path.join(&path).to_str().unwrap(), &content, true));
            }
        }
        
        if let Ok(epub_file) = self.book.options.get_path("output.epub") {
            let res = try!(zipper.generate_epub(self.book.options.get_str("zip.command").unwrap(), &epub_file));
            Ok(res)
        } else {
            Err(Error::Render("no output epub file specified in book config"))
        }
    }
    
    /// Render the titlepgae
    fn render_titlepage(&self) -> Result<String> {
        let template = mustache::compile_str(if self.book.options.get_i32("epub.version").unwrap() == 3 {epub3::TITLE} else {TITLE});
        let data = self.book.get_mapbuilder("none")
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML in titlepage was not utf-8 valid")),
            Ok(res) => Ok(res)
        }
    }
    
    /// Render toc.ncx
    fn render_toc(&self) -> Result<String> {
        let mut nav_points = String::new();

        for (n, ref title) in self.toc.iter().enumerate() {
            let filename = filenamer(n);
            let id = format!("navPoint-{}", n + 1);
            nav_points.push_str(&format!(
"   <navPoint id=\"{}\">
      <navLabel>
        <text>{}</text>
      </navLabel>
      <content src = \"{}\" />
    </navPoint>\n", id, title, filename));
        }
        let template = mustache::compile_str(TOC);
        let data = self.book.get_mapbuilder("none")
            .insert_str("nav_points", nav_points)
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML in toc.ncx was not valid utf-8")),
            Ok(res) => Ok(res)
        }
    }

    /// Render content.opf
    fn render_opf(&mut self) -> Result<String> {
        // Optional metadata
        let mut cover_xhtml = String::new();
        let mut optional = String::new();
        if let Ok(s) = self.book.options.get_str("description") {
            optional.push_str(&format!("<dc:description>{}</dc:description>\n", s));
        }
        if let Ok(s) = self.book.options.get_str("subject") {
            optional.push_str(&format!("<dc:subject>{}</dc:subject>\n", s));
        }
        if let Ok(ref s) = self.book.options.get_path("cover") {
            optional.push_str(&format!("<meta name = \"cover\" content = \"{}\" />\n",
                                       self.html.handler.map_image(Cow::Borrowed(s))));
            cover_xhtml.push_str(&format!("<reference type=\"cover\" title=\"Cover\" href=\"cover.xhtml\" />"));
        }

        // date
        let date = chrono::UTC::now().format("%Y-%m-%dT%H:%M:%SZ");

        // uuid
        let uuid = uuid::Uuid::new_v4().to_urn_string();
        
        let mut items = String::new();
        let mut itemrefs = String::new();
        let mut coverref = String::new();
        if self.book.options.get("cover").is_ok() {
            items.push_str("<item id = \"cover_xhtml\" href = \"cover.xhtml\" media-type = \"application/xhtml+xml\" />\n");
            coverref.push_str("<itemref idref = \"cover_xhtml\" />");
        }
        for n in 0..self.toc.len() {
            let filename = filenamer(n);
            items.push_str(&format!("<item id = \"{}\" href = \"{}\" media-type=\"application/xhtml+xml\" />\n",
                                    to_id(&filename),
                                    filename));
            itemrefs.push_str(&format!("<itemref idref=\"{}\" />\n", to_id(&filename)));
        }

        // put the images in the manifest too
        for image in self.html.handler.images_mapping().values() {
            let format = self.get_format(image);
            items.push_str(&format!("<item media-type = \"{}\" id = \"{}\" href = \"{}\" />\n",
                                    format, to_id(image), image));
        }

        // and additional files too
        if let Ok(list) = self.book.options.get_paths_list("resources.files") {
            let data_path = Path::new(self.book.options.get_relative_path("resources.path").unwrap());
            for path in list {
                let format = self.get_format(&path);
                let path = data_path.join(&path);
                let path_str = path.to_str().unwrap();
                items.push_str(&format!("<item media-type = \"{}\" id = \"{}\" href = \"{}\" />\n",
                                        format,
                                        to_id(path_str),
                                        path_str));
            }
        }

        let template = mustache::compile_str(if self.book.options.get_i32("epub.version").unwrap() == 3 {epub3::OPF} else {OPF});
        let data = self.book.get_mapbuilder("none")
            .insert_str("optional", optional)
            .insert_str("items", items)
            .insert_str("itemrefs", itemrefs)
            .insert_str("date", date)
            .insert_str("uuid", uuid)
            .insert_str("cover_xhtml", cover_xhtml)
            .insert_str("coverref", coverref)
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML in content.opf was not valid utf-8")),
            Ok(res) => Ok(res)
        }
    }

    /// Render cover.xhtml
    fn render_cover(&mut self) -> Result<String> {
        if let Ok(cover) = self.book.options.get_path("cover") {
            let template = mustache::compile_str(if self.book.options.get_i32("epub.version").unwrap() == 3 {epub3::COVER} else {COVER});
            let data = self.book.get_mapbuilder("none")
                .insert_str("cover", self.html.handler.map_image(Cow::Owned(cover)).into_owned())
                .build();
            let mut res:Vec<u8> = vec!();
            template.render_data(&mut res, &data);
            match String::from_utf8(res) {
                Err(_) => Err(Error::Render("generated HTML for cover.xhtml was not utf-8 valid")),
                Ok(res) => Ok(res)
            }
        } else {
            panic!("Why is this method called if cover is None???");
        }
    }

    /// Render nav.xhtml
    fn render_nav(&self) -> Result<String> {
        let content = self.html.toc.render();
        
        let template = mustache::compile_str(if self.book.options.get_i32("epub.version").unwrap() == 3 {epub3::NAV} else {NAV});
        let data = self.book.get_mapbuilder("none")
            .insert_str("content", content)
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML in nav.xhtml was not utf-8 valid")),
            Ok(res) => Ok(res)
        }
    }

    /// Render a chapter
    pub fn render_chapter(&mut self, v: &[Token]) -> Result<String> {
        let mut content = String::new();
        let mut title = String::new();

        for token in v {
            content.push_str(&self.parse_token(&token, &mut title));
            self.html.render_side_notes(&mut content);
        }
        self.html.render_end_notes(&mut content);

        if title.is_empty() {
            if self.html.current_numbering >= 1 {
                let number = self.html.current_chapter[0] + 1;
                title = try!(self.book.get_header(number, ""));
            } 
        }
        self.toc.push(title.clone());

        let template = mustache::compile_str(try!(self.book.get_template("epub.template")).as_ref());
        let data = self.book.get_mapbuilder("none")
            .insert_str("content", content)
            .insert_str("chapter_title", title)
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML was not utf-8 valid")),
            Ok(res) => Ok(res)
        }
    }

    fn parse_token(&mut self, token: &Token, title: &mut String) -> String {
        match *token {
            Token::Header(n, ref vec) => {
                if n == 1 {
                    if self.html.current_hide || self.html.current_numbering == 0 {
                        if title.is_empty() {
                            *title = self.html.render_vec(vec);
                        } else {
                            self.book.logger.warning("EPUB: detected two chapter titles inside the same markdown file...");
                            self.book.logger.warning("EPUB: ...in a file where chapter titles are not even rendered.");
                        }
                    } else {
                        let res = self.book.get_header(self.html.current_chapter[0] + 1, &self.html.render_vec(vec));
                        let s = res.unwrap();
                        if title.is_empty() {
                            *title = s;
                        } else {
                            self.book.logger.warning("EPUB: detected two chapters inside the same markdown file.");
                            self.book.logger.warning(format!("EPUB: conflict between: {} and {}", title, s));
                        }
                    }
                }
                self.html.parse_token(token)
            },
            _ => self.html.parse_token(token)
        }
    }

    // Get the format of an image file, based on its extension
    fn get_format(&self, s: &str) -> String {
        let opt = guess_mime_type_opt(s);
        match opt {
            Some(s) => s.to_string(),
            None => {
                self.book.logger.warning(format!("EPUB: could not guess the format of {} based on extension. Assuming png.", s));
                String::from("png")
            }
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
