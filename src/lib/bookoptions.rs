use error::{Error,Result};
use bookoption::BookOption;

use yaml_rust::{Yaml, YamlLoader};
use std::collections::HashMap;
use std::path::PathBuf;
use std::env;

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
base_path:path                      # By default, links and images are relative to the Markdown file. If this is set, it will be to this path.
base_path.links:path                # Set base path but only for links. Useless if base_path is set.
base_path.images:path               # Set base path but only for images. Useless if base_path is set.
zip.command:str:zip                 # Command to use to zip files (for EPUB/ODT)
numbering:int:1                     # The  maximum heading levels to number (0: no numbering, 1: only chapters, ..., 6: all)
display_toc:bool:false              # If true, display a table of content in the document
toc_name:str:Table of contents      # Name of the table of contents if toc is displayed in line
autoclean:bool:true                 # Toggles cleaning of input markdown (not used for LaTeX)
verbose:bool:false                  # If set to true, print warnings in Markdown processing
side_notes:bool:false               # Display footnotes as side notes in HTML/Epub
temp_dir:path:                      # Path where to create a temporary directory (default: uses result from Rust's std::env::temp_dir())
numbering_template:str:\"{{number}}. {{title}}\" # Format of numbered titles
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



/// Contains the options of a book.
#[derive(Debug)]
pub struct BookOptions {
    options: HashMap<String, BookOption>,
    valid_bools: Vec<&'static str>,
    valid_chars: Vec<&'static str>,
    valid_strings: Vec<&'static str>,
    valid_paths: Vec<&'static str>,
    valid_ints: Vec<&'static str>,

    /// Root path of the book (unnecessary copy :/)
    pub root: PathBuf,
}

impl BookOptions {
    /// Creates a new BookOptions struct from the default compliled string
    pub fn new() -> BookOptions {
        let mut options = BookOptions {
            options: HashMap::new(),
            valid_bools:vec!(),
            valid_chars:vec!(),
            valid_ints:vec!(),
            valid_strings:vec!(),
            valid_paths:vec!(),
            root: PathBuf::new(),
        };
            
        for (_, key, option_type, default_value) in Self::options_to_vec() {
            if key.is_none() {
                continue;
            }
            let key = key.unwrap();
            match option_type.unwrap() {
                "str" => options.valid_strings.push(key),
                "bool" => options.valid_bools.push(key),
                "int" => options.valid_ints.push(key),
                "char" => options.valid_chars.push(key),
                "path" => options.valid_paths.push(key),
                _ => panic!(format!("Ill-formatted OPTIONS string: unrecognized type '{}'", option_type.unwrap())),
            }
            if key == "temp_dir" {
                // "temp_dir" has a special default value that depends on the environment
                options.set(key, &env::temp_dir().to_string_lossy()).unwrap();
                continue;
            }
            if let Some(value) = default_value {
                options.set(key, value).unwrap();
            }
        }
        options
    }

    /// Sets an option from a Yaml tuple
    ///
    /// # Arguments
    /// * `key`: the identifier of the option, must be Yaml::String(_)
    /// * `value`: the value of the option
    ///
    /// **Returns** an error if `key` is not a String, not a valid option, of if
    /// the value is not of the right type.
    pub fn set_yaml(&mut self, key: Yaml, value: Yaml) -> Result<Option<BookOption>> {
        let key = if let Yaml::String(key) = key {
            key
        } else {
            return Err(Error::BookOption(format!("Expected a String as a key, found {:?}", key)));
        };
        
        if self.valid_strings.contains(&key.as_ref()) {
            // value is a string
            if let Yaml::String(value) = value {
                return Ok(self.options.insert(key, BookOption::String(value)));
            } else {
                return Err(Error::BookOption(format!("Expected a string as value for key {}, found {:?}", &key, &value)));
            }
        } else if self.valid_paths.contains(&key.as_ref()) {
            // value is a path
            if let Yaml::String(value) = value {
                return Ok(self.options.insert(key, BookOption::Path(value)));
            } else {
                return Err(Error::BookOption(format!("Expected a string as value for key {}, found {:?}", &key, &value)));
            }
        } else if self.valid_chars.contains(&key.as_ref()) {
            // value is a char
            if let Yaml::String(value) = value {
                let chars: Vec<_> = value.chars().collect();
                if chars.len() != 1 {
                    return Err(Error::BookOption(format!("could not parse {} as a char: does not contain exactly one char", &value)));
                }
                return Ok(self.options.insert(key.to_owned(), BookOption::Char(chars[0])));
            } else {
                return Err(Error::BookOption(format!("Expected a string as value containing a char for key {}, found {:?}", &key, &value)));
            }
        } else if self.valid_bools.contains(&key.as_ref()) {
            // value is a bool
            if let Yaml::Boolean(value) = value {
                return Ok(self.options.insert(key, BookOption::Bool(value)));
            } else {
                return Err(Error::BookOption(format!("Expected a boolean as value for key {}, found {:?}", &key, &value)));
            }
        } else if self.valid_ints.contains(&key.as_ref()) {
            // value is an int
            if let Yaml::Integer(value) = value {
                return Ok(self.options.insert(key, BookOption::Int(value as i32)));
            } else {
                return Err(Error::BookOption(format!("Expected an integer as value for key {}, found {:?}", &key, &value)));
            }
        } else {
            // key not recognized
            return Err(Error::BookOption(format!("Unrecognized key: {}", &key)));
        }
    }
    
    /// Sets an option
    ///
    /// # Arguments
    /// * `key`: the identifier of the option, e.g.: "author"
    /// * `value`: the value of the option as a string
    ///
    /// **Returns** an error either if `key` is not a valid option or if the
    /// value is not of the right type. An option containing None if key was
    /// not set, and Some(previous_value) if key was already present.
    ///
    /// # Examples
    /// ```
    /// use crowbook::Book;
    /// let mut book = Book::new();
    /// book.options.set("author", "Joan Doe").unwrap(); // ok
    /// book.options.set("numbering", "2").unwrap(); // sets numbering to chapters and subsections
    /// let result = book.options.set("autor", "John Smith"); 
    /// assert!(result.is_err()); // error: "author" was mispelled "autor"
    ///
    /// let result = book.options.set("numbering", "foo"); 
    /// assert!(result.is_err()); // error: numbering must be an int
    /// ```
    pub fn set(&mut self, key: &str, value: &str) -> Result<Option<BookOption>> {
        let result = YamlLoader::load_from_str(value);
        if let Ok(yaml_docs) = result {
            if yaml_docs.len() == 1 {
                let yaml_value = yaml_docs.into_iter().next().unwrap();
                self.set_yaml(Yaml::String(key.to_owned()), yaml_value)
            } else {
                Err(Error::BookOption(format!("value {} for key {} does not contain one and only one YAML value", value, key)))
            }
        } else {
            Err(Error::BookOption(format!("could not parse {} as a valid YAML value", value)))
        }
    }
        
    /// get an option
    pub fn get(&self, key: &str) -> Result<&BookOption> {
        self.options.get(key).ok_or(Error::InvalidOption(format!("option {} is not persent", key)))
    }

    
    /// Gets a string option 
    pub fn get_str(&self, key: &str) -> Result<&str> {
        try!(self.get(key)).as_str()
    }

    /// Get a path option
    ///
    /// Adds book's root path before it
    pub fn get_path(&self, key: &str) -> Result<String> {
        let path: &str = try!(try!(self.get(key)).as_path());
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
        try!(self.get(key)).as_path()
    }

    /// gets a bool option
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        try!(self.get(key)).as_bool()
    }

    /// gets a char option
    pub fn get_char(&self, key: &str) -> Result<char> {
        try!(self.get(key)).as_char()
    }

    /// gets an int  option
    pub fn get_i32(&self, key: &str) -> Result<i32> {
        try!(self.get(key)).as_i32()
    }



    /// Returns a description of all options valid to pass to a book.
    ///
    /// # arguments
    /// * `md`: whether the output should be formatted in Markdown
    pub fn description(md: bool) -> String {
        let mut out = String::new();
        let mut previous_is_comment = true;
        for (comment, key, o_type, default) in Self::options_to_vec() {
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
