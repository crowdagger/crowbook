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

use crate::number::Number;
use crate::token::Token;

/// Represents the content of a chapter.
#[derive(Debug)]
pub struct Chapter {
    /// The numbering scheme of this chapter.
    pub number: Number,
    /// The filename of this chapter (used for inline links)
    pub filename: String,
    /// The (already parsed) content of this chapter
    pub content: Vec<Token>,
}

impl Chapter {
    /// Creates a new chapter
    ///
    /// # Arguments
    /// * `number`: the numbering scheme, to specify if this is a numbered chapter or not.
    /// * `filename`: the path of the Markdown source file of the chapter.
    /// * `content`: a vector of `Token`, as returned by `Parser`.
    pub fn new<S: Into<String>>(number: Number, filename: S, content: Vec<Token>) -> Chapter {
        Chapter {
            number,
            filename: filename.into(),
            content,
        }
    }
}
