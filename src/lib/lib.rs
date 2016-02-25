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

extern crate pulldown_cmark as cmark;
extern crate mustache;
extern crate chrono;
extern crate uuid;

pub mod html;
pub mod error;
pub mod book;
pub mod epub;
pub mod latex;
pub mod odt;

pub use book::Book;
pub use error::{Result, Error};
pub use html::HtmlRenderer;
pub use epub::EpubRenderer;
pub use latex::LatexRenderer;
pub use odt::OdtRenderer;

mod cleaner;
mod escape;
mod toc;
mod zipper;
mod templates;
mod bookoption;
mod parser;
mod token;

#[cfg(test)]
mod tests;
