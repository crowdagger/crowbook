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
use error::{Result,Error, Source};
use logger::Logger;

use std::mem;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use std::io::Read;
use std::collections::HashMap;

use cmark::{Parser as CMParser, Event, Tag, Options, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES};

/// A parser that reads markdown and convert it to AST (a vector of `Token`s)
///
/// This AST can then be used by various renderes.
///
/// As this Parser uses Pulldown-cmark's one, it should be able to parse most
/// *valid* CommonMark variant of Markdown.
///
/// Compared to other Markdown parser, it might fail more often on invalid code, e.g.
/// footnotes references that are not defined anywhere.
///
/// # Examples
///
/// ```
/// use crowbook::Parser;
/// let mut parser = Parser::new();
/// let result = parser.parse("Some *valid* Markdown[^1]\n\n[^1]: with a valid footnote");
/// assert!(result.is_ok());
/// ```
///
/// ```
/// use crowbook::Parser;
/// let mut parser = Parser::new();
/// let result = parser.parse("Some footnote pointing to nothing[^1] ");
/// assert!(result.is_err());
/// ```
pub struct Parser {
    footnotes: HashMap<String, Vec<Token>>,
    source: Source,
}

impl Parser {
    /// Creates a parser
    pub fn new() -> Parser {
        Parser {
            footnotes: HashMap::new(),
            source: Source::empty(),
        }
    }

    /// Sets a parser's source file
    pub fn set_source_file(&mut self, s: &str) {
        self.source = Source::new(s);
    }

    /// Parse a file and returns an AST or an error
    pub fn parse_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<Vec<Token>> {
        let path: &Path = filename.as_ref();
        let mut f = try!(File::open(path).map_err(|_| Error::file_not_found(&self.source,
                                                                                lformat!("markdown file"),
                                                                                format!("{}", path.display()))));
        let mut s = String::new();

        try!(f.read_to_string(&mut s).map_err(|_| Error::parser(&self.source,
                                                                lformat!("file {file} contains invalid UTF-8, could not parse it",
                                                                         file = path.display()))));
        self.parse(&s)
    }

    /// Parse a string and returns an AST, or an Error.
    pub fn parse(&mut self, s: &str) -> Result<Vec<Token>> {
        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);
        let mut p = CMParser::new_ext(s, opts);
        

        let mut res = vec!();
        try!(self.parse_events(&mut p, &mut res, None));

        try!(self.parse_footnotes(&mut res));

        collapse(&mut res);

        find_standalone(&mut res);
        Ok(res)
    }

    /// Parse an inline string and returns a list of `Token`.
    ///
    /// This function removes the outermost `Paragraph` in most of the
    /// cases, as it is meant to be used for an inline string (e.g. metadata)
    pub fn parse_inline(&mut self, s: &str) -> Result<Vec<Token>> {
        let mut tokens = try!(self.parse(s));
        // Unfortunately, parser will put all this in a paragraph, so we might need to remove it.
        if tokens.len() == 1 {
            let res = match tokens[0] {
                Token::Paragraph(ref mut v) => {
                    Some(mem::replace(v, vec!()))
                },
                _ => None,
            };
            match res {
                Some(tokens) => Ok(tokens),
                _ => Ok(tokens),
            }
        } else {
            Ok(tokens)
        }
    }


    /// Replace footnote reference with their definition
    fn parse_footnotes(&mut self, v: &mut Vec<Token>) ->Result<()> {
        for token in v {
            match *token {
                Token::Footnote(ref mut content) => {
                    let reference = if let Token::Str(ref text) = content[0] {
                        text.clone()
                    } else {
                        panic!("Reference is not a vector of a single Token::Str");
                    };
                    if let Some(in_vec) = self.footnotes.get(&reference) {
                        *content = in_vec.clone();
                    } else {
                        return Err(Error::parser(&self.source,
                                                 lformat!("footnote reference {reference} does not have a matching definition",
                                                          reference = &reference)));
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
    
    fn parse_events<'a>(&mut self, p: &mut CMParser<'a>, v: &mut Vec<Token>, current_tag: Option<&Tag>) -> Result<()> {
        while let Some(event) = p.next() {
            match event {
                Event::Html(text) | Event::InlineHtml(text) | Event::Text(text) => {
                    v.push(Token::Str(text.into_owned()));
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
                    Logger::display_warning(lformat!("in {file}, found footnote definition for note '{reference}' \
                                                      but previous definition already exist, overriding it",
                                                     file = self.source,
                                                     reference = reference));
                }
                self.footnotes.insert(reference.into_owned(), res);
                Token::SoftBreak
            },
        };
        v.push(token);
        Ok(())
    }
}

/// Replace consecutives Strs by a Str of both, collapse soft breaks to previous std and so on
fn collapse(ast: &mut Vec<Token>) {
    if ast.len() < 2 {
        return;
    }
        
    let mut i = 0;
    while i < ast.len() {
        if ast[i].is_str() {
            if i < ast.len() - 1 {
                if ast[i+1].is_str() {
                    // Two consecutives Str, concatenate them
                    let token = ast.remove(i+1);
                    if let (&mut Token::Str(ref mut dest), Token::Str(ref source)) = (&mut ast[i], token) {
//                        dest.push(' ');
                        dest.push_str(source);
                        continue;
                    } else {
                        unreachable!();
                    }
                } else if ast[i+1] == Token::SoftBreak {
                    ast.remove(i+1);
                    if let &mut Token::Str(ref mut dest) = &mut ast[i] {
                        dest.push(' ');
                        continue;
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        // If token is containing others, recurse into them
        if let Some(ref mut inner) = ast[i].inner_mut() {
            collapse(inner);
        }
        i += 1;
    }
}

/// Replace images which are alone in a paragraph by standalone images
fn find_standalone(ast: &mut Vec<Token>) {
    for token in ast {
        let res = if let &mut Token::Paragraph(ref mut inner) = token {
            if inner.len() == 1 {
                if inner[0].is_image() {
                    if let Token::Image(source, title, inner) = mem::replace(&mut inner[0], Token::Rule) {
                        Token::StandaloneImage(source, title, inner)
                    } else {
                        unreachable!();
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            continue;
        };

        *token = res;
    }
}
