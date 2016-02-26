use token::Token;

use std::collections::HashMap;
use std::path::Path;

/// Resource Handler.
///
/// Its task is to make sure that some resource (image, link) is available
/// for the book and to list images used in Markdown files so they can be used for the book
#[derive(Debug)]
pub struct ResourceHandler {
    /// Maps an original url (e.g.) "foo/Readme.md" to a valid link
    /// (e.g.) chapter3.html
    links: HashMap<String, String>
}

impl ResourceHandler {
    /// Creates a new, empty Resource Handler
    pub fn new() -> ResourceHandler {
        ResourceHandler {
            links: HashMap::new(),
        }
    }

    /// Add a match between an original file and a dest file
    pub fn add_link(&mut self, from: String, to: String) {
        self.links.insert(from, to);
    }

    /// Get a destination link from an original link
    pub fn get_link<'a>(&'a self, from: &'a str) -> &'a str {
        if let Some(link) = self.links.get(from) {
            link
        } else {
            println!("Warning: resource handler could not find a match for link {}", from);
            from
        }
    }

    
    /// Tell whether a file name is a local resource or net
    pub fn is_local(path: &str) -> bool{
        !path.contains("://") // todo: use better algorithm
    }

    /// Add a path offset to all linked urls
    pub fn add_offset(offset: &Path, ast: &mut [Token]) {
        if offset == Path::new("") {
            //nothing do to
            return;
        }
        for mut token in ast {
            match *token {
                Token::Link(ref mut url, _, ref mut v)
                    | Token::Image (ref mut url, _, ref mut v) => {
                        if ResourceHandler::is_local(url) {
                            let new_url = format!("{}", offset.join(&url).display());
                            println!("converting link '{}' to '{}'", url, &new_url);
                            *url = new_url;
                        }
                        Self::add_offset(offset, v);
                    },
                _ => {
                    if let Some(ref mut inner) = token.inner_mut() {
                            Self::add_offset(offset, inner);
                    }
                }
            }
        }
    }
}
