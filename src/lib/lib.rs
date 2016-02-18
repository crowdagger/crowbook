extern crate pulldown_cmark as cmark;

pub mod escape;
pub mod parser;
pub mod ast_to_md;
pub mod ast_to_html;
pub mod cleaner;
pub mod token;
pub mod error;

pub use ast_to_html::ast_to_html;
pub use parser::Parser;
pub use token::Token;
pub use cleaner::Cleaner;
pub use cleaner::French;
pub use error::{Result, Error};
