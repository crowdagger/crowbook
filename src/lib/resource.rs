use token::Token;

use std::collections::HashMap;
use std::path::Path;

/// Resource Handler.
///
/// Its task is to make sure that some resource (image, link) is available
/// for the book and to list images used in Markdown files so they can be used for the book
pub struct ResourceHandler {
    images: HashMap<String, String>
}

impl ResourceHandler {
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
