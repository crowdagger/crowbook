// Copyright (C) 2016-2023 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use std::borrow::Cow;
use std::error;
use std::fmt;
use std::result;
use std::string::FromUtf8Error;

use rust_i18n::t;

#[derive(Debug, PartialEq, Clone)]
/// Source of an error.
///
/// Contains (if it's possible) the file and ideally the line that the user should
/// look at to correct their error.
pub struct Source {
    /// File name of the source
    #[doc(hidden)]
    pub file: Option<String>,

    /// Line number of the source
    #[doc(hidden)]
    pub line: Option<u32>,
}

impl Source {
    /// Create an empty source, with both fields set to None
    pub fn empty() -> Source {
        Source {
            file: None,
            line: None,
        }
    }

    /// Create a new source pointing to file
    pub fn new<S: Into<String>>(s: S) -> Source {
        Source {
            file: Some(s.into()),
            line: None,
        }
    }

    /// Sets line number of a source.
    pub fn set_line(&mut self, line: u32) -> &mut Self {
        self.line = Some(line);
        self
    }

    /// Unsets a line number of a source
    #[doc(hidden)]
    pub fn unset_line(&mut self) -> &mut Self {
        self.line = None;
        self
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{file}")?;
            if let Some(line) = self.line {
                write!(f, ":{line}")?;
            }
        } else {
            write!(f, "<UNKNOWN FILE>")?;
        }
        Ok(())
    }
}

impl<'a> From<&'a Source> for Source {
    fn from(s: &'a Source) -> Source {
        s.clone()
    }
}
#[derive(Debug, PartialEq)]
/// Crowbook Error type.
///
/// This type tries (when it can) to track where the error came from, to
/// pinpoint the file (at least) and, if possible, the line the user needs
/// to look at.
pub struct Error {
    /// Origin (file, line) of the error, if there is one
    source: Source,
    inner: Inner,
}

impl Error {
    /// Creates a new default error.
    pub fn default<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Default(msg.into()),
        }
    }

    /// Creates a new parser error.
    ///
    /// Error when parsing markdown file.
    pub fn parser<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Parser(msg.into()),
        }
    }

    /// Creates a new syntect error.
    ///
    /// Error when parsing (and higlighting) syntax code
    pub fn syntect<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Syntect(msg.into()),
        }
    }

    /// Creates a new config parser error.
    ///
    /// Error when parsing the book configuration file.
    pub fn config_parser<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::ConfigParser(msg.into()),
        }
    }

    /// Creates a new "file not found" error
    ///
    /// # Arguments
    /// * source: the source of the error.
    /// * msg: description of why the file was needed.
    /// * file: file name that wasn't found.
    pub fn file_not_found<
        S1: Into<Cow<'static, str>>,
        S2: Into<Cow<'static, str>>,
        O: Into<Source>,
    >(
        source: O,
        msg: S1,
        file: S2,
    ) -> Error {
        Error {
            source: source.into(),
            inner: Inner::FileNotFound(msg.into(), file.into()),
        }
    }

    /// Creates a new render error.
    ///
    /// Error when rendering the book to a given format.
    pub fn render<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Render(msg.into()),
        }
    }

    /// Creates a new template error.
    ///
    /// Error when compiling a template.
    pub fn template<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Template(msg.into()),
        }
    }

    /// Creates a new invalid option error.
    ///
    /// Error when trying to set an option.
    pub fn invalid_option<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::InvalidOption(msg.into()),
        }
    }

    /// Creates a new zipper error.
    ///
    /// Error when moving/copying files to temporary dir, e.g. using `zip` commmand.
    pub fn zipper<S: Into<Cow<'static, str>>>(msg: S) -> Error {
        Error {
            source: Source::empty(),
            inner: Inner::Zipper(msg.into()),
        }
    }

    /// Creates a new book option error
    ///
    /// Used when converting an error to invalid type.
    pub fn book_option<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::BookOption(msg.into()),
        }
    }

    /// Change the source of an error.
    pub fn with_source<O: Into<Source>>(mut self, source: O) -> Error {
        self.source = source.into();
        self
    }

    /// Returns true if self is a default option error, false else.
    pub fn is_default(&self) -> bool {
        matches!(self.inner, Inner::Default(..))
    }

    /// Returns true if self is a parser error, false else.
    pub fn is_parser(&self) -> bool {
        matches!(self.inner, Inner::Parser(..))
    }

    /// Returns true if self is a config parser error, false else.
    pub fn is_config_parser(&self) -> bool {
        matches!(self.inner, Inner::ConfigParser(..))
    }

    /// Returns true if self is a file not found error, false else.
    pub fn is_file_not_found(&self) -> bool {
        matches!(self.inner, Inner::FileNotFound(..))
    }

    /// Returns true if self is a render error, false else.
    pub fn is_render(&self) -> bool {
        matches!(self.inner, Inner::Render(..))
    }

    /// Returns true if self is a zipper error, false else.
    pub fn is_zipper(&self) -> bool {
        matches!(self.inner, Inner::Zipper(..))
    }

    /// Returns true if self is a book option error, false else.
    pub fn is_book_option(&self) -> bool {
        matches!(self.inner, Inner::BookOption(..))
    }

    /// Returns true if self is an invalid option error, false else.
    pub fn is_invalid_option(&self) -> bool {
        matches!(self.inner, Inner::InvalidOption(..))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.inner {
            Inner::Default(ref s)
            | Inner::Parser(ref s)
            | Inner::Zipper(ref s)
            | Inner::BookOption(ref s)
            | Inner::ConfigParser(ref s)
            | Inner::InvalidOption(ref s)
            | Inner::Render(ref s)
            | Inner::Template(ref s)
            | Inner::Syntect(ref s) => s.as_ref(),
            Inner::FileNotFound(..) => "File not found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let source = &self.source;
        if let Some(ref file) = source.file {
            write!(f, "{file}")?;
            if let Some(line) = source.line {
                write!(f, ":{line}")?;
            }
            write!(f, ": ")?;
        }

        match self.inner {
            Inner::Default(ref s) => write!(f, "{s}"),
            Inner::Parser(ref s) => {
                write!(
                    f,
                    "{}",
                    t!("error.markdown", error = s)
                )
            }
            Inner::ConfigParser(ref s) => {
                f.write_str(&t!("error.config"))?;
                f.write_str(s)
            }
            Inner::FileNotFound(ref description, ref file) => {
                write!(
                    f,
                    "{}",
                    t!(
                        "error.file_not_found",
                        file = file,
                        description = description
                    )
                )
            }
            Inner::Template(ref s) => {
                write!(
                    f,
                    "{}",
                    t!("error.template", template = s)
                )
            }
            Inner::Render(ref s) => {
                f.write_str(&t!("error.render_error"))?;
                f.write_str(s)
            }
            Inner::Zipper(ref s) => {
                f.write_str(&t!("error.zipper"))?;
                f.write_str(s)
            }
            Inner::BookOption(ref s) => {
                f.write_str(&t!("error.bookoption"))?;
                f.write_str(s)
            }
            Inner::InvalidOption(ref s) => {
                f.write_str(&t!("error.invalid_option"))?;
                f.write_str(s)
            }
            Inner::Syntect(ref s) => {
                f.write_str(&t!("error.syntect"))?;
                f.write_str(s)
            }
        }?;
        Ok(())
    }
}

/// Crowbook's Result type, used by many methods that can fail
pub type Result<T> = result::Result<T, Error>;

/// Implement our Error from upon::error
impl From<upon::Error> for Error {
    fn from(err: upon::Error) -> Error {
        Error::template(Source::empty(), format!("{:#}", err))
    }
}


impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::render(
            Source::empty(),
            t!("error.utf8_error", error = err),
        )
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::render(
            Source::empty(),
            t!("error.utf8_error", error = err),
        )
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Error {
        Error::default(
            Source::empty(),
            t!("error.format", error = err),
        )
    }
}

impl From<syntect::Error> for Error {
    fn from(err: syntect::Error) -> Error {
        Error::syntect(
            Source::empty(),
            format!("{msg}: {error}",
                    msg = t!("error.syntect"),
                    error = err),
        )
    }
}

#[derive(Debug, PartialEq)]
enum Inner {
    /// Default variant
    Default(Cow<'static, str>),
    /// An error in Parsing markdown file
    Parser(Cow<'static, str>),
    /// An error in parsing a book configuration file
    ConfigParser(Cow<'static, str>),
    /// An error when a file is not found
    FileNotFound(Cow<'static, str>, Cow<'static, str>), // description, file
    /// An error in a renderer
    Render(Cow<'static, str>),
    /// An error during "zipping" processus
    Zipper(Cow<'static, str>),
    /// An error relative to BookOption convertion (usually a type error)
    BookOption(Cow<'static, str>),
    /// An invalid option
    InvalidOption(Cow<'static, str>),
    /// Error when compiling template
    Template(Cow<'static, str>),
    /// Error when parsing code syntax
    Syntect(Cow<'static, str>),
}
