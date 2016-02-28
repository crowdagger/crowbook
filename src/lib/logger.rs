use std::convert::AsRef;
use std::io;
use std::io::Write;

/// The level of information to display to a logger#
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum InfoLevel {
    /// Debug: the lowest level
    Debug = 0,
    /// Info: won't be displayed by default
    Warning,
    /// Warning: the default level
    Info,
    /// Error
    Error
}

use self::InfoLevel::*;

/// Logs info and warning message and choose whether to display them
/// according to verbosity
#[derive(Debug)]
pub struct Logger {
    verbosity: InfoLevel,        
}


impl Logger {
    /// Creates a new logger with defined verbosity
    pub fn new(verbosity: InfoLevel) -> Logger {
        Logger {
            verbosity: verbosity
        }
    }

    /// Sets verbosity
    pub fn set_verbosity(&mut self, verbosity: InfoLevel) {
        self.verbosity = verbosity;
    }
    
    /// Prints a message if logger's verbosity <= level
    pub fn log<S: AsRef<str>>(&self, level: InfoLevel, s: S) {
        if level >= self.verbosity {
            match level {
                Debug => writeln!(&mut io::stderr(), "debug: {}", s.as_ref()).unwrap(),
                Info => writeln!(&mut io::stderr(), "info: {}", s.as_ref()).unwrap(),
                Warning => writeln!(&mut io::stderr(), "warning: {}", s.as_ref()).unwrap(),
                Error => writeln!(&mut io::stderr(), "error: {}", s.as_ref()).unwrap(),
            }
        }
    }

    /// Equivalent of log(Debug, s)
    pub fn debug<S: AsRef<str>>(&self, s: S) {
        self.log(InfoLevel::Debug, s);
    }

    /// Equivalent of log(Info, s)
    pub fn info<S: AsRef<str>>(&self, s: S) {
        self.log(InfoLevel::Info, s);
    }

    /// Equivalent of log(Warning, s)
    pub fn warning<S: AsRef<str>>(&self, s: S) {
        self.log(InfoLevel::Warning, s);
    }

    /// Equivalent of log(Error, s)
    pub fn error<S: AsRef<str>>(&self, s: S) {
        self.log(InfoLevel::Error, s);
    }
}
