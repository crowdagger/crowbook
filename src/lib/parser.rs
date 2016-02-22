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

use token::Token;
use error::{Result,Error};

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::mem::replace;

use cmark::{Parser as CMParser, Event, Tag, Options, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES};

/// A parser that reads markdown and convert it to AST (a vector of `Token`s)
pub struct Parser {
    footnotes: HashMap<String, Vec<Token>>,
}

impl Parser {
    /// Creates a parser with the default options
    pub fn new() -> Parser {
        Parser {
            footnotes: HashMap::new(),
        }
    }

    /// Parse a file and returns an AST
    pub fn parse_file(&mut self, filename: &str) -> Result<Vec<Token>> {
        let mut f = try!(File::open(filename).map_err(|_| Error::FileNotFound(String::from(filename))));
        let mut s = String::new();

        try!(f.read_to_string(&mut s).map_err(|_| Error::Parser("file contains invalid UTF-8, could not parse it")));
        self.parse(&s)
    }

    /// Replace footnote reference with their definition
    pub fn parse_footnotes(&mut self, v: &mut Vec<Token>) ->Result<()> {
        for token in v {
            match *token {
                Token::Footnote(ref mut content) => {
                    let reference = if let Token::Str(ref text) = content[0] {
                        text.clone()
                    } else {
                        panic!("Oups");
                    };
                    if let Some(in_vec) = self.footnotes.get(&reference) {
                        *content = in_vec.clone();
                    } else {
                        return Err(Error::Parser("footnote reference without matching definition"));
                    }
                },
                Token::Paragraph(ref mut vec) | Token::Header(_, ref mut vec) | Token::Emphasis(ref mut vec)
                    | Token::Strong(ref mut vec) | Token::Code(ref mut vec) | Token::BlockQuote(ref mut vec)
                    | Token::CodeBlock(_, ref mut vec) | Token::List(ref mut vec) | Token::OrderedList(_, ref mut vec)
                    | Token::Item(ref mut vec) | Token::Table(_, ref mut vec) | Token::TableHead(ref mut vec)
                    | Token::TableRow(ref mut vec) | Token::TableCell(ref mut vec) | Token::Link(_, _, ref mut vec)
                    | Token::Image(_, _, ref mut vec) => try!(self.parse_footnotes(vec)),
                _ => (),
            }
        }
        Ok(())
    }
    

    /// Parse a string and returns an AST, that is a vector of `Token`s
    ///
    /// Returns a result, at this method might fail.
    pub fn parse(&mut self, s: &str) -> Result<Vec<Token>> {
        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);
        let mut p = CMParser::new_ext(s, opts);
        

        let mut res = vec!();
        try!(self.parse_events(&mut p, &mut res, None));

        try!(self.parse_footnotes(&mut res));
        Ok(res)
    }
    
    fn parse_events<'a>(&mut self, p: &mut CMParser<'a>, v: &mut Vec<Token>, current_tag: Option<&Tag>) -> Result<()> {
        while let Some(event) = p.next() {
            match event {
                Event::Html(text) | Event::InlineHtml(text) | Event::Text(text) => {
                    if let Some(&Token::Str(_)) = v.last() {
                        if let Token::Str(ref mut s) = *v.last_mut().unwrap() {
                            s.push_str(text.as_ref());
                        } else {
                            unreachable!();
                        }
                    } else {
                        v.push(Token::Str(text.into_owned()));
                    }
                },
                Event::Start(tag) => try!(self.parse_tag(p, v, tag)),
                Event::End(tag) => {
                    debug_assert!(format!("{:?}", Some(&tag)) == format!("{:?}", current_tag),
                                  format!("Error: opening and closing tags mismatch!\n{:?} ≠ {:?}",
                                          tag,
                                          current_tag));
                    break;
                },
                Event::SoftBreak => v.push(Token::SoftBreak),
                Event::HardBreak => v.push(Token::HardBreak),
                Event::FootnoteReference(text) => v.push(Token::Footnote(vec!(Token::Str(text.into_owned())))),
            }
        }
        Ok(())
    }

    fn parse_tag<'a>(&mut self, p: &mut CMParser<'a>, v: &mut Vec<Token>, tag: Tag<'a>) -> Result<()> {
        let mut res = vec!();

        try!(self.parse_events(p, &mut res, Some(&tag)));

        
        let token = match tag {
            Tag::Paragraph => Token::Paragraph(res),
            Tag::Emphasis => Token::Emphasis(res),
            Tag::Strong => Token::Strong(res),
            Tag::Code => Token::Code(res),
            Tag::Header(x) => Token::Header(x, res),
            Tag::Link(url, title) => Token::Link(url.into_owned(), title.into_owned(), res),
            Tag::Image(url, title) => Token::Image(url.into_owned(), title.into_owned(), res),
            Tag::Rule => Token::Rule,
            Tag::List(opt) => {
                if let Some(n) = opt {
                    Token::OrderedList(n, res)
                } else {
                    Token::List(res)
                }},
            Tag::Item => Token::Item(res),
            Tag::BlockQuote => Token::BlockQuote(res),
            Tag::CodeBlock(language) => Token::CodeBlock(language.into_owned(), res),
            Tag::Table(n) => Token::Table(n, res),
            Tag::TableHead => Token::TableHead(res),
            Tag::TableRow => Token::TableRow(res),
            Tag::TableCell => Token::TableCell(res),
            Tag::FootnoteDefinition(reference) => {
                if self.footnotes.contains_key(reference.as_ref()) {
                    println!("Warning: footnote definition for {} but previous definition already exist, overriding it", reference);
                }
                self.footnotes.insert(reference.into_owned(), res);
                Token::SoftBreak
            },
        };
        v.push(token);
        Ok(())
    }
}



