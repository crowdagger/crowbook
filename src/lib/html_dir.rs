use error::{Error,Result};
use html::HtmlRenderer;
use book::Book;
use number::Number;
use templates;

use mustache;

use std::io::{Read,Write};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

/// Multiple files HTML renderer
///
/// Renders HTML in a given directory.
#[derive(Debug)]
pub struct HtmlDirRenderer<'a> {
    book: &'a Book,
    html: HtmlRenderer<'a>,
}

impl<'a> HtmlDirRenderer<'a> {
    /// Creates a new HtmlDirRenderer
    pub fn new(book: &'a Book) -> HtmlDirRenderer<'a> {
        let mut html = HtmlRenderer::new(book);
        html.handler.set_images_mapping(true);
        HtmlDirRenderer {
            book: book,
            html: html,
        }
    }

    /// Render a book
    pub fn render_book(&mut self) -> Result<()> {
        // Add internal files to resource handler
        for (i, filename) in self.book.filenames.iter().enumerate() {
            self.html.handler.add_link(filename.clone(), filenamer(i));
        }

        // Create the directory 
        let dest_path = try!(self.book.options.get_path("output.html_dir"));
        match fs::metadata(&dest_path) {
            Ok(metadata) => if metadata.is_file() {
                return Err(Error::Render(format!("{} already exists and is not a directory", &dest_path)));
            } else if metadata.is_dir() {
                self.book.logger.warning(format!("{} already exists, deleting it", &dest_path));
                try!(fs::remove_dir_all(&dest_path)
                     .map_err(|e| Error::Render(format!("error deleting directory {}: {}", &dest_path, e))));
            },
            Err(_) => (),
        }
        try!(fs::DirBuilder::new()
             .recursive(true)
             .create(&dest_path)
             .map_err(|e| Error::Render(format!("could not create HTML directory {}:{}", &dest_path, e))));

        try!(self.write_css());
        try!(self.write_html());
        
        Ok(())
    }

    // Render each chapter and write them, and index.html too
    fn write_html(&mut self) -> Result<()> {
        let mut chapters = vec!();
        for (i, &(n, ref v)) in self.book.chapters.iter().enumerate() {
            self.html.filename = filenamer(i);
            // Todo: this part could be factorized between html, epub and html_dir
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

            let chapter = self.html.render_html(v);
            chapters.push(chapter);
        }
        let toc = self.html.toc.render();

        for (i, content) in chapters.into_iter().enumerate() {
            // Render each HTML document
            let data = self.book.get_mapbuilder("none")
                .insert_str("content", content)
                .insert_str("toc", toc.clone())
                .insert_bool(self.book.options.get_str("lang").unwrap(), true)
                .build();
            let template = mustache::compile_str(try!(self.book.get_template("html_dir.chapter.html")).as_ref());        
            let mut res = vec!();
            template.render_data(&mut res, &data);
            try!(self.write_file(&filenamer(i), &res));
        }

        // Render index.html and write it too
        let data = self.book.get_mapbuilder("none")
            .insert_str("content", "")
            .insert_str("toc", toc.clone())
            .insert_bool(self.book.options.get_str("lang").unwrap(), true)
            .build();
        let template = mustache::compile_str(try!(self.book.get_template("html_dir.index.html")).as_ref());        
        let mut res = vec!();
        template.render_data(&mut res, &data);
        try!(self.write_file("index.html", &res));
        
        Ok(())
    }

    // Render the CSS file and write it
    fn write_css(&self) -> Result<()> {
        // Render the CSS 
        let template_css = mustache::compile_str(try!(self.book.get_template("html_dir.css")).as_ref());
        let data = self.book.get_mapbuilder("none")
            .insert_bool(self.book.options.get_str("lang").unwrap(), true)
            .build();
        let mut res:Vec<u8> = vec!();
        template_css.render_data(&mut res, &data);
        let css = String::from_utf8_lossy(&res);

        // Write it
        self.write_file("stylesheet.css", css.as_bytes())
    }

    // Write content to a file
    fn write_file(&self, file: &str, content: &[u8]) -> Result<()> {
        let dest_path = PathBuf::from(&self.book.options.get_path("output.html_dir").unwrap());
        if dest_path.starts_with("..") {
            panic!("html dir is asked to create a file outside of its directory, no way!");
        }
        let dest_file = dest_path.join(file);
        let dest_dir = dest_file.parent().unwrap();
        if !fs::metadata(dest_dir).is_ok() { // dir does not exist, create it
            try!(fs::DirBuilder::new()
                 .recursive(true)
                 .create(&dest_dir)
                 .map_err(|e| Error::Render(format!("could not create directory in {}:{}", dest_dir.display(), e))));
        }
        let mut f = try!(File::create(&dest_file)
                         .map_err(|e| Error::Render(format!("could not create file {}:{}", dest_file.display(), e))));
        f.write_all(content)
            .map_err(|e| Error::Render(format!("could not write to file {}:{}", dest_file.display(), e)))
    }
}

/// Generate a file name given an int   
fn filenamer(i: usize) -> String {
    format!("chapter_{:03}.html", i)
}
