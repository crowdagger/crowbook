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

#[derive(Debug)]
/// Crowbook error type
pub enum Error {
    Parser(String),
    ConfigParser(&'static str, String), //error, line
    FileNotFound(String), //file
    Render(&'static str),
    Zipper(String),
    BookOption(String)
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigParser(ref s, _) | Error::Render(ref s)  => s,
            Error::Parser(ref s) | Error::Zipper(ref s) | Error::BookOption(ref s)=> s,
            Error::FileNotFound(_) => "File not found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Parser(ref s) => {
                try!(f.write_str("Error parsing markdown: "));
                f.write_str(&s)
            },
            Error::ConfigParser(ref s, ref line) => {
                try!(f.write_str("Error parsing configuration file: "));
                try!(f.write_str(s));
                try!(f.write_str(" in: "));
                f.write_str(line)
            },
            Error::FileNotFound(ref file) => {
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
            Error::BookOption(ref s) => {
                try!(f.write_str("Error converting BookOption: "));
                f.write_str(s)
            }
        }
    }
}

pub type Result<T> = result::Result<T, Error>;


