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
use error::{Result, Error, Source};
use book::Book;

use std::mem;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use std::io::Read;
use std::collections::HashMap;
use std::ops::BitOr;

use cmark::{Parser as CMParser, Event, Tag, Options, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES};




#[derive(Debug, Copy, Clone, PartialEq)]
/// The list of features used in a document.
pub struct Features {
    pub image: bool,
    pub blockquote: bool,
    pub codeblock: bool,
    pub ordered_list: bool,
    pub footnote: bool,
    pub table: bool,
    pub url: bool,
    pub subscript: bool,
    pub superscript: bool,
}

impl Features {
    /// Creates a new set of features where all are set to false
    pub fn new() -> Features {
        Features {
            image: false,
            blockquote: false,
            codeblock: false,
            ordered_list: false,
            footnote: false,
            table: false,
            url: false,
            subscript: false,
            superscript: false,
        }
    }
}


impl BitOr for Features {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Features {
            image: self.image | rhs.image,
            blockquote: self.blockquote | rhs.blockquote,
            codeblock: self.codeblock | rhs.codeblock,
            ordered_list: self.ordered_list | rhs.ordered_list,
            footnote: self.footnote | rhs.footnote,
            table: self.table | rhs.table,
            url: self.url | rhs.url,
            subscript: self.subscript | rhs.subscript,
            superscript: self.superscript | rhs.superscript,
        }
    }
}


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
    features: Features,

    html_as_text: bool,
    superscript: bool,
}

impl Parser {
    /// Creates a parser
    pub fn new() -> Parser {
        Parser {
            footnotes: HashMap::new(),
            source: Source::empty(),
            features: Features::new(),
            html_as_text: true,
            superscript: false,
        }
    }

    /// Creates a parser with options from a book configuration file
    pub fn from(book: &Book) -> Parser {
        let mut parser = Parser::new();
        parser.html_as_text = book.options.get_bool("crowbook.html_as_text").unwrap();
        parser.superscript = book.options.get_bool("crowbook.markdown.superscript").unwrap();
        parser
    }
    
    /// Enable/disable HTML as text
    pub fn html_as_text(&mut self, b: bool) {
        self.html_as_text = b;
    }

    /// Sets a parser's source file
    pub fn set_source_file(&mut self, s: &str) {
        self.source = Source::new(s);
    }

    /// Parse a file and returns an AST or  an error
    pub fn parse_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<Vec<Token>> {
        let path: &Path = filename.as_ref();
        let mut f = File::open(path)
            .map_err(|_| {
                Error::file_not_found(&self.source,
                                      lformat!("markdown file"),
                                      format!("{}", path.display()))
            })?;
        let mut s = String::new();

        f.read_to_string(&mut s)
            .map_err(|_| {
                Error::parser(&self.source,
                              lformat!("file {file} contains invalid UTF-8, could not parse it",
                                       file = path.display()))
            })?;
        self.parse(&s)
    }

    /// Parse a string and returns an AST  an Error.
    pub fn parse(&mut self, s: &str) -> Result<Vec<Token>> {
        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);
        let mut p = CMParser::new_ext(s, opts);


        let mut res = vec![];
        self.parse_events(&mut p, &mut res, None)?;

        self.parse_footnotes(&mut res)?;

        collapse(&mut res);

        find_standalone(&mut res);

        // Transform superscript and subscript
        if self.superscript {
            self.parse_super_vec(&mut res);
            self.parse_sub_vec(&mut res);
        }
        
        Ok(res)
    }

    /// Parse an inline string and returns a list of `Token`.
    ///
    /// This function removes the outermost `Paragraph` in most of the
    /// cases, as it is meant to be used for an inline string (e.g. metadata)
    pub fn parse_inline(&mut self, s: &str) -> Result<Vec<Token>> {
        let mut tokens = self.parse(s)?;
        // Unfortunately, parser will put all this in a paragraph, so we might need to remove it.
        if tokens.len() == 1 {
            let res = match tokens[0] {
                Token::Paragraph(ref mut v) => Some(mem::replace(v, vec![])),
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

    /// Returns the list of features used by this parser
    pub fn features(&self) -> Features {
        self.features
    }


    /// Replace footnote reference with their definition
    fn parse_footnotes(&mut self, v: &mut Vec<Token>) -> Result<()> {
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
                                                 lformat!("footnote reference {reference} does \
                                                           not have a matching definition",
                                                          reference = &reference)));
                    }
                }
                Token::Paragraph(ref mut vec) |
                Token::Header(_, ref mut vec) |
                Token::Emphasis(ref mut vec) |
                Token::Strong(ref mut vec) |
                Token::Code(ref mut vec) |
                Token::BlockQuote(ref mut vec) |
                Token::CodeBlock(_, ref mut vec) |
                Token::List(ref mut vec) |
                Token::OrderedList(_, ref mut vec) |
                Token::Item(ref mut vec) |
                Token::Table(_, ref mut vec) |
                Token::TableHead(ref mut vec) |
                Token::TableRow(ref mut vec) |
                Token::TableCell(ref mut vec) |
                Token::Link(_, _, ref mut vec) |
                Token::Image(_, _, ref mut vec) => self.parse_footnotes(vec)?,
                _ => (),
            }
        }
        Ok(())
    }
    
    /// Looks for super script in a vector of tokens
    fn parse_super_vec(&mut self, mut v: &mut Vec<Token>) {
        for i in 0..v.len() {
            let new = if v[i].is_str() {
                if let Token::Str(ref s) = v[i] {
                    parse_super_sub(s, b'^')
                } else {
                    unreachable!()
                }
            } else {
                if v[i].is_code() || !v[i].is_container() {
                    continue;
                } 
                if let Some(ref mut inner) = v[i].inner_mut() {
                    self.parse_super_vec(inner);
                }
                None
            };
            if let Some(mut new) = new {
                self.features.superscript = true;
                let mut post = v.split_off(i);
                post.remove(0);
                self.parse_super_vec(&mut post);
                v.append(&mut new);
                v.append(&mut post);
                return;
            }
        }
    }

    /// Looks for subscript in a vector of token
    fn parse_sub_vec(&mut self, mut v: &mut Vec<Token>) {
        for i in 0..v.len() {
            let new = if v[i].is_str() {
                if let Token::Str(ref s) = v[i] {
                    parse_super_sub(s, b'~')
                } else {
                    unreachable!()
                }
            } else {
                if v[i].is_code() || !v[i].is_container() {
                    continue;
                } 
                if let Some(ref mut inner) = v[i].inner_mut() {
                    self.parse_sub_vec(inner);
                }
                None
            };
            if let Some(mut new) = new {
                self.features.subscript = true;
                let mut post = v.split_off(i);
                post.remove(0);
                self.parse_sub_vec(&mut post);
                v.append(&mut new);
                v.append(&mut post);
                return;
            }
        }
    }

    fn parse_events<'a>(&mut self,
                        p: &mut CMParser<'a>,
                        v: &mut Vec<Token>,
                        current_tag: Option<&Tag>)
                        -> Result<()> {
        while let Some(event) = p.next() {
            match event {
                Event::Html(text) | Event::InlineHtml(text) => {
                    if self.html_as_text {
                        v.push(Token::Str(text.into_owned()));
                    } else {
                        debug!("{}", lformat!("ignoring HTML block '{}'", text));
                    }
                }, 

                Event::Text(text) => {
                    v.push(Token::Str(text.into_owned()));
                }
                Event::Start(tag) => self.parse_tag(p, v, tag)?,
                Event::End(tag) => {
                    debug_assert!(format!("{:?}", Some(&tag)) == format!("{:?}", current_tag),
                                  format!("Error: opening and closing tags mismatch!\n{:?} ≠ \
                                           {:?}",
                                          tag,
                                          current_tag));
                    break;
                }
                Event::SoftBreak => v.push(Token::SoftBreak),
                Event::HardBreak => v.push(Token::HardBreak),
                Event::FootnoteReference(text) => {
                    v.push(Token::Footnote(vec![Token::Str(text.into_owned())]))
                }
            }
        }
        Ok(())
    }

    fn parse_tag<'a>(&mut self,
                     p: &mut CMParser<'a>,
                     v: &mut Vec<Token>,
                     tag: Tag<'a>)
                     -> Result<()> {
        let mut res = vec![];

        self.parse_events(p, &mut res, Some(&tag))?;


        let token = match tag {
            Tag::Paragraph => Token::Paragraph(res),
            Tag::Emphasis => Token::Emphasis(res),
            Tag::Strong => Token::Strong(res),
            Tag::Code => Token::Code(res),
            Tag::Header(x) => Token::Header(x, res),
            Tag::Link(url, title) => {
                self.features.url = true;
                Token::Link(url.into_owned(), title.into_owned(), res)
            },
            Tag::Image(url, title) => {
                self.features.image = true;
                Token::Image(url.into_owned(), title.into_owned(), res)
            },
            Tag::Rule => Token::Rule,
            Tag::List(opt) => {
                if let Some(n) = opt {
                    self.features.ordered_list = true;
                    Token::OrderedList(n, res)
                } else {
                    Token::List(res)
                }
            }
            Tag::Item => Token::Item(res),
            Tag::BlockQuote => {
                self.features.blockquote = true;
                Token::BlockQuote(res)
            },
            Tag::CodeBlock(language) => {
                self.features.codeblock = true;
                Token::CodeBlock(language.into_owned(), res)
            },
            Tag::Table(v) => {
                self.features.table = true;
                // TODO: actually use v's alignments
                Token::Table(v.len() as i32, res)
            },
            Tag::TableHead => Token::TableHead(res),
            Tag::TableRow => Token::TableRow(res),
            Tag::TableCell => Token::TableCell(res),
            Tag::FootnoteDefinition(reference) => {
                if self.footnotes.contains_key(reference.as_ref()) {
                    warn!("{}", lformat!("in {file}, found footnote definition for \
                                          note '{reference}' but previous \
                                          definition already exist, overriding it",
                                         file = self.source,
                                         reference = reference));
                }
                self.footnotes.insert(reference.into_owned(), res);
                Token::SoftBreak
            }
        };
        v.push(token);
        Ok(())
    }
}


/// Look to a string and see if there is some superscript or subscript in it.
/// If there, returns a vec of tokens.
///
/// params: s: the string to parse, c, either b'^' for superscript or b'~' for subscript.
fn parse_super_sub(s: &str, c: u8) -> Option<Vec<Token>> {
    let match_indices:Vec<_> = s.match_indices(c as char).collect();
    if match_indices.is_empty() {
        return None;
    }
    let to_escape = format!("\\{}", c as char);
    let escaped = format!("{}", c as char);
    let escape = |s: String| -> String {
        s.replace(&to_escape, &escaped)
    };
    for (begin, _) in match_indices {
        let bytes = s.as_bytes();
        let len = bytes.len();
        // Check if ^ was escaped
        if begin > 0 && bytes[begin - 1] == b'\\' {
            continue;
        } else if begin + 1 >= len {
            return None;
        } else {
            let mut i = begin + 1;
            let mut sup = vec![];
            let mut end = None;
            while i < len {
                match bytes[i] {
                    b'\\' => {
                        if i+1 < len && bytes[i+1] == b' ' {
                            sup.push(b' ');
                            i += 2;
                        } else if i + 1 < len && bytes[i+1] == c {
                            sup.push(c);
                            i += 2;
                        } else {
                            sup.push(b'\\');
                            i += 1;
                        }
                    },
                    b' ' => {
                        return None;
                    },
                    b if b == c => {
                        end = Some(i);
                        break;
                    },
                    b => {
                        sup.push(b);
                        i += 1;
                    },
                }
            }
            if sup.is_empty() {
                return None;
            }
            if let Some(end) = end {
                let mut tokens = vec![];
                if begin > 0 {
                    let pre_part = String::from_utf8((&bytes[0..begin])
                                                     .to_owned())
                        .unwrap();
                    tokens.push(Token::Str(escape(pre_part)));
                }
                let sup_part = String::from_utf8(sup).unwrap();
                match c {
                    b'^' => tokens.push(Token::Superscript(vec![Token::Str(sup_part)])),
                    b'~' => tokens.push(Token::Subscript(vec![Token::Str(sup_part)])),
                    _ => unimplemented!(),
                }
                if end+1 < len {
                    let post_part = String::from_utf8((&bytes[end + 1..]).to_owned()).unwrap();
                    if let Some(mut v) = parse_super_sub(&post_part, c) {
                        tokens.append(&mut v);
                    } else {
                        tokens.push(Token::Str(escape(post_part)));
                    }
                }
                return Some(tokens);
            } else {
                return None;
            }
        }
    }
    return None;
}

/// Replace consecutives Strs by a Str of both, collapse soft breaks to previous std and so on
fn collapse(ast: &mut Vec<Token>) {
    let mut i = 0;
    while i < ast.len() {
        if ast[i].is_str() && i + 1 < ast.len() {
            if ast[i + 1].is_str() {
                // Two consecutives Str, concatenate them
                let token = ast.remove(i + 1);
                if let (&mut Token::Str(ref mut dest), Token::Str(ref source)) = (&mut ast[i],
                                                                                  token) {
                    //                        dest.push(' ');
                    dest.push_str(source);
                    continue;
                } else {
                    unreachable!();
                }
            } else if ast[i + 1] == Token::SoftBreak {
                ast.remove(i + 1);
                if let &mut Token::Str(ref mut dest) = &mut ast[i] {
                    dest.push(' ');
                    continue;
                } else {
                    unreachable!();
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
                    if let Token::Image(source, title, inner) = mem::replace(&mut inner[0],
                                                                             Token::Rule) {
                        Token::StandaloneImage(source, title, inner)
                    } else {
                        unreachable!();
                    }
                } else {
                    // If paragraph only contains a link only containing an image, ok too
                    // Fixme: messy code and unnecessary clone
                    if let Token::Link(ref url, ref alt, ref mut inner) = inner[0] {
                        if inner[0].is_image() {
                            if let Token::Image(source, title, inner) = mem::replace(&mut inner[0],
                                                                                     Token::Rule) {
                                Token::Link(url.clone(), alt.clone(), vec![Token::StandaloneImage(source, title, inner)])
                            } else {
                                unreachable!();
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
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


#[test]
fn test_parse_super_str() {
    let c = b'^';
    assert!(parse_super_sub("String without superscript", c).is_none());
    assert!(parse_super_sub("String \\^without\\^ superscript", c).is_none());
    assert!(parse_super_sub("String ^without superscript", c).is_none());
    assert!(parse_super_sub("String ^without superscript^", c).is_none());
    assert_eq!(parse_super_sub("^up^", c),
               Some(vec!(Token::Superscript(vec!(Token::Str(String::from("up")))))));
    assert_eq!(parse_super_sub("foo^up^ bar", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Superscript(vec!(Token::Str("up".to_owned()))),
                         Token::Str(" bar".to_owned()))));
    assert_eq!(parse_super_sub("foo^up^ bar^up^baz", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Superscript(vec!(Token::Str("up".to_owned()))),
                         Token::Str(" bar".to_owned()),
                         Token::Superscript(vec!(Token::Str("up".to_owned()))),
                         Token::Str("baz".to_owned()))));
    assert_eq!(parse_super_sub("foo^up^ bar^baz", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Superscript(vec!(Token::Str("up".to_owned()))),
                         Token::Str(" bar^baz".to_owned()))));
    assert_eq!(parse_super_sub("foo\\^bar^up^", c),
               Some(vec!(Token::Str("foo^bar".to_owned()),
                         Token::Superscript(vec!(Token::Str("up".to_owned()))))));
    assert_eq!(parse_super_sub("foo^bar\\^up^", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Superscript(vec!(Token::Str("bar^up".to_owned()))))));
    assert_eq!(parse_super_sub("foo^bar up^", c),
               None);
    assert_eq!(parse_super_sub("foo^bar\\ up^", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Superscript(vec!(Token::Str("bar up".to_owned()))))));

}

#[test]
fn test_parse_supb_str() {
    let c = b'~';
    assert!(parse_super_sub("String without subscript", c).is_none());
    assert!(parse_super_sub("String \\~without\\~ subscript", c).is_none());
    assert!(parse_super_sub("String ~without subscript", c).is_none());
    assert!(parse_super_sub("String ~without\nsubscript", c).is_none());
    assert!(parse_super_sub("String ~without subscript~", c).is_none());
    assert_eq!(parse_super_sub("~down~", c),
               Some(vec!(Token::Subscript(vec!(Token::Str(String::from("down")))))));
    assert_eq!(parse_super_sub("foo~down~ bar", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Subscript(vec!(Token::Str("down".to_owned()))),
                         Token::Str(" bar".to_owned()))));
    assert_eq!(parse_super_sub("foo~down~ bar~down~baz", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Subscript(vec!(Token::Str("down".to_owned()))),
                         Token::Str(" bar".to_owned()),
                         Token::Subscript(vec!(Token::Str("down".to_owned()))),
                         Token::Str("baz".to_owned()))));
    assert_eq!(parse_super_sub("foo~down~ bar~baz", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Subscript(vec!(Token::Str("down".to_owned()))),
                         Token::Str(" bar~baz".to_owned()))));
    assert_eq!(parse_super_sub("foo\\~bar~down~", c),
               Some(vec!(Token::Str("foo~bar".to_owned()),
                         Token::Subscript(vec!(Token::Str("down".to_owned()))))));
    assert_eq!(parse_super_sub("foo~bar\\~down~", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Subscript(vec!(Token::Str("bar~down".to_owned()))))));
    assert_eq!(parse_super_sub("foo~bar down~", c),
               None);
    assert_eq!(parse_super_sub("foo~bar\\ down~", c),
               Some(vec!(Token::Str("foo".to_owned()),
                         Token::Subscript(vec!(Token::Str("bar down".to_owned()))))));

}
