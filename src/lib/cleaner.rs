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

/// Custom function because we don't really want to touch \t or \n
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == ' ' || c == ' '
}

/// Trait for cleaning a string.
/// This trait should be called for text that is e.g. in a paragraph, a title,
/// NOT for code blocks, hyperlikns and so on!
pub trait Cleaner {
    /// Cleans a string, removing multiple whitespaces
    fn clean(&self, s: &mut String) {
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

impl Cleaner for () {}

/// Implementation for french 'cleaning'
pub struct French {
    nb_char: char,
}

impl French {
    /// Creates a new french cleaner, which will replace spaces with nb_char when appropriate.
    pub fn new(nb_char: char) -> French {
        French { nb_char: nb_char }
    }
}
    

impl Cleaner for French {
    // puts non breaking spaces between :, ;, ?, !, «, »
    fn clean(&self, s: &mut String) {
        fn is_trouble(c: char) -> bool {
            match c {
                '?'|'!'|';'|':'|'»'|'«'|'—' => true,
                _ => false
            }
        }

        
        if !s.contains(is_trouble) { // if not, no need to do anything
            return;
        }
        ().clean(s); // first pass with default impl
        let mut new_s = String::with_capacity(s.len());
        {
            let mut chars = s.chars();
            if let Some(mut current) = chars.next() {
                while let  Some(next) = chars.next() {
                    if is_whitespace(current) {
                        match next {
                            // handle nb space before char
                            '?' | '»' | '!' | ';' | ':' => new_s.push(self.nb_char),
                            _ => new_s.push(current)
                        }
                    } else {
                        new_s.push(current);
                        match current {
                            // handle nb space after char
                            '«'|'—' => {
                                if is_whitespace(next) {
                                    new_s.push(self.nb_char);
                                    if let Some(next) = chars.next() {
                                        current = next;
                                        continue;
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
            

