// Copyright (C) 2016-2023 Élisabeth HENRY.
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

//!  Note: this documentation is relative to `crowbook` *as a library*.
//!  For documentation regarding the *program* `crowbook`, see
//!  [the Github page](https://github.com/lise-henry/crowbook).
//!
//! # Usage
//!
//! Just like any other library, just add a dependency on `crowbook`
//! in your `Cargo.toml` file. You will probably want to deactivate
//! default features that are mostly useful for the binary:
//!
//! ```ignore
//! crowbook = {version = "0.14.1", default-features = false}
//! ```
//!
//! # Book
//!
//! The central structure of Crowbook is `Book`, which coordinates everything.
//!
//! Its roles are:
//!
//! * read a book configuration file and setting the book options accordingly;
//! * read the chapters (written in Markdown) listed in this
//!   configuration file and pass them to to `Parser`, get back an AST and store it in memory
//! * call the various renderers according to the book's parameters
//! and generate the appopriate files.
//!
//! ## Example
//!
//! ```ignore
//! use crowbook::Book;
//! // Reads configuration file "foo.book" and render all formats according to this
//! // configuration file
//! Book::new()
//!      .load_file("foo.book").unwrap()
//!      .render_all().unwrap();
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

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub use book::Book;
pub use book_renderer::BookRenderer;
pub use bookoption::BookOption;
pub use bookoptions::BookOptions;
pub use chapter::Chapter;
pub use error::{Error, Result, Source};
pub use number::Number;
pub use parser::Parser;
pub use renderer::Renderer;
pub use resource_handler::ResourceHandler;
pub use stats::Stats;
pub use token::Data;
pub use token::Token;

rust_i18n::i18n!("lang/lib", fallback="en");

#[macro_use]
mod html;
mod book;
mod book_renderer;
mod bookoptions;
mod chapter;
mod cleaner;
mod epub;
mod error;
mod html_dir;
mod html_if;
mod html_single;
mod lang;
mod latex;
mod number;
mod parser;
mod renderer;
mod resource_handler;
mod stats;
mod syntax;
mod token;

#[cfg(feature = "binary")]
mod style;
#[cfg(not(feature = "binary"))]
mod style_stubs;
#[cfg(not(feature = "binary"))]
use style_stubs as style;

#[cfg(feature = "indicatif")]
mod book_bars;
#[cfg(not(feature = "indicatif"))]
mod book_bars_stubs;
#[cfg(not(feature = "indicatif"))]
use book_bars_stubs as book_bars;

mod bookoption;
mod misc;
mod templates;
mod text_view;
mod zipper;

#[cfg(test)]
mod tests;
