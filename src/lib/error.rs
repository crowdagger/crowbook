use std::error;
use std::result;
use std::fmt;

#[derive(Debug)]
/// Crowbook error type
pub enum Error {
    Parser(&'static str),
    ConfigParser(&'static str, String), //error, line
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Parser(ref s) => s,
            &Error::ConfigParser(ref s, _) => s,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Parser(ref s) => {
                try!(f.write_str("Error parsing markdown: "));
                f.write_str(s)
            },
            &Error::ConfigParser(ref s, ref line) => {
                try!(f.write_str("Error parsing configuration file: "));
                try!(f.write_str(s));
                try!(f.write_str(" in:\n"));
                f.write_str(line)
            },
        }
    }
}

pub type Result<T> = result::Result<T, Error>;


