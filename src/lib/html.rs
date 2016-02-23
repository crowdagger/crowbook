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
    pub current_chapter: i32,
    epub3: bool,
    book: &'a Book,
    pub current_numbering: bool,
    pub current_hide: bool,
    table_head: bool,
    footnote_number: u32,
    pub footnotes: Vec<(String, String)>,
}

impl<'a> HtmlRenderer<'a> {
    /// Creates a new HTML renderer
    pub fn new(book: &'a Book) -> HtmlRenderer<'a> {
        HtmlRenderer {
            book: book,
            current_chapter: 1,
            current_numbering: book.get_bool("numbering").unwrap(),
            current_hide: false,
            table_head: false,
            footnote_number: 0,
            footnotes: vec!(),
            epub3: false,
        }
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        let mut content = String::new();

        for &(n, ref v) in &self.book.chapters {
            self.current_hide = false;
            let book_numbering = self.book.get_bool("numbering").unwrap();
            match n {
                Number::Unnumbered => self.current_numbering = false,
                Number::Default => self.current_numbering = book_numbering,
                Number::Specified(n) => {
                    self.current_numbering = book_numbering;
                    self.current_chapter = n;
                },
                Number::Hidden => {
                    self.current_numbering = false;
                    self.current_hide = true;
                },
            }
            content.push_str(&self.render_html(v));
        }

        let template = mustache::compile_str(try!(self.book.get_template("html.template")).as_ref());        
        let data = self.book.get_mapbuilder("none")
            .insert_str("content", content)
            .insert_str("style",
                        &try!(self.book.get_template("html.css")))
            .build();

        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML was not utf-8 valid")),
            Ok(res) => Ok(res)
        }
    }

    /// display side notes if option is to true
    pub fn render_side_notes(&mut self, res: &mut String) {
        if self.book.get_bool("side_notes").unwrap() {
            for (note_number, footnote) in self.footnotes.drain(..) {
                res.push_str(&format!("<div class = \"sidenote\">\n{} {}\n</div>\n", note_number, footnote));
            }
        }
    }

    // display end notes, if side_notes option is set to false
    pub fn render_end_notes(&mut self, res: &mut String) {
        if !self.footnotes.is_empty() {
            res.push_str("<h2 class = \"notes\">Notes</h2>");
            res.push_str("<table class = \"notes\">");
            for (note_number, footnote) in self.footnotes.drain(..) {
                res.push_str(&format!("<tr class = \"notes\"><td class = \"notes\">{}</td><td class = \"notes\">{}</td></tr>\n", note_number, footnote));
            }
            res.push_str("</table>");
        }
    }
    
    ///
    pub fn render_html(&mut self, tokens: &[Token])-> String {
        let mut res = String::new();
        for token in tokens {
            res.push_str(&self.parse_token(&token));
            self.render_side_notes(&mut res);
        }
        self.render_end_notes(&mut res);
        res
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
            Token::Str(ref text) => escape_html(&self.book.clean(text.clone())),
            Token::Paragraph(ref vec) => format!("<p>{}</p>\n", self.render_vec(vec)),
            Token::Header(n, ref vec) => {
                if n == 1 && self.current_hide {
                    return String::new();
                }
                let s = if n == 1 && self.current_numbering {
                    let chapter = self.current_chapter;
                    self.current_chapter += 1;
                    self.book.get_header(chapter, &self.render_vec(vec)).unwrap()
                } else {
                    self.render_vec(vec)
                };
                format!("<h{}>{}</h{}>\n", n, self.book.clean(s), n)
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
                                                                 self.render_vec(alt)),
            Token::Table(_, ref vec) => format!("<div class = \"table\">
    <table>\n{}
    </table>
</div>\n", self.render_vec(vec)),
            Token::TableRow(ref vec) => format!("<tr>\n{}</tr>\n", self.render_vec(vec)),
            Token::TableCell(ref vec) => {
                let tag = if self.table_head {"th"} else {"td"};
                format!("<{}>{}</{}>", tag, self.render_vec(vec), tag)
            },
            Token::TableHead(ref vec) => {
                self.table_head = true;
                let s = self.render_vec(vec);
                self.table_head = false;
                format!("<tr>\n{}</tr>\n", s)
            },
            Token::Footnote(ref vec) => {
                self.footnote_number += 1;
                let number = self.footnote_number;
                assert!(!vec.is_empty());

                let note_number = format!("<p class = \"note-number\">
  <a href = \"#note-source-{}\">[{}]</a>:
</p>", number, number);

                let inner = format!("<aside {} id = \"note-dest-{}\">{}</aside>",
                                    if self.epub3 {r#"epub:type="footnote"#}else{""},
                                    number, self.render_vec(vec));
                self.footnotes.push((note_number, inner));
                
                format!("<a {} href = \"#note-dest-{}\"><sup id = \"note-source-{}\">{}</sup></a>",
                        if self.epub3 {"epub:type = \"noteref\""} else {""},
                        number, number, number)
            },
        }
    }
}
    



