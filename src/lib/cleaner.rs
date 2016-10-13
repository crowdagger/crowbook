// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Crowbook is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

//! This module contains the `Cleaner` traits and various implementations of it.

use std::borrow::Cow;
use crowbook_text_processing::clean::remove_whitespaces;
use crowbook_text_processing::french::FrenchFormatter;


/// Trait for cleaning a string.
///
/// This trait must be called for text that is e.g. in a paragraph, a title,
/// NOT for code blocks, hyperlinks and so on!
pub trait Cleaner: Sync {
    /// Cleans a string. The default implementation is to remove multiple consecutive whitespaces
    ///
    /// # Argumets
    ///
    /// * `str`: the string that must be cleaned
    /// * `latex`: a bool specifying whether output is Latex code or not
    fn clean<'a>(&self, str: Cow<'a, str>, _latex: bool) -> Cow<'a, str> {
        str
    }
}

/// Cleaner implementation that does nothing
///
/// # Examples
///
/// ```
/// use crowbook::cleaner::Cleaner;
/// use crowbook::cleaner::Off;
/// use std::borrow::Cow;
/// let off = Off;
/// let s = off.clean(Cow::Borrowed("  A string   that won't be cleaned "), false);
/// assert_eq!(&s, "  A string   that won't be cleaned ");
/// ```
pub struct Off;
impl Cleaner for Off {}

/// Default implementation of cleaner trait.
///
/// Only removes unnecessary whitespaces.
///
/// # Examples
///
/// ```
/// use crowbook::cleaner::Cleaner;
/// use crowbook::cleaner::Default;
/// use std::borrow::Cow;
/// let s = Default.clean(Cow::Borrowed("  A  string   with   more   whitespaces  than  needed   "),
///                                     false);
/// assert_eq!(&s, " A string with more whitespaces than needed ");
/// ```
pub struct Default;
impl Cleaner for Default {
    /// Remove unnecessary whitespaces
    fn clean<'a>(&self, input: Cow<'a, str>, _: bool) -> Cow<'a, str> {
        remove_whitespaces(input)
    }
}

/// Implementation for french 'cleaning'
///
/// This implementation replaces spaces before some characters (e.g. `?` or `;`)
/// with non-breaking spaces
///
/// # Examples
///
/// ```
/// use crowbook::cleaner::Cleaner;
/// use crowbook::cleaner::French;
/// use std::borrow::Cow;
/// let s =  French.clean(Cow::Borrowed("  Bonjour ! Comment allez-vous   ?   "), true);
/// assert_eq!(&s, " Bonjour~! Comment allez-vous~? ");
/// ```
pub struct French {
    formatter: FrenchFormatter,
}

impl French {
    /// Creates a new french cleaner
    pub fn new() -> French {
        French { formatter: FrenchFormatter::new() }
    }
}


impl Cleaner for French {
    /// Puts non breaking spaces before/after `:`, `;`, `?`, `!`, `«`, `»`, `—`
    fn clean<'a>(&self, s: Cow<'a, str>, latex: bool) -> Cow<'a, str> {
        let s = if latex {
            self.formatter.format_tex(s)
        } else {
            self.formatter.format(s)
        };
        s
    }
}
