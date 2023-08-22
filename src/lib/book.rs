// Copyright (C) 2016-2023 Élisabeth HENRY.
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

use crate::book_bars::Bars;
use crate::book_renderer::BookRenderer;
use crate::bookoptions::BookOptions;
use crate::chapter::Chapter;
use crate::cleaner::{Cleaner, CleanerParams, Default, French, Off};
use crate::epub::Epub;
use crate::error::{Error, Result, Source};
use crate::html_dir::HtmlDir;
use crate::html_if::HtmlIf;
use crate::html_single::HtmlSingle;
use crate::lang;
use crate::latex::{Latex, Pdf};
use crate::misc;
use crate::number::Number;
use crate::parser::Features;
use crate::parser::Parser;
use crate::resource_handler::ResourceHandler;
use crate::templates::{epub, epub3, highlight, html, html_dir, html_if, html_single, latex};
use crate::text_view::view_as_text;
use crate::token::Token;

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{HashMap, BTreeMap};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::iter::IntoIterator;
use std::path::{Path, PathBuf};

use numerals::roman::Roman;
use rayon::prelude::*;
use yaml_rust::{Yaml, YamlLoader};
use rust_i18n::t;

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

/// The types of bars
#[derive(Copy, Clone)]
pub enum Crowbar {
    Main,
    Second,
    Spinner(usize),
}

/// The state of bars
#[derive(Copy, Clone)]
pub enum CrowbarState {
    Running,
    Success,
    Error,
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
pub struct Book<'a> {
    /// Internal structure. You should not accesss this directly except if
    /// you are writing a new renderer.
    pub chapters: Vec<Chapter>,

    /// Options of the book
    pub options: BookOptions,

    /// Root path of the book
    #[doc(hidden)]
    pub root: PathBuf,

    /// Source for error files
    #[doc(hidden)]
    pub source: Source,

    /// Features used in the book content
    #[doc(hidden)]
    pub features: Features,

    cleaner: Box<dyn Cleaner>,
    formats: HashMap<&'static str, (String, Box<dyn BookRenderer>)>,

    #[doc(hidden)]
    pub bars: Bars,

    /// Store the templates registry
    pub registry: upon::Engine<'a>,
}

impl<'a> Book<'a> {
    /// Creates a new, empty `Book`
    pub fn new() -> Book<'a> {
        let mut book = Book {
            source: Source::empty(),
            chapters: vec![],
            cleaner: Box::new(Off),
            root: PathBuf::new(),
            options: BookOptions::new(),
            formats: HashMap::new(),
            features: Features::new(),
            bars: Bars::new(),
            registry: upon::Engine::new(),
        };

        // Add some filters to registry that are useful for some templates
        book.registry.add_filter("eq", str::eq);
        book.registry.add_filter("starts", |a: &str, b: &str| a.starts_with(b));

        book.add_format(
            "html",
            t!("format.html_single"),
            Box::new(HtmlSingle {}),
        )
        .add_format(
            "html.dir",
            t!("format.html_dir"),
            Box::new(HtmlDir {}),
        )
        .add_format("tex", t!("format.tex"), Box::new(Latex {}))
        .add_format("pdf", t!("format.pdf"), Box::new(Pdf {}))
        .add_format("epub", t!("format.epub"), Box::new(Epub {}))
        .add_format(
            "html.if",
            t!("html_if"),
            Box::new(HtmlIf {}),
        );
        book
    }

    /// Sets an error message to the progress bar, if it is set
    pub fn set_error(&self, msg: &str) {
        self.bar_finish(Crowbar::Main, CrowbarState::Error, msg)
    }

    /// Adds a progress bar where where info should be written.
    ///
    /// See [indicatif doc](https://docs.rs/indicatif) for more information.
    pub fn add_progress_bar(&mut self, emoji: bool) {
        self.private_add_progress_bar(emoji);
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
    pub fn add_format<S: Into<String>>(
        &mut self,
        format: &'static str,
        description: S,
        renderer: Box<dyn BookRenderer>,
    ) -> &mut Self {
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
    pub fn set_options<'b, I>(&mut self, options: I) -> &mut Self
    where
        I: IntoIterator<Item = &'b (&'b str, &'b str)>,
    {
        // set options
        for (key, value) in options {
            if let Err(err) = self.options.set(key, value) {
                error!(
                    "{}",
                    t!(
                        "error.book_init",
                        key = key,
                        value = value,
                        error = err
                    )
                );
            }
        }
        // set cleaner according to lang and autoclean settings
        self.update_cleaner();
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
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let filename = format!("{}", path.as_ref().display());
        self.source = Source::new(filename.as_str());
        self.options.source = Source::new(filename.as_str());

        let f = File::open(path.as_ref()).map_err(|_| {
            Error::file_not_found(Source::empty(), t!("format.book"), filename.clone())
        })?;
        // Set book path to book's directory
        if let Some(parent) = path.as_ref().parent() {
            self.root = parent.to_owned();
            self.options.root = self.root.clone();
        }

        match self.read_config(&f) {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.is_config_parser() && path.as_ref().ends_with(".md") {
                    let err = Error::default(
                        Source::empty(),
                        t!("error.parse_book",
                           file = misc::normalize(path)
                        ),
                    );
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
    pub fn load_markdown_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
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
        let relative_path = Path::new(path.as_ref().components().last().unwrap().as_os_str());

        // Update grammar checker according to options
        self.add_chapter(Number::Hidden, &relative_path.to_string_lossy(), false)?;

        Ok(())
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
    pub fn read_markdown_config<R: Read>(&mut self, source: R) -> Result<()> {
        self.options.set("tex.class", "article").unwrap();
        self.options.set("input.yaml_blocks", "true").unwrap();

        // Update grammar checker according to options
        self.add_chapter_from_source(Number::Hidden, source, false)?;

        Ok(())
    }

    /// Sets options from a YAML block
    fn set_options_from_yaml(&mut self, yaml: &str) -> Result<&mut Self> {
        self.options.source = self.source.clone();
        match YamlLoader::load_from_str(yaml) {
            Err(err) => {
                return Err(Error::config_parser(
                    &self.source,
                    t!("error.yaml_block", error = err),
                ))
            }
            Ok(mut docs) => {
                if docs.len() == 1 && docs[0].as_hash().is_some() {
                    if let Yaml::Hash(hash) = docs.pop().unwrap() {
                        for (key, value) in hash {
                            if let Err(err) = self.options.set_yaml(key, value) {
                                error!("{}", err);
                            };
                        }
                    } else {
                        unreachable!();
                    }
                } else {
                    return Err(Error::config_parser(
                        &self.source,
                        t!("error.parse_book"),
                    ));
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
    pub fn read_config<R: Read>(&mut self, mut source: R) -> Result<()> {
        fn get_filename<'b>(source: &Source, s: &'b str) -> Result<&'b str> {
            let words: Vec<&str> = (s[1..]).split_whitespace().collect();
            if words.len() > 1 {
                return Err(Error::config_parser(
                    source,
                    t!("error.chapter_whitespace"),
                ));
            } else if words.is_empty() {
                return Err(Error::config_parser(
                    source,
                    t!("error.no_chapter_name"),
                ));
            }
            Ok(words[0])
        }

        self.bar_set_message(Crowbar::Main, &t!("ui.options"));

        let mut s = String::new();
        source.read_to_string(&mut s).map_err(|err| {
            Error::config_parser(
                Source::empty(),
                t!("error.source", error = err),
            )
        })?;

        // Parse the YAML block, that is, until first chapter
        let mut yaml = String::new();
        let mut lines = s.lines().peekable();
        let mut line;

        let mut line_number = 0;
        let mut is_next_line_ok: bool;

        while let Some(next_line) = lines.peek() {
            if next_line.starts_with(|c| match c {
                '-' | '+' | '!' | '@' => true,
                _ => c.is_ascii_digit(),
            }) {
                break;
            }

            line = lines.next().unwrap();
            line_number += 1;
            self.source.set_line(line_number);
            yaml.push_str(line);
            yaml.push('\n');

            if line
                .trim()
                .ends_with(|c| matches!(c, '>' | '|' | ':' | '-'))
            {
                // line ends with the start of a block indicator
                continue;
            }

            if let Some(next_line) = lines.peek() {
                let doc = YamlLoader::load_from_str(next_line);
                if let Ok(doc) = doc {
                    if !doc.is_empty() && doc[0].as_hash().is_some() {
                        is_next_line_ok = true;
                    } else {
                        is_next_line_ok = false;
                    }
                } else {
                    is_next_line_ok = false;
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

        self.bar_set_message(Crowbar::Main, &t!("ui.chapters"));

        // Parse chapters
        let lines: Vec<_> = lines.collect();
        self.add_second_bar(&t!("ui.processing"), lines.len() as u64);
        for line in lines {
            self.inc_second_bar();
            line_number += 1;
            self.source.set_line(line_number);
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("--") {
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
                self.add_chapter(Number::Unnumbered, file, false)?;
            } else if line.starts_with('+') {
                // numbered chapter
                let file = get_filename(&self.source, line)?;
                self.add_chapter(Number::Default, file, true)?;
            } else if line.starts_with('!') {
                // hidden chapter
                let file = get_filename(&self.source, line)?;
                self.add_chapter(Number::Hidden, file, false)?;
            } else if line.starts_with(|c: char| c.is_ascii_digit()) {
                // chapter with specific number
                let parts: Vec<_> = line
                    .splitn(2, |c: char| c == '.' || c == ':' || c == '+')
                    .collect();
                if parts.len() != 2 {
                    return Err(Error::config_parser(
                        &self.source,
                        t!("error.format_line"),
                    ));
                }
                let file = get_filename(&self.source, parts[1])?;
                let number = parts[0].parse::<i32>().map_err(|err| {
                    Error::config_parser(
                        &self.source,
                        t!("error.chapter_number", error = err),
                    )
                })?;
                self.add_chapter(Number::Specified(number), file, true)?;
            } else if let Some(subline) = line.strip_prefix('@') {
                /* Part */
                if subline.starts_with(|c: char| c.is_whitespace()) {
                    let subline = subline.trim();
                    let ast = Parser::from(self).parse_inline(subline)?;
                    let ast = vec![Token::Header(1, ast)];
                    self.chapters
                        .push(Chapter::new(Number::DefaultPart, String::new(), ast));
                } else if subline.starts_with('-') {
                    /* Unnumbered part */
                    let file = get_filename(&self.source, subline)?;
                    self.add_chapter(Number::UnnumberedPart, file, true)?;
                } else if subline.starts_with('+') {
                    /* Numbered part */
                    let file = get_filename(&self.source, subline)?;
                    self.add_chapter(Number::DefaultPart, file, true)?;
                } else if subline.starts_with(|c: char| c.is_ascii_digit()) {
                    /* Specified  part*/
                    let parts: Vec<_> = subline
                        .splitn(2, |c: char| c == '.' || c == ':' || c == '+')
                        .collect();
                    if parts.len() != 2 {
                        return Err(Error::config_parser(
                            &self.source,
                            t!("error.part_number_line")
                        ));
                    }
                    let file = get_filename(&self.source, parts[1])?;
                    let number = parts[0].parse::<i32>().map_err(|err| {
                        Error::config_parser(
                            &self.source,
                            t!("error.part_number", error = err),
                        )
                    })?;
                    self.add_chapter(Number::SpecifiedPart(number), file, true)?;
                } else {
                    return Err(Error::config_parser(
                        &self.source,
                        t!("error.part_definition"),
                    ));
                }
            } else {
                return Err(Error::config_parser(
                    &self.source,
                    t!("error.chapter_definition"),
                ));
            }
        }

        self.bar_finish(Crowbar::Second, CrowbarState::Success, "");

        self.source.unset_line();
        self.set_chapter_template()?;
        Ok(())
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
    pub fn render_all(&mut self) {
        let mut keys: Vec<_> = self
            .formats
            .keys()
            .filter(|fmt| {
                self.options.get_path(&format!("output.{fmt}")).is_ok()
            })
            .map(|s| s.to_string())
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

        for key in &keys {
            self.add_spinner_to_multibar(key);
        }

        keys.par_iter().enumerate().for_each(|(i, fmt)| {
            self.render_format_with_bar(fmt, i);
        });

        self.bar_finish(Crowbar::Main, CrowbarState::Success, &t!("ui.finished"));

        // if handles.is_empty() {
        //     Logger::display_warning(lformat!("Crowbook generated no file because no output file was \
        //                              specified. Add output.{{format}} to your config file."));
        // }
    }

    /// Renders the book to the given format and reports to progress bar if set
    pub fn render_format_with_bar(&self, format: &str, bar: usize) {
        let mut key = String::from("output.");
        key.push_str(format);
        if let Ok(path) = self.options.get_path(&key) {
            self.bar_set_message(Crowbar::Spinner(bar), &t!("ui.rendering_format"));
            let result = self.render_format_to_file_with_bar(format, path, bar);
            if let Err(err) = result {
                self.bar_finish(
                    Crowbar::Spinner(bar),
                    CrowbarState::Error,
                    &format!("{err}"),
                );
                error!(
                    "{}",
                    t!("error.rendering",
                        name = format,
                        error = err
                    )
                );
            }
        }
    }

    pub fn render_format_to_file_with_bar<P: Into<PathBuf>>(
        &self,
        format: &str,
        path: P,
        bar: usize,
    ) -> Result<()> {
        debug!(
            "{}",
            t!("msg.attempting", format = format)
        );
        let path = path.into();
        match self.formats.get(format) {
            Some((description, renderer)) => {
                let path = if path.ends_with("auto") {
                    let file = if let Some(s) = self
                        .source
                        .file
                        .as_ref()
                        .and_then(|f| Path::new(f).file_stem())
                    {
                        s.to_string_lossy().into_owned()
                    } else {
                        return Err(Error::default(&self.source, t!("error.infer",
                                                                     format = description)));
                    };
                    let file = renderer.auto_path(&file).map_err(|_| {
                        Error::default(
                            &self.source,
                            t!("error.support",
                                format = description
                            ),
                        )
                    })?;
                    path.with_file_name(file)
                } else {
                    path
                };
                renderer.render_to_file(self, &path)?;
                let path = misc::normalize(path);
                let msg = t!(
                    "msg.generated",
                    format = description,
                    path = &path
                );
                info!("{}", &msg);
                self.bar_finish(
                    Crowbar::Spinner(bar),
                    CrowbarState::Success,
                    &t!("ui.generated", path = path),
                );
                Ok(())
            }
            None => Err(Error::default(
                Source::empty(),
                t!("error.unknown", format = format),
            )),
        }
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
    pub fn render_format_to<T: Write>(&mut self, format: &str, f: &mut T) -> Result<()> {
        debug!(
            "{}",
            t!("msg.attempting", format = format)
        );
        let bar = self.add_spinner_to_multibar(format);
        match self.formats.get(format) {
            Some((description, renderer)) => match renderer.render(self, f) {
                Ok(_) => {
                    self.bar_finish(
                        Crowbar::Spinner(bar),
                        CrowbarState::Success,
                        &t!("ui.generated", path = format),
                    );
                    self.bar_finish(Crowbar::Main, CrowbarState::Success, &t!("ui.finished"));
                    info!(
                        "{}",
                        t!("msg.generated_short", format = description)
                    );
                    Ok(())
                }
                Err(e) => {
                    self.bar_finish(
                        Crowbar::Spinner(bar),
                        CrowbarState::Error,
                        &format!("{error}", error = e),
                    );
                    self.bar_finish(Crowbar::Main, CrowbarState::Error, &t!("ui.error"));
                    Err(e)
                }
            },
            None => {
                self.bar_finish(
                    Crowbar::Spinner(bar),
                    CrowbarState::Error,
                    &t!("error.unknown_short"),
                );
                Err(Error::default(
                    Source::empty(),
                    t!("error.unknown", format = format),
                ))
            }
        }
    }

    /// Render book to specified format according to book options. Creates a new file
    /// and write the result in it.
    ///
    /// This method will fail if the format is not handled by the book, or if there is a
    /// problem during rendering.
    ///
    /// # Arguments
    ///
    /// * `format`: the format to render;
    /// * `path`: a path to the file that will be created;
    /// * `bar`: a Progressbar, or `None`
    ///
    /// # See also
    /// * `render_format_to`, which writes in any `Write`able object.
    /// * `render_format`, which won't do anything if `output.{format}` isn't specified
    ///   in the book configuration file.

    pub fn render_format_to_file<P: Into<PathBuf>>(&mut self, format: &str, path: P) -> Result<()> {
        let bar = self.add_spinner_to_multibar(format);
        self.render_format_to_file_with_bar(format, path, bar)?;
        self.bar_finish(Crowbar::Main, CrowbarState::Success, &t!("ui.finished"));
        Ok(())
    }

    /// Adds a chapter to the book.
    ///
    /// This method is the backend used both by `add_chapter` and `add_chapter_from_source`.
    pub fn add_chapter_from_named_source<R: Read>(
        &mut self,
        number: Number,
        file: &str,
        mut source: R,
        mut add_title_if_empty: bool,
    ) -> Result<&mut Self> {
        self.bar_set_message(
            Crowbar::Main,
            &t!("ui.processing_file", file = file),
        );
        let mut content = String::new();
        source.read_to_string(&mut content).map_err(|_| {
            Error::parser(
                &self.source,
                t!(
                    "error.utf8",
                    file = misc::normalize(file)
                ),
            )
        })?;

        // parse the file
        self.bar_set_message(Crowbar::Second, &t!("ui.parsing..."));

        let mut parser = Parser::from(self);
        parser.set_source_file(file);
        let mut yaml_block = String::from("");
        let mut tokens = parser.parse(&content, Option::Some(&mut yaml_block))?;

        // Parse YAML block
        self.parse_yaml(&yaml_block);
        self.features = self.features | parser.features();

        // transform the AST to make local links and images relative to `book` directory
        let offset = if let Some(f) = Path::new(file).parent() {
            f
        } else {
            Path::new("")
        };
        if offset.starts_with("..") {
            debug!(
                "{}",
                t!(
                    "warn.above",
                    file = misc::normalize(file)
                )
            );
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
        ResourceHandler::add_offset(link_offset, image_offset, &mut tokens);

        // If files_mean_chapters is set, override the default setting
        if let Ok(x) = self.options.get_bool("crowbook.files_mean_chapters") {
            add_title_if_empty = x;
        }

        // Add a title if there is none in the chapter (unless this is subchapter)
        if add_title_if_empty {
            misc::insert_title(&mut tokens);
        }

        self.bar_set_message(Crowbar::Second, "");

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
        self.add_chapter(number, file, false)?;

        // Adjust header levels
        {
            let last = self.chapters.last_mut().unwrap();
            for token in &mut last.content {
                if let Token::Header(ref mut n, _) = *token {
                    let new = *n + level;
                    if !(0..=6).contains(&new) {
                        return Err(Error::parser(Source::new(file),
                                                     t!("error.heading", n = new)));
                    }
                    *n = new;
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
    pub fn add_chapter(
        &mut self,
        number: Number,
        file: &str,
        add_title_if_empty: bool,
    ) -> Result<&mut Self> {
        self.bar_set_message(
            Crowbar::Main,
            &t!("ui.parsing_file", file = misc::normalize(file)),
        );

        // try to open file
        let path = self.root.join(file);
        let f = File::open(&path).map_err(|_| {
            Error::file_not_found(
                &self.source,
                t!("format.book_chapter"),
                format!("{}", path.display()),
            )
        })?;

        self.add_chapter_from_named_source(number, file, f, add_title_if_empty)
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
    pub fn add_chapter_from_source<R: Read>(
        &mut self,
        number: Number,
        source: R,
        add_title_if_empty: bool,
    ) -> Result<&mut Self> {
        self.add_chapter_from_named_source(number, "", source, add_title_if_empty)
    }

    /// Either clean a string or does nothing,
    /// according to book `lang` and `autoclean` options
    #[doc(hidden)]
    pub fn clean<'s, S: Into<Cow<'s, str>>>(&self, text: S) -> Cow<'s, str> {
        self.cleaner.clean(text.into())
    }

    /// Returns a template
    ///
    /// Returns the default one if no option was set, or the one set by the user.
    ///
    /// Returns an error if `template` isn't a valid template name.
    #[doc(hidden)]
    pub fn get_template(&self, template: &str) -> Result<Cow<'static, str>> {
        let option = self.options.get_path(template);
        let epub3 = template.starts_with("epub") && self.options.get_i32("epub.version")? == 3;
        let fallback = match template {
            "epub.css" => epub::CSS,
            "epub.titlepage.xhtml" => {
                if epub3 {
                    epub3::TITLE
                } else {
                    epub::TITLE
                }
            }
            "epub.chapter.xhtml" => {
                if epub3 {
                    epub3::TEMPLATE
                } else {
                    epub::TEMPLATE
                }
            }
            "html.css" => html::CSS,
            "html.css.colors" => html::CSS_COLORS,
            "html.css.print" => html::PRINT_CSS,
            "html.standalone.template" => html_single::HTML,
            "html.standalone.js" => html_single::JS,
            "html.js" => html::JS,
            "html.dir.template" => html_dir::TEMPLATE,
            "html.highlight.js" => highlight::JS,
            "html.highlight.css" => highlight::CSS,
            "html.if.js" => html_if::JS,
            "html.if.new_game" => html_if::NEW_GAME,
            "tex.template" => latex::TEMPLATE,
            _ => {
                return Err(Error::config_parser(
                    &self.source,
                    t!("error.invalid_template"),
                ))
            }
        };
        if let Ok(ref s) = option {
            let mut f = File::open(s).map_err(|_| {
                Error::file_not_found(&self.source, format!("template '{template}'"), s.to_owned())
            })?;
            let mut res = String::new();
            f.read_to_string(&mut res).map_err(|_| {
                Error::config_parser(
                    &self.source,
                    t!("error.read_file", file = s),
                )
            })?;
            Ok(Cow::Owned(res))
        } else {
            Ok(Cow::Borrowed(fallback))
        }
    }

    /// Sets the chapter_template once and for all (also sets part template)
    fn set_chapter_template(&mut self) -> Result<()> {
        self.register_template("rendering.chapter.template")?;
        self.register_template("rendering.part.template")?;
        Ok(())
    }

    
    fn register_template(&mut self, tpl: &'static str) -> Result<()> {
        self.registry.add_template(tpl,
            self.options.get_str(tpl).unwrap().to_owned())
            .map_err(|e| Error::template(
                &self.source,
                t!(
                    "error.compile_template",
                    template = "tpl",
                    error = e
                ))
            )?;
        Ok(())
    }

    /// Returns the formatted (roman or arabic) number of chapter
    #[doc(hidden)]
    pub fn get_header_number(&self, header: Header, n: i32) -> Result<String> {
        let boolean = match header {
            Header::Part => self
                .options
                .get_bool("rendering.part.roman_numerals")
                .unwrap(),
            Header::Chapter => self
                .options
                .get_bool("rendering.chapter.roman_numerals")
                .unwrap(),
        };
        let number = if boolean {
            if n <= 0 {
                return Err(Error::render(
                    Source::empty(),
                    t!(
                        "error.roman_numerals",
                        n = n
                    ),
                ));
            }
            format!("{:X}", Roman::from(n as i16))
        } else {
            format!("{n}")
        };
        Ok(number)
    }

    /// Returns the string corresponding to a number, title, and the numbering template for chapter
    #[doc(hidden)]
    pub fn get_header<F>(
        &self,
        header: Header,
        n: i32,
        title: String,
        mut f: F,
    ) -> Result<HeaderData>
    where
        F: FnMut(&str) -> Result<String>,
    {
        let header_type = match header {
            Header::Part => "part",
            Header::Chapter => "chapter",
        };
        let mut data = self.get_metadata(&mut f)?;
        if !title.is_empty() {
            data.insert(format!("has_{header_type}_title"), true.into());
        }
        let number = self.get_header_number(header, n)?;
        let header_name = self
            .options
            .get_str(&format!("rendering.{header_type}"))
            .map(|s| s.to_owned())
            .unwrap_or_else(|_| lang::get_str(self.options.get_str("lang").unwrap(), header_type));

        data.insert(format!("{header_type}_title"), title.clone().into());
        data.insert(header_type.into(), header_name.clone().into());
        data.insert("number".into(), number.clone().into());

        let res = self.registry.get_template(&format!("rendering.{header_type}.template"))
            .expect(&format!("Error accessing template rendering.{header_type}.template"))
            .render(&data)
            .to_string()?;
        Ok(HeaderData {
            text: res,
            number,
            header: header_name,
            title,
        })
    }

    /// Returns the string corresponding to a number, title, and the numbering template for chapter
    #[doc(hidden)]
    pub fn get_chapter_header<F>(&self, n: i32, title: String, f: F) -> Result<HeaderData>
    where
        F: FnMut(&str) -> Result<String>,
    {
        self.get_header(Header::Chapter, n, title, f)
    }

    /// Returns the string corresponding to a number, title, and the numbering template for part
    #[doc(hidden)]
    pub fn get_part_header<F>(&self, n: i32, title: String, f: F) -> Result<HeaderData>
    where
        F: FnMut(&str) -> Result<String>,
    {
        self.get_header(Header::Part, n, title, f)
    }

    /// Returns a `Map of Key/Value` (used by `Upon` for templating), to be used (and completed)
    /// by renderers. It fills it with the metadata options.
    ///
    /// It also uses the lang/xx.yaml file corresponding to the language and fills
    /// `loc_xxx` fiels with it that corresponds to translated versions.
    ///
    /// This method treats the metadata as Markdown and thus calls `f` to render it.
    /// This is why we can’t really cache this as it will depend on the renderer. 
    #[doc(hidden)]
    pub fn get_metadata<F>(&self, mut f: F) -> Result<BTreeMap<String, upon::Value>>
    where
        F: FnMut(&str) -> Result<String>,
    {
        let mut m: BTreeMap<String, upon::Value> = BTreeMap::new();
        m.insert("crowbook_version".into(), env!("CARGO_PKG_VERSION").into());
        m.insert(format!("lang_{}", self.options.get_str("lang").unwrap()), true.into());

        // Add metadata to map
        for key in self.options.get_metadata() {
            if let Ok(s) = self.options.get_str(key) {
                let key = key.replace('.', "_");

                // Don't render lang as markdown
                let content = match key.as_ref() {
                    "lang" => Ok(s.to_string()),
                    _ => f(s),
                };
                let raw = view_as_text(&Parser::from(self).parse(s, None)?);
                match content {
                    Ok(content) => {
                        if !content.is_empty() {
                            m.insert(format!("{key}_raw"), raw.into());
                            m.insert(key.clone(), content.into());

                            m.insert(format!("has_{key}"), true.into());
                        } else {
                            m.insert(format!("{key}_raw"), "".into());
                            m.insert(key.clone(), "".into());
                            m.insert(format!("has_{key}"), false.into());
                        }
                    }
                    Err(err) => {
                        return Err(Error::render(
                            &self.source,
                            t!(
                                "error.render_key",
                                key = &key,
                                error = err
                            ),
                        ));
                    }
                }
            } else {
                m.insert(key.clone(), "".into());
                m.insert(format!("has_{key}"), false.into());
            }
        }

        // Add localization strings
        let hash = lang::get_hash(self.options.get_str("lang").unwrap());
        for (key, value) in hash {
            let key = format!("loc_{}", key.as_str().unwrap());
            let value = value.as_str().unwrap();
            m.insert(key, value.into());
        }
        Ok(m)
    }

    /// Calls upon::engine::compile, does NOT registre the complete 
    pub fn compile_str<'s, O>(&self, template: &'s str, source: O, template_name: &str) -> Result<upon::Template<'_, 's>>
    where
        O: Into<Source>,
    {
        let result = self.registry.compile(template);
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::template(
                source,
                t!(
                    "error.compile_template",
                    template = template_name,
                    error = format!("{:#}", err)
                ),
            )),
        }
    }


    /// Remove YAML blocks from a string and try to parse them to set options
    ///
    /// YAML blocks start with
    /// ---
    /// and end either with
    /// ---
    /// or
    /// ...
    fn parse_yaml(&mut self, yaml_block: &String) {
        // Checks that this is valid YAML
        match YamlLoader::load_from_str(yaml_block) {
            Ok(docs) => {
                // Use this yaml block to set options only if 1) it is valid
                // 2) the option is activated
                if !docs.is_empty() && docs[0].as_hash().is_some() {
                    let hash = docs[0].as_hash().unwrap();
                    for (key, value) in hash {
                        match self
                            .options
                            //todo: remove clone
                            .set_yaml(key.clone(), value.clone())
                        {
                            Ok(opt) => {
                                if let Some(old_value) = opt {
                                    debug!(
                                        "{}",
                                        t!("debug.yaml_replace",
                                           key = format!("{:?}", key),
                                           old_val = format!("{:?}", old_value),
                                           new_val = format!("{:?}", value)
                                        )
                                    );
                                } else {
                                    debug!(
                                        "{}",
                                        t!("debug.yaml_set",
                                           key = format!("{:?}", key),
                                           value = format!("{:?}", value)
                                        )
                                    );
                                }
                            }
                            Err(e) => {
                                error!(
                                    "{}",
                                    t!(
                                        "error.yaml_set",
                                        key = format!("{:?}", key,),
                                        value = format!("{:?}", value),
                                        err = e
                                    )
                                )
                            }
                        }
                    }

                    self.update_cleaner();
                } else {
                    debug!(
                        "{}",
                        t!(
                            "debug.yaml_ignore",
                            block = &yaml_block
                        )
                    );
                }
            }
            Err(err) => {
                error!(
                    "{}",
                    t!("debug.found_yaml_block",
                        block = &yaml_block
                    )
                );
                error!(
                    "{}",
                    t!("debug.found_yaml_block2",
                        error = err
                    )
                );
            }
        }
    }

    // Update the cleaner according to autoclean and lang options
    fn update_cleaner(&mut self) {
        let params = CleanerParams {
            smart_quotes: self.options.get_bool("input.clean.smart_quotes").unwrap(),
            ligature_dashes: self
                .options
                .get_bool("input.clean.ligature.dashes")
                .unwrap(),
            ligature_guillemets: self
                .options
                .get_bool("input.clean.ligature.guillemets")
                .unwrap(),
        };
        if self.options.get_bool("input.clean").unwrap() {
            let lang = self.options.get_str("lang").unwrap().to_lowercase();
            let cleaner: Box<dyn Cleaner> = if lang.starts_with("fr") {
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

impl std::default::Default for Book<'_> {
    fn default() -> Self {
        Self::new()
    }
}

