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


//!  Note: this documentation is relative to `crowbook` *as a library*.
//!  For documentation regarding the *program* `crowbook`, see
//!  [the Github page](https://github.com/lise-henry/crowbook).


extern crate pulldown_cmark as cmark;
extern crate mustache;
extern crate chrono;
extern crate uuid;

pub use parser::Parser;
pub use book::Book;
pub use html::HtmlRenderer;
pub use epub::EpubRenderer;
pub use latex::LatexRenderer;
pub use odt::OdtRenderer;
pub use error::{Result, Error};
pub use token::Token;
pub use cleaner::{Cleaner, French};
pub use number::Number;

mod html;
mod error;
mod book;
mod epub;
mod latex;
mod odt;
mod parser;
mod token;
mod cleaner;
mod number;


mod escape;
mod toc;
mod zipper;
mod templates;
mod bookoption;

#[cfg(test)]
mod tests;

