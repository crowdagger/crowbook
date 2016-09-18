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
    fn clean(&self, _str: &mut String, _latex: bool) {}
}

/// Cleaner implementation that does nothing
///
/// # Examples
///
/// ```
/// use crowbook::Cleaner;
/// use crowbook::cleaner::Off;
/// let off = Off;
/// let mut s = "  A string   that won't be cleaned ".to_owned();
/// off.clean(&mut s, false);
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
/// use crowbook::Cleaner;
/// use crowbook::cleaner::Default;
/// let default = Default;
/// let mut s = "  A  string   with   more   whitespaces  than  needed   ".to_owned();
/// default.clean(&mut s, false);
/// assert_eq!(&s, " A string with more whitespaces than needed ");
/// ```
pub struct Default;
impl Cleaner for Default {
    /// Remove unnecessary whitespaces
    fn clean(&self, s: &mut String, _: bool) {
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
            
            *s = new_s
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
/// use crowbook::Cleaner;
/// use crowbook::cleaner::French;
/// let french = French;
/// let mut s = "  Bonjour ! Comment allez-vous   ?   ".to_owned();
/// french.clean(&mut s, true); // clean for latex so we see the non-breaking spaces easily
/// assert_eq!(&s, " Bonjour~! Comment allez-vous~? ");
/// ```
pub struct French;

impl Cleaner for French {
    /// Puts non breaking spaces before/after `:`, `;`, `?`, `!`, `«`, `»`, `—`
    fn clean(&self, s: &mut String, latex: bool) {
        fn is_trouble(c: char) -> bool {
            match c {
                '?'|'!'|';'|':'|'»'|'«'|'—' => true,
                _ => false
            }
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

        
        if !s.contains(is_trouble) { // if not, no need to do anything
            return;
        }
        Default.clean(s, latex); // first pass with default impl
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

        *s = new_s
    }
}

