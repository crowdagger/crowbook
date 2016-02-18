use std::error;
use std::result;
use std::fmt;
use std::str::ParseBoolError;
use std::num::ParseIntError;

#[derive(Debug)]
/// Crowbook error type
pub enum Error {
    Parser(&'static str),
    ConfigParser(&'static str),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Parser(ref s) => s,
            &Error::ConfigParser(ref s) => s,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Parser(ref s) => {
                try!(f.write_str("Error parsing markdown file: "));
                f.write_str(s)
            },
            &Error::ConfigParser(ref s) => {
                try!(f.write_str("Error parsing configuration file: "));
                f.write_str(s)
            },
        }
    }
}

impl From<ParseBoolError> for Error {
    fn from(_: ParseBoolError) -> Error {
        Error::ConfigParser("could not parse bool")
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::ConfigParser("could not parse int")
    }
}


pub type Result<T> = result::Result<T, Error>;


