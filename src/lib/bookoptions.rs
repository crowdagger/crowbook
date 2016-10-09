use error::{Error,Result, Source};
use bookoption::BookOption;
use book::Book;
use logger::{Logger, InfoLevel, SHELL_COLOUR_OFF, SHELL_COLOUR_RED, SHELL_COLOUR_BLUE, SHELL_COLOUR_ORANGE, SHELL_COLOUR_GREEN};

use yaml_rust::{Yaml, YamlLoader};
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::env;


static OPTIONS:&'static str = include_str!("bookoptions.txt");



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
    valid_floats: Vec<&'static str>,
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
            valid_bools: vec!(),
            valid_chars: vec!(),
            valid_ints: vec!(),
            valid_floats: vec!(),
            valid_strings: vec!(),
            valid_paths: vec!(),
            valid_tpls: vec!(),
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
                "float" => options.valid_floats.push(key),
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
                _ => panic!(lformat!("Ill-formatted OPTIONS string: unrecognized type '{}'", option_type.unwrap())),
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
                                              lformat!("Expected a String as a key, found {:?}", key)));
        };

        if self.valid_strings.contains(&key.as_ref()) {
            // value is a string
            if let Yaml::String(value) = value {
                Ok(self.options.insert(key, BookOption::String(value)))
            } else {
                Err(Error::book_option(&self.source,
                                           lformat!("Expected a string as value for key {}, found {:?}", &key, &value)))
            }
        } else if self.valid_paths.contains(&key.as_ref()) {
            // value is a path
            if let Yaml::String(value) = value {
                if &key == "import_config" {
                    // special case, not a real option
                    let book = try!(Book::new_from_file(try!(self.root.join(&value)
                                                             .to_str()
                                                             .ok_or(Error::book_option(&self.source,
                                                                                           lformat!("'{}''s path contains invalid UTF-8 code", &value)))),
                                                        InfoLevel::Info, &[]));
                    try!(self.merge(book.options));
                    Ok(None)
                } else {
                    Ok(self.options.insert(key, BookOption::Path(value)))
                }
            } else {
                Err(Error::book_option(&self.source,
                                           lformat!("expected a string as value for key '{}', found {:?}", &key, &value)))
            }
        } else if self.valid_chars.contains(&key.as_ref()) {
            // value is a char
            if let Yaml::String(value) = value {
                let chars: Vec<_> = value.chars().collect();
                if chars.len() != 1 {
                    return Err(Error::book_option(&self.source,
                                                      lformat!("could not parse '{}' as a char: does not contain exactly one char", &value)));
                }
                Ok(self.options.insert(key.to_owned(), BookOption::Char(chars[0])))
            } else {
                Err(Error::book_option(&self.source,
                                      lformat!("expected a string as value containing a char for key '{}', found {:?}", &key, &value)))
            }
        } else if self.valid_bools.contains(&key.as_ref()) {
            // value is a bool
            if let Yaml::Boolean(value) = value {
                Ok(self.options.insert(key, BookOption::Bool(value)))
            } else {
                Err(Error::book_option(&self.source,
                                           lformat!("expected a boolean as value for key '{}', found {:?}", &key, &value)))
            }
        } else if self.valid_ints.contains(&key.as_ref()) {
            // value is an int
            if let Yaml::Integer(value) = value {
                Ok(self.options.insert(key, BookOption::Int(value as i32)))
            } else {
                Err(Error::book_option(&self.source,
                                           lformat!("expected an integer as value for key '{}', found {:?}", &key, &value)))
            }
        } else if self.valid_floats.contains(&key.as_ref()) {
            // value is a float
            if let Yaml::Real(value) = value {
                match value.parse::<f32>() {
                    Ok(value) => Ok(self.options.insert(key, BookOption::Float(value))),
                    Err(_) => Err(Error::book_option(&self.source,
                                           lformat!("could not parse '{}' as a float for key '{}'", &value, &key)))
                }
            } else {
                Err(Error::book_option(&self.source,
                                           lformat!("expected a float as value for key '{}', found {:?}", &key, &value)))
            }
        } else if self.deprecated.contains_key(&key) {
            let opt = self.deprecated.get(&key).unwrap().clone();
            if let Some(new_key) = opt {
                Logger::display_warning(lformat!("'{}' has been deprecated, you should now use '{}'", &key, &new_key));
                self.set_yaml(Yaml::String(new_key), value)
            } else {
                Err(Error::book_option(self.source.clone(),
                                           lformat!("key '{}' has been deprecated.", &key)))
            }
        } else if key.starts_with("metadata.") {
            // key is a custom metadata
            // value must be a string
            if let Yaml::String(value) = value {
                self.metadata.push(key.clone());
                Ok(self.options.insert(key, BookOption::String(value)))
            } else {
                Err(Error::book_option(&self.source,
                                           lformat!("expected a string as value for key '{}', found {:?}", &key, &value)))
            }
        } else {
            // key not recognized
            Err(Error::book_option(self.source.clone(),
                                       lformat!("unrecognized key '{}'", &key)))
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
                                       lformat!("value '{}' for key '{}' does not contain one and only one YAML value", value, key)))
            }
        } else {
            Err(Error::book_option(&self.source,
                                   lformat!("could not parse '{}' as a valid YAML value", value)))
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
                                                                  lformat!("option '{}' is not present", key)))
    }

    /// Gets a list of path. Only used for resources.files.
    #[doc(hidden)]
    pub fn get_paths_list(&self, key: &str) -> Result<Vec<String>> {
        if key != "resources.files" {
            return Err(Error::book_option(&self.source,
                                          lformat!("can't get '{}' as a list of files, only valid if key is resources.files", key)));
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
                | "output.proofread.html"
                | "output.proofread.html_dir"
                | "output.proofread.pdf"
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
                                   lformat!("'{}''s path contains invalid UTF-8 code", key)))
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

    /// gets a float option
    pub fn get_f32(&self, key: &str) -> Result<f32> {
        try!(self.get(key)).as_f32()
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
                                                  lformat!("'{}''s path contains invalid UTF-8 code", key)));
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
            // Don't display deprecated options if md is not set
            if !md && comment.trim() == "Deprecated options" {
                return out;
            }
            if key.is_none() {
                if !previous_is_comment {
                    out.push_str("\n");
                    previous_is_comment = true;
                }
                let header = format!("### {} ###\n", comment.trim());
                let header = if md {
                    header
                } else {
                    format!("{}{}{}",
                            SHELL_COLOUR_RED,
                            header,
                            SHELL_COLOUR_OFF)
                };
                out.push_str(&header);
                continue;
            }
            previous_is_comment = false;
            let o_type = match o_type.unwrap() {
                "bool" => "boolean",
                "float" => "float",
                "int" => "integer",
                "char" => "char",
                "str" => "string",
                "path" => "path",
                "tpl" => "template path",
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
                out.push_str(&format!("{}{}{} ({}{}{}) (default: {}{}{})\n  {}\n",
                                      SHELL_COLOUR_ORANGE,
                                      key.unwrap(),
                                      SHELL_COLOUR_OFF,
                                      SHELL_COLOUR_BLUE,
                                      o_type,
                                      SHELL_COLOUR_OFF,
                                      SHELL_COLOUR_GREEN,
                                      def,
                                      SHELL_COLOUR_OFF,
                                      comment.trim()));
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
