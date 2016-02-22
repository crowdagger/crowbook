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
use cleaner::Cleaner;
use error::{Result,Error};

use std::fs::File;
use std::io::Read;

use cmark::{Parser as CMParser, Event, Tag};

/// A parser that reads markdown and convert it to AST (a vector of `Token`s)
pub struct Parser {
    cleaner: Option<Box<Cleaner>>, // An optional parameter to clean source code

    verbatim: bool, // set to true when in e.g. a code block
}

impl Parser {
    /// Creates a parser with the default options
    pub fn new() -> Parser {
        Parser {
            verbatim: false,
            cleaner: Some(Box::new(())),
        }
    }

    /// Sets cleaner implementation
    pub fn with_cleaner(mut self, cleaner: Box<Cleaner>) -> Parser {
        self.cleaner = Some(cleaner);
        self
    }

    /// Parse a file and returns an AST
    pub fn parse_file(&mut self, filename: &str) -> Result<Vec<Token>> {
        let mut f = try!(File::open(filename).map_err(|_| Error::FileNotFound(String::from(filename))));
        let mut s = String::new();

        try!(f.read_to_string(&mut s).map_err(|_| Error::Parser("file contains invalid UTF-8, could not parse it")));
        self.parse(&s)
    }

    /// Parse a string and returns an AST, that is a vector of `Token`s
    ///
    /// Returns a result, at this method might fail.
    pub fn parse(&mut self, s: &str) -> Result<Vec<Token>> {
        let mut p = CMParser::new(s);

        let mut res = vec!();
        try!(self.parse_events(&mut p, &mut res, None));
        Ok(res)
    }
    
    fn parse_events<'a>(&mut self, p: &mut CMParser<'a>, v: &mut Vec<Token>, current_tag: Option<&Tag>) -> Result<()> {
        while let Some(event) = p.next() {
            match event {
                Event::Html(text) | Event::InlineHtml(text) | Event::Text(text) => {
                    let mut text = text.into_owned();
                    if let Some(&Token::Str(_)) = v.last() {
                        if let Token::Str(ref mut s) = *v.last_mut().unwrap() {
                            s.push_str(&text);
                            if let Some(ref cleaner) = self.cleaner {
                                if !self.verbatim {
                                    cleaner.clean(s);
                                }
                            }
                        } else {
                            unreachable!();
                        }
                    } else {
                        if let Some(ref cleaner) = self.cleaner {
                            if !self.verbatim {
                                cleaner.clean(&mut text);
                            }
                        }
                        v.push(Token::Str(text));
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
                Event::FootnoteReference(_) => return Err(Error::Parser("no support for footnotes yet."))
            }
        }
        Ok(())
    }

    fn parse_tag<'a>(&mut self, p: &mut CMParser<'a>, v: &mut Vec<Token>, tag: Tag<'a>) -> Result<()> {
        let mut res = vec!();

        match tag {
            Tag::Code | Tag::CodeBlock(_) => self.verbatim = true,
            _ => (),
        }
        
        try!(self.parse_events(p, &mut res, Some(&tag)));

        self.verbatim = false;
        
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
            Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell => return Err(Error::Parser("no support for tables yet")),
            Tag::FootnoteDefinition(_) => return Err(Error::Parser("no support for footnotes")),
        };
        v.push(token);
        Ok(())
    }
}



