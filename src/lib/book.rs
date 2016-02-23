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

use std::fs::File;
use std::io::{self, Write,Read};
use std::env;
use std::path::Path;
use std::borrow::Cow;
use std::collections::HashMap;

use mustache;
use mustache::MapBuilder;

/// Numbering for a given chapter
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Hidden, // chapter's title is hidden
    Unnumbered, // chapter is not numbered
    Default, // chapter follows books numbering, number is given automatically
    Specified(i32), //chapter number set to specified number
}

static OPTIONS:&'static str = "
# Metadata
author:str:Anonymous                # The author of the book
title:str:Untitled                  # The title of the book
lang:str:en                         # The language of the book
subject:str                         # Subject of the book (used for EPUB metadata)
description:str                     # Description of the book (used for EPUB metadata)
cover:str                           # File name of the cover of the book 
# Output options
output.epub:str                     # Output file name for EPUB rendering
output.html:str                     # Output file name for HTML rendering
output.tex:str                      # Output file name for LaTeX rendering
output.pdf:str                      # Output file name for PDF rendering
output.odt:str                      # Output file name for ODT rendering


# Misc options
numbering:int:1                     # The  maximum heading levels to number (0: no numbering, 1: only chapters, ..., 6: all)
autoclean:bool:true                 # Toggles cleaning of input markdown (not used for LaTeX)
verbose:bool:false                  # Toggle verbose mode
side_notes:bool:false               # Display footnotes as side notes in HTML/Epub
nb_char:char:' '                    # The non-breaking character to use for autoclean when lang is set to fr
temp_dir:str:.                      # Path where to create a temporary directory
numbering_template:str:{{number}}. {{title}} # Format of numbered titles

# HTML options
html.template:str                   # Path of an HTML template
html.css:str                        # Path of a stylesheet to use with HTML rendering

# EPUB options
epub.version:int:2                  # The EPUB version to generate
epub.css:str                        # Path of a stylesheet to use with EPUB rendering
epub.template:str                   # Path of an epub template for chapter

# LaTeX options
tex.links_as_footnotes:bool:true    # If set to true, will add foontotes to URL of links in LaTeX/PDF output
tex.command:str:pdflatex            # LaTeX flavour to use for generating PDF
tex.template:str                    # Path of a LaTeX template file
";


// Configuration of the book
#[derive(Debug)]
pub struct Book {
    // internal structure
    pub chapters: Vec<(Number, Vec<Token>)>, 

    /// book options
    options: HashMap<String, BookOption>,
    valid_bools: Vec<&'static str>,
    valid_chars: Vec<&'static str>,
    valid_strings: Vec<&'static str>,
    valid_ints: Vec<&'static str>,
}



impl Book {
    /// Returns a description for all options
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

    /// OPTIONS str to a vec of tuples (comment, key, type, default value)
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
    
    /// Creates a new Book with default options
    pub fn new() -> Book {
        let mut book = Book {
            chapters: vec!(),
            options: HashMap::new(),
            valid_bools:vec!(),
            valid_chars:vec!(),
            valid_ints:vec!(),
            valid_strings:vec!(),
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
                _ => panic!(format!("Ill-formatted OPTIONS string: unrecognized type '{}'", option_type.unwrap())),
            }
            if let Some(value) = default_value {
                book.set_option(key, value).unwrap();
            }
        }
        book
    }

    /// Prints to stderr
    pub fn println(&self, s:&str) {
        writeln!(&mut io::stderr(), "{}", s).unwrap();
    }

    /// Prints to stderr but only if verbose is set to true
    pub fn debug(&self, s:&str) {
        if self.get_bool("verbose").unwrap() {
            writeln!(&mut io::stderr(), "{}", s).unwrap();
        }
    }

    /// Creates a new book from a file
    ///
    /// This method also changes the current directory to the one of this file
    pub fn new_from_file(filename: &str, verbose: bool) -> Result<Book> {
        let path = Path::new(filename);
        let mut f = try!(File::open(&path).map_err(|_| Error::FileNotFound(String::from(filename))));

        // change current directory
        if let Some(parent) = path.parent() {
            if !parent.to_string_lossy().is_empty() {
                if !env::set_current_dir(&parent).is_ok() {
                    return Err(Error::ConfigParser("could not change current directory to the one of the config file",
                                                   format!("{}", parent.display())));
                }
            }
        }

        
        let mut s = String::new();

        try!(f.read_to_string(&mut s).map_err(|_| Error::ConfigParser("file contains invalid UTF-8, could not parse it",
                                                                      String::from(filename))));
        let mut book = Book::new();
        if verbose {
            book.set_option("verbose", "true").unwrap();
        }
        try!(book.set_from_config(&s));
        Ok(book)
    }

    /// Returns a MapBuilder, to be used (and completed) for templating
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

    /// Either clean a string or does nothing
    pub fn clean(&self, mut text:String) -> String  {
        if let Some(cleaner) = self.get_cleaner() {
            cleaner.clean(&mut text)
        }
        text
    }
    
    /// Return a Box<Cleaner> corresponding to the appropriate cleaning method, or None
    pub fn get_cleaner(&self) -> Option<Box<Cleaner>> {
        if self.get_bool("autoclean").unwrap() {
            let lang = self.get_str("lang").unwrap().to_lowercase();
            if lang.starts_with("fr") {
                Some(Box::new(French::new(self.get_char("nb_char").unwrap())))
            } else {
                Some(Box::new(()))
            }
        } else {
            None
        }
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

    /// gets a string option as str
    pub fn get_str(&self, key: &str) -> Result<&str> {
        try!(self.get_option(key)).as_str()
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

    /// Sets an option
    pub fn set_option(&mut self, key: &str, value: &str) -> Result<()> {
        if self.valid_strings.contains(&key) {
            self.options.insert(key.to_owned(), BookOption::String(value.to_owned()));
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

        for line in s.lines() {
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
                try!(self.set_option(key, value));
            }
        }

        Ok(())
    }
    
    /// Render book to pdf according to book options
    pub fn render_pdf(&self) -> Result<()> {
        self.debug("Attempting to generate pdf...");
        let mut latex = LatexRenderer::new(&self);
        let result = try!(latex.render_pdf());
        self.debug(&result);
        self.println(&format!("Successfully generated pdf file: {}", self.get_str("output.pdf").unwrap()));
        Ok(())
    }

    /// Render book to epub according to book options
    pub fn render_epub(&self) -> Result<()> {
        self.debug("Attempting to generate epub...");
        let mut epub = EpubRenderer::new(&self);
        let result = try!(epub.render_book());
        self.debug(&result);
        self.println(&format!("Successfully generated epub file: {}", self.get_str("output.epub").unwrap()));
        Ok(())
    }

        /// Render book to odt according to book options
    pub fn render_odt(&self) -> Result<()> {
        self.debug("Attempting to generate Odt...");
        let mut odt = OdtRenderer::new(&self);
        let result = try!(odt.render_book());
        self.debug(&result);
        self.println(&format!("Successfully generated odt file: {}", self.get_str("output.odt").unwrap()));
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

        if let Ok(file) = self.get_str("output.html") {
            did_some_stuff = true;
            let mut f = try!(File::create(file).map_err(|_| Error::Render("could not create HTML file")));
            try!(self.render_html(&mut f));
        }
        if let Ok(file) = self.get_str("output.tex") {
            did_some_stuff = true;
            let mut f = try!(File::create(file).map_err(|_| Error::Render("could not create LaTeX file")));
            try!(self.render_tex(&mut f));
        }
        if self.get_str("output.pdf").is_ok() {
            did_some_stuff = true;
            try!(self.render_pdf());
        }
        
        if self.get_str("output.odt").is_ok() {
            did_some_stuff = true;
            try!(self.render_odt());
        }
        if !did_some_stuff {
            self.println("Warning: generated no file because no output file speficied. Add output_{{format}} to your config file.");
        }
        Ok(())
    }

    
    /// File: location of the file for this chapter
    pub fn add_chapter(&mut self, number: Number, file: &str) -> Result<()> {
        self.debug(&format!("Parsing chapter: {}...", file));
        let mut parser = Parser::new();
        let v = try!(parser.parse_file(file));
        self.chapters.push((number, v));
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
            _ => panic!("")//return Err(Error::ConfigParser("invalid template", template.to_owned())),
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
}
