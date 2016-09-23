use error::{Error,Result, Source};

/// Structure for storing a book option
///
/// This Enum might grow additional variants, so library users should
/// **not** count on exhaustive matching.
#[derive(Debug, PartialEq, Clone)]
pub enum BookOption {
    /// Stores a String
    String(String),

    /// Stores a boolean
    Bool(bool),

    /// Stores a single char
    Char(char),

    /// Stores an int
    Int(i32),
    
    /// Stores a path
    ///
    /// Stored the same way as a string, but some base path is usually prepended to it
    Path(String),

    /// Hint that destructuring should not be exhaustive
    #[doc(hidden)]
    __NonExhaustive,
}

impl BookOption {
    /// Returns the BookOption as a &str, or an error if it isn't a `String` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use crowbook::BookOption;
    ///
    /// let option = BookOption::String("foo".to_owned());
    /// assert_eq!(option.as_str(), Ok("foo"));
    ///
    /// let option = BookOption::Int(42);
    /// assert!(option.as_str().is_err());
    /// ```
    pub fn as_str(&self) -> Result<&str> {
        match *self {
            BookOption::String(ref s) => Ok(s),
            _ => Err(Error::book_option(Source::empty(), format!("{:?} is not a string", self)))
        }
    }

    /// Returns the BookOption as a &str, on an error if it isn't a `Path` variant.
    pub fn as_path(&self) -> Result<&str> {
        match *self {
            BookOption::Path(ref s) => Ok(s),
            _ => Err(Error::book_option(Source::empty(), format!("{:?} is not a path", self)))
        }
    }

    /// Retuns the BookOption as a bool, or an error if it isn't a `Bool` variant.
    pub fn as_bool(&self) -> Result<bool> {
        match *self {
            BookOption::Bool(b) => Ok(b),
            _ => Err(Error::book_option(Source::empty(), format!("{:?} is not a bool", self)))
        }
    }

    /// Returns the BookOption as a char, or an error if it isn't a `Char` variant.
    pub fn as_char(&self) -> Result<char> {
        match *self {
            BookOption::Char(c) => Ok(c),
            _ => Err(Error::book_option(Source::empty(), format!("{:?} is not a char", self)))
        }
    }
    
    /// Retuns the BookOption as an i32, or an error if it isn't an `Int` variant.
    pub fn as_i32(&self) -> Result<i32> {
        match *self {
            BookOption::Int(i) => Ok(i),
            _ => Err(Error::book_option(Source::empty(), format!("{:?} is not an i32", self)))

        }
    }
}



