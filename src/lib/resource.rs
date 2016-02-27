use token::Token;

use std::collections::HashMap;
use std::path::Path;
use std::borrow::Cow;

/// Resource Handler.
///
/// Its task is to make sure that some resource (image, link) is available
/// for the book and to list images used in Markdown files so they can be used for the book
#[derive(Debug)]
pub struct ResourceHandler {
    /// Maps an original url (e.g.) "foo/Readme.md" to a valid link
    /// (e.g.) chapter3.html
    links: HashMap<String, String>,
    /// Maps an original (local) file name to a new file name. Allows to
    // make sure all image files will be included in e.g. the Epub document.
    pub images: HashMap<String, String>,
    map_images: bool,
}

impl ResourceHandler {
    /// Creates a new, empty Resource Handler
    pub fn new() -> ResourceHandler {
        ResourceHandler {
            links: HashMap::new(),
            images: HashMap::new(),
            map_images: false,
        }
    }

    /// Turns mapping for image files on
    pub fn set_images_mapping(&mut self, b: bool) {
        self.map_images = b;
    }

    /// Add a local image file and get the resulting transformed
    /// file name
    pub fn map_image<'a>(&'a mut self, file: Cow<'a, str>) -> Cow<'a, str> {
        // if image mapping is not activated, do nothing
        if !self.map_images {
            return file;
        }

        // If image is not local, do nothing either
        if !Self::is_local(file.as_ref()) {
            println!("Warning: book includes non-local images which might not be displayed correctly in EPUB");
            return file;
        }
        
        // If this image has already been registered, returns it
        if self.images.contains_key(file.as_ref()) {
            return Cow::Borrowed(self.images.get(file.as_ref()).unwrap());
        }

        // Else, create a new file name that has same extension 
        let dest_file = if let Some(extension) = Path::new(file.as_ref()).extension() {
            format!("images/image{}.{}", self.images.len(), extension.to_string_lossy())
        } else {
            format!("image{}", self.images.len())
        };
        
        self.images.insert(file.into_owned(), dest_file.clone());
        Cow::Owned(dest_file)
    }

    /// Returns an iterator the the images files mapping
    pub fn images_mapping(&self) -> &HashMap<String,String> {
        &self.images
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
