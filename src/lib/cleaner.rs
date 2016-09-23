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

const THRESHOLD_QUOTE: usize = 23; // after that, assume it's a dialogue
const THRESHOLD_REAL_WORD: usize = 3; // after that, can be reasonably sure it is not an abbreviation

impl Cleaner for French {
    /// Puts non breaking spaces before/after `:`, `;`, `?`, `!`, `«`, `»`, `—`
    fn clean<'a>(&self, s: Cow<'a, str>, latex: bool) -> Cow<'a, str> {
        fn is_trouble(c: char) -> bool {
            match c {
                '?'|'!'|';'|':'|'»'|'«'|'—'|'–' | '0'...'9' => true,
                _ => false
            }
        }
        if !s.contains(is_trouble) { // if not, no need to do anything
            return Default.clean(s, latex);
        }
        let nb_char = if latex {
            '~'
        } else {
            ' ' // non breaking space
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

        // Find first char `c` in slice `v` after index `n`
        fn find_next(v: &[char], c: char, n: usize) -> Option<usize> {
            for i in n..v.len() {
                if v[i] == c  {
                    return Some(i);
                } 
            }
            None
        }

        // Return true if next non whitespace char in `v` after index `n` is uppercase
        fn is_next_char_uppercase(v: &[char], n: usize)-> bool {
            for i in n..v.len() {
                if v[i].is_whitespace() {
                    continue;
                }
                if v[i].is_uppercase() {
                    return true;
                }
                if v[i].is_lowercase() {
                    return false;
                }
            }
            false
        }

        // Return true(some) if a closing dash was found before what looks like the end of a sentence, None else
        fn find_closing_dash(v: &[char], n: usize) -> Option<usize> {
            let mut word = String::new();
            for j in n..v.len() {
                match v[j] {
                    
                    '!' | '?' => if is_next_char_uppercase(v, j+1) {
                        return None;
                    },
                    '-' | '–' | '—' => if v[j-1].is_whitespace() {
                        return Some(j-1);
                    },
                    '.' => if !is_next_char_uppercase(v, j+1) {
                        continue;
                    } else {
                        if let Some(c) = word.chars().next() {
                            if !c.is_uppercase() {
                                return None;
                            } else {
                                if word.len() > THRESHOLD_REAL_WORD {
                                    return None;
                                }
                            }
                        } 
                    },
                    c if c.is_whitespace() => word = String::new(),
                    c => word.push(c),
                }
            }
            return None;
        }

        /// Return true if the character is a symbol that is used after number and should have a nb_char before
        fn char_is_symbol(v: &[char], i: usize) -> bool {
            let is_next_letter = if i < v.len() - 1 {
                v[i+1].is_alphabetic()
            } else {
                false
            };
            if is_next_letter {
                match v[i] {
                    '°' => true,
                    _ => false
                }
            } else {
                match v[i] {
                    c if (!c.is_alphabetic() && !c.is_whitespace()) => true,
                    c if c.is_uppercase() => true,
                    _ => false,
                }
            }
        }


        let s = Default.clean(s, latex); // first pass with default impl
        let mut found_opening_quote = false; // we didn't find an opening quote yet
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut is_number_series = false;

        for i in 0..(chars.len()-1) {
            // Handle numbers (that's easy)
            let current = chars[i];
            let next = chars[i+1];

            match current {
                '0'...'9' => if i == 0 {
                    is_number_series = true;
                } else if !chars[i-1].is_alphabetic() {
                    is_number_series = true;
                },
                c if c.is_whitespace() => {
                    if is_number_series && (next.is_digit(10) || char_is_symbol(&chars, i+1)) {
                        // Next char is a number or symbol such as $, and previous was number
                        chars[i] = nb_char_narrow;
                    }
                },
                _ => { is_number_series = false; }
            }
        }
        
        for i in 0..(chars.len()-1) {
            let current = chars[i];
            let next = chars[i+1];

            
            // Handle the rest (that's hard)
            if is_whitespace(current) {
                match next {
                    // handle narrow nb space before char
                    '?' | '!' | ';' => chars[i] = nb_char_narrow,
                    ':' => chars[i] = nb_char,
                    '»' => if current == ' ' {
                        // Assumne that if it isn't a normal space it was used here for good reason, don't replace it
                        if found_opening_quote {
                            // not the end of a dialogue
                            chars[i] = nb_char;
                        } else {
                            chars[i] = nb_char;
                        }
                    },
                    _ => (),
                }
            } else {
                match current {
                    // handle nb space after char
                    '—' | '«' | '-' | '–' => {
                        if is_whitespace(next) {
                            let replacing_char = match current {
                                '—' | '-' | '–' => {
                                    if i <= 1 {
                                        nb_char_em
                                    } else {
                                        if chars[i-1] == nb_char {
                                            // non breaking space before, so probably should have a breakable one after
                                            ' '
                                        } else {
                                            if let Some(closing) = find_closing_dash(&chars, i+1) {
                                                chars[closing] = nb_char;
                                            }
                                            nb_char
                                        }
                                    }
                                },
                                '«' => {
                                    found_opening_quote = true;
                                    if i <= 1 {
                                        nb_char
                                    } else {
                                        let j = find_next(&chars, '»', i);
                                        if let Some(j) = j {
                                            if chars[j-1].is_whitespace() {
                                                if j == chars.len() {
                                                    // » is at the end, assume it is a dialogue
                                                    chars[j-1] = nb_char;
                                                    nb_char
                                                } else {
                                                    if j - i > THRESHOLD_QUOTE {
                                                        // It's a quote, so use large space?
                                                        chars[j-1] = nb_char;
                                                        nb_char
                                                    } else {
                                                        // Not long enough to be a quote, use narrow nb char
                                                        chars[j-1] = nb_char_narrow;
                                                        nb_char_narrow
                                                    }
                                                }
                                            } else {
                                                // wtf formatting?
                                                nb_char
                                            }
                                        } else {
                                            // No ending quote found, assume is a dialogue
                                            nb_char
                                        }
                                    }
                                }, // TODO: better heuristic: use narrow nb_char if not at front???
                                _ => unreachable!(),
                            };
                            chars[i+1] = replacing_char;
                        }
                    }
                    _ => (),
                }
            }
        }
        Cow::Owned(chars.into_iter().collect())
    }
}

