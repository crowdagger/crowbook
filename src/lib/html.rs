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

use escape::escape_html;
use token::Token;
use book::{Book, Number};
use error::{Error,Result};

use mustache;

/// Renderer for HTML
///
/// Also used by Epub.
pub struct HtmlRenderer<'a> {
    current_chapter: i32,
    book: &'a Book,
    current_numbering: bool
}

impl<'a> HtmlRenderer<'a> {
    /// Creates a new HTML renderer
    pub fn new(book: &'a Book) -> HtmlRenderer<'a> {
        HtmlRenderer {
            book: book,
            current_chapter: 1,
            current_numbering: book.numbering,
        }
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        let mut content = String::new();

        for &(n, ref v) in &self.book.chapters {
            match n {
                Number::Unnumbered => self.current_numbering = false,
                Number::Default => self.current_numbering = self.book.numbering,
                Number::Specified(n) => {
                    self.current_numbering = self.book.numbering;
                    self.current_chapter = n;
                }
            }
            for token in v {
                content.push_str(&self.parse_token(token));
            }
        }

        let template = mustache::compile_str(try!(self.book.get_template("html_template")).as_ref());        
        let data = self.book.get_mapbuilder()
            .insert_str("content", content)
            .insert_str("style",
                        &try!(self.book.get_template("html_css")))
            .build();

        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML was not utf-8 valid")),
            Ok(res) => Ok(res)
                          
        }
    }

    /// Transform a vector of `Token`s to HTML format.
    pub fn render_vec(&mut self, tokens: &[Token]) -> String {
        let mut res = String::new();
        
        for token in tokens {
            res.push_str(&self.parse_token(&token));
        }
        res
    }

    /// Parse a single token.
    pub fn parse_token(&mut self, token: &Token) -> String {
        match *token {
            Token::Str(ref text) => escape_html(&*text),
            Token::Paragraph(ref vec) => format!("<p>{}</p>\n", self.render_vec(vec)),
            Token::Header(n, ref vec) => {
                let s = if n == 1 && self.current_numbering {
                    let chapter = self.current_chapter;
                    self.current_chapter += 1;
                    self.book.get_header(chapter, &self.render_vec(vec)).unwrap()
                } else {
                    self.render_vec(vec)
                };
                format!("<h{}>{}</h{}>\n", n, s, n)
            },
            Token::Emphasis(ref vec) => format!("<em>{}</em>", self.render_vec(vec)),
            Token::Strong(ref vec) => format!("<b>{}</b>", self.render_vec(vec)),
            Token::Code(ref vec) => format!("<code>{}</code>", self.render_vec(vec)),
            Token::BlockQuote(ref vec) => format!("<blockquote>{}</blockquote>\n", self.render_vec(vec)),
            Token::CodeBlock(ref language, ref vec) => {
                let s = self.render_vec(vec);
                if language.is_empty() {
                    format!("<pre><code>{}</code></pre>\n", s)
                } else {
                    format!("<pre><code class = \"language-{}\">{}</code></pre>\n", language, s)
                }
            },
            Token::Rule => String::from("<p class = \"rule\">***</p>\n"),
            Token::SoftBreak => String::from(" "),
            Token::HardBreak => String::from("<br />\n"),
            Token::List(ref vec) => format!("<ul>\n{}</ul>\n", self.render_vec(vec)),
            Token::OrderedList(n, ref vec) => format!("<ol{}>\n{}</ol>\n",
                                                      if n != 1 {
                                                          format!(" start = \"{}\"", n)
                                                      } else {
                                                          String::new()
                                                      },
                                                      self.render_vec(vec)),
            Token::Item(ref vec) => format!("<li>{}</li>\n", self.render_vec(vec)),
            Token::Link(ref url, ref title, ref vec) => format!("<a href = \"{}\"{}>{}</a>",
                                                    url,
                                                    if title.is_empty() {
                                                        String::new()
                                                    } else {
                                                        format!(" title = \"{}\"", title)
                                                    },
                                                    self.render_vec(vec)),
            Token::Image(ref url, ref title, ref alt) => format!("<img src = \"{}\" title = \"{}\" alt = \"{}\" />",
                                                     url,
                                                     title,
                                                     self.render_vec(alt))
                
        }
    }
}
    



