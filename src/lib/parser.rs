use cmark::{Parser as CMParser, Event, Tag};
use token::Token;
use cleaner::Cleaner;
use error::{Result,Error};

/// A parser that reads markdown and convert it to AST (a vector of `Token`s)
pub struct Parser {
    numbering: Option<String>, // None for no numbering, or a String with the name you want
    cleaner: Option<Box<Cleaner>>, // An optional parameter to clean source code

    verbatim: bool, // set to true when in e.g. a code block
}

impl Parser {
    /// Creates a parser with the default options
    pub fn new() -> Parser {
        Parser {
            verbatim: false,
            numbering: None,
            cleaner: Some(Box::new(())),
        }
    }

    pub fn with_cleaner(mut self, cleaner: Box<Cleaner>) -> Parser {
        self.cleaner = Some(cleaner);
        self
    }

    /// Parse a string and returns an AST, that is a vector of `Token`s
    ///
    /// Returns a result, at this method might fail.
    pub fn parse<'a>(&mut self, s: &'a str) -> Result<Vec<Token<'a>>> {
        let mut p = CMParser::new(s);

        let mut res = vec!();
        try!(self.parse_events(&mut p, &mut res, None));
        Ok(res)
    }
    
    fn parse_events<'a>(&mut self, p: &mut CMParser<'a>, v: &mut Vec<Token<'a>>, current_tag: Option<&Tag>) -> Result<()> {
        while let Some(event) = p.next() {
            match event {
                Event::Text(mut text) => {
                    if let Some(ref cleaner) = self.cleaner {
                        if !self.verbatim {
                            cleaner.clean(&mut text);
                        }
                    }
                    v.push(Token::Str(text));
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
                Event::Html(_) | Event::InlineHtml(_) => return Err(Error::Parser("No support for HTML code inside of Markdown, sorry.")),
                Event::FootnoteReference(_) => return Err(Error::Parser("No support for footnotes yet."))
            }
        }
        Ok(())
    }

    fn parse_tag<'a>(&mut self, p: &mut CMParser<'a>, v: &mut Vec<Token<'a>>, tag: Tag<'a>) -> Result<()> {
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
            Tag::Link(url, title) => Token::Link(url, title, res),
            Tag::Image(url, title) => Token::Image(url, title, res),
            Tag::Rule => Token::Rule,
            Tag::List(opt) => {
                if let Some(n) = opt {
                    Token::OrderedList(n, res)
                } else {
                    Token::List(res)
                }},
            Tag::Item => Token::Item(res),
            Tag::BlockQuote => Token::BlockQuote(res),
            Tag::CodeBlock(language) => Token::CodeBlock(language, res),
            Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell => return Err(Error::Parser("No support for tables yet")),
            Tag::FootnoteDefinition(_) => return Err(Error::Parser("No support for footnotes")),
        };
        v.push(token);
        Ok(())
    }
}



