use error::{Error,Result};
use cleaner::{Cleaner, French};
use bookoption::BookOption;
use parser::Parser;
use token::Token;
use epub::EpubRenderer;
use html::HtmlRenderer;
use latex::LatexRenderer;
use odt::OdtRenderer;
use templates::{epub, html, epub3, latex};
use escape;
use number::Number;

use std::env;
use std::fs::File;
use std::io::{self, Write,Read};
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::collections::HashMap;

use mustache;
use mustache::MapBuilder;

static OPTIONS:&'static str = "
# Metadata
author:str:Anonymous                # The author of the book
title:str:Untitled                  # The title of the book
lang:str:en                         # The language of the book
subject:str                         # Subject of the book (used for EPUB metadata)
description:str                     # Description of the book (used for EPUB metadata)
cover:path                          # File name of the cover of the book 
# Output options
output.epub:path                    # Output file name for EPUB rendering
output.html:path                    # Output file name for HTML rendering
output.tex:path                     # Output file name for LaTeX rendering
output.pdf:path                     # Output file name for PDF rendering
output.odt:path                     # Output file name for ODT rendering


# Misc options
zip.command:str:zip                 # Command to use to zip files (for EPUB/ODT)
numbering:int:1                     # The  maximum heading levels to number (0: no numbering, 1: only chapters, ..., 6: all)
display_toc:bool:false              # If true, display a table of content in the document
toc_name:str:Table of contents      # Name of the table of contents if toc is displayed in line
autoclean:bool:true                 # Toggles cleaning of input markdown (not used for LaTeX)
verbose:bool:false                  # Toggle verbose mode
side_notes:bool:false               # Display footnotes as side notes in HTML/Epub
temp_dir:path:                      # Path where to create a temporary directory (default: uses result from Rust's std::env::temp_dir())
numbering_template:str:{{number}}. {{title}} # Format of numbered titles
nb_char:char:' '                    # The non-breaking character to use for autoclean when lang is set to fr

# HTML options
html.template:path                  # Path of an HTML template
html.css:path                       # Path of a stylesheet to use with HTML rendering

# EPUB options
epub.version:int:2                  # The EPUB version to generate
epub.css:path                       # Path of a stylesheet to use with EPUB rendering
epub.template:path                  # Path of an epub template for chapter

# LaTeX options
tex.links_as_footnotes:bool:true    # If set to true, will add foontotes to URL of links in LaTeX/PDF output
tex.command:str:pdflatex            # LaTeX flavour to use for generating PDF
tex.template:path                   # Path of a LaTeX template file
";


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
/// book.set_option("author", "Joan Doe");
/// book.set_option("title", "An untitled book");
/// book.set_option("lang", "en");
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

    /// book options
    options: HashMap<String, BookOption>,
    valid_bools: Vec<&'static str>,
    valid_chars: Vec<&'static str>,
    valid_strings: Vec<&'static str>,
    valid_paths: Vec<&'static str>,
    valid_ints: Vec<&'static str>,

    /// root path of the book
    root: PathBuf,
}

impl Book {
    /// Creates a new, empty `Book` with default options
    pub fn new() -> Book {
        let mut book = Book {
            chapters: vec!(),
            options: HashMap::new(),
            valid_bools:vec!(),
            valid_chars:vec!(),
            valid_ints:vec!(),
            valid_strings:vec!(),
            valid_paths:vec!(),
            root: PathBuf::new(),
        };
        for (_, key, option_type, default_value) in Book::options_to_vec() {
            if key.is_none() {
                continue;
            }
            let key = key.unwrap();
            match option_type.unwrap() {
                "str" => book.valid_strings.push(key),
                "bool" => book.valid_bools.push(key),
                "int" => book.valid_ints.push(key),
                "char" => book.valid_chars.push(key),
                "path" => book.valid_paths.push(key),
                _ => panic!(format!("Ill-formatted OPTIONS string: unrecognized type '{}'", option_type.unwrap())),
            }
            if key == "temp_dir" {
                book.set_option(key, &env::temp_dir().to_string_lossy()).unwrap();
                continue;
            }
            if let Some(value) = default_value {
                book.set_option(key, value).unwrap();
            }
        }
        book
    }

    /// Creates a new book from a file
    ///
    /// Note that this method also changes the current directory to the one of this file
    ///
    /// # Arguments
    /// * `filename`: the path of file to load.
    /// * `verbose`: sets the book to verbose mode even if the file's doesn't specify it
    ///    or specifies `verbose: false`
    pub fn new_from_file(filename: &str, verbose: bool) -> Result<Book> {
        let mut book = Book::new();
        if verbose {
            book.set_option("verbose", "true").unwrap();
        }
        
        let path = Path::new(filename);
        let mut f = try!(File::open(&path).map_err(|_| Error::FileNotFound(String::from(filename))));
        // Set book path to book's directory
        if let Some(parent) = path.parent() {
            book.root = parent.to_owned();
        }

        let mut s = String::new();
        try!(f.read_to_string(&mut s).map_err(|_| Error::ConfigParser("file contains invalid UTF-8, could not parse it",
                                                                      filename.to_owned())));

        try!(book.set_from_config(&s));
        Ok(book)
    }
    

    /// Sets an option
    ///
    /// # Arguments
    /// * `key`: the identifier of the option, e.g.: "author"
    /// * `value`: the value of the option as a string
    ///
    /// **Returns** an error either if `key` is not a valid option or if the
    /// value is not of the right type
    ///
    /// # Examples
    /// ```
    /// use crowbook::Book;
    /// let mut book = Book::new();
    /// book.set_option("author", "Joan Doe").unwrap(); // ok
    /// book.set_option("numbering", "2").unwrap(); // sets numbering to chapters and subsections
    /// let result = book.set_option("autor", "John Smith"); 
    /// assert!(result.is_err()); // error: "author" was mispelled "autor"
    ///
    /// let result = book.set_option("numbering", "foo"); 
    /// assert!(result.is_err()); // error: numbering must be an int
    /// ```
    pub fn set_option(&mut self, key: &str, value: &str) -> Result<()> {
        if self.valid_strings.contains(&key) {
            self.options.insert(key.to_owned(), BookOption::String(value.to_owned()));
            Ok(())
        } else if self.valid_paths.contains(&key) {
            self.options.insert(key.to_owned(), BookOption::Path(value.to_owned()));
            Ok(())
        } else if self.valid_chars.contains(&key) {
            let words: Vec<_> = value.trim().split('\'').collect();
            if words.len() != 3 {
                return Err(Error::ConfigParser("could not parse char", String::from(value)));
            }
            let chars: Vec<_> = words[1].chars().collect();
            if chars.len() != 1 {
                return Err(Error::ConfigParser("could not parse char", String::from(value)));
            }
            self.options.insert(key.to_owned(), BookOption::Char(chars[0]));
            Ok(())
        } else if self.valid_bools.contains(&key) {
            match value.parse::<bool>() {
                Ok(b) => {
                    self.options.insert(key.to_owned(), BookOption::Bool(b));
                    ()
                },
                Err(_) => return Err(Error::ConfigParser("could not parse bool", format!("{}:{}", key, value))),
            }
            Ok(())
        } else if self.valid_ints.contains(&key) {
            match value.parse::<i32>() {
                Ok(i) => {
                    self.options.insert(key.to_owned(), BookOption::Int(i));
                }
                Err(_) => return Err(Error::ConfigParser("could not parse int", format!("{}:{}", key, value))),
            }
            Ok(())
        } else {
            Err(Error::ConfigParser("unrecognized key", String::from(key)))
        }
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
                    try!(self.set_option(&prev_key, prev_value.trim()));
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
                    _ => try!(self.set_option(key, value)),
                }
            }
        }
        if multiline {
            try!(self.set_option(&prev_key, &prev_value));
        }

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
        self.debug(&format!("Parsing chapter: {}...", file));
        let mut parser = Parser::new();
        let file = self.root.join(file);
        let v = try!(parser.parse_file(file));
        self.chapters.push((number, v));
        Ok(())
    }


    /// Returns a description of all options valid to pass to a book.
    ///
    /// # arguments
    /// * `md`: whether the output should be formatted in Markdown
    pub fn description(md: bool) -> String {
        let mut out = String::new();
        let mut previous_is_comment = true;
        for (comment, key, o_type, default) in Book::options_to_vec() {
            if key.is_none() {
                if !previous_is_comment {
                    out.push_str("\n");
                    previous_is_comment = true;
                }
                out.push_str(&format!("### {} ###\n", comment));
                continue;
            }
            previous_is_comment = false;
            let o_type = match o_type.unwrap() {
                "bool" => "boolean",
                "int" => "integer",
                "char" => "char",
                "str" => "string",
                "path" => "path",
                _ => unreachable!()
            };
            let def = if let Some(value) = default {
                value
            } else {
                "not set"
            };
            if md {
                out.push_str(&format!("- **`{}`**
    - **type**: {}
    - **default value**: `{}`
    - {}\n", key.unwrap(), o_type, def, comment));
            } else {
                out.push_str(&format!("- {} (type: {}) (default: {}) {}\n", key.unwrap(), o_type, def,comment));
            }
        }
        out
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
        Ok(())
    }

    
    /// Prints a message to stderr
    pub fn println(&self, s:&str) {
        writeln!(&mut io::stderr(), "{}", s).unwrap();
    }

    /// Prints a message to stderr if verbose is set to true
    pub fn debug(&self, s:&str) {
        if self.get_bool("verbose").unwrap() {
            writeln!(&mut io::stderr(), "{}", s).unwrap();
        }
    }



    /// Either clean a string or does nothing,
    /// according to book `lang` and `autoclean` options
    pub fn clean(&self, mut text:String) -> String  {
        // todo: not very efficient!
        if self.get_bool("autoclean").unwrap() {
            let lang = self.get_str("lang").unwrap().to_lowercase();
            let cleaner: Box<Cleaner> = if lang.starts_with("fr") {
                Box::new(French::new(self.get_char("nb_char").unwrap()))
            } else {
                Box::new(())
            };
            cleaner.clean(&mut text);
        }
        text
    }
    
    /// Returns the string corresponding to a number, title, and the numbering template
    pub fn get_header(&self, n: i32, title: &str) -> Result<String> {
        let template = mustache::compile_str(self.get_str("numbering_template").unwrap());
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

    /// get an option
    pub fn get_option(&self, key: &str) -> Result<&BookOption> {
        self.options.get(key).ok_or(Error::InvalidOption(format!("option {} is not present", key)))
    }

    /// Gets a string option 
    pub fn get_str(&self, key: &str) -> Result<&str> {
        try!(self.get_option(key)).as_str()
    }

    /// Get a path option
    ///
    /// Adds book's root path before it
    pub fn get_path(&self, key: &str) -> Result<String> {
        let path: &str = try!(try!(self.get_option(key)).as_path());
        let new_path:PathBuf = self.root.join(path);
        if let Some(path) = new_path.to_str() {
            Ok(path.to_owned())
        } else {
            Err(Error::BookOption(format!("'{}''s path contains invalid UTF-8 code", key)))
        }
    }

    /// Get a path option
    ///
    /// Don't add book's root path before it
    pub fn get_relative_path(&self, key: &str) -> Result<&str> {
        try!(self.get_option(key)).as_path()
    }

    /// gets a bool option
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        try!(self.get_option(key)).as_bool()
    }

    /// gets a char option
    pub fn get_char(&self, key: &str) -> Result<char> {
        try!(self.get_option(key)).as_char()
    }

    /// gets an int  option
    pub fn get_i32(&self, key: &str) -> Result<i32> {
        try!(self.get_option(key)).as_i32()
    }

    
    /// Render book to pdf according to book options
    pub fn render_pdf(&self) -> Result<()> {
        self.debug("Attempting to generate pdf...");
        let mut latex = LatexRenderer::new(&self);
        let result = try!(latex.render_pdf());
        self.debug(&result);
        self.println(&format!("Successfully generated pdf file: {}", self.get_path("output.pdf").unwrap()));
        Ok(())
    }

    /// Render book to epub according to book options
    pub fn render_epub(&self) -> Result<()> {
        self.debug("Attempting to generate epub...");
        let mut epub = EpubRenderer::new(&self);
        let result = try!(epub.render_book());
        self.debug(&result);
        self.println(&format!("Successfully generated epub file: {}", self.get_path("output.epub").unwrap()));
        Ok(())
    }

        /// Render book to odt according to book options
    pub fn render_odt(&self) -> Result<()> {
        self.debug("Attempting to generate Odt...");
        let mut odt = OdtRenderer::new(&self);
        let result = try!(odt.render_book());
        self.debug(&result);
        self.println(&format!("Successfully generated odt file: {}", self.get_path("output.odt").unwrap()));
        Ok(())
    }

    /// Render book to html according to book options
    pub fn render_html<T: Write>(&self, f: &mut T) -> Result<()> {
        self.debug("Attempting to generate HTML...");
        let mut html = HtmlRenderer::new(&self);
        let result = try!(html.render_book());
        try!(f.write_all(&result.as_bytes()).map_err(|_| Error::Render("problem when writing to HTML file")));
        self.println("Successfully generated HTML");
        Ok(())
    }

    /// Render book to pdf according to book options
    pub fn render_tex<T:Write>(&self, f: &mut T) -> Result<()> {
        self.debug("Attempting to generate LaTeX...");

        let mut latex = LatexRenderer::new(&self);
        let result = try!(latex.render_book());
        try!(f.write_all(&result.as_bytes()).map_err(|_| Error::Render("problem when writing to LaTeX file")));
        self.println("Successfully generated LaTeX");
        Ok(())
    }
        
    /// Generates output files acccording to book options
    pub fn render_all(&self) -> Result<()> {
        let mut did_some_stuff = false;
        
        if self.get_option("output.epub").is_ok() {
            did_some_stuff = true;
            try!(self.render_epub());
        }

        if let Ok(file) = self.get_path("output.html") {
            did_some_stuff = true;
            let mut f = try!(File::create(file).map_err(|_| Error::Render("could not create HTML file")));
            try!(self.render_html(&mut f));
        }
        if let Ok(file) = self.get_path("output.tex") {
            did_some_stuff = true;
            let mut f = try!(File::create(file).map_err(|_| Error::Render("could not create LaTeX file")));
            try!(self.render_tex(&mut f));
        }
        if self.get_option("output.pdf").is_ok() {
            did_some_stuff = true;
            try!(self.render_pdf());
        }
        
        if self.get_option("output.odt").is_ok() {
            did_some_stuff = true;
            try!(self.render_odt());
        }
        if !did_some_stuff {
            self.println("Warning: generated no file because no output file speficied. Add output_{{format}} to your config file.");
        }
        Ok(())
    }

    
    /// Returns the template (default or modified version)
    pub fn get_template(&self, template: &str) -> Result<Cow<'static, str>> {
        let (option, fallback) = match template {
            "epub.css" => (self.get_str("epub.css"), epub::CSS),
            "epub.template" => (self.get_str("epub.template"),
                                if try!(self.get_i32("epub.version")) == 3 {epub3::TEMPLATE} else {epub::TEMPLATE}),
            "html.css" => (self.get_str("html.css"), html::CSS),
            "html.template" => (self.get_str("html.template"), html::TEMPLATE),
            "tex.template" => (self.get_str("tex.template"), latex::TEMPLATE),
            _ => return Err(Error::ConfigParser("invalid template", template.to_owned())),
        };
        if let Ok (s) = option {
            let mut f = try!(File::open(s).map_err(|_| Error::FileNotFound(s.to_owned())));
            let mut res = String::new();
            try!(f.read_to_string(&mut res)
                 .map_err(|_| Error::ConfigParser("file could not be read", s.to_owned())));
            Ok(Cow::Owned(res))
        } else {
            Ok(Cow::Borrowed(fallback))
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
            .insert_str("author", f(self.get_str("author").unwrap()))
            .insert_str("title", f(&self.get_str("title").unwrap()))
            .insert_str("lang", self.get_str("lang").unwrap().to_owned())
    }

    /// OPTIONS to a vec of tuples (comment, key, type, default value)
    fn options_to_vec() -> Vec<(&'static str, Option<&'static str>,
                                Option<&'static str>, Option<&'static str>)> {
        let mut out = vec!();
        for line in OPTIONS.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('#') {
                out.push((&line[1..], None, None, None));
                continue;
            }
            let v:Vec<_> = line.split('#').collect();
            let content = v[0];
            let comment = v[1];
            let v:Vec<_> = content.split(':').collect();
            let key = Some(v[0].trim());
            let option_type = Some(v[1].trim());
            let default_value = if v.len() > 2 {
                Some(v[2].trim())
            } else {
                None
            };
            out.push((comment, key, option_type, default_value));
        }
        out
    }

}
