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
use book::Book;
use number::Number;
use error::Result;
use toc::Toc;
use resource_handler::ResourceHandler;
use std::borrow::Cow;
use templates::html;

use mustache;
use rustc_serialize::base64::{self, ToBase64};

/// Renders HTML document in a standalone file.
///
/// Also used by `EpubRenderer` and `HtmlDirRenderer`.
#[derive(Debug)]
pub struct HtmlRenderer<'a> {
    book: &'a Book,
    table_head: bool,
    footnote_number: u32,
    epub3: bool,
    verbatim: bool,
    link_number: u32,

    // fields used by EpubRenderer so marked public but hidden
    #[doc(hidden)]
    pub toc: Toc,
    #[doc(hidden)]
    pub footnotes: Vec<(String, String)>,
    #[doc(hidden)]
    pub current_chapter: [i32;6],
    #[doc(hidden)]
    pub current_numbering: i32,
    #[doc(hidden)]
    pub current_hide: bool,
    #[doc(hidden)]
    pub filename: String,
    #[doc(hidden)]
    pub handler: ResourceHandler<'a>,
}

impl<'a> HtmlRenderer<'a> {
    /// Creates a new HTML renderer
    pub fn new(book: &'a Book) -> HtmlRenderer<'a> {
        let mut html = HtmlRenderer {
            book: book,
            toc: Toc::new(),
            link_number: 0,
            current_chapter: [0, 0, 0, 0, 0, 0],
            current_numbering: book.options.get_i32("numbering").unwrap(),
            current_hide: false,
            table_head: false,
            footnote_number: 0,
            footnotes: vec!(),
            epub3: false,
            verbatim: false,
            filename: String::new(),
            handler: ResourceHandler::new(&book.logger),
        };
        html.handler.set_images_mapping(true);
        html.handler.set_base64(true);
        html
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        let menu_svg = html::MENU_SVG.to_base64(base64::STANDARD);
        let menu_svg = format!("data:image/svg+xml;base64,{}", menu_svg);
         
        for (i, filename) in self.book.filenames.iter().enumerate() {
            self.handler.add_link(filename.clone(), format!("#chapter-{}", i));
        }
        let mut content = String::new();

        let mut i = 0;
        for &(n, ref v) in &self.book.chapters {
            content.push_str(&format!("<a id = \"chapter-{}\"></a>\n", i));
            self.current_hide = false;
            let book_numbering = self.book.options.get_i32("numbering").unwrap();
            match n {
                Number::Unnumbered => self.current_numbering = 0,
                Number::Default => self.current_numbering = book_numbering,
                Number::Specified(n) => {
                    self.current_numbering = book_numbering;
                    self.current_chapter[0] = n - 1;
                },
                Number::Hidden => {
                    self.current_numbering = 0;
                    self.current_hide = true;
                },
            }
            content.push_str(&self.render_html(v));
            i+= 1;
        }
        let toc = self.toc.render();

        // If display_toc, display the toc inline
        if self.book.options.get_bool("display_toc").unwrap() {
            content = format!("<h1>{}</h1>
<div id = \"toc\">
{}
</div>
{}",
                              self.book.options.get_str("toc_name").unwrap(),
                              &toc,
                              content);
        }

        // Render the CSS
        let template_css = mustache::compile_str(try!(self.book.get_template("html.css")).as_ref());
        let data = self.book.get_mapbuilder("none")
            .insert_bool(self.book.options.get_str("lang").unwrap(), true)
            .build();
        let mut res:Vec<u8> = vec!();
        template_css.render_data(&mut res, &data);
        let css = String::from_utf8_lossy(&res);

        // Render the HTML document
        let data = self.book.get_mapbuilder("none")
            .insert_str("content", content)
            .insert_str("toc", toc)
            .insert_str("script", self.book.get_template("html.script").unwrap())
            .insert_bool(self.book.options.get_str("lang").unwrap(), true)
            .insert_str("style", css.as_ref())
            .insert_str("menu_svg", menu_svg)
            .build();
        let template = mustache::compile_str(try!(self.book.get_template("html.template")).as_ref());        
        let mut res = vec!();
        template.render_data(&mut res, &data);
        Ok(String::from_utf8_lossy(&res).into_owned())
    }

    /// Renders a chapter to HTML
    pub fn render_html(&mut self, tokens: &[Token])-> String {
        let mut res = String::new();
        for token in tokens {
            res.push_str(&self.parse_token(&token));
            self.render_side_notes(&mut res);
        }
        self.render_end_notes(&mut res);
        res
    }


    /// Increase a header
    fn inc_header(&mut self, n: i32) {
        let n = n as usize;
        assert!(n < self.current_chapter.len());
        self.current_chapter[n] += 1;
        for i in n+1..self.current_chapter.len() {
            self.current_chapter[i] = 0;
        }
    }

    /// Returns a "x.y.z"
    fn get_numbers(&self) -> String {
        let mut output = String::new();
        for i in 0..self.current_chapter.len() {
            if self.current_chapter[i] == 0 {
                if i == self.current_chapter.len() - 1 {
                    break;
                }
                let bools:Vec<_> = self.current_chapter[i+1..].iter().map(|x| *x != 0).collect();
                if !bools.contains(&true) {
                    break;
                }
            }
            output.push_str(&format!("{}.", self.current_chapter[i])); //todo
        }
        output
    }


    /// Display side notes if option is to true
    ///
    /// Only public because EpubRenderer uses it
    #[doc(hidden)]
    pub fn render_side_notes(&mut self, res: &mut String) {
        if self.book.options.get_bool("side_notes").unwrap() {
            for (note_number, footnote) in self.footnotes.drain(..) {
                res.push_str(&format!("<div class = \"sidenote\">\n{} {}\n</div>\n", note_number, footnote));
            }
        }
    }

    /// Display end notes, if side_notes option is set to false
    ///
    /// Only public because EpubRenderer uses it
    #[doc(hidden)]
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
    
    /// Transform a vector of `Token`s to HTML format.
    ///
    /// Only public becauses `EpubRenderer` uses it
    #[doc(hidden)]
    pub fn render_vec(&mut self, tokens: &[Token]) -> String {
        let mut res = String::new();
        
        for token in tokens {
            res.push_str(&self.parse_token(&token));
        }
        res
    }

    /// Parse a single token.
    ///
    /// Only public because EpubRenderer uses it
    #[doc(hidden)]
    pub fn parse_token(&mut self, token: &Token) -> String {
        match *token {
            Token::Str(ref text) => if self.verbatim {
                escape_html(text)
            } else {
                escape_html(&self.book.clean(text.clone()))
            },
            Token::Paragraph(ref vec) => format!("<p>{}</p>\n", self.render_vec(vec)),
            Token::Header(n, ref vec) => {
                if self.current_numbering >= n {
                    self.inc_header(n - 1);
                }
                self.link_number += 1;
                let s = if n == 1 && self.current_numbering >= 1 {
                    let chapter = self.current_chapter[0];
                    self.book.get_header(chapter, &self.render_vec(vec)).unwrap()
                } else if self.current_numbering >= n {
                    format!("{} {}", self.get_numbers(), self.render_vec(vec))
                } else {
                    self.render_vec(vec)
                };
                if n <= self.book.options.get_i32("numbering").unwrap() {
                    self.toc.add(n,
                                 format!("{}#link-{}",
                                            self.filename,
                                            self.link_number),
                                 s.clone());
                }
                if n == 1 && self.current_hide {
                    format!("<a id = \"link-{}\"></a>", self.link_number)
                } else {
                    format!("<h{} id = \"link-{}\">{}</h{}>\n",
                            n, self.link_number, s, n)
                }
            },
            Token::Emphasis(ref vec) => format!("<em>{}</em>", self.render_vec(vec)),
            Token::Strong(ref vec) => format!("<b>{}</b>", self.render_vec(vec)),
            Token::Code(ref vec) => format!("<code>{}</code>", self.render_vec(vec)),
            Token::BlockQuote(ref vec) => format!("<blockquote>{}</blockquote>\n", self.render_vec(vec)),
            Token::CodeBlock(ref language, ref vec) => {
                self.verbatim = true;
                let s = self.render_vec(vec);
                let output = if language.is_empty() {
                    format!("<pre><code>{}</code></pre>\n", s)
                } else {
                    format!("<pre><code class = \"language-{}\">{}</code></pre>\n", language, s)
                };
                self.verbatim = false;
                output
            },
            Token::Rule => String::from("<p class = \"rule\">***</p>\n"),
            Token::SoftBreak => String::from(" "),
            Token::HardBreak => String::from("<br />\n"),
            Token::List(ref vec) => format!("<ul>\n{}</ul>\n", self.render_vec(vec)),
            Token::OrderedList(n, ref vec) => format!("<ol{}>\n{}</ol>\n",
                                                      if n == 1 {
                                                          String::new()
                                                      } else {
                                                          format!(" start = \"{}\"", n)
                                                      },
                                                      self.render_vec(vec)),
            Token::Item(ref vec) => format!("<li>{}</li>\n", self.render_vec(vec)),
            Token::Link(ref url, ref title, ref vec) => {
                let url: String = if ResourceHandler::is_local(url) {
                    self.handler.get_link(url).to_owned()
                } else {
                    url.clone()
                };
                
                format!("<a href = \"{}\"{}>{}</a>", url,
                        if title.is_empty() {
                            String::new()
                        } else {
                            format!(" title = \"{}\"", title)
                        },
                        self.render_vec(vec))
            },
            Token::Image(ref url, ref title, ref alt) => {
                let content = self.render_vec(alt);
                let url = self.handler.map_image(Cow::Borrowed(url));
                format!("<img src = \"{}\" title = \"{}\" alt = \"{}\" />",
                        url,
                        title,
                        content)
            },
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
  <a href = \"#note-source-{}\">[{}]</a>
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
    



