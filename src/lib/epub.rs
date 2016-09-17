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

use error::{Error,Result,Source};
use token::Token;
use html::HtmlRenderer;
use book::Book;
use zipper::Zipper;
use templates::epub::*;
use templates::epub3;
use resource_handler::ResourceHandler;
use renderer::Renderer;

use mustache;
use chrono;
use uuid;

use std::io::{Read};
use std::convert::{AsRef,AsMut};
use std::fs;
use std::fs::File;
use std::path::{Path};
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
            toc: vec!(),
            chapter_title: String::new(),
        }
    }

    /// Render a book
    pub fn render_book(&mut self) -> Result<String> {
        for (i, filename) in self.html.book.filenames.iter().enumerate() {
            self.html.handler.add_link(filename.clone(), filenamer(i));
        }

        let mut zipper = try!(Zipper::new(&self.html.book.options.get_path("temp_dir").unwrap()));
        
        // Write mimetype
        try!(zipper.write("mimetype", b"application/epub+zip", true));

        // Write cover.xhtml (if needs be)
        if self.html.book.options.get_path("cover").is_ok() {
            try!(zipper.write("cover.xhtml", &try!(self.render_cover()).as_bytes(), true));
        }

        // Write chapters        
        for (i, &(n, ref v)) in self.html.book.chapters.iter().enumerate() {
            self.html.chapter_config(i, n);
            self.html.filename = filenamer(i);
            let chapter = try!(self.render_chapter(v));

            try!(zipper.write(&filenamer(i), &chapter.as_bytes(), true));
        }
        self.html.source = Source::empty();
        
        // Render the CSS file and write it
        let template_css = mustache::compile_str(try!(self.html.book.get_template("epub.css")).as_ref());
        let data = self.html.book.get_mapbuilder("none")
            .insert_bool(self.html.book.options.get_str("lang").unwrap(), true)
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

        // Write all images (including cover)
        for (source, dest) in self.html.handler.images_mapping() {
            let mut f = try!(File::open(source).map_err(|_| Error::FileNotFound(self.html.source.clone(),
                                                                                "image or cover".to_owned(),
                                                                                source.to_owned())));
            let mut content = vec!();
            try!(f.read_to_end(&mut content).map_err(|e| Error::Render(format!("error while reading image file: {}", e))));
            try!(zipper.write(dest, &content, true));
        }

        // Write additional resources
        if let Ok(list) = self.html.book.options.get_paths_list("resources.files") {
            let base_path_files = self.html.book.options.get_path("resources.base_path.files").unwrap();
            let list = try!(ResourceHandler::get_files(list, &base_path_files));
            let data_path = Path::new(try!(self.html.book.options.get_relative_path("resources.out_path")));
            for path in list{
                let abs_path = Path::new(&base_path_files).join(&path);
                let mut f = try!(File::open(&abs_path)
                                 .map_err(|_| Error::FileNotFound(self.html.book.source.clone(),
                                                                  "additional resource from resources.files".to_owned(),
                                                                  abs_path.to_string_lossy().into_owned())));
                let mut content = vec!();
                try!(f.read_to_end(&mut content).map_err(|e| Error::Render(format!("error while reading resource file: {}", e))));
                try!(zipper.write(data_path.join(&path).to_str().unwrap(), &content, true));
            }
        }
        
        if let Ok(epub_file) = self.html.book.options.get_path("output.epub") {
            let res = try!(zipper.generate_epub(self.html.book.options.get_str("zip.command").unwrap(), &epub_file));
            Ok(res)
        } else {
            Err(Error::Render(String::from("no output epub file specified in book config")))
        }
    }
    
    /// Render the titlepgae
    fn render_titlepage(&self) -> Result<String> {
        let template = mustache::compile_str(if self.html.book.options.get_i32("epub.version").unwrap() == 3 {epub3::TITLE} else {TITLE});
        let data = self.html.book.get_mapbuilder("none")
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => panic!("generated HTML in titlepage was not utf-8 valid"),
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
        let data = self.html.book.get_mapbuilder("none")
            .insert_str("nav_points", nav_points)
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => panic!("generated HTML in toc.ncx was not valid utf-8"),
            Ok(res) => Ok(res)
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
                                       try!(self.html.handler.map_image(&self.html.source,
                                                                        Cow::Borrowed(s)))));
            cover_xhtml.push_str("<reference type=\"cover\" title=\"Cover\" href=\"cover.xhtml\" />");
        }

        // date
        let date = chrono::UTC::now().format("%Y-%m-%dT%H:%M:%SZ");

        // uuid
        let uuid = uuid::Uuid::new_v4().urn().to_string();
        
        let mut items = String::new();
        let mut itemrefs = String::new();
        let mut coverref = String::new();
        if self.html.book.options.get("cover").is_ok() {
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
        if let Ok(list) = self.html.book.options.get_paths_list("resources.files") {
            let list = try!(ResourceHandler::get_files(list, &self.html.book.options.get_path("resources.base_path.files").unwrap()));
            let data_path = Path::new(self.html.book.options.get_relative_path("resources.out_path").unwrap());
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

        let template = mustache::compile_str(if self.html.book.options.get_i32("epub.version").unwrap() == 3 {epub3::OPF} else {OPF});
        let data = self.html.book.get_mapbuilder("none")
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
            Err(_) => panic!("generated HTML in content.opf was not valid utf-8"),
            Ok(res) => Ok(res)
        }
    }

    /// Render cover.xhtml
    fn render_cover(&mut self) -> Result<String> {
        if let Ok(cover) = self.html.book.options.get_path("cover") {
            // Check that cover can be found
            if fs::metadata(&cover).is_err() {
                return Err(Error::FileNotFound(self.html.book.source.clone(),
                                               "cover".to_owned(),
                                               cover));

            }
            let template = mustache::compile_str(if self.html.book.options.get_i32("epub.version").unwrap() == 3 {epub3::COVER} else {COVER});
            let data = self.html.book.get_mapbuilder("none")
                .insert_str("cover", try!(self.html.handler.map_image(&self.html.source,
                                                                      Cow::Owned(cover))).into_owned())
                .build();
            let mut res:Vec<u8> = vec!();
            template.render_data(&mut res, &data);
            match String::from_utf8(res) {
                Err(_) => panic!("generated HTML for cover.xhtml was not utf-8 valid"),
                Ok(res) => Ok(res)
            }
        } else {
            panic!("Why is this method called if cover is None???");
        }
    }

    /// Render nav.xhtml
    fn render_nav(&self) -> Result<String> {
        let content = self.html.toc.render();
        
        let template = mustache::compile_str(if self.html.book.options.get_i32("epub.version").unwrap() == 3 {epub3::NAV} else {NAV});
        let data = self.html.book.get_mapbuilder("none")
            .insert_str("content", content)
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => panic!("generated HTML in nav.xhtml was not utf-8 valid"),
            Ok(res) => Ok(res)
        }
    }

    /// Render a chapter
    pub fn render_chapter(&mut self, v: &[Token]) -> Result<String> {
        let mut content = String::new();

        for token in v {
            let res = try!(self.render_token(&token));
            content.push_str(&res);
            self.html.render_side_notes(&mut content);
        }
        self.html.render_end_notes(&mut content);

        if self.chapter_title.is_empty() && self.html.current_numbering >= 1 {
            let number = self.html.current_chapter[0] + 1;
            self.chapter_title = try!(self.html.book.get_header(number, ""));
        }
        self.toc.push(self.chapter_title.clone());
        
        let template = mustache::compile_str(try!(self.html.book.get_template("epub.template")).as_ref());
        let data = self.html.book.get_mapbuilder("none")
            .insert_str("content", content)
            .insert_str("chapter_title", mem::replace(&mut self.chapter_title, String::new()))
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => panic!("generated HTML was not utf-8 valid"),
            Ok(res) => Ok(res)
        }
    }

    /// Renders the header section of the book, finding the title of the chapter
    fn find_title(&mut self, vec: &[Token]) -> Result<()> {
        if self.html.current_hide || self.html.current_numbering == 0 {
            if self.chapter_title.is_empty() {
                self.chapter_title = try!(self.html.render_vec(vec));
            } else {
                self.html.book.logger.warning("EPUB: detected two chapter titles inside the same markdown file...");
                self.html.book.logger.warning("EPUB: ...in a file where chapter titles are not even rendered.");
            }
        } else {
            let res = self.html.book.get_header(self.html.current_chapter[0] + 1,
                                           &try!(self.html.render_vec(vec)));
            let s = res.unwrap();
            if self.chapter_title.is_empty() {
                self.chapter_title = s;
            } else {
                self.html.book.logger.warning("EPUB: detected two chapters inside the same markdown file.");
                self.html.book.logger.warning(format!("EPUB: conflict between: {} and {}", self.chapter_title, s));
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
                self.html.book.logger.error(format!("EPUB: could not guess the format of {} based on extension. Assuming png.", s));
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
    pub fn static_render_token<T>(this: &mut T, token: &Token) -> Result<String>
    where T: AsMut<EpubRenderer<'a>>+AsRef<EpubRenderer<'a>> + Renderer {
        match *token {
            Token::Header(1, ref vec) => {
                try!(this.as_mut().find_title(vec));
                HtmlRenderer::static_render_token(this.as_mut(), token)
            },
            _ => HtmlRenderer::static_render_token(this.as_mut(), token)
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

impl<'a> AsRef<HtmlRenderer<'a>> for EpubRenderer<'a> {
    fn as_ref(&self) -> &HtmlRenderer<'a> {
        &self.html
    }
}

impl<'a> AsMut<HtmlRenderer<'a>> for EpubRenderer<'a> {
    fn as_mut(&mut self) -> &mut HtmlRenderer<'a> {
        &mut self.html
    }
}

impl<'a> AsRef<EpubRenderer<'a>> for EpubRenderer<'a> {
    fn as_ref(&self) -> &EpubRenderer<'a> {
        self
    }
}

impl<'a> AsMut<EpubRenderer<'a>> for EpubRenderer<'a> {
    fn as_mut(&mut self) -> &mut EpubRenderer<'a> {
        self
    }
}


impl<'a> Renderer for EpubRenderer<'a> {
    fn render_token(&mut self, token: &Token) -> Result<String> {
        EpubRenderer::static_render_token(self, token)
    }
}
