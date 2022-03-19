// Copyright (C) 2016-2020 Élisabeth HENRY.
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

use crowbook_text_processing::clean;
use crowbook_text_processing::FrenchFormatter;
use std::borrow::Cow;

/// Contains cleaning parameters
pub struct CleanerParams {
    pub smart_quotes: bool,
    pub ligature_guillemets: bool,
    pub ligature_dashes: bool,
}

/// Trait for cleaning a string.
///
/// This trait must be called for text that is e.g. in a paragraph, a title,
/// NOT for code blocks, hyperlinks and so on!
pub trait Cleaner: Sync {
    /// Cleans a string. The default implementation is to remove multiple consecutive whitespaces
    ///
    /// # Arguments
    ///
    /// * `str`: the string that must be cleaned
    fn clean<'a>(&self, str: Cow<'a, str>) -> Cow<'a, str> {
        str
    }
}

/// Cleaner implementation that does nothing
pub struct Off;
impl Cleaner for Off {}

/// Default implementation of cleaner trait.
///
/// Only removes unnecessary whitespaces.
pub struct Default {
    params: CleanerParams,
}

impl Default {
    /// New Default cleaner
    pub fn new(params: CleanerParams) -> Default {
        Default { params }
    }
}

impl Cleaner for Default {
    /// Remove unnecessary whitespaces
    fn clean<'a>(&self, input: Cow<'a, str>) -> Cow<'a, str> {
        let mut s = clean::whitespaces(input);
        if self.params.smart_quotes {
            s = clean::quotes(s);
        }
        if self.params.ligature_dashes {
            s = clean::dashes(s);
        }
        if self.params.ligature_guillemets {
            s = clean::guillemets(s);
        }
        s
    }
}

/// Implementation for french 'cleaning'
///
/// This implementation replaces spaces before some characters (e.g. `?` or `;`)
/// with non-breaking spaces
pub struct French {
    formatter: FrenchFormatter,
    params: CleanerParams,
}

impl French {
    /// Creates a new french cleaner
    pub fn new(params: CleanerParams) -> French {
        let mut this = French {
            formatter: FrenchFormatter::new(),
            params,
        };
        this.formatter.typographic_quotes(this.params.smart_quotes);
        this.formatter.ligature_dashes(this.params.ligature_dashes);
        this.formatter
            .ligature_guillemets(this.params.ligature_guillemets);
        this
    }
}

impl Cleaner for French {
    /// Puts non breaking spaces before/after `:`, `;`, `?`, `!`, `«`, `»`, `—`
    fn clean<'a>(&self, s: Cow<'a, str>) -> Cow<'a, str> {
        self.formatter.format(s)
    }
}
