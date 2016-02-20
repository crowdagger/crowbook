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

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Str(String), 
    Paragraph(Vec<Token>), 
    Header(i32, Vec<Token>), //title level, list of tokens
    Emphasis(Vec<Token>),
    Strong(Vec<Token>),
    Code(Vec<Token>),
    BlockQuote(Vec<Token>),
    CodeBlock(String, Vec<Token>), //language, content of the block

    List(Vec<Token>),
    OrderedList(usize, Vec<Token>), //starting number, list
    Item(Vec<Token>),
    
    Rule,
    SoftBreak,
    HardBreak,
    
    Link(String, String, Vec<Token>), //url, title, list
    Image(String, String, Vec<Token>), //url, title, alt text
}

