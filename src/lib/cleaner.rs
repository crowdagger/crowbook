use std::borrow::Cow;

/// Custom function because we don't really want to touch \t or \n
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == ' ' || c == ' '
}

/// Trait for cleaning a string.
/// This trait should be called for text that is e.g. in a paragraph, a title,
/// NOT for code blocks, hyperlikns and so on!
pub trait Cleaner {
    /// Cleans a string, removing multiple whitespaces
    fn clean<'a>(&self, s: &mut Cow<'a, str>) {
        if s.contains(is_whitespace) { // if not, no need to do anything
            let mut new_s = String::with_capacity(s.len());
            let mut previous_space = false;
            let mut modified = false;
            for c in s.chars() {
                if is_whitespace(c) {
                    if previous_space {
                        // previous char already a space, don't copy it
                        // but signal the new string is different
                        modified = true;
                    } else {
                        new_s.push(c);
                        previous_space = true;
                    }
                } else {
                    previous_space = false;
                    new_s.push(c);
                }
            }

            if modified {
                // only copy new string if it is modified
                // (if it is not we just wasted our time in this function call)
                let old_s = s.to_mut();
                *old_s = new_s
            } 
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
    fn clean<'a>(&self, s: &mut Cow<'a, str>) {
        fn is_trouble(c: char) -> bool {
            match c {
                '?'|'!'|';'|':'|'»'|'«' => true,
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
                            '?' | '»' | '!' | ';' | ':' => new_s.push(self.nb_char),
                            _ => new_s.push(current)
                        }
                    } else {
                        new_s.push(current);
                        if current == '«' {
                            if is_whitespace(next) {
                                new_s.push(self.nb_char);
                                if let Some(next) = chars.next() {
                                    current = next;
                                    continue;
                                }
                            }
                        }
                    }
                    current = next;
                }
                new_s.push(current);
            }
        }
            
        let old_s = s.to_mut();
        *old_s = new_s
    }
}
            

