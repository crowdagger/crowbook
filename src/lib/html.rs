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

use error::{Result, Error, Source};
use escape::escape_html;
use token::Token;
use book::Book;
use number::Number;
use toc::Toc;
use resource_handler::ResourceHandler;
use renderer::Renderer;
use lang;

use std::borrow::Cow;
use std::convert::{AsMut,AsRef};

/// Base structure for rendering HTML files
///
/// Used by EpubRenderer, HtmlSingleRenderer, HtmlDirRenderer
pub struct HtmlRenderer<'a> {
    table_head: bool,
    verbatim: bool,
    current_par: u32,
    first_letter: bool,
    first_paragraph: bool,
    footnotes: Vec<(String, String)>,
    filename: String,

    /// Current chapter (and subsection, subsubsection and so on)
    pub current_chapter: [i32;6],
    /// Current numbering level 
    pub current_numbering: i32,
    /// Whether current chapter's title must be displayed
    pub current_hide: bool,
    /// Resource handler
    pub handler: ResourceHandler<'a>,
    /// Current footnote number
    pub footnote_number: u32,
    /// Book that must be renderer
    pub book: &'a Book,
    /// Source for error messages
    pub source: Source,
    /// Table of contents
    pub toc: Toc,
    /// Current link number
    pub link_number: u32,

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
            current_par: 0,
            current_hide: false,
            table_head: false,
            footnote_number: 0,
            footnotes: vec!(),
            verbatim: false,
            filename: String::new(),
            handler: ResourceHandler::new(&book.logger),
            source: Source::empty(),
            first_letter: false,
            first_paragraph: true,
        };
        html.handler.set_images_mapping(true);
        html.handler.set_base64(true);
        html
    }

    /// Add a footnote which will be renderer later on
    pub fn add_footnote(&mut self, number: String, content: String) {
        self.footnotes.push((number, content));
    }

    /// Configure the Renderer for this chapter
    pub fn chapter_config(&mut self, i: usize, n: Number, filename: String) {
        self.source = Source::new(&self.book.filenames[i]);
        self.first_paragraph = true;
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
        self.filename = filename;
    }

    /// Renders a chapter to HTML
    pub fn render_html<T>(this: &mut T, tokens: &[Token])-> Result<String>
    where T: AsMut<HtmlRenderer<'a>>+AsRef<HtmlRenderer<'a>> + Renderer {
        let mut res = String::new();
        for token in tokens {
            res.push_str(&try!(this.render_token(token)));
            this.as_mut().render_side_notes(&mut res);
        }
        this.as_mut().render_end_notes(&mut res);
        Ok(res)
    }

    /// Renders a title (without <h1> tags), increasing header number beforehand
    pub fn render_title(&mut self, n: i32, vec: &[Token]) -> Result<String> {
        self.inc_header(n);
        let s = if n == 1 && self.current_numbering >= 1 {
            let chapter = self.current_chapter[0];
            try!(self.book.get_header(chapter, &try!(self.render_vec(vec))))
        } else if self.current_numbering >= n {
            format!("{} {}", self.get_numbers(), try!(self.render_vec(vec)))
        } else {
            try!(self.render_vec(vec))
        };
        Ok(s)
    }

    /// Renders a title, including <h1> tags and appropriate links
    pub fn render_title_full(&mut self, n: i32, inner: String) -> String {
        if n == 1 && self.current_hide {
            format!("<h1 id = \"link-{}\"></h1>", self.link_number)
        } else {
            format!("<h{} id = \"link-{}\">{}</h{}>\n",
                    n, self.link_number, inner, n)
        }
    }
    
    /// Increases a header if it needs to be
    ///
    /// Also sets up first_paragraph, link stuff and so on
    fn inc_header(&mut self, n: i32) {
        if n == 1 {
            self.first_paragraph = true;
        }
        if self.current_numbering >= n {
            assert!(n >= 1);
            let n = (n - 1) as usize;
            assert!(n < self.current_chapter.len());
            self.current_chapter[n] += 1;
            for i in n+1..self.current_chapter.len() {
                self.current_chapter[i] = 0;
            }
        }
        self.link_number += 1;
    }

    /// Returns a "x.y.z" corresponding to current chapter/section/...
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
    pub fn render_side_notes(&mut self, res: &mut String) {
        if self.book.options.get_bool("html.side_notes").unwrap() {
            for (note_number, footnote) in self.footnotes.drain(..) {
                res.push_str(&format!("<div class = \"sidenote\">\n{} {}\n</div>\n", note_number, footnote));
            }
        }
    }

    /// Display end notes, if side_notes option is set to false
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

    /// Renders a token
    ///
    /// Used by render_token implementation of Renderer trait. Separate function
    /// because we need to be able to call it from other renderers.
    ///
    /// See http://lise-henry.github.io/articles/rust_inheritance.html
    pub fn static_render_token<T>(this: &mut T, token: &Token) -> Result<String>
    where T: AsMut<HtmlRenderer<'a>>+AsRef<HtmlRenderer<'a>> + Renderer {
        match *token {
            Token::Str(ref text) => {
                let content = if this.as_ref().verbatim {
                    escape_html(text)
                } else {
                    escape_html(&this.as_ref().book.clean(text.clone(), false))
                };
                if this.as_ref().first_letter {
                    this.as_mut().first_letter = false;
                    if this.as_ref().book.options.get_bool("use_initials").unwrap() {
                        // Use initial
                        let mut chars = content.chars();
                        let initial = try!(chars.next()
                                          .ok_or(Error::Parser(this.as_ref().book.source.clone(),
                                                               "empty str token, could not find initial".to_owned())));
                        let mut new_content = if initial.is_alphanumeric() {
                            format!("<span class = \"initial\">{}</span>", initial)
                        } else {
                            format!("{}", initial)
                        };
                        for c in chars {
                            new_content.push(c);
                        }
                        Ok(new_content)
                    } else {
                        Ok(content)
                    }
                } else {
                    Ok(content)
                }
            },
            Token::Paragraph(ref vec) => {
                if this.as_ref().first_paragraph {
                    this.as_mut().first_paragraph = false;
                    if !vec.is_empty() && vec[0].is_str() {
                        // Only use initials if first element is a Token::str
                        this.as_mut().first_letter = true;
                    }
                }
                let class = if this.as_ref().first_letter && this.as_ref().book.options.get_bool("use_initials").unwrap() {
                    " class = \"first-para\""
                } else {
                    ""
                };
                let content = try!(this.render_vec(vec));
                this.as_mut().current_par += 1;
                let par = this.as_ref().current_par;
                Ok(format!("<p id = \"para-{}\"{}>{}</p>\n",
                           par,
                           class,
                           content))
            },
            Token::Header(n, ref vec) => {
                let s = try!(this.as_mut().render_title(n, vec));
                if n <= this.as_ref().book.options.get_i32("numbering").unwrap() {
                    let url = format!("{}#link-{}",
                                      this.as_ref().filename,
                                      this.as_ref().link_number);
                    this.as_mut().toc.add(n, url, s.clone());
                }
                Ok(this.as_mut().render_title_full(n, s))
            },
            Token::Emphasis(ref vec) => Ok(format!("<em>{}</em>", try!(this.render_vec(vec)))),
            Token::Strong(ref vec) => Ok(format!("<b>{}</b>", try!(this.render_vec(vec)))),
            Token::Code(ref vec) => Ok(format!("<code>{}</code>", try!(this.render_vec(vec)))),
            Token::BlockQuote(ref vec) => Ok(format!("<blockquote>{}</blockquote>\n", try!(this.render_vec(vec)))),
            Token::CodeBlock(ref language, ref vec) => {
                this.as_mut().verbatim = true;
                let s = try!(this.render_vec(vec));
                let output = if language.is_empty() {
                    format!("<pre><code>{}</code></pre>\n", s)
                } else {
                    format!("<pre><code class = \"language-{}\">{}</code></pre>\n", language, s)
                };
                this.as_mut().verbatim = false;
                Ok(output)
            },
            Token::Rule => Ok(String::from("<p class = \"rule\">***</p>\n")),
            Token::SoftBreak => Ok(String::from(" ")),
            Token::HardBreak => Ok(String::from("<br />\n")),
            Token::List(ref vec) => Ok(format!("<ul>\n{}</ul>\n", try!(this.render_vec(vec)))),
            Token::OrderedList(n, ref vec) => Ok(format!("<ol{}>\n{}</ol>\n",
                                                      if n == 1 {
                                                          String::new()
                                                      } else {
                                                          format!(" start = \"{}\"", n)
                                                      },
                                                      try!(this.render_vec(vec)))),
            Token::Item(ref vec) => Ok(format!("<li>{}</li>\n", try!(this.render_vec(vec)))),
            Token::Link(ref url, ref title, ref vec) => {
                let url = escape_html(url);
                let url = if ResourceHandler::is_local(&url) {
                    this.as_ref().handler.get_link(&url).to_owned()
                } else {
                    url
                };
                
                Ok(format!("<a href = \"{}\"{}>{}</a>", url,
                        if title.is_empty() {
                            String::new()
                        } else {
                            format!(" title = \"{}\"", title)
                        },
                        try!(this.render_vec(vec))))
            },
            Token::Image(ref url, ref title, ref alt)
                | Token::StandaloneImage(ref url, ref title, ref alt) => {
                    let content = try!(this.render_vec(alt));
                    let html: &mut HtmlRenderer = this.as_mut();
                    let url = try!(html.handler.map_image(&html.source,
                                                          Cow::Borrowed(url)));

                    if token.is_image() {
                        Ok(format!("<img src = \"{}\" title = \"{}\" alt = \"{}\" />",
                                url,
                                title,
                                content))
                    } else {
                        Ok(format!("<div class = \"image\">
  <img src = \"{}\" title = \"{}\" alt = \"{}\" />
</div>", 
                                url,
                                title,
                                content))
                    }
                },
            Token::Table(_, ref vec) => Ok(format!("<div class = \"table\">
    <table>\n{}
    </table>
</div>\n",
                                                   try!(this.render_vec(vec)))),
            Token::TableRow(ref vec) => Ok(format!("<tr>\n{}</tr>\n", try!(this.render_vec(vec)))),
            Token::TableCell(ref vec) => {
                let tag = if this.as_ref().table_head {"th"} else {"td"};
                Ok(format!("<{}>{}</{}>", tag, try!(this.render_vec(vec)), tag))
            },
            Token::TableHead(ref vec) => {
                this.as_mut().table_head = true;
                let s = try!(this.render_vec(vec));
                this.as_mut().table_head = false;
                Ok(format!("<tr>\n{}</tr>\n", s))
            },
            Token::Footnote(ref vec) => {
                this.as_mut().footnote_number += 1;
                let number = this.as_ref().footnote_number;
                assert!(!vec.is_empty());

                let note_number = format!("<p class = \"note-number\">
  <a href = \"#note-source-{}\">[{}]</a>
</p>", number, number);

                let inner = format!("<aside id = \"note-dest-{}\">{}</aside>",
                                    number,
                                    try!(this.render_vec(vec)));
                this.as_mut().footnotes.push((note_number, inner));
                
                Ok(format!("<a href = \"#note-dest-{}\"><sup id = \"note-source-{}\">{}</sup></a>",
                           number, number, number))
            },
        }
    }


    /// Renders a footer, which can include a "Generated by Crowboook" link
    /// or a customized text
    pub fn get_footer(&self) -> String {
        let content =
            if let Ok(footer) = self.book.options.get_str("html.footer") {
                footer.to_owned()
            } else {
                if self.book.options.get_bool("html.crowbook_link") == Ok(true) {
                    lang::get_str(self.book.options.get_str("lang").unwrap(), "generated_by_crowbook")
                } else {
                    String::new()
                }
            };
        if content.is_empty() {
            content
        } else {
            format!("<footer><p>{}</p></footer>", content)
        }
    }

    /// Renders a header
    pub fn get_top(&self) -> String {
        if let Ok(top) = self.book.options.get_str("html.top") {
            format!("<div id = \"top\"><p>{}</p></div>", top)
        } else {
            String::new()
        }
    }
}

impl<'a> AsMut<HtmlRenderer<'a>> for HtmlRenderer<'a> {
    fn as_mut(&mut self) -> &mut HtmlRenderer<'a> {
        self
    }
}

impl<'a> AsRef<HtmlRenderer<'a>> for HtmlRenderer<'a> {
    fn as_ref(&self) -> &HtmlRenderer<'a> {
        self
    }
}

impl<'a> Renderer for HtmlRenderer<'a> {
    fn render_token(&mut self, token: &Token) -> Result<String> {
        HtmlRenderer::static_render_token(self, token)
    }
}


/// This macro automatically generates AsRef and AsMut implementations
/// for a type, to itself and to HtmlRenderer. Type must have a .html element
/// and use a <'a> lifetime parameter.
macro_rules! derive_html {
    {$t:ty, $f:path} => (
        impl<'a> AsRef<HtmlRenderer<'a>> for $t {
            fn as_ref(&self) -> &HtmlRenderer<'a> {
                &self.html
            }
        }

        impl<'a> AsMut<HtmlRenderer<'a>> for $t {
            fn as_mut(&mut self) -> &mut HtmlRenderer<'a> {
                &mut self.html
            }
        }

        impl<'a> AsRef<$t> for $t {
            fn as_ref(&self) -> &$t {
                self
            }
        }
        
        impl<'a> AsMut<$t> for $t {
            fn as_mut(&mut self) -> &mut $t {
                self
            }
        }

        impl<'a> Renderer for $t {
            fn render_token(&mut self, token: &Token) -> Result<String> {
                $f(self, token)
            }
        }

    );
}
    



