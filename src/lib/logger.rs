use term;

use std::convert::AsRef;
use std::io;
use std::io::Write;
use std::fmt::Display;

/// The level of information to display to a logger
///
/// This enum should only be used as parameters for `Logger` or `Book` methods. Library
/// users should **not** do exhaustive pattern matching on the variants of the enums,
/// as it might grow variants later.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum InfoLevel {
    /// Debug: the lowest level
    Debug = 0,
    /// Warning: won't be displayed by default
    Warning,
    /// Info: won't be displayed by default
    Info,
    /// Error
    Error,
    /// Quiet
    Quiet,

    /// Hint that destructuring should not be exhaustive
    #[doc(hidden)]
    __NonExhaustive,
}

use self::InfoLevel::*;

/// Abstract over eithr term outupt or (if it fails) io::stderr()
enum Output {
    Terminal(Box<term::StderrTerminal>),
    Stderr(io::Stderr),
}

impl Output {
    pub fn new() -> Output {
        if let Some(term) = term::stderr() {
            if (*term).supports_color() {
                return Output::Terminal(term)
            }
        }
        return Output::Stderr(io::stderr())
    }

    pub fn print_msg<S: Display>(&mut self, level: InfoLevel, msg: S) {
        let (colour, head_msg) = match level {
            Debug => (term::color::BRIGHT_BLUE, lformat!("Debug: ")),
            Warning => (term::color::BRIGHT_YELLOW, lformat!("Warning: ")),
            Info => (term::color::BRIGHT_GREEN, lformat!("Info: ")),
            Error => (term::color::BRIGHT_RED, lformat!("Error: ")),
            _ => unreachable!(),
        };
        match *self {
            Output::Stderr(ref mut stderr) => {
                writeln!(stderr, "{}{}",
                       head_msg,
                       msg)
                    .unwrap();
            },
            Output::Terminal(ref mut term) => {
                term.fg(colour)
                    .unwrap();
                term.attr(term::Attr::Bold)
                    .unwrap();
                write!(term, "{}",
                       head_msg)
                    .unwrap();
                term.reset()
                    .unwrap();
                writeln!(term, "{}",
                       msg)
                    .unwrap();
            }
        }
    }
}

/// Logs info and warning message and choose whether to display them
/// according to verbosity
#[derive(Debug)]
pub struct Logger {
    verbosity: InfoLevel,
}


impl Logger {
    /// Creates a new logger with defined verbosity
    pub fn new(verbosity: InfoLevel) -> Logger {
        Logger { verbosity: verbosity }
    }

    /// Sets verbosity
    pub fn set_verbosity(&mut self, verbosity: InfoLevel) {
        self.verbosity = verbosity;
    }

    /// Prints a message
    pub fn display_msg<S: AsRef<str>>(level: InfoLevel, s: S) {
        let mut output = Output::new();
        output.print_msg(level, s.as_ref());
    }
    
    /// Prints a debug message
    pub fn display_debug<S: AsRef<str>>(s: S) {
        Self::display_msg(Debug, s);
    }

    /// Prints an info message
    pub fn display_info<S: AsRef<str>>(s: S) {
        Self::display_msg(Info, s);
    }

    /// Prints a warning message
    pub fn display_warning<S: AsRef<str>>(s: S) {
        Self::display_msg(Warning, s);
    }

    /// Prints an error message
    pub fn display_error<S: AsRef<str>>(s: S) {
        Self::display_msg(Error, s);
    }


    /// Prints a message if logger's verbosity <= level
    pub fn log<S: AsRef<str>>(&self, level: InfoLevel, s: S) {
        if level >= self.verbosity {
            match level {
                Debug => Self::display_debug(s),
                Info => Self::display_info(s),
                Warning => Self::display_warning(s),
                Error => Self::display_error(s),
                Quiet => unreachable!(),
                __NonExhaustive => unreachable!(),
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

// Code to end shell colouring
pub const SHELL_COLOUR_OFF: &'static str = "\x1B[0m";
pub const SHELL_COLOUR_RED: &'static str = "\x1B[1m\x1B[31m";
pub const SHELL_COLOUR_BLUE: &'static str = "\x1B[1m\x1B[36m";
pub const SHELL_COLOUR_ORANGE: &'static str = "\x1B[1m\x1B[33m";
pub const SHELL_COLOUR_GREEN: &'static str = "\x1B[1m\x1B[32m";
