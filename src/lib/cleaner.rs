// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

//! This module contains the `Cleaner` traits and various implementations of it.

use std::borrow::Cow;

/// Custom function because we don't really want to touch \t or \n
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == ' ' || c == ' '
}

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
/// let s = Default.clean(Cow::Borrowed("  A  string   with   more   whitespaces  than  needed   "), false);
/// assert_eq!(&s, " A string with more whitespaces than needed ");
/// ```
pub struct Default;
impl Cleaner for Default {
    /// Remove unnecessary whitespaces
    fn clean<'a>(&self, s: Cow<'a, str>, _: bool) -> Cow<'a, str> {
        if s.contains(is_whitespace) { // if not, no need to do anything
            let mut new_s = String::with_capacity(s.len());
            let mut previous_space = false;
            for c in s.chars() {
                if is_whitespace(c) {
                    if previous_space {
                        // previous char already a space, don't copy it
                    } else {
                        new_s.push(c);
                        previous_space = true;
                    }
                } else {
                    previous_space = false;
                    new_s.push(c);
                }
            }
            
            Cow::Owned(new_s)
        } else {
            s
        }
    }
}

/// Implementation for french 'cleaning'
///
/// This implementation replaces spaces before some characters (e.g. `?` or `;` with non-breaking spaces
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
pub struct French;

impl Cleaner for French {
    /// Puts non breaking spaces before/after `:`, `;`, `?`, `!`, `«`, `»`, `—`
    fn clean<'a>(&self, s: Cow<'a, str>, latex: bool) -> Cow<'a, str> {
        fn is_trouble(c: char) -> bool {
            match c {
                '?'|'!'|';'|':'|'»'|'«'|'—' => true,
                _ => false
            }
        }
        if !s.contains(is_trouble) { // if not, no need to do anything
            return Default.clean(s, latex);
        }
        let nb_char = if latex {
            '~'
        } else {
            ' '
        };
        let nb_char_narrow = if latex {
            '~'
        } else {
            '\u{202F}' // narrow non breaking space
        };
        let nb_char_em = if latex {
            '~'
        } else {
            '\u{2002}' // demi em space
        };

        let s = Default.clean(s, latex); // first pass with default impl
        let mut new_s = String::with_capacity(s.len());
        {
            let mut chars = s.chars();
            if let Some(mut current) = chars.next() {
                while let Some(next) = chars.next() {
                    if is_whitespace(current) {
                        match next {
                            // handle narrow nb space before char
                            '?' | '!' | ';' => new_s.push(nb_char_narrow),
                            ':' | '»' => new_s.push(nb_char),
                            _ => new_s.push(current)
                        }
                    } else {
                        new_s.push(current);
                        match current {
                            // handle nb space after char
                            '—' | '«' => {
                                if is_whitespace(next) {
                                    let replacing_char = match current {
                                        '—' => nb_char_em,
                                        '«' => nb_char,
                                        _ => unreachable!(),
                                    };
                                    if let Some(next) = chars.next() {
                                        new_s.push(replacing_char);
                                        current = next;
                                        continue;
                                    } else {
                                        // current will be added after the loop, do don't do it now
                                        current = replacing_char;
                                        break;
                                    }
                                }
                            },
                            _ => (),
                        }
                    }
                    current = next;
                }
                new_s.push(current);
            }
        }

        Cow::Owned(new_s)
    }
}

