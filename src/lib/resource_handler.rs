use crate::error::{Error, Result, Source};
use crate::token::Token;
use crate::misc;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use rust_i18n::t;

/// Resource Handler.
///
/// Its task is to make sure that some resource (image, link) is available
/// for the book and to list images used in Markdown files so they can be used for the book
#[derive(Debug)]
pub struct ResourceHandler {
    /// Maps an original url (e.g.) "foo/Readme.md" to a valid link
    /// (e.g.) chapter3.html
    links: HashMap<String, String>,
    map_images: bool,
    base64: bool,

    /// Maps an original (local) file name to a new file name. Allows to
    /// make sure all image files will be included in e.g. the Epub document.
    #[doc(hidden)]
    pub images: HashMap<String, String>,
}

impl ResourceHandler {
    /// Creates a new, empty Resource Handler
    pub fn new() -> ResourceHandler {
        ResourceHandler {
            links: HashMap::new(),
            images: HashMap::new(),
            map_images: false,
            base64: false,
        }
    }

    /// Turns on mapping for image files
    ///
    /// # Argument: an offset (should be book.root)
    pub fn set_images_mapping(&mut self, b: bool) -> &mut Self {
        self.map_images = b;
        self
    }

    /// Sets base64 mode for image mapping
    ///
    /// If set to true, instead of returning a destination file path,
    /// `map_image` will include the image as base64
    pub fn set_base64(&mut self, b: bool) {
        self.base64 = b;
    }

    /// Add a local image file and get the resulting transformed
    /// file name
    pub fn map_image<'a, S: Into<Cow<'a, str>>>(
        &'a mut self,
        source: &Source,
        file: S,
    ) -> Result<Cow<'a, str>> {
        // If image is not local, do nothing much
        let file = file.into();
        if !Self::is_local(file.as_ref()) {
            warn!(
                "{}",
                t!("resources.non_local",
                   file = file
                )
            );
            return Ok(file);
        }

        // Check exisence of the file
        if fs::metadata(file.as_ref()).is_err() {
            return Err(Error::file_not_found(
                source,
                t!("format.image"),
                format!("{file}"),
            ));
        }

        // if image mapping is not activated do nothing else
        if !self.map_images {
            return Ok(file);
        }

        // If this image has already been registered, returns it
        if self.images.contains_key(file.as_ref()) {
            return Ok(Cow::Borrowed(&self.images[file.as_ref()]));
        }

        // Else, create a new file name that has same extension
        // (or a base64 version of the file)
        let dest_file = if !(self.base64) {
            if let Some(extension) = Path::new(file.as_ref()).extension() {
                format!(
                    "images/image_{}.{}",
                    self.images.len(),
                    extension.to_string_lossy()
                )
            } else {
                warn!(
                    "{}",
                    t!("resources.no_ext",
                        file = file
                    )
                );
                format!("images/image_{}", self.images.len())
            }
        } else {
            let mut f = match fs::canonicalize(file.as_ref()).and_then(fs::File::open) {
                Ok(f) => f,
                Err(_) => {
                    return Err(Error::file_not_found(
                        source,
                        t!("format.image"),
                        format!("{file}"),
                    ));
                }
            };
            let mut content: Vec<u8> = vec![];
            if f.read_to_end(&mut content).is_err() {
                error!(
                    "{}",
                    t!("resources.read_error", file = file)
                );
                return Ok(file);
            }
            let base64 = misc::u8_to_base64(&content);
            match mime_guess::from_path(file.as_ref()).first() {
                None => {
                    error!(
                        "{}",
                        t!("resources.guess",
                            file = file
                        )
                    );
                    return Ok(file);
                }
                Some(s) => format!("data:{s};base64,{base64}"),
            }
        };

        self.images.insert(file.into_owned(), dest_file.clone());
        Ok(Cow::Owned(dest_file))
    }

    /// Returns an iterator the the images files mapping
    #[doc(hidden)]
    pub fn images_mapping(&self) -> &HashMap<String, String> {
        &self.images
    }

    /// Add a match between an original file and a dest file
    pub fn add_link<S1: Into<String>, S2: Into<String>>(&mut self, from: S1, to: S2) {
        self.links.insert(from.into(), to.into());
    }

    /// Get a destination link from an original link
    pub fn get_link<'a>(&'a self, from: &'a str) -> &'a str {
        if let Some(link) = self.links.get(from) {
            link
        } else {
            // Try to get a link by changing the extension
            let new_from = format!("{}", Path::new(from).with_extension("md").display()).replace("\\", "/");
            if let Some(link) = self.links.get(&new_from) {
                link
            } else {
                warn!(
                    "{}",
                    t!("resources.no_match",
                        file = from,
                        new_from = new_from
                    )
                );
                from
            }
        }
    }

    /// Tell whether a file name is a local resource or net
    pub fn contains_link(&self, from: &str) -> bool {
        if self.links.contains_key(from) {
            true
        } else {
            // Try to get a link by changing the extension and rewriting backlashes
            let new_from = format!("{}", Path::new(from).with_extension("md").display()).replace("\\", "/");
            self.links.contains_key(&new_from)
        }
    }

    pub fn is_local(path: &str) -> bool {
        !path.contains("://") // todo: use better algorithm
    }

    /// Add a path offset to all linked urls and images src
    pub fn add_offset(link_offset: &Path, image_offset: &Path, ast: &mut [Token]) {
        if link_offset == Path::new("") && image_offset == Path::new("") {
            // nothing do to
            return;
        }
        for token in ast {
            match *token {
                Token::Link(ref mut url, _, ref mut v) => {
                    if ResourceHandler::is_local(url) {
                        let new_url = format!("{}", link_offset.join(&url).display());
                        *url = new_url;
                    }
                    Self::add_offset(link_offset, image_offset, v);
                }
                Token::Image(ref mut url, _, ref mut v)
                | Token::StandaloneImage(ref mut url, _, ref mut v) => {
                    if ResourceHandler::is_local(url) {
                        let new_url = format!("{}", image_offset.join(&url).display());
                        *url = new_url;
                    }
                    Self::add_offset(link_offset, image_offset, v);
                }
                _ => {
                    if let Some(ref mut inner) = token.inner_mut() {
                        Self::add_offset(link_offset, image_offset, inner);
                    }
                }
            }
        }
    }
}

impl Default for ResourceHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the list of all files, walking recursively in directories
///
/// # Arguments
/// - list: a list of files
/// - base: the path where to get them
///
/// # Returns
/// A list of files (relative to `base`), or an error.
pub fn get_files(list: &[String], base: &str) -> Result<Vec<String>> {
    let mut out: Vec<String> = vec![];
    let base = Path::new(base);
    for path in list {
        let abs_path = base.join(path);
        let res = fs::metadata(&abs_path);
        match res {
            Err(err) => {
                return Err(Error::render(
                    Source::empty(),
                    t!("resources.read_file",
                        file = abs_path.display(),
                        error = err
                    ),
                ))
            }
            Ok(metadata) => {
                if metadata.is_file() {
                    out.push(path.clone());
                } else if metadata.is_dir() {
                    let files = WalkDir::new(&abs_path)
                        .follow_links(true)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_type().is_file())
                        .map(|e| PathBuf::from(e.path().strip_prefix(base).unwrap()));
                    for file in files {
                        out.push(file.to_string_lossy().into_owned());
                    }
                } else {
                    return Err(Error::render(
                        Source::empty(),
                        t!("resources.no_path",
                            path = &path
                        ),
                    ));
                }
            }
        }
    }
    Ok(out)
}
