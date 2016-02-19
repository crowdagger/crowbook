use error::{Error,Result};
use token::Token;
use parser::Parser;
use html::HtmlRenderer;
use book::{Book,Number};

use zip;
use mustache;
use chrono;
use uuid;

use std::env;
use std::io;
use std::io::{Read,Write};
use std::fs::{self, File,DirBuilder};
use std::path::Path;
use std::process::Command;

/// Renderer for Epub
///
/// Uses part of the HTML renderer
pub struct EpubRenderer<'a> {
    book: &'a Book,
    current_numbering: bool,
    current_chapter: i32,
    toc: Vec<String>,
    html: HtmlRenderer<'a>,
}

impl<'a> EpubRenderer<'a> {
    /// Creates a new Epub renderer
    pub fn new(book: &'a Book) -> EpubRenderer<'a> {
         EpubRenderer {
            book: book,
            html: HtmlRenderer::new(book),
            current_numbering: book.numbering,
            current_chapter: 1,
            toc: vec!(),
        }
    }

    /// Render a book
    pub fn render_book(&mut self) -> Result<Vec<u8>> {
        let mut args:Vec<String> = vec!();
        let path = "/tmp/crowbook/";
        let meta_inf = "/tmp/crowbook/META-INF";
        DirBuilder::new()
            .recursive(true)
            .create(meta_inf).unwrap();

        
        let error_writing = |_| Error::Render("Error writing to file in zip");

        let buffer: Vec<u8> = vec!();
        let cursor = io::Cursor::new(buffer);
        let mut zip = zip::ZipWriter::new(cursor);

        // Write mimetype
        let mut f = File::create("/tmp/crowbook/mimetype").unwrap();
        f.write_all(b"application/epub+zip").unwrap();
        args.push(String::from("mimetype"));

        // Write chapters        
        let mut parser = Parser::new();
        if let Some(cleaner) = self.book.get_cleaner() {
            parser = parser.with_cleaner(cleaner);
        }

        let mut i = 0;
        for &(ref n, ref file) in &self.book.chapters {
            match n {
                &Number::Unnumbered => self.current_numbering = false,
                &Number::Default => self.current_numbering = self.book.numbering,
                &Number::Specified(n) => {
                    self.current_numbering = self.book.numbering;
                    self.current_chapter = n;
                }
            }
            let v = try!(parser.parse_file(file));
            let chapter = try!(self.render_chapter(&v));

            let mut f = File::create(&format!("{}{}", path, filenamer(i))).unwrap();
            args.push(filenamer(i));
            f.write_all(&chapter.as_bytes()).unwrap();
            i += 1;
        }
        
        // Write CSS file
        let mut f = File::create("/tmp/crowbook/stylesheet.css").unwrap();
        f.write_all(include_str!("../../templates/epub/stylesheet.css").as_bytes()).unwrap();
        args.push(String::from("stylesheet.css"));

        // Write titlepage
        let mut f = File::create("/tmp/crowbook/title_page.xhtml").unwrap();
        f.write_all(&try!(self.render_titlepage()).as_bytes()).unwrap();
        args.push(String::from("title_page.xhtml"));

        // Write file for ibook (why?)
        let mut f = File::create("/tmp/crowbook/META-INF/com.apple.ibooks.display-options.xml").unwrap();
        try!(f.write_all(include_str!("../../templates/epub/ibookstuff.xml").as_bytes())
             .map_err(&error_writing));
        args.push(String::from("META-INF/com.apple.ibooks.display-options.xml"));

        // Write container.xml
        let mut f = File::create("/tmp/crowbook/META-INF/container.xml").unwrap();
        try!(f.write_all(include_str!("../../templates/epub/container.xml").as_bytes())
             .map_err(&error_writing));
        args.push(String::from("META-INF/container.xml"));

        // Write nav.xhtml
        let mut f = File::create("/tmp/crowbook/nav.xhtml").unwrap();
        try!(f.write_all(&try!(self.render_nav()).as_bytes())
             .map_err(&error_writing));
        args.push(String::from("nav.xhtml"));

        // Write content.opf
        let mut f = File::create("/tmp/crowbook/content.opf").unwrap();
        try!(f.write_all(&try!(self.render_opf()).as_bytes())
             .map_err(&error_writing));
        args.push(String::from("content.opf"));

        // Write toc.ncx
        let mut f = File::create("/tmp/crowbook/toc.ncx").unwrap();
        try!(f.write_all(&try!(self.render_toc()).as_bytes())
             .map_err(&error_writing));
        args.push(String::from("toc.ncx"));

        // Write the cover (if needs be)
        if let Some(ref cover) = self.book.cover {
            let s: &str = &*cover;
            let mut f_target = File::create(&format!("{}{}", path, s)).unwrap();
            let mut f = try!(File::open(s).map_err(|_| Error::FileNotFound(String::from(s))));
            let mut content = vec!();
            try!(f.read_to_end(&mut content).map_err(|_| Error::Render("Error while reading cover file")));
            try!(f_target.write_all(&content)
                 .map_err(&error_writing));
            args.push(String::from(s));

            // also write cover.xhtml
            let mut f = File::create("/tmp/crowbook/cover.xhtml").unwrap();
            try!(f.write_all(&try!(self.render_cover()).as_bytes())
                 .map_err(&error_writing));
            args.push(String::from("cover.xhtml"));
        }

        let dir = env::current_dir().unwrap();
        env::set_current_dir(path).unwrap();


        
        let output = Command::new("zip")
            .arg("test.epub")
            .args(&args)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
        println!("{}", String::from_utf8(output.stdout).unwrap());
        env::set_current_dir(dir);
        fs::copy("/tmp/crowbook/test.epub", "test.epub").unwrap();
        fs::remove_dir_all(path).unwrap();
        Ok(vec!())
    }
    

    /// Render a book
    pub fn render_book_old(&mut self) -> Result<Vec<u8>> {
        let error_creating = |_| Error::Render("Error creating new file in zip");
        let error_writing = |_| Error::Render("Error writing to file in zip");

        let buffer: Vec<u8> = vec!();
        let cursor = io::Cursor::new(buffer);
        let mut zip = zip::ZipWriter::new(cursor);

        // Write mimetype
        try!(zip.start_file("mimetype", zip::CompressionMethod::Stored)
             .map_err(&error_creating));
        try!(zip.write(b"application/epub+zip")
             .map_err(&error_writing));

        // Write chapters        
        let mut parser = Parser::new();
        if let Some(cleaner) = self.book.get_cleaner() {
            parser = parser.with_cleaner(cleaner);
        }

        let mut i = 0;
        for &(ref n, ref file) in &self.book.chapters {
            match n {
                &Number::Unnumbered => self.current_numbering = false,
                &Number::Default => self.current_numbering = self.book.numbering,
                &Number::Specified(n) => {
                    self.current_numbering = self.book.numbering;
                    self.current_chapter = n;
                }
            }
            let v = try!(parser.parse_file(file));
            let chapter = try!(self.render_chapter(&v));
            
            try!(zip.start_file(filenamer(i), self.book.zip_compression)
                 .map_err(&error_creating));
            try!(zip.write(chapter.as_bytes())
                 .map_err(&error_writing));
            i += 1;
        }
        
        // Write CSS file
        try!(zip.start_file("stylesheet.css", self.book.zip_compression)
             .map_err(&error_creating));
        try!(zip.write(include_str!("../../templates/epub/stylesheet.css").as_bytes())
             .map_err(&error_writing));

        // Write titlepage
        try!(zip.start_file("title_page.xhtml", self.book.zip_compression)
             .map_err(&error_creating));
        try!(zip.write(try!(self.render_titlepage()).as_bytes())
             .map_err(&error_writing));

        // Write file for ibook (why?)
        try!(zip.start_file("META-INF/com.apple.ibooks.display-options.xml", self.book.zip_compression)
             .map_err(&error_creating));
        try!(zip.write(include_str!("../../templates/epub/ibookstuff.xml").as_bytes())
             .map_err(&error_writing));        

        // Write container.xml
        try!(zip.start_file("META-INF/container.xml", self.book.zip_compression)
             .map_err(&error_creating));
        try!(zip.write(include_str!("../../templates/epub/container.xml").as_bytes())
             .map_err(&error_writing));

        // Write nav.xhtml
        try!(zip.start_file("nav.xhtml", self.book.zip_compression)
             .map_err(&error_creating));
        try!(zip.write(try!(self.render_nav()).as_bytes())
             .map_err(&error_writing));

        // Write content.opf
        try!(zip.start_file("content.opf", self.book.zip_compression)
             .map_err(&error_creating));
        try!(zip.write(try!(self.render_opf()).as_bytes())
             .map_err(&error_writing));

        // Write toc.ncx
        try!(zip.start_file("toc.ncx", self.book.zip_compression)
             .map_err(&error_creating));
        try!(zip.write(try!(self.render_toc()).as_bytes())
             .map_err(&error_writing));

        // Write the cover (if needs be)
        if let Some(ref cover) = self.book.cover {
            let s: &str = &*cover;
            try!(zip.start_file(s, self.book.zip_compression)
                 .map_err(&error_creating));
            let mut f = try!(File::open(s).map_err(|_| Error::FileNotFound(String::from(s))));
            let mut content = vec!();
            try!(f.read_to_end(&mut content).map_err(|_| Error::Render("Error while reading cover file")));
            try!(zip.write(&content)
                 .map_err(&error_writing));

            // also write cover.xhtml
            try!(zip.start_file("cover.xhtml", self.book.zip_compression)
             .map_err(&error_creating));
            try!(zip.write(try!(self.render_cover()).as_bytes())
                 .map_err(&error_writing));
        }
        

        // Get back the buffer
        let buf = try!(zip.finish()
                       .map_err(|_| Error::Render("Error finishing zip file")));
        Ok(buf.into_inner())
    }

    /// Render the titlepgae
    fn render_titlepage(&self) -> Result<String> {
        let template = mustache::compile_str(include_str!("../../templates/epub/titlepage.xhtml"));
        let data = self.book.get_mapbuilder()
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
        let template = mustache::compile_str(include_str!("../../templates/epub/toc.ncx"));
        let data = self.book.get_mapbuilder()
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
    fn render_opf(&self) -> Result<String> {
        // Optional metadata
        let mut cover_xhtml = String::new();
        let mut optional = String::new();
        if let Some(ref s) = self.book.description {
            optional.push_str(&format!("<dc:description>{}</dc:description>\n", s));
        }
        if let Some(ref s) = self.book.subject {
            optional.push_str(&format!("<dc:subject>{}</dc:subject>\n", s));
        }
        if let Some(ref s) = self.book.cover {
            optional.push_str(&format!("<meta name = \"cover\" content = \"{}\" />\n", s));
            cover_xhtml.push_str(&format!("<reference type=\"cover\" title=\"Cover\" href=\"cover.xhtml\" />"));
        }

        // date
        let date = chrono::UTC::now().format("%Y-%m-%d").to_string();

        // uuid
        let uuid = uuid::Uuid::new_v4().to_urn_string();
        
        let mut items = String::new();
        let mut itemrefs = String::new();
        let mut coverref = String::new();
        if let Some(ref s) = self.book.cover {
            items.push_str("<item id = \"cover.xhtml\" href = \"cover.xhtml\" media-type = \"application/xhtml+xml\" />\n");
            coverref.push_str("<itemref idref = \"cover.xhtml\" />");
        }
        for n in 0..self.toc.len() {
            let filename = filenamer(n);
            items.push_str(&format!("<item id = \"{}\" href = \"{}\" media-type=\"application/xhtml+xml\" />\n",
                                    filename,
                                    filename));
            itemrefs.push_str(&format!("<itemref idref=\"{}\" />\n", filename));
        }
        // oh we must put cover in the manifest too
        if let Some(ref s) = self.book.cover {
            let format = if let Some(ext) = Path::new(s).extension() {
                if let Some(extension) = ext.to_str() {
                    match extension {
                        "png" => "png",
                        "jpg" | "jpeg" => "jpeg",
                        "gif" => "gif",
                        _ => {
                            println!("Warning: could not guess cover format based on extension. Assuming png.");
                            "png"
                        },
                    }
                } else {
                    println!("Warning: could not guess cover format based on extension. Assuming png.");
                    "png"
                }
            } else {
                println!("Warning: could not guess cover format based on extension. Assuming png.");
                "png"
            };
            items.push_str(&format!("<item media-type = \"image/{}\" id =\"{}\" href = \"{}\" />\n", format, s, s));
        }

        let template = mustache::compile_str(include_str!("../../templates/epub/content.opf"));
        let data = self.book.get_mapbuilder()
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
    fn render_cover(&self) -> Result<String> {
        if let Some(ref cover) = self.book.cover {
            let template = mustache::compile_str(include_str!("../../templates/epub/cover.xhtml"));
            let data = self.book.get_mapbuilder()
                .insert_str("cover", cover.clone())
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
        let mut content = String::new();
        for (x, ref title) in self.toc.iter().enumerate() {
            let link = filenamer(x);
            content.push_str(&format!("<li><a href = \"{}\">{}</a></li>\n",
                                 link,
                                 title));
        }           
        
        let template = mustache::compile_str(include_str!("../../templates/epub/nav.xhtml"));
        let data = self.book.get_mapbuilder()
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
        }
        if title.is_empty() {
            if self.current_numbering {
                self.current_chapter += 1;
                title = format!("Chapitre {}", self.current_chapter);
            } else {
                return Err(Error::Render("chapter without h1 tag is not OK if numbering is off"));
            }
        }
        self.toc.push(title.clone());

        let template = mustache::compile_str(include_str!("../../templates/epub/template.xhtml"));
        let data = self.book.get_mapbuilder()
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
                let s = if n == 1 && self.current_numbering {
                    let chapter = self.current_chapter;
                    self.current_chapter += 1;
                    self.book.get_header(chapter, &self.html.render_vec(vec)).unwrap()
                } else {
                    self.html.render_vec(vec)
                };
                if title.is_empty() {
                    *title = s.clone();
                } else {
                    println!("Warning: detected two chapters inside the same markdown file.");
                }
                format!("<h{}>{}</h{}>\n", n, s, n)
            },
            _ => self.html.parse_token(token)
        }
    }
}
    
fn filenamer(i: usize) -> String {
    format!("chapter_{:03}.xhtml", i)
}
