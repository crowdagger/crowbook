// Copyright (C) 2016 Élisabeth HENRY.
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

use crate::book::Book;
use crate::error::{Error, Result, Source};

use std::fs::File;
use std::io::Write;
use std::path::Path;
use rust_i18n::t;

/// Trait that must be implemented by the various renderers to render a whole book.

pub trait BookRenderer: Sync {
    /// Path destination when output is set to auto
    fn auto_path(&self, _book_file: &str) -> Result<String> {
        Err(Error::default(
            Source::empty(),
            t!("error.renderer.no_output"),
        ))
    }

    /// Render the book and write the result to the specified writer
    fn render(&self, book: &Book, to: &mut dyn Write) -> Result<()>;

    /// Render the book to a given file.
    ///
    /// The default implementation creates a file and calls `render` to write to it,
    /// but in some cases it might be useful to override it.
    fn render_to_file(&self, book: &Book, path: &Path) -> Result<()> {
        // Not optimal but avoid creating an empty file if it fails
        let mut content = vec![];
        self.render(book, &mut content)?;
        let mut file = File::create(path).map_err(|err| {
            Error::default(
                Source::empty(),
                t!(
                    "error.renderer.file_creation",
                    file = path.display(),
                    err = err
                ),
            )
        })?;
        file.write_all(&content).map_err(|err| {
            Error::default(
                Source::empty(),
                t!(
                    "error.renderer.write",
                    file = path.display(),
                    err = err
                ),
            )
        })?;
        Ok(())
    }
}
