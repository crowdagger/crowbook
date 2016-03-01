use error::{Error,Result};

/// Structure for storing a book option
#[derive(Debug, PartialEq)]
pub enum BookOption {
    String(String), // Stores a string
    Bool(bool), // stores a boolean
    Char(char), // stores a char
    Int(i32), // stores an int
    Path(String), // Stores a path
}

impl BookOption {
    /// Retuns the BookOption as a &str
    pub fn as_str(&self) -> Result<&str> {
        match *self {
            BookOption::String(ref s) => Ok(s),
            _ => Err(Error::BookOption(format!("{:?} is not a string", self)))
        }
    }

    /// Returns the BookOption as a &str, but only if it is a path
    pub fn as_path(&self) -> Result<&str> {
        match *self {
            BookOption::Path(ref s) => Ok(s),
            _ => Err(Error::BookOption(format!("{:?} is not a path", self)))
        }
    }

    /// Retuns the BookOption as a bool
    pub fn as_bool(&self) -> Result<bool> {
        match *self {
            BookOption::Bool(b) => Ok(b),
            _ => Err(Error::BookOption(format!("{:?} is not a bool", self)))
        }
    }

    /// Retuns the BookOption as a char
    pub fn as_char(&self) -> Result<char> {
        match *self {
            BookOption::Char(c) => Ok(c),
            _ => Err(Error::BookOption(format!("{:?} is not a char", self)))
        }
    }
    
    /// Retuns the BookOption as an i32
    pub fn as_i32(&self) -> Result<i32> {
        match *self {
            BookOption::Int(i) => Ok(i),
            _ => Err(Error::BookOption(format!("{:?} is not an i32", self)))

        }
    }
}



