use error::{Error,Result};
use html::HtmlRenderer;
use book::Book;
use number::Number;
use token::Token;
use templates::html;
use resource_handler::ResourceHandler;

use mustache;

use std::io::{Read,Write};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::borrow::Cow;


/// Multiple files HTML renderer
///
/// Renders HTML in a given directory.
pub struct HtmlDirRenderer<'a> {
    book: &'a Book,
    html: HtmlRenderer<'a>,
}

impl<'a> HtmlDirRenderer<'a> {
    /// Creates a new HtmlDirRenderer
    pub fn new(book: &'a Book) -> HtmlDirRenderer<'a> {
        let mut html = HtmlRenderer::new(book);
        html.handler.set_images_mapping(true);
        html.handler.set_base64(false);
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

        // Write CSS
        try!(self.write_css());
        // Write print.css
        try!(self.write_file("print.css",
                             &self.book.get_template("html.print_css").unwrap().as_bytes()));
        // Write index.html and chapter_xxx.html
        try!(self.write_html());
        // Write menu.svg
        try!(self.write_file("menu.svg", html::MENU_SVG));
        
        // Write all images (including cover)
        let images_path = PathBuf::from(&self.book.options.get_path("resources.base_path.images").unwrap());
        for (source, dest) in self.html.handler.images_mapping() {
            let mut f = try!(File::open(images_path.join(source)).map_err(|_| Error::FileNotFound(source.to_owned())));
            let mut content = vec!();
            try!(f.read_to_end(&mut content).map_err(|e| Error::Render(format!("error while reading image file {}: {}", source, e))));
            try!(self.write_file(dest, &content));
        }

        // Write additional files
        if let Ok(list) = self.book.options.get_paths_list("resources.files") {
            let files_path = self.book.options.get_path("resources.base_path.files").unwrap();
            let data_path = Path::new(self.book.options.get_relative_path("resources.out_path").unwrap());
            let list = try!(ResourceHandler::get_files(list, &files_path));
            for path in list {
                let abs_path = Path::new(&files_path).join(&path);
                let mut f = try!(File::open(&abs_path)
                                 .map_err(|_| Error::FileNotFound(abs_path.to_string_lossy().into_owned())));
                let mut content = vec!();
                try!(f.read_to_end(&mut content).map_err(|e| Error::Render(format!("error while reading resource file: {}", e))));
                try!(self.write_file(data_path.join(&path).to_str().unwrap(), &content));
            }
        }
        
        Ok(())
    }

    // Render each chapter and write them, and index.html too
    fn write_html(&mut self) -> Result<()> {
        let mut chapters = vec!();
        let mut titles = vec!();
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

            let mut title = String::new();
            for token in v {
                match *token {
                    Token::Header(1, ref vec) => {
                        if self.html.current_hide || self.html.current_numbering == 0 {
                            title = self.html.render_vec(vec);
                        } else {
                            title = try!(self.book.get_header(self.html.current_chapter[0] + 1,
                                                              &self.html.render_vec(vec)));
                        }
                        break;
                    },
                    _ => {
                        continue;
                    }
                }
            }
            titles.push(title);
            
            let chapter = self.html.render_html(v);
            chapters.push(chapter);
        }
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
            let data = self.book.get_mapbuilder("none")
                .insert_str("content", content)
                .insert_str("chapter_title", format!("{} – {}",
                                             self.book.options.get_str("title").unwrap(),
                                             titles[i]))
                .insert_str("toc", toc.clone())
                .insert_str("prev_chapter", prev_chapter)
                .insert_str("next_chapter", next_chapter)
                .insert_str("script", self.book.get_template("html_dir.script").unwrap())
                .insert_bool(self.book.options.get_str("lang").unwrap(), true)
                .build();
            let template = mustache::compile_str(try!(self.book.get_template("html_dir.chapter.html")).as_ref());        
            let mut res = vec!();
            template.render_data(&mut res, &data);
            try!(self.write_file(&filenamer(i), &res));
        }

        let mut content = if let Ok(cover) = self.book.options.get_path("cover") {
            format!("<div id = \"cover\">
  <img class = \"cover\" alt = \"{}\" src = \"{}\" />
</div>",
                    self.book.options.get_str("title").unwrap(),
                    self.html.handler.map_image(Cow::Owned(cover)).as_ref())
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
        let data = self.book.get_mapbuilder("none")
            .insert_str("content", content)
            .insert_str("toc", toc.clone())
            .insert_str("script", self.book.get_template("html_dir.script").unwrap())
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
