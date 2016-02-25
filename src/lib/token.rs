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

/// A single token representing a Markdown element.
///
/// A Markdown document is, thus, a Vec of `Token`s.
#[derive(Debug, PartialEq, Clone)]
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
    Code(Vec<Token>),
    /// > A quote
    BlockQuote(Vec<Token>),
    /// Code block with language and content
    CodeBlock(String, Vec<Token>), 

    /// Unordered list, with a vector of `Item`s
    List(Vec<Token>),
    /// Ordered list, with a starting number, and a list of `Item`s
    OrderedList(usize, Vec<Token>),
    /// Item of a list
    Item(Vec<Token>),

    /// Table with number of rows, and a list of `TableHead` and `TableRows`
    Table(i32, Vec<Token>),
    /// Table header, contains `TableCell`s
    TableHead(Vec<Token>),
    /// Row of a table, contains `TableCell`s
    TableRow(Vec<Token>),
    /// Cell of a table
    TableCell(Vec<Token>),

    /// A footnote, contains the content it is pointing to.
    Footnote(Vec<Token>),

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
}

