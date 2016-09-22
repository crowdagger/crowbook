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

use error::{Error,Result, Source};
use html::HtmlRenderer;
use book::{Book, compile_str};
use token::Token;
use templates::img;
use resource_handler;
use renderer::Renderer;
use parser::Parser;

use std::io::{Read,Write};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::borrow::Cow;
use std::convert::{AsRef, AsMut};


/// Multiple files HTML renderer
///
/// Renders HTML in a given directory.
pub struct HtmlDirRenderer<'a> {
    html: HtmlRenderer<'a>,
}

impl<'a> HtmlDirRenderer<'a> {
    /// Creates a new HtmlDirRenderer
    pub fn new(book: &'a Book) -> HtmlDirRenderer<'a> {
        let mut html = HtmlRenderer::new(book);
        html.handler.set_images_mapping(true);
        html.handler.set_base64(false);
        HtmlDirRenderer {
            html: html,
        }
    }

    /// Render a book
    pub fn render_book(&mut self) -> Result<()> {
        // Add internal files to resource handler
        for (i, filename) in self.html.book.filenames.iter().enumerate() {
            self.html.handler.add_link(filename.clone(), filenamer(i));
        }

        // Create the directory 
        let dest_path = try!(self.html.book.options.get_path("output.html_dir"));
        match fs::metadata(&dest_path) {
            Ok(metadata) => if metadata.is_file() {
                return Err(Error::render(&self.html.book.source,
                                         format!("{} already exists and is not a directory", &dest_path)));
            } else if metadata.is_dir() {
                self.html.book.logger.warning(format!("{} already exists, deleting it", &dest_path));
                try!(fs::remove_dir_all(&dest_path)
                     .map_err(|e| Error::render(&self.html.book.source,
                                                format!("error deleting directory {}: {}", &dest_path, e))));
            },
            Err(_) => (),
        }
        try!(fs::DirBuilder::new()
             .recursive(true)
             .create(&dest_path)
             .map_err(|e| Error::render(&self.html.book.source,
                                        format!("could not create HTML directory {}:{}", &dest_path, e))));

        // Write CSS
        try!(self.write_css());
        // Write print.css
        try!(self.write_file("print.css",
                             &self.html.book.get_template("html.css.print").unwrap().as_bytes()));
        // Write index.html and chapter_xxx.html
        try!(self.write_html());
        // Write menu.svg
        try!(self.write_file("menu.svg", img::MENU_SVG));

        // Write highlight files if they are needed
        if self.html.book.options.get_bool("html.highlight_code") == Ok(true) {
            try!(self.write_file("highlight.js", self.html.book.get_template("html.highlight.js").unwrap().as_bytes()));
            try!(self.write_file("highlight.css", self.html.book.get_template("html.highlight.css").unwrap().as_bytes()));
        }
        
        // Write all images (including cover)
        for (source, dest) in self.html.handler.images_mapping() {
            let mut f = try!(File::open(source).map_err(|_| Error::file_not_found(&self.html.book.source,
                                                                                  "image or cover",
                                                                                  source.clone())));
            let mut content = vec!();
            try!(f.read_to_end(&mut content).map_err(|e| Error::render(&self.html.book.source,
                                                                       format!("error while reading image file {}: {}", source, e))));
            try!(self.write_file(dest, &content));
        }

        // Write additional files
        if let Ok(list) = self.html.book.options.get_paths_list("resources.files") {
            let files_path = self.html.book.options.get_path("resources.base_path.files").unwrap();
            let data_path = Path::new(self.html.book.options.get_relative_path("resources.out_path").unwrap());
            let list = try!(resource_handler::get_files(list, &files_path));
            for path in list {
                let abs_path = Path::new(&files_path).join(&path);
                let mut f = try!(File::open(&abs_path)
                                 .map_err(|_| Error::file_not_found(&self.html.book.source,
                                                                    "additional resource from resources.files",
                                                                    abs_path.to_string_lossy().into_owned())));
                let mut content = vec!();
                try!(f.read_to_end(&mut content).map_err(|e| Error::render(&self.html.book.source,
                                                                           format!("error while reading resource file: {}", e))));
                try!(self.write_file(data_path.join(&path).to_str().unwrap(), &content));
            }
        }
        
        Ok(())
    }

    // Render each chapter and write them, and index.html too
    fn write_html(&mut self) -> Result<()> {
        let mut chapters = vec!();
        let mut titles = vec!();
        for (i, &(n, ref v)) in self.html.book.chapters.iter().enumerate() {
            self.html.chapter_config(i, n, filenamer(i));
            let mut title = String::new();
            for token in v {
                match *token {
                    Token::Header(1, ref vec) => {
                        if self.html.current_hide || self.html.current_numbering == 0 {
                            title = try!(self.html.render_vec(vec));
                        } else {
                            title = try!(self.html.book.get_header(self.html.current_chapter[0] + 1,
                                                              &try!(self.html.render_vec(vec))));
                        }
                        break;
                    },
                    _ => {
                        continue;
                    }
                }
            }
            titles.push(title);
            
            let chapter = HtmlRenderer::render_html(self, v);
            chapters.push(chapter);
        }
        self.html.source = Source::empty();
        let toc = self.html.toc.render();

        for (i, content) in chapters.into_iter().enumerate() {
            let prev_chapter = if i > 0 {
                format!("<p class = \"prev_chapter\">
  <a href = \"{}\">
    « {}
  </a>
</p>",
                        filenamer(i-1),
                        titles[i-1])
            } else {
                String::new()
            };

            let next_chapter = if i < titles.len() - 1 {
                format!("<p class = \"next_chapter\">
  <a href = \"{}\">
    {} »
  </a>
</p>",
                        filenamer(i+1),
                        titles[i+1])
            } else {
                String::new()
            };

            
            // Render each HTML document
            let mut mapbuilder = try!(self.html.book.get_metadata(|s| self.render_vec(&try!(Parser::new().parse_inline(s)))))
                .insert_str("content", try!(content))
                .insert_str("chapter_title", format!("{} – {}",
                                             self.html.book.options.get_str("title").unwrap(),
                                             titles[i]))
                .insert_str("toc", toc.clone())
                .insert_str("prev_chapter", prev_chapter)
                .insert_str("next_chapter", next_chapter)
                .insert_str("footer", try!(self.html.get_footer()))
                .insert_str("top", try!(self.html.get_header()))
                .insert_str("script", self.html.book.get_template("html.js").unwrap())
                .insert_bool(self.html.book.options.get_str("lang").unwrap(), true);
            
            if self.html.book.options.get_bool("html.highlight_code").unwrap() == true {
                mapbuilder = mapbuilder.insert_bool("highlight_code", true);
            }
            let data = mapbuilder.build();
            let template = try!(compile_str(try!(self.html.book.get_template("html_dir.chapter.html")).as_ref(),
                                            &self.html.book.source,
                                            "could not compile template 'html_dir.chapter.html"));
            let mut res = vec!();
            template.render_data(&mut res, &data);
            try!(self.write_file(&filenamer(i), &res));
        }

        let mut content = if let Ok(cover) = self.html.book.options.get_path("cover") {
            // checks first that cover exists
            if fs::metadata(&cover).is_err() {
                return Err(Error::file_not_found(&self.html.book.source,
                                               "cover",
                                                 cover));
                
            }
            format!("<div id = \"cover\">
  <img class = \"cover\" alt = \"{}\" src = \"{}\" />
</div>",
                    self.html.book.options.get_str("title").unwrap(),
                    try!(self.html.handler.map_image(&self.html.book.source,
                                                     Cow::Owned(cover))).as_ref())
        } else {
            String::new()
        };
        if titles.len() > 1 {
            content.push_str(&format!("<p class = \"next_chapter\">
  <a href = \"{}\">
    {} »
  </a>
</p>",
                        filenamer(0),
                        titles[0]));
        }
        // Render index.html and write it too
        let mut mapbuilder = try!(self.html.book.get_metadata(|s| self.render_vec(&try!(Parser::new().parse_inline(s)))))
            .insert_str("content", content)
            .insert_str("top", try!(self.html.get_header()))
            .insert_str("footer", try!(self.html.get_footer()))
            .insert_str("toc", toc.clone())
            .insert_str("script", self.html.book.get_template("html.js").unwrap())
            .insert_bool(self.html.book.options.get_str("lang").unwrap(), true);
        if self.html.book.options.get_bool("html.highlight_code").unwrap() == true {
            mapbuilder = mapbuilder.insert_bool("highlight_code", true);
        }
        let data = mapbuilder.build();
        let template = try!(compile_str(try!(self.html.book.get_template("html_dir.index.html")).as_ref(),
                                        &self.html.book.source,
                                        "could not compile template 'html_dir.index.html"));
        let mut res = vec!();
        template.render_data(&mut res, &data);
        try!(self.write_file("index.html", &res));
        
        Ok(())
    }

    // Render the CSS file and write it
    fn write_css(&self) -> Result<()> {
        // Render the CSS 
        let template_css = try!(compile_str(try!(self.html.book.get_template("html.css")).as_ref(),
                                            &self.html.book.source,
                                            "could not compile template 'html.css"));
        let data = try!(self.html.book.get_metadata(|s| Ok(s.to_owned())))
            .build();
        let mut res:Vec<u8> = vec!();
        template_css.render_data(&mut res, &data);
        let css = String::from_utf8_lossy(&res);

        // Write it
        self.write_file("stylesheet.css", css.as_bytes())
    }

    // Write content to a file
    fn write_file(&self, file: &str, content: &[u8]) -> Result<()> {
        let dest_path = PathBuf::from(&self.html.book.options.get_path("output.html_dir").unwrap());
        if dest_path.starts_with("..") {
            panic!("html dir is asked to create a file outside of its directory, no way!");
        }
        let dest_file = dest_path.join(file);
        let dest_dir = dest_file.parent().unwrap();
        if !fs::metadata(dest_dir).is_ok() { // dir does not exist, create it
            try!(fs::DirBuilder::new()
                 .recursive(true)
                 .create(&dest_dir)
                 .map_err(|e| Error::render(&self.html.book.source,
                                            format!("could not create directory in {}:{}", dest_dir.display(), e))));
        }
        let mut f = try!(File::create(&dest_file)
                         .map_err(|e| Error::render(&self.html.book.source,
                                                   format!("could not create file {}:{}", dest_file.display(), e))));
        f.write_all(content)
            .map_err(|e| Error::render(&self.html.book.source,
                                       format!("could not write to file {}:{}", dest_file.display(), e)))
    }
}

/// Generate a file name given an int   
fn filenamer(i: usize) -> String {
    format!("chapter_{:03}.html", i)
}

derive_html!{HtmlDirRenderer<'a>, HtmlRenderer::static_render_token}
