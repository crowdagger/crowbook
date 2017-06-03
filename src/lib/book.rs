// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Crowbook is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use error::{Error, Result, Source};
use cleaner::{Cleaner, CleanerParams, French, Off, Default};
use bookoptions::BookOptions;
use parser::Parser;
use parser::Features;
use epub::{Epub};
use html_single::{HtmlSingle, ProofHtmlSingle};
use html_dir::{HtmlDir, ProofHtmlDir};
use html_if::{HtmlIf};
use latex::{Latex, ProofLatex, Pdf, ProofPdf};
use odt::{Odt};
use templates::{epub, html, epub3, latex, html_dir, highlight, html_single};
use number::Number;
use resource_handler::ResourceHandler;
use logger::{Logger, InfoLevel};
use lang;
use misc;
use book_renderer::BookRenderer;
use chapter::Chapter;
use token::Token;
use text_view::view_as_text;

#[cfg(feature = "proofread")]
use repetition_check::RepetitionDetector;

#[cfg(feature = "proofread")]
use grammar_check::GrammarChecker;

// Dummy grammarchecker thas does nothing to let the compiler compile
#[cfg(not(feature = "proofread"))]
struct GrammarChecker {}
#[cfg(not(feature = "proofread"))]
impl GrammarChecker {
    fn check_chapter(&self, _: &[Token]) -> Result<()> {
        Ok(())
    }
}
// Dummy RepetitionDetector thas does nothing to let the compiler compile
#[cfg(not(feature = "proofread"))]
struct RepetitionDetector {}
#[cfg(not(feature = "proofread"))]
impl RepetitionDetector {
    fn check_chapter(&self, _: &[Token]) -> Result<()> {
        Ok(())
    }
}


use std::fs::File;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::iter::IntoIterator;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

use rayon::prelude::*;
use mustache;
use mustache::{MapBuilder, Template};
use yaml_rust::{YamlLoader, Yaml};
use numerals::roman::Roman;

/// Type of header (part or chapter)
#[derive(Copy, Clone, Debug)]
pub enum Header {
    /// Chapter (default)
    Chapter,
    /// Part (or "book" or "episode" or whatever)
    Part,
}


/// Header data (for chapter or part)
#[derive(Debug, Clone)]
pub struct HeaderData {
    /// A string containnig the full text version, for e.g. TOCs
    pub text: String,
    /// The title of the header, e.g. "Part" or "Chapter" or nothing
    pub header: String,
    /// The number, formatted in roman or arabic
    pub number: String,
    /// Only the title
    pub title: String,
}

impl fmt::Display for HeaderData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

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
/// // Create a book with some options 
/// let mut book = Book::new();
/// book.set_options(&[("author", "Joan Doe"),
///                    ("title", "An untitled book"),
///                    ("lang", "en")]);
///
/// // Add a chapter to the book
/// book.add_chapter_from_source(Number::Default, "# The beginning#\nBla, bla, bla".as_bytes()).unwrap();
///
/// // Render the book as html to stdout
/// book.render_format_to("html", &mut std::io::stdout()).unwrap();
/// ```
pub struct Book {
    /// Internal structure. You should not accesss this directly except if
    /// you are writing a new renderer.
    pub chapters: Vec<Chapter>,

    /// Options of the book
    pub options: BookOptions,

    /// Root path of the book
    #[doc(hidden)]
    pub root: PathBuf,

    /// Logger
    #[doc(hidden)]
    pub logger: Logger,

    /// Source for error files
    #[doc(hidden)]
    pub source: Source,

    /// Features used in the book content
    #[doc(hidden)]
    pub features: Features,

    cleaner: Box<Cleaner>,
    chapter_template: Option<Template>,
    part_template: Option<Template>,
    checker: Option<GrammarChecker>,
    detector: Option<RepetitionDetector>,
    formats: HashMap<&'static str, (String, Box<BookRenderer>)>,
}

impl Book {
    /// Creates a new, empty `Book`
    pub fn new() -> Book {
        let mut book = Book {
            source: Source::empty(),
            chapters: vec![],
            cleaner: Box::new(Off),
            root: PathBuf::new(),
            options: BookOptions::new(),
            logger: Logger::new(),
            chapter_template: None,
            part_template: None,
            checker: None,
            detector: None,
            formats: HashMap::new(),
            features: Features::new(),
        };
        book.add_format("html", lformat!("HTML (standalone page)"), Box::new(HtmlSingle{}))
            .add_format("proofread.html", lformat!("HTML (standalone page/proofreading)"), Box::new(ProofHtmlSingle{}))
            .add_format("html.dir", lformat!("HTML (multiple pages)"), Box::new(HtmlDir{}))
            .add_format("proofread.html.dir", lformat!("HTML (multiple pages/proofreading)"), Box::new(ProofHtmlDir{}))
            .add_format("tex", lformat!("LaTeX"), Box::new(Latex{}))
            .add_format("proofread.tex", lformat!("LaTeX (proofreading)"), Box::new(ProofLatex{}))
            .add_format("pdf", lformat!("PDF"), Box::new(Pdf{}))
            .add_format("proofread.pdf", lformat!("PDF (proofreading)"), Box::new(ProofPdf{}))
            .add_format("epub", lformat!("EPUB"), Box::new(Epub{}))
            .add_format("odt", lformat!("ODT"), Box::new(Odt{}))
            .add_format("html.if", lformat!("HTML (interactive fiction)"), Box::new(HtmlIf{}));
        book
    }

    /// Register a format that can be rendered.
    ///
    /// The renderer for this format must implement the `BookRenderer` trait.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::{Result, Book, BookRenderer};
    /// use std::io::Write;
    /// struct Dummy {}
    /// impl BookRenderer for Dummy {
    ///     fn render(&self, book: &Book, to: &mut Write) -> Result<()> {
    ///         write!(to, "This does nothing useful").unwrap();
    ///         Ok(())
    ///      }
    /// }
    ///
    /// let mut book = Book::new();
    /// book.add_format("foo",
    ///                 "Some dummy implementation",
    ///                 Box::new(Dummy{}));
    /// ```
    pub fn add_format<S: Into<String>>(&mut self,
                                       format: &'static str,
                                       description: S,
                                       renderer: Box<BookRenderer>) -> &mut Self {
        self.formats.insert(format, (description.into(), renderer));
        self
    }
    
    /// Sets the options of a `Book`
    ///
    /// # Arguments
    /// * `options`: a (possibly empty) list (or other iterator) of (key, value) tuples.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::Book;
    /// let mut book = Book::new();
    /// book.set_options(&[("author", "Foo"), ("title", "Bar")]);
    /// assert_eq!(book.options.get_str("author").unwrap(), "Foo");
    /// assert_eq!(book.options.get_str("title").unwrap(), "Bar");
    /// ```
    pub fn set_options<'a, I>(&mut self, options: I) -> &mut Book
        where I: IntoIterator<Item = &'a (&'a str, &'a str)>
    {
        // set options
        for &(key, value) in options {
            if let Err(err) = self.options.set(key, value) {
                self.logger
                    .error(lformat!("Error initializing book: could not set {key} to {value}: {error}",
                                    key = key,
                                    value = value,
                                    error = err));
            }
        }
        // set cleaner according to lang and autoclean settings
        self.update_cleaner();
        self
    }

    /// Sets the verbosity of a book
    ///
    /// See `InfoLevel` for more information on verbosity
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::{Book, InfoLevel};
    /// let mut book = Book::new();
    /// book.set_verbosity(InfoLevel::Warning);
    /// ```
    pub fn set_verbosity(&mut self, verbosity: InfoLevel) -> &mut Book {
        self.logger.set_verbosity(verbosity);
        self
    }

    /// Loads a book configuration file
    ///
    /// # Argument
    /// * `path`: the path of the file to load. The directory of this file is used as
    ///   a "root" directory for all paths referenced in books, whether chapter files,
    ///   templates, cover images, and so on.
    ///
    /// # Example
    ///
    /// ```
    /// # use crowbook::Book;
    /// let mut book = Book::new();
    /// let result = book.load_file("some.book");
    /// ```
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<&mut Book> {
        let filename = format!("{}", path.as_ref().display());
        self.source = Source::new(filename.as_str());
        self.options.source = Source::new(filename.as_str());

        let f = File::open(path.as_ref())
            .map_err(|_| {
                Error::file_not_found(Source::empty(), lformat!("book"), filename.clone())
            })?;
        // Set book path to book's directory
        if let Some(parent) = path.as_ref().parent() {
            self.root = parent.to_owned();
            self.options.root = self.root.clone();
        }

        let result = self.read_config(&f);
        match result {
            Ok(book) => Ok(book),
            Err(err) => {
                if err.is_config_parser() && path.as_ref().ends_with(".md") {
                    let err = Error::default(Source::empty(),
                                             lformat!("could not parse {file} as a book \
                                                       file.\nMaybe you meant to run crowbook \
                                                       with the --single argument?",
                                                      file = misc::normalize(path)));
                    Err(err)
                } else {
                    Err(err)
                }
            }
        }
    }
    
    /// Loads a single markdown file
    ///
    /// This is *not* used to add a chapter to an existing book, but to to load the
    /// book configuration file from a single Markdown file.
    ///
    /// Since it is designed for single-chapter short stories, this method also sets
    /// the `tex.class` option to `article`.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::Book;
    /// let mut book = Book::new();
    /// book.load_markdown_file("foo.md"); // not unwraping since foo.md doesn't exist
    /// ```
    pub fn load_markdown_file<P:AsRef<Path>>(&mut self, path: P) -> Result<&mut Self> {
        let filename = format!("{}", path.as_ref().display());
        self.source = Source::new(filename.as_str());

        // Set book path to book's directory
        if let Some(parent) = path.as_ref().parent() {
            self.root = parent.to_owned();
            self.options.root = self.root.clone();
        }
        self.options.set("tex.class", "article").unwrap();
        self.options.set("input.yaml_blocks", "true").unwrap();

        // Add the file as chapter with hidden title
        // hideous line, but basically transforms foo/bar/baz.md to baz.md
        let relative_path = Path::new(path
                                      .as_ref()
                                      .components()
                                      .last()
                                      .unwrap()
                                      .as_os_str());

        // Update grammar checker according to options
        self.add_chapter(Number::Hidden, &relative_path.to_string_lossy())?;

        Ok(self)
    }

    /// Reads a single markdown config from a `Read`able object.
    ///
    /// Similar to `load_markdown_file`, except it reads a source instead of a file.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::Book;
    /// let content = "\
    /// ---
    /// author: Foo
    /// title: Bar
    /// ---
    ///
    /// # Book #
    ///
    /// Some content in *markdown*.";
    ///
    /// let mut book = Book::new();
    /// book.read_markdown_config(content.as_bytes()).unwrap();
    /// assert_eq!(book.options.get_str("title").unwrap(), "Bar");
    /// ```
    pub fn read_markdown_config<R: Read>(&mut self, source: R) -> Result<&mut Self> {
        self.options.set("tex.class", "article").unwrap();
        self.options.set("input.yaml_blocks", "true").unwrap();

        // Update grammar checker according to options
        self.add_chapter_from_source(Number::Hidden, source)?;

        Ok(self)
    }

    /// Sets options from a YAML block
    fn set_options_from_yaml(&mut self, yaml: &str) -> Result<&mut Book> {
        self.options.source = self.source.clone();
        match YamlLoader::load_from_str(yaml) {
            Err(err) => {
                return Err(Error::config_parser(&self.source,
                                                lformat!("YAML block was not valid YAML: {error}",
                                                         error = err)))
            }
            Ok(mut docs) => {
                if docs.len() == 1 && docs[0].as_hash().is_some() {
                    if let Yaml::Hash(hash) = docs.pop().unwrap() {
                        for (key, value) in hash {
                            if let Err(err) = self.options.set_yaml(key, value) {
                                self.logger.error(format!("{}", err));
                            };
                        }
                    } else {
                        unreachable!();
                    }
                } else {
                    return Err(Error::config_parser(&self.source,
                                                    lformat!("YAML part of the book is not a \
                                                              valid hashmap")));
                }
            }
        }
        Ok(self)
    }
        
    /// Reads a book configuration from a `Read`able source.
    ///
    /// # Book configuration
    ///
    /// A line with "option: value" sets the option to value
    ///
    /// + chapter_name.md adds the (default numbered) chapter
    ///
    /// - chapter_name.md adds the (unnumbered) chapter
    ///
    /// 3. chapter_name.md adds the (custom numbered) chapter
    ///
    /// # See also
    /// * `load_file`
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::Book;
    /// let content = "\
    /// author: Foo
    /// title: Bar
    ///
    /// ! intro.md
    /// + chapter_01.md";
    /// 
    /// let mut book = Book::new();
    /// book.read_config(content.as_bytes()); // no unwraping as `intro.md` and `chapter_01.md` don't exist
    /// ```
    pub fn read_config<R: Read>(&mut self, mut source: R) -> Result<&mut Book> {
        fn get_filename<'a>(source: &Source, s: &'a str) -> Result<&'a str> {
            let words: Vec<&str> = (&s[1..]).split_whitespace().collect();
            if words.len() > 1 {
                return Err(Error::config_parser(source,
                                                lformat!("chapter filenames must not contain \
                                                          whitespace")));
            } else if words.len() < 1 {
                return Err(Error::config_parser(source, lformat!("no chapter name specified")));
            }
            Ok(words[0])
        }

        let mut s = String::new();
        source.read_to_string(&mut s)
            .map_err(|err| Error::config_parser(Source::empty(),
                                                lformat!("could not read source: {error}",
                                                         error = err)))?;
        
        // Parse the YAML block, that is, until first chapter
        let mut yaml = String::new();
        let mut lines = s.lines().peekable();
        let mut line;

        let mut line_number = 0;
        let mut is_next_line_ok: bool;

        loop {
            if let Some(next_line) = lines.peek() {
                if next_line.starts_with(|c| match c {
                    '-' | '+' | '!' | '@'  => true,
                    _ => c.is_digit(10),
                }) {
                    break;
                }
            } else {
                break;
            }
            line = lines.next().unwrap();
            line_number += 1;
            self.source.set_line(line_number);
            yaml.push_str(line);
            yaml.push_str("\n");

            if line.trim().ends_with(|c| match c {
                '>' | '|' | ':' | '-' => true,
                _ => false,
            }) {
                // line ends with the start of a block indicator
                continue;
            }

            if let Some(next_line) = lines.peek() {
                let doc = YamlLoader::load_from_str(next_line);
                if !doc.is_ok() {
                    is_next_line_ok = false;
                } else {
                    let doc = doc.unwrap();
                    if !doc.is_empty() && doc[0].as_hash().is_some() {
                        is_next_line_ok = true;
                    } else {
                        is_next_line_ok = false;
                    }
                }
            } else {
                break;
            }
            if !is_next_line_ok {
                // If next line is not valid yaml, probably means we are in a multistring
                continue;
            }
            let result = self.set_options_from_yaml(&yaml);
            match result {
                Ok(_) => {
                    // Fine, we can remove previous lines
                    yaml = String::new();
                }
                Err(err) => {
                    if err.is_book_option() {
                        // book option error: abort
                        return Err(err);
                    } else {
                        // Other error: we do nothing, hoping it will work
                        // itself out when more lines are added to yaml
                    }
                }
            }
        }
        self.set_options_from_yaml(&yaml)?;

        // Update cleaner according to options (autoclean/lang)
        self.update_cleaner();

        // Update grammar checker according to options (proofread.*)
        self.init_checker();

        // Parse chapters
        while let Some(line) = lines.next() {
            line_number += 1;
            self.source.set_line(line_number);
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            } if line.starts_with("--") {
                // Subchapter
                let mut level = 0;
                for b in line.bytes() {
                    if b == b'-' {
                        level += 1;
                    } else {
                        break;
                    }
                }
                assert!(level > 1);
                level -= 1;
                let file = get_filename(&self.source, &line[level..])?;
                self.add_subchapter(level as i32, file)?;
            } else if line.starts_with('-') {
                // unnumbered chapter
                let file = get_filename(&self.source, line)?;
                self.add_chapter(Number::Unnumbered, file)?;
            } else if line.starts_with('+') {
                // numbered chapter
                let file = get_filename(&self.source, line)?;
                self.add_chapter(Number::Default, file)?;
            } else if line.starts_with('!') {
                // hidden chapter
                let file = get_filename(&self.source, line)?;
                self.add_chapter(Number::Hidden, file)?;
            } else if line.starts_with(|c: char| c.is_digit(10)) {
                // chapter with specific number
                let parts: Vec<_> = line.splitn(2, |c: char| c == '.' || c == ':' || c == '+')
                    .collect();
                if parts.len() != 2 {
                    return Err(Error::config_parser(&self.source,
                                                    lformat!("ill-formatted line specifying \
                                                              chapter number")));
                }
                let file = get_filename(&self.source, parts[1])?;
                let number = parts[0].parse::<i32>()
                    .map_err(|err| {
                        Error::config_parser(&self.source,
                                             lformat!("error parsing chapter number: {error}",
                                             error = err))})?;
                self.add_chapter(Number::Specified(number), file)?;
            } else if line.starts_with('@') {
                /* Part */
                let subline = &line[1..];
                if subline.starts_with(|c: char| c.is_whitespace()) {
                    let subline = subline.trim();
                    let ast = Parser::from(&self)
                        .parse_inline(subline)?;
                    let ast = vec!(Token::Header(1, ast));
                    self.chapters.push(Chapter::new(Number::DefaultPart, String::new(), ast));
                } else if subline.starts_with('+') {
                    /* Numbered part */
                    let file = get_filename(&self.source, subline)?;
                    self.add_chapter(Number::DefaultPart, file)?;
                } else if subline.starts_with('-') {
                    /* Unnumbered part */
                    let file = get_filename(&self.source, line)?;
                    self.add_chapter(Number::UnnumberedPart, file)?;
                } else if subline.starts_with(|c: char| c.is_digit(10)) {
                    /* Specified  part*/
                    let parts: Vec<_> = subline.splitn(2, |c: char| c == '.' || c == ':' || c == '+')
                        .collect();
                    if parts.len() != 2 {
                        return Err(Error::config_parser(&self.source,
                                                        lformat!("ill-formatted line specifying \
                                                                  part number")));
                    }
                    let file = get_filename(&self.source, parts[1])?;
                    let number = parts[0].parse::<i32>()
                        .map_err(|err| {
                            Error::config_parser(&self.source,
                                                 lformat!("error parsing part number: {error}",
                                                          error = err))})?;
                    self.add_chapter(Number::SpecifiedPart(number), file)?;
                } else {
                    return Err(Error::config_parser(&self.source,
                                                    lformat!("found invalid part definition in the chapter list")));
                }
            } else {
                return Err(Error::config_parser(&self.source,
                                                lformat!("found invalid chapter definition in \
                                                          the chapter list")));
            }
        }

        self.source.unset_line();
        self.set_chapter_template()?;
        Ok(self)
    }

    /// Determine whether proofreading is activated or not
    fn is_proofread(&self) -> bool {
        self.options.get_bool("proofread").unwrap() &&
        (self.options.get("output.proofread.html").is_ok() ||
         self.options.get("output.proofread.html.dir").is_ok() ||
         self.options.get("output.proofread.pdf").is_ok())
    }

    /// Initialize the grammar checker and repetetion detector if they needs to be
    #[cfg(feature = "proofread")]
    fn init_checker(&mut self) {
        if self.is_proofread() {
            if self.options.get_bool("proofread.languagetool").unwrap() {
                let port = self.options.get_i32("proofread.languagetool.port").unwrap() as usize;
                let lang = self.options.get_str("lang").unwrap();
                let checker = GrammarChecker::new(port, lang);
                match checker {
                    Ok(checker) => self.checker = Some(checker),
                    Err(e) => {
                        self.logger
                            .error(lformat!("{error}. Proceeding without checking grammar.", error = e))
                    }
                }
            }
            if self.options.get_bool("proofread.repetitions").unwrap() {
                self.detector = Some(RepetitionDetector::new(self));
            }
        }
    }

    #[cfg(not(feature = "proofread"))]
    fn init_checker(&mut self) {}

    /// Renders the book to the given format if output.{format} is set;
    /// do nothing otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::Book;
    /// let mut book = Book::new();
    /// /* Will do nothing as book is empty and has no output format specified */
    /// book.render_format("pdf");
    /// ```
    pub fn render_format(&self, format: &str) -> () {
        let mut key = String::from("output.");
        key.push_str(format);
        if let Ok(path) = self.options.get_path(&key) {
            let result = self.render_format_to_file(format, path);
            if let Err(err) = result {
                self.logger
                    .error(lformat!("Error rendering {name}: {error}", name = format, error = err));
            }
        }
    }

    /// Generates output files acccording to book options.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::Book;
    /// let content = "\
    /// ---
    /// title: Foo
    /// output.tex: /tmp/foo.tex
    /// ---
    ///
    /// # Foo
    ///
    /// Bar and baz, too.";
    ///
    /// Book::new()
    ///       .read_markdown_config(content.as_bytes())
    ///       .unwrap()
    ///       .render_all(); // renders foo.tex in /tmp
    /// ```
    pub fn render_all(&self) -> () {
        let mut keys: Vec<_> = self.formats
            .keys()
            .filter(|fmt| {
                if !self.is_proofread() {
                    !fmt.contains("proofread")
                } else {
                    true
                }
            })
            .collect();
        // Make sure that PDF comes first since running latex takes lots of time
        keys.sort_by(|fmt1, fmt2| {
            if fmt1.contains("pdf") {
                Ordering::Less
            } else if fmt2.contains("pdf") {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        keys.par_iter()
            .for_each(|fmt| self.render_format(fmt));

        // if handles.is_empty() {
        //     Logger::display_warning(lformat!("Crowbook generated no file because no output file was \
        //                              specified. Add output.{{format}} to your config file."));
        // }
    }


    /// Render book to specified format according to book options, and write the results
    /// in the `Write` object.
    ///
    /// This method will fail if the format is not handled by the book, or if there is a
    /// problem during rendering, or if the renderer can't render to a byte stream (e.g.
    /// multiple files HTML renderer can't, as it must create a directory.)
    ///
    /// # See also
    /// * `render_format_to_file`, which creates a new file (that *can* be a directory).
    /// * `render_format`, which won't do anything if `output.{format}` isn't specified
    ///   in the book configuration file.
    pub fn render_format_to<T: Write>(&self, format: &str, f: &mut T) -> Result<()> {
        self.logger.debug(lformat!("Attempting to generate {format}...",
                                   format = format));
        match self.formats.get(format) {
            Some(&(ref description, ref renderer)) => {
                renderer.render(self, f)?;
                self.logger.info(lformat!("Succesfully generated {format}",
                                          format = description));
                Ok(())
            },
            None => {
                Err(Error::default(Source::empty(),
                                   lformat!("unknown format {format}",
                                            format = format)))
            }
        }
    }

    /// Render book to specified format according to book options. Creates a new file
    /// and write the result in it.
    ///
    /// This method will fail if the format is not handled by the book, or if there is a
    /// problem during rendering.
    ///
    /// # See also
    /// * `render_format_to`, which writes in any `Write`able object.
    /// * `render_format`, which won't do anything if `output.{format}` isn't specified
    ///   in the book configuration file.
    pub fn render_format_to_file<P:AsRef<Path>>(&self, format: &str, path: P) -> Result<()> {
        self.logger.debug(lformat!("Attempting to generate {format}...",
                                   format = format));
        match self.formats.get(format) {
            Some(&(ref description, ref renderer)) => {
                renderer.render_to_file(self, path.as_ref())?;
                self.logger.info(lformat!("Succesfully generated {format}: {path}",
                                          format = description,
                                          path = misc::normalize(path)));
                Ok(())
            },
            None => {
                Err(Error::default(Source::empty(),
                                   lformat!("unknown format {format}",
                                            format = format)))
            }
        }
    }

    /// Adds a chapter to the book.
    ///
    /// This method is the backend used both by `add_chapter` and `add_chapter_from_source`.
    pub fn add_chapter_from_named_source<R: Read>(&mut self,
                                                  number: Number,
                                                  file: &str,
                                                  mut source: R)
                                                  -> Result<&mut Self> {
        let mut content = String::new();
        source.read_to_string(&mut content)
            .map_err(|_| {
                Error::parser(&self.source,
                              lformat!("file {file} contains invalid UTF-8",
                                       file = misc::normalize(file)))
            })?;

        // Ignore YAML blocks (or not)
        self.parse_yaml(&mut content);

        // parse the file
        let mut parser = Parser::from(self);
        parser.set_source_file(file);
        let mut tokens = parser.parse(&content)?;
        self.features = self.features | parser.features();

        // transform the AST to make local links and images relative to `book` directory
        let offset = if let Some(f) = Path::new(file).parent() {
            f
        } else {
            Path::new("")
        };
        if offset.starts_with("..") {
            self.logger
                .debug(lformat!("Warning: book contains chapter '{file}' in a directory above \
                                   the book file, this might cause problems",
                                  file = misc::normalize(file)));
        }

        
        // For offset: if nothing is specified, it is the filename's directory
        // If base_path.{images/links} is specified, override it for one of them.
        // If base_path is specified, override it for both.
        let res_base = self.options.get_path("resources.base_path");
        let res_base_img = self.options.get_path("resources.base_path.images");
        let res_base_lnk = self.options.get_path("resources.base_path.links");
        let mut link_offset = offset;
        let mut image_offset = offset;
        if let Ok(ref path) = res_base {
            link_offset = Path::new(path);
            image_offset = Path::new(path);
        } else {
            if let Ok(ref path) = res_base_img {
                image_offset = Path::new(path);
            }
            if let Ok(ref path) = res_base_lnk {
                link_offset = Path::new(path);
            }
        }
        // add offset
        ResourceHandler::add_offset(link_offset.as_ref(), image_offset.as_ref(), &mut tokens);

        // Add a title if there is none in the chapter
        misc::insert_title(&mut tokens);

        // If one of the renderers requires it, perform grammarcheck
        if cfg!(feature = "proofread") && self.is_proofread() {
            if let Some(ref checker) = self.checker {
                self.logger
                    .info(lformat!("Trying to run grammar check on {file}, this might take a \
                                    while...",
                                   file = misc::normalize(file)));
                if let Err(err) = checker.check_chapter(&mut tokens) {
                    self.logger.error(lformat!("Error running grammar check on {file}: {error}",
                                               file = misc::normalize(file),
                                               error = err));
                }
            }
            if let Some(ref detector) = self.detector {
                self.logger
                    .info(lformat!("Trying to run repetition detector on {file}, this might take a \
                                    while...",
                                   file = misc::normalize(file)));
                if let Err(err) = detector.check_chapter(&mut tokens) {
                    self.logger.error(lformat!("Error running repetition detector on {file}: {error}",
                                               file = misc::normalize(file),
                                               error = err));
                }
            }
        }

        self.chapters.push(Chapter::new(number, file, tokens));

        Ok(self)
    }

    /// Adds a chapter, as a file name, to the book
    pub fn add_subchapter(&mut self, level: i32, file: &str) -> Result<&mut Self> {
        let number = {
            if let Some(chapter) = self.chapters.last() {
                chapter.number
            } else {
                Number::Hidden
            }
        };
        self.add_chapter(number, file)?;

        // Adjust header levels
        {
            let last = self.chapters.last_mut().unwrap();
            for mut token in &mut last.content {
                match *token {
                    Token::Header(ref mut n, _) => {
                        let new = *n + level;
                        if new > 6 || new < 0 {
                            return Err(Error::parser(Source::new(file),
                                                     lformat!("this subchapter contains a heading that, when adjusted, is not in the right range ({} instead of [0-6])", new)));
                        }
                        *n = new;

                    },
                    _ => {},
                }
            }
        }
        
        Ok(self)
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
    pub fn add_chapter(&mut self, number: Number, file: &str) -> Result<&mut Self> {
        self.logger.debug(lformat!("Parsing chapter: {file}...",
                                   file = misc::normalize(file)));

        // try to open file
        let path = self.root.join(file);
        let f = File::open(&path)
            .map_err(|_| {
                Error::file_not_found(&self.source,
                                      lformat!("book chapter"),
                                      format!("{}", path.display()))
            })?;
     
        self.add_chapter_from_named_source(number, file, f)
    }

    /// Adds a chapter to the book from a source (any object implementing `Read`)
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
    pub fn add_chapter_from_source<R: Read>(&mut self, number: Number, source: R) -> Result<&mut Self> {
        self.add_chapter_from_named_source(number, "", source)
    }


    /// Either clean a string or does nothing,
    /// according to book `lang` and `autoclean` options
    #[doc(hidden)]
    pub fn clean<'s, S: Into<Cow<'s, str>>>(&self, text: S, tex: bool) -> Cow<'s, str> {
        self.cleaner.clean(text.into(), tex)
    }



    /// Returns a template
    ///
    /// Returns the default one if no option was set, or the one set by the user.
    ///
    /// Returns an error if `template` isn't a valid template name.
    #[doc(hidden)]
    pub fn get_template(&self, template: &str) -> Result<Cow<'static, str>> {
        let option = self.options.get_path(template);
        let fallback = match template {
            "epub.css" => epub::CSS,
            "epub.chapter.xhtml" => {
                if self.options.get_i32("epub.version")? == 3 {
                    epub3::TEMPLATE
                } else {
                    epub::TEMPLATE
                }
            }
            "html.css" => html::CSS,
            "html.css.colours" => html::CSS_COLOURS,
            "html.css.print" => html::PRINT_CSS,
            "html.standalone.template" => html_single::HTML,
            "html.standalone.js" => html_single::JS,
            "html.js" => html::JS,
            "html.dir.template" => html_dir::TEMPLATE,
            "html.highlight.js" => highlight::JS,
            "html.highlight.css" => highlight::CSS,
            "tex.template" => latex::TEMPLATE,
            _ => {
                return Err(Error::config_parser(&self.source,
                                                lformat!("invalid template '{template}'",
                                                         template = template)))
            }
        };
        if let Ok(ref s) = option {
            let mut f = File::open(s)
                .map_err(|_| {
                    Error::file_not_found(&self.source,
                                          format!("template '{template}'", template = template),
                                          s.to_owned())
                })?;
            let mut res = String::new();
            f.read_to_string(&mut res)
                .map_err(|_| {
                    Error::config_parser(&self.source,
                                         lformat!("file '{file}' could not be read", file = s))
                })?;
            Ok(Cow::Owned(res))
        } else {
            Ok(Cow::Borrowed(fallback))
        }
    }


    /// Sets the chapter_template once and for all
    fn set_chapter_template(&mut self) -> Result<()> {
        let template =
            compile_str(self.options.get_str("rendering.chapter.template").unwrap(),
                        &self.source,
                        "rendering.chapter.template")?;
        self.chapter_template = Some(template);
        Ok(())
    }

    /// Returns the formatted (roman or arabic) number of chapter
    #[doc(hidden)]
    pub fn get_header_number(&self, header: Header, n: i32) -> Result<String> {
        let boolean = match header {
            Header::Part => self.options.get_bool("rendering.part.roman_numerals").unwrap(),
            Header::Chapter => self.options.get_bool("rendering.chapter.roman_numerals").unwrap(),
        };
        let number = if boolean {
            if n <= 0 {
                return Err(Error::render(Source::empty(),
                                            lformat!("can not use roman numerals with zero or negative chapter numbers ({n})",
                                                     n = n)));
            }
            format!("{:X}", Roman::from(n as i16))
        } else {
            format!("{}", n)
        };
        Ok(number)
    }


    /// Returns the string corresponding to a number, title, and the numbering template for chapter
    #[doc(hidden)]
    pub fn get_header<F>(&self, header: Header, n: i32, title: String, mut f: F) -> Result<HeaderData>
        where F: FnMut(&str) -> Result<String>
    {
        let header_type = match header {
            Header::Part => "part",
            Header::Chapter => "chapter",
        };
        let mut data = self.get_metadata(&mut f)?;
        if !title.is_empty() {
            data = data.insert_bool(&format!("has_{}_title", header_type),
                                    true);
        }
        let number = self.get_header_number(header, n)?;
        let header_name = self
            .options
            .get_str(&format!("rendering.{}", header_type))
            .map(|s| s.to_owned())
            .unwrap_or_else(|_| lang::get_str(self.options.get_str("lang").unwrap(),
                                              header_type));

        data = data.insert_str(format!("{}_title", header_type),
                               title.clone())
            .insert_str(header_type, header_name.clone())
            .insert_str("number", number.clone());
        let data = data.build();
        let mut res: Vec<u8> = vec![];

        let opt_template = match header {
            Header::Part => &self.part_template,
            Header::Chapter => &self.chapter_template,
        };
        
        if let Some(ref template) = *opt_template {
            template.render_data(&mut res, &data)?;
        } else {
            let template =
                compile_str(self.options.get_str(&format!("rendering.{}.template", header_type))
                            .unwrap(),
                            &self.source,
                            &format!("rendering.{}.template", header_type))?;
            template.render_data(&mut res, &data)?;
        }

        match String::from_utf8(res) {
            Err(_) => panic!(lformat!("header generated by mustache was not valid utf-8")),
            Ok(res) => Ok(HeaderData {
                text: res,
                number: number,
                header: header_name,
                title: title,
            }),
        }
    }

    /// Returns the string corresponding to a number, title, and the numbering template for chapter
    #[doc(hidden)]
    pub fn get_chapter_header<F>(&self, n: i32, title: String, f: F) -> Result<HeaderData>
        where F: FnMut(&str) -> Result<String>
    {
        self.get_header(Header::Chapter, n, title, f)
    }

    /// Returns the string corresponding to a number, title, and the numbering template for part
    #[doc(hidden)]
    pub fn get_part_header<F>(&self, n: i32, title: String, f: F) -> Result<HeaderData>
        where F: FnMut(&str) -> Result<String>
    {
        self.get_header(Header::Part, n, title, f)
    }

    /// Returns a `MapBuilder` (used by `Mustache` for templating), to be used (and completed)
    /// by renderers. It fills it with the metadata options.
    ///
    /// It also uses the lang/xx.yaml file corresponding to the language and fills
    /// `loc_xxx` fiels with it that corresponds to translated versions.
    ///
    /// This method treats the metadata as Markdown and thus calls `f` to render it.
    #[doc(hidden)]
    pub fn get_metadata<F>(&self, mut f: F) -> Result<MapBuilder>
        where F: FnMut(&str) -> Result<String>
    {
        let mut mapbuilder = MapBuilder::new();
        mapbuilder = mapbuilder.insert_str("crowbook_version", env!("CARGO_PKG_VERSION"));
        mapbuilder =
            mapbuilder.insert_bool(&format!("lang_{}", self.options.get_str("lang").unwrap()),
                                   true);

        // Add metadata to mapbuilder
        for key in self.options.get_metadata() {
            if let Ok(s) = self.options.get_str(key) {
                let key = key.replace(".", "_");

                // Don't render lang as markdown
                let content = match key.as_ref() {
                    "lang" => Ok(s.to_string()),
                    _ => f(s),
                };
                let raw = view_as_text(&Parser::from(&self).parse(s)?);
                match content {
                    Ok(content) => {
                        mapbuilder = mapbuilder.insert_str(&format!("{}_raw", key), raw);
                        mapbuilder = mapbuilder.insert_str(&key, content);
                        mapbuilder = mapbuilder.insert_bool(&format!("has_{}", key), true);
                    }
                    Err(err) => {
                        return Err(Error::render(&self.source,
                                                 lformat!("could not render `{key}` for \
                                                           metadata:\n{error}",
                                                          key = &key,
                                                          error = err)));
                    }
                }
            }
        }

        // Add localization strings
        let hash = lang::get_hash(self.options.get_str("lang").unwrap());
        for (key, value) in hash {
            let key = format!("loc_{}", key.as_str().unwrap());
            let value = value.as_str().unwrap();
            mapbuilder = mapbuilder.insert_str(&key, value);
        }
        Ok(mapbuilder)
    }

    /// Remove YAML blocks from a string and try to parse them to set options
    ///
    /// YAML blocks start with
    /// ---
    /// and end either with
    /// ---
    /// or
    /// ...
    fn parse_yaml(&mut self, content: &mut String) {
        if !(content.starts_with("---\n") || content.contains("\n---\n") ||
             content.starts_with("---\r\n") || content.contains("\n---\r\n")) {
            // Content can't contain YAML, so aborting early
            return;
        }
        let mut new_content = String::new();
        let mut previous_empty = true;
        {
            let mut lines = content.lines();
            while let Some(line) = lines.next() {
                if line == "---" && previous_empty {
                    previous_empty = false;
                    let mut yaml_block = String::new();
                    let mut valid_block = false;
                    while let Some(new_line) = lines.next() {
                        if new_line == "---" || new_line == "..." {
                            // Checks that this is valid YAML
                            match YamlLoader::load_from_str(&yaml_block) {
                                Ok(docs) => {
                                    // Use this yaml block to set options only if 1) it is valid
                                    // 2) the option is activated
                                    if docs.len() == 1 && docs[0].as_hash().is_some() &&
                                       self.options.get_bool("input.yaml_blocks") == Ok(true) {
                                        let hash = docs[0].as_hash().unwrap();
                                        for (key, value) in hash {
                                            match self.options
                                                //todo: remove clone
                                                .set_yaml(key.clone(), value.clone()) {
                                                Ok(opt) => {
                                                    if let Some(old_value) = opt {
                                                        self.logger
                                                            .debug(lformat!("Inline YAML block \
                                                                             replaced {:?} \
                                                                             previously set to \
                                                                             {:?} to {:?}",
                                                                            key,
                                                                            old_value,
                                                                            value));
                                                    } else {
                                                        self.logger
                                                            .debug(lformat!("Inline YAML block \
                                                                             set {:?} to {:?}",
                                                                            key,
                                                                            value));
                                                    }
                                                }
                                                Err(e) => {
                                                    self.logger
                                                        .error(lformat!("Inline YAML block could \
                                                                        not set {:?} to {:?}: {}",
                                                                       key,
                                                                       value,
                                                                       e))
                                                }
                                            }
                                        }
                                    } else {
                                        self.logger.debug(lformat!("Ignoring YAML \
                                                                    block:\n---\n{block}---",
                                                                   block = &yaml_block));
                                    }
                                    valid_block = true;
                                }
                                Err(err) => {
                                    self.logger
                                        .error(lformat!("Found something that looked like a \
                                                         YAML block:\n{block}",
                                                        block = &yaml_block));
                                    self.logger
                                        .error(lformat!("... but it didn't parse correctly as \
                                                         YAML('{error}'), so treating it like \
                                                         Markdown.",
                                                        error = err));
                                }
                            }
                            break;
                        } else {
                            yaml_block.push_str(new_line);
                            yaml_block.push_str("\n");
                        }
                    }
                    if !valid_block {
                        // Block was invalid, so add it to markdown content
                        new_content.push_str(&yaml_block);
                        new_content.push_str("\n");
                    }
                } else if line.is_empty() {
                    previous_empty = true;
                    new_content.push_str("\n");
                } else {
                    previous_empty = false;
                    new_content.push_str(line);
                    new_content.push_str("\n");
                }
            }
        }
        *content = new_content;
        self.update_cleaner();
        self.init_checker();
    }


    // Update the cleaner according to autoclean and lang options
    fn update_cleaner(&mut self) {
        let params = CleanerParams {
            smart_quotes: self.options.get_bool("input.clean.smart_quotes").unwrap(),
            ligature_dashes: self.options.get_bool("input.clean.ligature.dashes").unwrap(),
            ligature_guillemets: self.options.get_bool("input.clean.ligature.guillemets").unwrap(),
        };
        if self.options.get_bool("input.clean").unwrap() {
            let lang = self.options.get_str("lang").unwrap().to_lowercase();
            let cleaner: Box<Cleaner> = if lang.starts_with("fr") {
                Box::new(French::new(params))
            } else {
                Box::new(Default::new(params))
            };
            self.cleaner = cleaner;
        } else {
            self.cleaner = Box::new(Off);
        }
    }
}


/// Calls mustache::compile_str but catches panics and returns a result
pub fn compile_str<O>(template: &str, source: O, template_name: &str) -> Result<mustache::Template>
    where O: Into<Source>
{
    let input: String = template.to_owned();
    let result = mustache::compile_str(&input);
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(Error::template(source, lformat!("could not compile '{template}': {error}",
                                                         template = template_name,
                                                         error = err))),
    }
}
