// Copyright (C) 2016-2019 Élisabeth HENRY.
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
// You should have received a copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

#[derive(Debug, Clone, PartialEq)]
/// The inner type for an annotation.
///
/// This Enum might grow additional variants, so library users should
/// **not** rely on exhaustive matching.
#[non_exhaustive]
pub enum Data {
    GrammarError(String),
    Repetition(String),
}

/// A single token representing a Markdown element.
///
/// A Markdown document is, thus, a Vec of `Token`s.
///
/// This Enum might grow additional variants, so library users should
/// **not** rely on exhaustive matching.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum Token {
    /// The most simple element, containing a String
    Str(String),
    /// A paragraph, containing a list of elements
    Paragraph(Vec<Token>),
    /// A header with a header number and the title
    Header(i32, Vec<Token>),
    /// *Emphasis*, a.k.a. italics
    Emphasis(Vec<Token>),
    /// **Strong**, a.k.a. bold
    Strong(Vec<Token>),
    /// `Code`, a.k.a. verbatim
    /// Strikethrough
    Strikethrough(Vec<Token>),
    Code(String),
    /// A quote
    BlockQuote(Vec<Token>),
    /// Code block with language and content
    CodeBlock(String, String),

    /// Superscript, indicated with ^...^
    Superscript(Vec<Token>),
    /// Subscript, indicated with ~...~
    Subscript(Vec<Token>),

    /// TaskItem. `bool` indicates wheteh it is checked.
    TaskItem(bool, Vec<Token>),

    /// Unordered list, with a vector of `Item`s
    List(Vec<Token>),
    /// Ordered list, with a starting number, and a list of `Item`s
    OrderedList(usize, Vec<Token>),
    /// Item of a list
    Item(Vec<Token>),

    /// Description list, containing description items
    DescriptionList(Vec<Token>),
    /// Description item, containing term and details
    DescriptionItem(Vec<Token>),
    /// Description term
    DescriptionTerm(Vec<Token>),
    /// Description details
    DescriptionDetails(Vec<Token>),

    /// Table with number of rows, and a list of `TableHead` and `TableRows`
    Table(i32, Vec<Token>),
    /// Table header, contains `TableCell`s
    TableHead(Vec<Token>),
    /// Row of a table, contains `TableCell`s
    TableRow(Vec<Token>),
    /// Cell of a table
    TableCell(Vec<Token>),

    /// A footnote reference, only contains the identifier of the footnote
    FootnoteReference(String),
    /// A footnote definition, contains the name and the content of the footnote
    FootnoteDefinition(String, Vec<Token>),

    /// Horizontal rule
    Rule,
    /// Softbreak, usually rendered by a space
    SoftBreak,
    /// Hardbreak
    HardBreak,

    /// A link with an url, a title, and the linked text
    Link(String, String, Vec<Token>),
    /// An image with a source url, a title and an alt tex
    Image(String, String, Vec<Token>),
    /// Similar to previous, but when image is in a standalone paragraph
    StandaloneImage(String, String, Vec<Token>),

    /// An annotation inserted by crowbook for e.g. grammar checking
    Annotation(Data, Vec<Token>),
}

use Token::*;

impl Token {
    /// Returns the inner list of tokens contained in this token (if any)
    pub fn inner(&self) -> Option<&[Token]> {
        match *self {
            Rule
            | SoftBreak
            | HardBreak
            | Str(_)
            | CodeBlock(_, _)
            | Code(_)
            | FootnoteReference(_) => None,

            Paragraph(ref v)
            | Header(_, ref v)
            | Emphasis(ref v)
            | Strong(ref v)
            | BlockQuote(ref v)
            | Subscript(ref v)
            | Superscript(ref v)
            | List(ref v)
            | OrderedList(_, ref v)
            | Item(ref v)
            | DescriptionList(ref v)
            | DescriptionItem(ref v)
            | DescriptionTerm(ref v)
            | DescriptionDetails(ref v)
            | Table(_, ref v)
            | TableHead(ref v)
            | TableRow(ref v)
            | TableCell(ref v)
            | FootnoteDefinition(_, ref v)
            | Link(_, _, ref v)
            | Image(_, _, ref v)
            | StandaloneImage(_, _, ref v)
            | Strikethrough(ref v)
            | TaskItem(_, ref v)
            | Annotation(_, ref v) => Some(v),
        }
    }

    /// Returns the inner list of tokens contained in this token (if any) (mutable version)
    pub fn inner_mut(&mut self) -> Option<&mut Vec<Token>> {
        match *self {
            Rule
            | SoftBreak
            | HardBreak
            | Str(_)
            | CodeBlock(_, _)
            | Code(_)
            | FootnoteReference(_) => None,

            Paragraph(ref mut v)
            | Annotation(_, ref mut v)
            | Header(_, ref mut v)
            | Emphasis(ref mut v)
            | Strong(ref mut v)
            | BlockQuote(ref mut v)
            | Subscript(ref mut v)
            | Superscript(ref mut v)
            | List(ref mut v)
            | OrderedList(_, ref mut v)
            | Item(ref mut v)
            | DescriptionList(ref mut v)
            | DescriptionItem(ref mut v)
            | DescriptionTerm(ref mut v)
            | DescriptionDetails(ref mut v)
            | Table(_, ref mut v)
            | TableHead(ref mut v)
            | TableRow(ref mut v)
            | TableCell(ref mut v)
            | FootnoteDefinition(_, ref mut v)
            | Link(_, _, ref mut v)
            | Image(_, _, ref mut v)
            | Strikethrough(ref mut v)
            | TaskItem(_, ref mut v)
            | StandaloneImage(_, _, ref mut v) => Some(v),
        }
    }

    /// Checks whether token is an str
    pub fn is_str(&self) -> bool {
        matches!(*self, Token::Str(_))
    }

    /// Checks whether token is an image
    ///
    /// **Returns** `true` if and only if token is Image variant
    /// (StandaloneImage returns *false*, like other variants)
    pub fn is_image(&self) -> bool {
        matches!(*self, Token::Image(_, _, _))
    }

    /// Checks whether token is a header.
    ///
    /// **Returns** `true` if and only if token is Header variant.
    pub fn is_header(&self) -> bool {
        matches!(*self, Token::Header(..))
    }

    /// Returns true if token is code or code block
    pub fn is_code(&self) -> bool {
        matches!(*self, Token::CodeBlock(..) | Token::Code(..))
    }

    /// Returns true if token is a container (paragraph, quote, emphasis, ..., but not links, images, and so on).
    pub fn is_container(&self) -> bool {
        matches!(
            *self,
            Token::Paragraph(..)
                | Token::Header(..)
                | Token::Emphasis(..)
                | Token::Strong(..)
                | Token::List(..)
                | Token::OrderedList(..)
                | Token::Table(..)
                | Token::TableHead(..)
                | Token::TableRow(..)
                | Token::FootnoteDefinition(..)
                | Token::TableCell(..)
                | Token::Annotation(..)
                | Token::Item(..)
                | Token::BlockQuote(..)
        )
    }
}
