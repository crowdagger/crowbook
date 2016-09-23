use error::{Error,Result, Source};
use bookoption::BookOption;
use book::Book;
use logger::{Logger, InfoLevel};

use yaml_rust::{Yaml, YamlLoader};
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::env;

static OPTIONS:&'static str = r#"
# Metadata
author:str:Anonymous                # Author of the book
title:str:Untitled                  # Title of the book
lang:str:en                         # Language of the book
subject:str                         # Subject of the book (used for EPUB metadata)
description:str                     # Description of the book (used for EPUB metadata)
cover:path                          # Path to the cover of the book 

# Additional metadata 
license:str                         # License of the book
version:str                         # Version of the book
date:str                            # Date the book was revised

# Output options
output.epub:path                    # Output file name for EPUB rendering
output.html:path                    # Output file name for HTML rendering
output.html_dir:path                # Output directory name for HTML rendering
output.tex:path                     # Output file name for LaTeX rendering
output.pdf:path                     # Output file name for PDF rendering
output.odt:path                     # Output file name for ODT rendering
output.base_path:path:""            # Directory where those output files will we written

# Rendering options
rendering.initials:bool:false                                        # Use initials ('lettrines') for first letter of a chapter (experimental)
rendering.inline_toc:bool:false                                      # Display a table of content in the document
rendering.inline_toc.name:str:"{{{loc_toc}}}"                        # Name of the table of contents if it is displayed in document
rendering.num_depth:int:1                                            # The  maximum heading levels that should be numbered (0: no numbering, 1: only chapters, ..., 6: all)
rendering.chapter_template:str:"{{{number}}}\\. {{{chapter_title}}}" # Naming scheme of chapters

# Special option
import_config:path                  # Import another book configuration file

# HTML options
html.header:str                     # Custom header to display at the beginning of html file(s) 
html.footer:str                     # Custom footer to display at the end of HTML file(s)
html.css:tpl                        # Path of a stylesheet for HTML rendering
html.js:tpl                         # Path of a javascript file
html.css.print:tpl                  # Path of a media print stylesheet for HTML rendering
html.highlight_code:bool:true       # Provides syntax highlighting for code blocks (using highlight.js) 
html.highlight.js:tpl               # Set another highlight.js version than the bundled one
html.highlight.css:tpl              # Set another highlight.js CSS theme than the default one
html.side_notes:bool:false          # Display footnotes as side notes in HTML/Epub (experimental)


# Standalone HTML options
html_single.one_chapter:bool:false  # Display only one chapter at a time (with a button to display all)
html_single.html:tpl                # Path of an HTML template
html_single.js:tpl                  # Path of a javascript file


# Multifile HTML options
html_dir.index.html:tpl             # Path of index.html template
html_dir.chapter.html:tpl           # Path of a chapter.html template

# EPUB options
epub.version:int:2                  # EPUB version to generate (2 or 3)
epub.css:tpl                        # Path of a stylesheet for EPUB
epub.chapter.xhtml:tpl              # Path of an xhtml template for each chapter

# LaTeX options
tex.links_as_footnotes:bool:true    # Add foontotes to URL of links so they are readable when printed
tex.command:str:pdflatex            # LaTeX command to use for generating PDF
tex.template:tpl                    # Path of a LaTeX template file
tex.class:str:book                  # LaTeX class to use

# Resources option
resources.files:str                 # Whitespace-separated list of files to embed in e.g. EPUB file; useful for including e.g. fonts
resources.base_path:path            # Path where to find resources (in the source tree). By default, links and images are relative to the Markdown file. If this is set, it will be to this path. 
resources.base_path.links:path      # Set base path but only for links. Useless if resources.base_path is set.
resources.base_path.images:path:.   # Set base path but only for images. Useless if resources.base_path is set.
resources.base_path.files:path:.    # Set base path but only for additional files. Useless if resources.base_path is set.
resources.base_path.templates:path:. # Set base path but only for templates files. Useless if resources.base_path is set.
resources.out_path:path:data        # Paths where additional resources should be copied in the EPUB file or HTML directory 


# Input options
input.autoclean:bool:true           # Toggle cleaning of input markdown according to lang
input.yaml_blocks:bool:false        # Enable inline YAML blocks to override options set in config file


# Crowbook options
crowbook.temp_dir:path:             # Path where to create a temporary directory (default: uses result from Rust's std::env::temp_dir())
crowbook.zip.command:str:zip        # Command to use to zip files (for EPUB/ODT)
crowbook.verbose:bool:false         # Make Crowbook display more messages


# Deprecated options
base_path:alias:resources.base_path                 # Renamed
base_path.links:alias:resources.base_path.links     # Renamed
base_path.images:alias:resources.base_path.images   # Renamed
side_notes:alias:html.side_notes                    # Renamed
html.top:alias:html.header                          # Renamed
autoclean:alias:input.autoclean                     # Renamed
enable_yaml_blocks:alias:input.yaml_blocks          # Renamed
use_initials:alias:rendering.initials               # Renamed
toc_name:alias:rendering.inline_toc.name            # Renamed
display_toc:alias:rendering.inline_toc              # Renamed
numbering:alias:rendering.num_depth                 # Renamed
numbering_template:alias:rendering.chapter_template # Renamed
html.display_chapter:alias:html_single.one_chapter  # Renamed
temp_dir:alias:crowbook.temp_dir                    # Renamed
zip.command:alias:crowbook.zip.command              # Renamed
verbose:alias:crowbook.verbose                      # Renamed
html.script:alias:html_singe.js                     # Renamed
html.print_css:alias:html.css.print                 # Renamed
html.template:alias:html_single.html                # Renamed
html_dir.script:alias:html_dir.js                   # Renamed
epub.template:alias:epub.chapter.xhtml              # Renamed
html_dir.css:alias:html.css                         # Renamed
nb_char:alias                                       # Removed
tex.short:alias                                     # Removed
html.crowbook_link:alias                            # Removed
"#;



/// Contains the options of a book.
#[derive(Debug)]
pub struct BookOptions {
    options: HashMap<String, BookOption>,
    defaults: HashMap<String, BookOption>,
    deprecated: HashMap<String, Option<String>>,
    valid_tpls: Vec<&'static str>,
    valid_bools: Vec<&'static str>,
    valid_chars: Vec<&'static str>,
    valid_strings: Vec<&'static str>,
    valid_paths: Vec<&'static str>,
    valid_ints: Vec<&'static str>,
    metadata: Vec<String>,

    /// Source for errors (unnecessary copy :/)
    #[doc(hidden)]
    pub source: Source,

    /// Root path of the book (unnecessary copy :/)
    #[doc(hidden)]
    pub root: PathBuf,
}

impl BookOptions {
    /// Creates a new BookOptions struct from the default compliled string
    pub fn new() -> BookOptions {
        let mut options = BookOptions {
            options: HashMap::new(),
            deprecated: HashMap::new(),
            defaults: HashMap::new(),
            valid_bools:vec!(),
            valid_chars:vec!(),
            valid_ints:vec!(),
            valid_strings:vec!(),
            valid_paths:vec!(),
            valid_tpls:vec!(),
            metadata: vec!(),
            root: PathBuf::new(),
            source: Source::empty(),
        };

        // Load default options and types from OPTIONS
        let mut is_metadata = false;
        for (comment, key, option_type, default_value) in Self::options_to_vec() {
            if key.is_none() {
                if comment.contains("Metadata") || comment.contains("metadata") {
                    is_metadata = true;
                } else {
                    is_metadata = false;
                }
                continue;
            }
            let key = key.unwrap();
            match option_type.unwrap() {
                "str" => {
                    if is_metadata {
                        options.metadata.push(key.to_owned());
                    }
                    options.valid_strings.push(key);
                },
                "bool" => options.valid_bools.push(key),
                "int" => options.valid_ints.push(key),
                "char" => options.valid_chars.push(key),
                "path" => options.valid_paths.push(key),
                "tpl" => {
                    options.valid_tpls.push(key);
                    options.valid_paths.push(key);
                },
                "alias" => {
                    options.deprecated.insert(key.to_owned(), default_value.map(|s| s.to_owned()));
                    continue;
                }
                _ => panic!(format!("Ill-formatted OPTIONS string: unrecognized type '{}'", option_type.unwrap())),
            }
            if key == "crowbook.temp_dir" {
                // "temp_dir" has a special default value that depends on the environment
                options.set(key, &env::temp_dir().to_string_lossy()).unwrap();
                continue;
            }
            if let Some(value) = default_value {
                options.set(key, value).unwrap();
                // hack to get the BookOption without changing the API
                let option = options.set(key, value).unwrap();
                options.defaults.insert(key.to_owned(), option.unwrap());
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
    /// # Returns
    /// 
    /// * an error either if `key` is not a valid option or if the value is not of the right type.
    /// * an option containing None if key was not set, and Some(previous_value) if key was already present.
    #[doc(hidden)]
    pub fn set_yaml(&mut self, key: Yaml, value: Yaml) -> Result<Option<BookOption>> {
        let key:String = if let Yaml::String(key) = key {
            key
        } else {
            return Err(Error::book_option(&self.source,
                                              format!("Expected a String as a key, found {:?}", key)));
        };

        if self.valid_strings.contains(&key.as_ref()) {
            // value is a string
            if let Yaml::String(value) = value {
                Ok(self.options.insert(key, BookOption::String(value)))
            } else {
                Err(Error::book_option(&self.source,
                                           format!("Expected a string as value for key {}, found {:?}", &key, &value)))
            }
        } else if self.valid_paths.contains(&key.as_ref()) {
            // value is a path
            if let Yaml::String(value) = value {
                if &key == "import_config" {
                    // special case, not a real option
                    let book = try!(Book::new_from_file(try!(self.root.join(&value)
                                                             .to_str()
                                                             .ok_or(Error::book_option(&self.source,
                                                                                           format!("'{}''s path contains invalid UTF-8 code", &value)))),
                                                        InfoLevel::Info, &[]));
                    try!(self.merge(book.options));
                    Ok(None)
                } else {
                    Ok(self.options.insert(key, BookOption::Path(value)))
                }
            } else {
                Err(Error::book_option(&self.source,
                                           format!("expected a string as value for key '{}', found {:?}", &key, &value)))
            }
        } else if self.valid_chars.contains(&key.as_ref()) {
            // value is a char
            if let Yaml::String(value) = value {
                let chars: Vec<_> = value.chars().collect();
                if chars.len() != 1 {
                    return Err(Error::book_option(&self.source,
                                                      format!("could not parse '{}' as a char: does not contain exactly one char", &value)));
                }
                Ok(self.options.insert(key.to_owned(), BookOption::Char(chars[0])))
            } else {
                Err(Error::book_option(&self.source,
                                      format!("expected a string as value containing a char for key '{}', found {:?}", &key, &value)))
            }
        } else if self.valid_bools.contains(&key.as_ref()) {
            // value is a bool
            if let Yaml::Boolean(value) = value {
                Ok(self.options.insert(key, BookOption::Bool(value)))
            } else {
                Err(Error::book_option(&self.source,
                                           format!("expected a boolean as value for key '{}', found {:?}", &key, &value)))
            }
        } else if self.valid_ints.contains(&key.as_ref()) {
            // value is an int
            if let Yaml::Integer(value) = value {
                Ok(self.options.insert(key, BookOption::Int(value as i32)))
            } else {
                Err(Error::book_option(&self.source,
                                           format!("expected an integer as value for key '{}', found {:?}", &key, &value)))
            }
        } else if self.deprecated.contains_key(&key) {
            let opt = self.deprecated.get(&key).unwrap().clone();
            if let Some(new_key) = opt {
                Logger::display_warning(format!("'{}' has been deprecated, you should now use '{}'", &key, &new_key));
                self.set_yaml(Yaml::String(new_key), value)
            } else {
                Err(Error::book_option(self.source.clone(),
                                           format!("key '{}' has been deprecated.", &key)))
            }
        } else if key.starts_with("metadata.") {
            // key is a custom metadata
            // value must be a string
            if let Yaml::String(value) = value {
                self.metadata.push(key.clone());
                Ok(self.options.insert(key, BookOption::String(value)))
            } else {
                Err(Error::book_option(&self.source,
                                           format!("expected a string as value for key '{}', found {:?}", &key, &value)))
            }
        } else {
            // key not recognized
            Err(Error::book_option(self.source.clone(),
                                       format!("unrecognized key '{}'", &key)))
        }
    }
    
    /// Sets an option
    ///
    /// # Arguments
    /// * `key`: the identifier of the option, e.g.: "author"
    /// * `value`: the value of the option as a string
    ///
    /// # Returns
    /// * an error either if `key` is not a valid option or if the
    ///   value is not of the right type.
    /// * an option containing None if key was
    ///   not set, and Some(previous_value) if key was already present.
    ///
    /// # Examples
    /// ```
    /// use crowbook::Book;
    /// let mut book = Book::new(&[]);
    /// book.options.set("author", "Joan Doe").unwrap(); // ok
    /// book.options.set("rendering.num_depth", "2").unwrap(); // sets numbering to chapters and subsections
    /// let result = book.options.set("autor", "John Smith"); 
    /// assert!(result.is_err()); // error: "author" was mispelled "autor"
    ///
    /// let result = book.options.set("rendering.num_depth", "foo"); 
    /// assert!(result.is_err()); // error: numbering must be an int
    /// ```
    pub fn set(&mut self, key: &str, value: &str) -> Result<Option<BookOption>> {
        let result = YamlLoader::load_from_str(value);
        if let Ok(yaml_docs) = result {
            if yaml_docs.len() == 1 {
                let yaml_value = yaml_docs.into_iter().next().unwrap();
                self.set_yaml(Yaml::String(key.to_owned()), yaml_value)
            } else {
                Err(Error::book_option(&self.source,
                                       format!("value '{}' for key '{}' does not contain one and only one YAML value", value, key)))
            }
        } else {
            Err(Error::book_option(&self.source,
                                   format!("could not parse '{}' as a valid YAML value", value)))
        }
    }

    /// Return the list of keys that are metadata
    #[doc(hidden)]
    pub fn get_metadata(&self) -> &[String] {
        &self.metadata
    }
        
    /// Gets an option
    #[doc(hidden)]
    pub fn get(&self, key: &str) -> Result<&BookOption> {
        self.options.get(key).ok_or_else(|| Error::invalid_option(&self.source,
                                                                  format!("option '{}' is not present", key)))
    }

    /// Gets a list of path. Only used for resources.files.
    #[doc(hidden)]
    pub fn get_paths_list(&self, key: &str) -> Result<Vec<String>> {
        if key != "resources.files" {
            return Err(Error::book_option(&self.source,
                                          format!("can't get '{}' as a list of files, only valid if key is resources.files", key)));
        }

        let list = try!(try!(self.get(key))
                        .as_str())
            .split_whitespace();
        let mut res = vec!();
        for s in list {
            res.push(s.to_owned());
        }
        Ok(res)
    }
    
    /// Gets a string option 
    pub fn get_str(&self, key: &str) -> Result<&str> {
        try!(self.get(key)).as_str()
    }

    /// Get a path option
    ///
    /// Adds the correct path correction before it
    pub fn get_path(&self, key: &str) -> Result<String> {
        let path: &str = try!(try!(self.get(key)).as_path());

        if Path::new(path).is_absolute() {
            // path is absolute, do nothing
            return Ok(path.to_owned());
        }

        let new_path:PathBuf = match key {
            "resources.base_path.links"
                | "resources.base_path.images"
                | "resources.base_path.files"
                | "resources.pase_path.templates"
                => {
                    // If resources.base_path is set, return it, else return itself
                    let base_path = self.get_path("resources.base_path");
                    if base_path.is_ok() {
                        return base_path;
                    }
                    self.root.join(path)
                },
            
            "cover"
                => {
                    // Translate according to resources.base_path.images
                    let base = self.get_path("resources.base_path.images").unwrap();
                    let new_path = Path::new(&base).join(path);
                    new_path
                },

            "output.epub"
                | "output.html"
                | "output.html_dir"
                | "output.pdf"
                | "output.tex"
                | "output.odt"
                => {
                    // Translate according to output.base_path
                    let base = self.get_path("output.base_path").unwrap();
                    let new_path = Path::new(&base).join(path);
                    new_path
                },

            key if self.valid_tpls.contains(&key)
                => {
                    // Translate according to resources.base_path.template
                    let base = self.get_path("resources.base_path.templates").unwrap();
                    let new_path = Path::new(&base).join(path);
                    new_path
                },


            _ => self.root.join(path)
        };
        if let Some(path) = new_path.to_str() {
            Ok(path.to_owned())
        } else {
            Err(Error::book_option(&self.source,
                                   format!("'{}''s path contains invalid UTF-8 code", key)))
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


    /// Merges the other list of options to the first one
    ///
    /// If option is already set in self, don't add it, unless it was the default.
    /// Option is not inserted either if new value is equal to default.
    #[doc(hidden)]
    pub fn merge(&mut self, other: BookOptions) -> Result<()> {
        let relative_path = match other.root.strip_prefix(&self.root) {
            Ok(path) => path,
            Err(_) => &other.root,
        };
        for (key, value) in &other.options {
            // Check if option was already set, and if it was to default or to something else
            if self.defaults.contains_key(key) {
                let previous_opt = self.options.get(key);
                let default = self.defaults.get(key).unwrap();
                // If new value is equal to default, don't insert it
                if value == default {
                    continue;
                }
                if let Some(previous_opt) = previous_opt {
                    if previous_opt != default {
                        // Previous value is other than default, don't merge
                        continue;
                    }
                }
            }
            // If it's a path, get the corrected path
            if let &BookOption::Path(ref path) = value {
                let new_path:PathBuf = if other.valid_tpls.contains(&key.as_ref()) {
                    // If key is a template, sets it with an absolute path so it won't be messed up if
                    // resources.base_path.templates is redefined later on
                    let path = other.get_path(&key).unwrap();
                    let new_path = try!(::std::env::current_dir().map_err(|_|
                                                                          Error::default(Source::empty(), "could not get current directory!!!")))
                        .join(&path);
                    new_path
                } else {
                    relative_path.join(path)
                };
                let new_path = if let Some(path) = new_path.to_str() {
                    path.to_owned()
                } else {
                    return Err(Error::book_option(Source::new(other.root.to_str().unwrap()),
                                                  format!("'{}''s path contains invalid UTF-8 code", key)));
                };
                self.options.insert(key.clone(), BookOption::Path(new_path));
            } else {
                self.options.insert(key.clone(), value.clone());
            }
        }
        Ok(())
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
                "alias" => "DEPRECATED",
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
