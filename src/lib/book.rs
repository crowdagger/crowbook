use error::{Error,Result};
use cleaner::{Cleaner, French};
use bookoption::BookOptions;
use parser::Parser;
use token::Token;
use epub::EpubRenderer;
use html::HtmlRenderer;
use latex::LatexRenderer;
use odt::OdtRenderer;
use templates::{epub, html, epub3, latex};
use escape;
use number::Number;
use resource_handler::ResourceHandler;
use logger::{Logger, InfoLevel};

use std::fs::File;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use std::borrow::Cow;

use mustache;
use mustache::MapBuilder;


/// A Book.
///
/// Probably the central structure for of Crowbook, as it is the one
/// that calls the other ones.
///
/// It has the tasks of loading a configuration file, loading chapters
/// and using `Parser`to parse them, and then calling various renderers
/// (`HtmlRendrer`, `LatexRenderer`, `EpubRenderer` and/or `OdtRenderer`)
/// to convert the AST into documents.
///
/// # Examples
///
/// ```
/// use crowbook::{Book, Number};
/// // Create an empty book
/// let mut book = Book::new();
//
/// // Set some options
/// book.options.set("author", "Joan Doe");
/// book.options.set("title", "An untitled book");
/// book.options.set("lang", "en");
///
/// // Add content to the book
/// book.add_chapter_as_str(Number::Default, "# The beginning#\nBla, bla, bla").unwrap();
///
/// // Render the book as html to stdout
/// book.render_html(&mut std::io::stdout()).unwrap();
/// ```
#[derive(Debug)]
pub struct Book {
    /// Internal structure. You should not accesss this directly except if
    /// you are writing a new renderer.
    pub chapters: Vec<(Number, Vec<Token>)>,
    /// A list of the filenames of the chapters
    pub filenames: Vec<String>,

    /// Options of the book
    pub options: BookOptions,

    /// Root path of the book
    pub root: PathBuf,

    /// Logger
    pub logger: Logger,
}

impl Book {
    /// Creates a new, empty `Book` with default options
    pub fn new() -> Book {
        Book {
            chapters: vec!(),
            filenames: vec!(),
            root: PathBuf::new(),
            options: BookOptions::new(),
            logger: Logger::new(InfoLevel::Info),
        }
    }

    /// Creates a new book from a file
    ///
    /// # Arguments
    /// * `filename`: the path of file to load. The directory of this file is used as
    ///   a "root" directory for all paths referenced in books, whether chapter files,
    ///   templates, cover images, and so on.
    /// * `verbose`: sets the book to verbose mode even if the file's doesn't specify it
    ///    or specifies `verbose: false`.
    pub fn new_from_file(filename: &str, verbose: bool) -> Result<Book> {
        let mut book = Book::new();
        if verbose {
            book.options.set("verbose", "true").unwrap();
            book.logger.set_verbosity(InfoLevel::Warning);
        }
        
        let path = Path::new(filename);
        let mut f = try!(File::open(&path).map_err(|_| Error::FileNotFound(String::from(filename))));
        // Set book path to book's directory
        if let Some(parent) = path.parent() {
            book.root = parent.to_owned();
            book.options.root = book.root.clone();
        }

        let mut s = String::new();
        try!(f.read_to_string(&mut s).map_err(|_| Error::ConfigParser("file contains invalid UTF-8, could not parse it",
                                                                      filename.to_owned())));

        try!(book.set_from_config(&s));
        Ok(book)
    }
    
    /// Sets options and load chapters according to configuration file
    ///
    /// A line with "option: value" sets the option to value
    /// + chapter_name.md adds the (default numbered) chapter
    /// - chapter_name.md adds the (unnumbered) chapter
    /// 3. chapter_name.md adds the (custom numbered) chapter
    pub fn set_from_config(&mut self, s: &str) -> Result<()> {
        fn get_filename(s: &str) -> Result<&str> {
            let words:Vec<&str> = (&s[1..]).split_whitespace().collect();
            if words.len() > 1 {
                return Err(Error::ConfigParser("chapter filenames must not contain whitespace", String::from(s)));
            } else if words.len() < 1 {
                return Err(Error::ConfigParser("no chapter name specified", String::from(s)));
            }
            Ok(words[0])
        }

        let mut multiline = false;
        let mut join_new_line = false;
        let mut prev_key = String::new();
        let mut prev_value = String::new();

        for line in s.lines() {
            // If we are multiline mode, we already have a key and a (building) value
            if multiline {
                if line.starts_with(' ') {
                    // multiline continues
                    prev_value.push_str(line.trim());
                    if join_new_line {
                        prev_value.push_str("\n");
                    } else {
                        prev_value.push_str(" ");
                    }
                    continue;
                } else {
                    // end multiline
                    try!(self.options.set(&prev_key, prev_value.trim()));
                    multiline = false;
                }
            }

            
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('-') {
                //unnumbered chapter
                let file = try!(get_filename(line));
                try!(self.add_chapter(Number::Unnumbered, file));
            } else if line.starts_with('+') {
                //nunmbered chapter
                let file = try!(get_filename(line));
                try!(self.add_chapter(Number::Default, file));
            } else if line.starts_with('!') {
                // hidden chapter
                let file = try!(get_filename(line));
                try!(self.add_chapter(Number::Hidden, file));
            } else if line.starts_with(|c: char| c.is_digit(10)) {
                // chapter with specific number
                let parts:Vec<_> = line.splitn(2, |c: char| c == '.' || c == ':' || c == '+').collect();
                if parts.len() != 2 {
                    return Err(Error::ConfigParser("ill-formatted line specifying chapter number", String::from(line)));
                } else {
                    let file = try!(get_filename(parts[1]));
                    let number = try!(parts[0].parse::<i32>().map_err(|_| Error::ConfigParser("Error parsing integer", String::from(line))));
                    try!(self.add_chapter(Number::Specified(number), file));
                }
            } else {
                // standard case: "option: value"
                let parts:Vec<_> = line.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err(Error::ConfigParser("option setting must be of the form option: value", String::from(line)));
                }
                let key = parts[0].trim();
                let value = parts[1].trim();
                match value {
                    ">" | "|" => { // multiline string
                        multiline = true;
                        join_new_line = value == "|";
                        prev_key = key.to_owned();
                        prev_value = String::new();
                    },
                    _ => try!(self.options.set(key, value)),
                }
            }
        }
        if multiline {
            try!(self.options.set(&prev_key, &prev_value));
        }

        if self.options.get_bool("verbose") == Ok(true) {
            if self.logger.verbosity() >= InfoLevel::Warning {
                self.logger.set_verbosity(InfoLevel::Warning);
            }
        }

        Ok(())
    }

    
    /// Generates output files acccording to book options
    pub fn render_all(&self) -> Result<()> {
        let mut did_some_stuff = false;
        
        if self.options.get("output.epub").is_ok() {
            did_some_stuff = true;
            try!(self.render_epub());
        }

        if let Ok(file) = self.options.get_path("output.html") {
            did_some_stuff = true;
            let mut f = try!(File::create(file).map_err(|_| Error::Render("could not create HTML file")));
            try!(self.render_html(&mut f));
        }
        if let Ok(file) = self.options.get_path("output.tex") {
            did_some_stuff = true;
            let mut f = try!(File::create(file).map_err(|_| Error::Render("could not create LaTeX file")));
            try!(self.render_tex(&mut f));
        }
        if self.options.get("output.pdf").is_ok() {
            did_some_stuff = true;
            try!(self.render_pdf());
        }
        
        if self.options.get("output.odt").is_ok() {
            did_some_stuff = true;
            try!(self.render_odt());
        }
        if !did_some_stuff {
            self.logger.warning("Warning: generated no file because no output file speficied. Add output_{{format}} to your config file.");
        }
        Ok(())
    }


    
    /// Render book to pdf according to book options
    pub fn render_pdf(&self) -> Result<()> {
        self.logger.debug("Attempting to generate pdf...");
        let mut latex = LatexRenderer::new(&self);
        let result = try!(latex.render_pdf());
        self.logger.debug(result);
        self.logger.info(format!("Successfully generated pdf file: {}", self.options.get_path("output.pdf").unwrap()));
        Ok(())
    }

    /// Render book to epub according to book options
    pub fn render_epub(&self) -> Result<()> {
        self.logger.debug("Attempting to generate epub...");
        let mut epub = EpubRenderer::new(&self);
        let result = try!(epub.render_book());
        self.logger.debug(&result);
        self.logger.info(&format!("Successfully generated epub file: {}", self.options.get_path("output.epub").unwrap()));
        Ok(())
    }

        /// Render book to odt according to book options
    pub fn render_odt(&self) -> Result<()> {
        self.logger.debug("Attempting to generate Odt...");
        let mut odt = OdtRenderer::new(&self);
        let result = try!(odt.render_book());
        self.logger.debug(&result);
        self.logger.info(format!("Successfully generated odt file: {}", self.options.get_path("output.odt").unwrap()));
        Ok(())
    }

    /// Render book to html according to book options
    pub fn render_html<T: Write>(&self, f: &mut T) -> Result<()> {
        self.logger.debug("Attempting to generate HTML...");
        let mut html = HtmlRenderer::new(&self);
        let result = try!(html.render_book());
        try!(f.write_all(&result.as_bytes()).map_err(|_| Error::Render("problem when writing to HTML file")));
        self.logger.info("Successfully generated HTML");
        Ok(())
    }

    /// Render book to pdf according to book options
    pub fn render_tex<T:Write>(&self, f: &mut T) -> Result<()> {
        self.logger.debug("Attempting to generate LaTeX...");

        let mut latex = LatexRenderer::new(&self);
        let result = try!(latex.render_book());
        try!(f.write_all(&result.as_bytes()).map_err(|_| Error::Render("problem when writing to LaTeX file")));
        self.logger.info("Successfully generated LaTeX");
        Ok(())
    }
        
    

    /// Adds a chapter, as a file name, to the book
    ///
    /// `Book` will then parse the file and store the AST (i.e., a vector
    /// of `Token`s).
    ///
    /// # Arguments
    /// * `number`: specifies if the chapter must be numbered, not numbered, or if its title
    ///   must be hidden. See `Number`.
    /// * `file`: path of the file for this chapter
    ///
    /// **Returns** an error if `file` does not exist, could not be read, of if there was
    /// some error parsing it.
    pub fn add_chapter(&mut self, number: Number, file: &str) -> Result<()> {
        self.logger.debug(&format!("Parsing chapter: {}...", file));
        
        // add file to the list of file names
        self.filenames.push(file.to_owned());
        
        // parse the file
        let mut parser = Parser::new();
        let mut v = try!(parser.parse_file(self.root.join(file)));
        
        // transform the AST to make local links and images relative to `book` directory
        let offset = Path::new(file).parent().unwrap();
        if offset.starts_with("..") {
            self.logger.warning(format!("Warning: book contains chapter '{}' in a directory above the book file, this might cause problems", file));
        }

        // add offset 
        ResourceHandler::add_offset(offset, &mut v);
                              
        self.chapters.push((number, v));
        Ok(())
    }

    /// Adds a chapter, as a string, to the book
    ///
    /// `Book` will then parse the string and store the AST (i.e., a vector
    /// of `Token`s).
    ///
    /// # Arguments
    /// * `number`: specifies if the chapter must be numbered, not numbered, or if its title
    ///   must be hidden. See `Number`.
    /// * `content`: the content of the chapter.
    ///
    /// **Returns** an error if there was some errror parsing `content`.
    pub fn add_chapter_as_str(&mut self, number: Number, content: &str) -> Result<()> {
        let mut parser = Parser::new();
        let v = try!(parser.parse(content));
        self.chapters.push((number, v));
        self.filenames.push(String::new());
        Ok(())
    }


    /// Either clean a string or does nothing,
    /// according to book `lang` and `autoclean` options
    pub fn clean(&self, mut text:String) -> String  {
        // todo: not very efficient!
        if self.options.get_bool("autoclean").unwrap() {
            let lang = self.options.get_str("lang").unwrap().to_lowercase();
            let cleaner: Box<Cleaner> = if lang.starts_with("fr") {
                Box::new(French::new(self.options.get_char("nb_char").unwrap()))
            } else {
                Box::new(())
            };
            cleaner.clean(&mut text);
        }
        text
    }

    
    
    /// Returns the template (default or modified version)
    pub fn get_template(&self, template: &str) -> Result<Cow<'static, str>> {
        let (option, fallback) = match template {
            "epub.css" => (self.options.get_path("epub.css"), epub::CSS),
            "epub.template" => (self.options.get_path("epub.template"),
                                if try!(self.options.get_i32("epub.version")) == 3 {epub3::TEMPLATE} else {epub::TEMPLATE}),
            "html.css" => (self.options.get_path("html.css"), html::CSS),
            "html.template" => (self.options.get_path("html.template"), html::TEMPLATE),
            "tex.template" => (self.options.get_path("tex.template"), latex::TEMPLATE),
            _ => return Err(Error::ConfigParser("invalid template", template.to_owned())),
        };
        if let Ok (ref s) = option {
            let mut f = try!(File::open(s).map_err(|_| Error::FileNotFound(s.to_owned())));
            let mut res = String::new();
            try!(f.read_to_string(&mut res)
                 .map_err(|_| Error::ConfigParser("file could not be read", s.to_owned())));
            Ok(Cow::Owned(res))
        } else {
            Ok(Cow::Borrowed(fallback))
        }
    }


    /// Returns the string corresponding to a number, title, and the numbering template
    pub fn get_header(&self, n: i32, title: &str) -> Result<String> {
        let template = mustache::compile_str(self.options.get_str("numbering_template").unwrap());
        let data = MapBuilder::new()
            .insert_str("title", String::from(title))
            .insert_str("number", format!("{}", n))
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("header generated by mustache was not valid utf-8")),
            Ok(res) => Ok(res)
        }
    }

    /// Returns a `MapBuilder` (used by `Mustache` for templating), to be used (and completed)
    /// by renderers. It fills it with the followings strings, corresponding to the matching
    /// `Book` options:
    ///
    /// * "author"
    /// * "title"
    /// * "lang"
    pub fn get_mapbuilder(&self, format: &str) -> MapBuilder {
        fn clone(x:&str) -> String {
            x.to_owned()
        }
        let f:fn(&str)->String = match format {
            "none" => clone,
            "html" => escape::escape_html,
            "tex" => escape::escape_tex,
            _ => panic!("get mapbuilder called with invalid escape format")
        };
        MapBuilder::new()
            .insert_str("author", f(self.options.get_str("author").unwrap()))
            .insert_str("title", f(&self.options.get_str("title").unwrap()))
            .insert_str("lang", self.options.get_str("lang").unwrap().to_owned())
    }


}
