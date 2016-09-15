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
//!
//! # Book
//!
//! The central structure of Crowbook is `Book`, who coordinates everything.
//!
//! Its roles are:
//!
//! * reading a book configuration file and setting the book options accordingly
//! * reading the chapters (written in Markdown) listed in this configuration file
//!   to `Parser`, get back an AST and store it in memory
//! * call `HtmlRenderer`, `EpubRenderer`, `LatexRenderer` and/or `OdtRenderer`
//!   according to the book's parameters and generate the appopriate files.
//!
//! ## Example
//!
//! ```ignore
//! use crowbook::{Book, InfoLevel};
//! // Reads configuration file "foo.book" (and set verbosity do default level)
//! let mut book = Book::new_from_file("foo.book", InfoLevel::Info, &[]).unwrap();
//! // Render all formats according to this configuration file
//! book.render_all().unwrap();
//! ```
//!
//! This is basically the code for the `crowbook` binary (though it contains a
//! bit more error handling, checking parameters from command line and so on).
//! This is, however, not very interesting for a library usage.
//!
//! The `Book` structure, however, exposes its `chapter` fields, which contains
//! a vector with an element by chapter. With it, you can access the Markdown
//! for all chapters represented as an Abstact Syntax Tree (i.e., a vector of `Token`s).
//! It is thus possible to create a new renderer (or manipulate this AST in other ways).
//!
//! # Parser
//!
//! It is also possible to directly use `Parser` to transform some markdown string or file
//! to this AST:
//!
//! ```
//! use crowbook::{Parser,Token};
//! let mut parser = Parser::new();
//! let result = parser.parse("Some *valid* Markdown").unwrap();
//! assert_eq!(format!("{:?}", result),
//! r#"[Paragraph([Str("Some "), Emphasis([Str("valid")]), Str(" Markdown")])]"#);
//! ```
//!
//! Of course, you probably want to do something else with this AST than display it.
//! Let's assume you want to count the number of links in a document.
//!
//! ```
//! use crowbook::{Parser,Token};
//! fn count(ast: &[Token]) -> u32 {
//!    let mut n = 0;
//!    for token in ast {
//!        match *token {
//!            // It's a link, increase counter
//!            Token::Link(_,_,_) => n += 1,
//!            // It's not a link, let's count the number of links
//!            // inside of the inner element (if there is one)
//!           _ => {
//!                if let Some(sub_ast) = token.inner() {
//!                    n += count(sub_ast);
//!                }
//!            }
//!       }
//!    }
//!    n
//! }
//! 
//! let md = "# Here's a [link](http://foo.bar) #\n And *another [one](http://foo.bar)* !";
//!
//! let mut parser = Parser::new();
//! let ast = parser.parse(md).unwrap();
//! assert_eq!(count(&ast), 2);
//! ```

#![deny(missing_docs)]

extern crate pulldown_cmark as cmark;
extern crate mustache;
extern crate chrono;
extern crate uuid;
extern crate yaml_rust;
extern crate mime_guess;
extern crate walkdir;
extern crate rustc_serialize;
extern crate crossbeam;
extern crate syntect;

pub use parser::Parser;
pub use book::Book;
pub use bookoption::BookOption;
pub use bookoptions::BookOptions;
pub use html::HtmlRenderer;
pub use epub::EpubRenderer;
pub use latex::LatexRenderer;
pub use odt::OdtRenderer;
pub use error::{Result, Error, Source};
pub use token::Token;
pub use cleaner::{Cleaner, French};
pub use number::Number;
pub use resource_handler::ResourceHandler;
pub use logger::{Logger, InfoLevel};
pub use html_dir::HtmlDirRenderer;
pub use renderer::Renderer;

mod html;
mod html_dir;
mod error;
mod book;
mod epub;
mod latex;
mod odt;
mod parser;
mod token;
mod cleaner;
mod number;
mod resource_handler;
mod logger;
mod bookoptions;
mod lang;
mod renderer;

mod escape;
mod toc;
mod zipper;
mod templates;
mod bookoption;


#[cfg(test)]
mod tests;

