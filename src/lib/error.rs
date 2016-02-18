use std::error;
use std::result;
use std::fmt;

#[derive(Debug)]
/// Crowbook error type
pub enum Error {
    Parser(&'static str),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Parser(ref s) => s
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Parser(ref s) => {
                try!(f.write_str("Parser error: "));
                f.write_str(s)
            }
        }
    }
}


pub type Result<T> = result::Result<T, Error>;


