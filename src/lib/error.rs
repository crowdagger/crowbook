// Copyright (C) 2016 Élisabeth HENRY.
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

use std::error;
use std::result;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
/// Source of an error file
pub struct Source {
    /// File name of the source
    pub file: Option<String>,
    /// Line number of the source
    pub line: Option<u32>,
}

impl Source {
    /// Create an empty source, with both fields set to None
    pub fn empty() -> Source {
        Source { file: None, line: None }
    }

    /// Create a new source pointing to file
    pub fn new(s: &str) -> Source {
        Source { file: Some(String::from(s)), line: None }
    }

    /// Sets line number of a source
    pub fn line(mut self, line: u32) -> Source {
        self.line = Some(line);
        self
    }

    /// Sets line number of a source (with &mut self)
    pub fn set_line(&mut self, line: u32) {
        self.line = Some(line);
    }

    /// Unsets a line number of a source
    pub fn unset_line(&mut self) {
        self.line = None;
    }
    
}

#[derive(Debug, PartialEq)]
/// Crowbook's error type
pub enum Error {
    /// An error in Parsing markdown file
    Parser(Source, String),
    /// An error in parsing a book configuration file
    ConfigParser(Source, String), 
    /// An error when a file is not found
    FileNotFound(Source, String),
    /// An error in a renderer
    Render(String),
    /// An error during "zipping" processus
    Zipper(String),
    /// An error relative to BookOption convertion (usually a type error)
    BookOption(Source, String),
    /// An invalid option
    InvalidOption(Source, String),
}

impl Error {
    /// Return the source of an error
    pub fn get_source(&self) -> Option<&Source> {
        use Error::*;
        match *self {
            Parser(ref s, _) | ConfigParser(ref s, _) | FileNotFound(ref s, _)
                | BookOption(ref s, _) | InvalidOption(ref s, _) => {
                    Some(s)
                },
            Render(_) | Zipper(_) => None,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parser(_, ref s) | Error::Zipper(ref s) | Error::BookOption(_, ref s) | Error::ConfigParser(_, ref s)
                | Error::InvalidOption(_, ref s) | Error::Render(ref s) => s,
            Error::FileNotFound(_, _) => "File not found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let source = self.get_source();
        if let Some(source) = source {
            if let Some(ref file) = source.file {
                try!(f.write_str("Error in file "));
                try!(f.write_str(file));
                if let Some(line) = source.line {
                    try!(f.write_str(", line "));
                    try!(write!(f, "{}", line));
                }
                try!(f.write_str(":\n"));
            }
        }
        
        match *self {
            Error::Parser(_, ref s) => {
                try!(f.write_str("Error parsing markdown: "));
                f.write_str(&s)
            },
            Error::ConfigParser(_, ref s) => {
                try!(f.write_str("Error parsing configuration file: "));
                f.write_str(s)
            },
            Error::FileNotFound(_, ref file) => {
                try!(f.write_str("File not found: "));
                f.write_str(file)
            },
            Error::Render(ref s) => {
                try!(f.write_str("Error during rendering: "));
                f.write_str(s)
            },
            Error::Zipper(ref s) => {
                try!(f.write_str("Error during temporary files editing: "));
                f.write_str(s)
            },
            Error::BookOption(_, ref s) => {
                try!(f.write_str("Error converting BookOption: "));
                f.write_str(s)
            },
            Error::InvalidOption(_, ref s) => {
                try!(f.write_str("Error accessing book option: "));
                f.write_str(s)
            },
        }
    }
}

/// Crowbook's Result type, used by many methods that can fail
pub type Result<T> = result::Result<T, Error>;

