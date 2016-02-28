/// The level of information to display to a logger#
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum InfoLevel {
    /// Debug: the lowest level
    Debug = 0,
    /// Info: won't be displayed by default
    Info = 1,
    /// Warning: the default level
    Warning = 2,
    /// Error
    Error = 3,    
}

use self::InfoLevel::*;

/// Logs info and warning message and choose whether to display them
/// according to verbosity
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
    
    /// Prints a message if logger's verbosity <= level
    pub fn print(&self, level: InfoLevel, s: &str) {
        if level <= self.verbosity {
            match level {
                Debug => println!("debug: {}", s),
                Info => println!("info: {}", s),
                Warning => println!("warning: {}", s),
                Error => println!("error: {}", s),
            }
        }
    }
}
