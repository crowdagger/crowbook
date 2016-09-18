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
use templates::{html};
use renderer::Renderer;
use lang;

use std::borrow::Cow;
use std::convert::{AsMut,AsRef};

use mustache;
use rustc_serialize::base64::{self, ToBase64};

/// Renders HTML document in a standalone file.
///
/// Also used by `EpubRenderer` and `HtmlDirRenderer`.
pub struct HtmlRenderer<'a> {
    table_head: bool,
    verbatim: bool,
    link_number: u32,
    add_script: bool,
    current_par: u32,
    current_chapter_internal: i32,
    first_letter: bool,
    first_paragraph: bool,

    /// Current footnote number
    pub footnote_number: u32,
    /// Book that must be renderer
    pub book: &'a Book,
    /// Source for error messages
    pub source: Source,
    /// Table of contents
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
            current_chapter_internal: -1,
            current_numbering: book.options.get_i32("numbering").unwrap(),
            current_par: 0,
            add_script: false,
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

    /// Configure the Renderer for this chapter
    pub fn chapter_config(&mut self, i: usize, n: Number) {
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
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        self.add_script = self.book.options.get_bool("html.display_chapter").unwrap();
        let menu_svg = html::MENU_SVG.to_base64(base64::STANDARD);
        let menu_svg = format!("data:image/svg+xml;base64,{}", menu_svg);

        let book_svg = html::BOOK_SVG.to_base64(base64::STANDARD);
        let book_svg = format!("data:image/svg+xml;base64,{}", book_svg);

        let pages_svg = html::PAGES_SVG.to_base64(base64::STANDARD);
        let pages_svg = format!("data:image/svg+xml;base64,{}", pages_svg);

        for (i, filename) in self.book.filenames.iter().enumerate() {
            self.handler.add_link(filename.clone(), format!("#chapter-{}", i));
        }
        let mut content = String::new();

        let mut titles = vec!();
        let mut chapters = vec!();

        for (i, &(n, ref v)) in self.book.chapters.iter().enumerate() {
            self.chapter_config(i, n);
            
            let mut title = String::new();
            for token in v {
                match *token {
                    Token::Header(1, ref vec) => {
                        if self.current_hide || self.current_numbering == 0 {
                            title = try!(self.render_vec(vec));
                        } else {
                            title = try!(self.book.get_header(
                                self.current_chapter[0] + 1,
                                &try!(self.render_vec(vec))));
                        }
                        break;
                    },
                    _ => {
                        continue;
                    }
                }
            }
            titles.push(title);
            
            chapters.push(format!(
                "<div id = \"chapter-{}\" class = \"chapter\">
  {}
</div>",
                i,
                try!(self.render_html(v))));
        }
        self.source = Source::empty();

        for (i, chapter) in chapters.iter().enumerate() {
            if self.book.options.get_bool("html.display_chapter").unwrap()
                && i != 0 {
                content.push_str(&format!(
                    "<p onclick = \"javascript:showChapter({})\" class = \"chapterControls prev_chapter chapter-{}\">
  <a href = \"#chapter-{}\">
  « {}
  </a>
</p>",
                    i - 1,
                    i,
                    i - 1,
                    titles[i -1]));
            }
            content.push_str(chapter);
            if self.book.options.get_bool("html.display_chapter").unwrap()
                && i < titles.len() - 1 {
                content.push_str(&format!(
                    "<p onclick = \"javascript:showChapter({})\" class = \"chapterControls next_chapter chapter-{}\">
  <a href = \"#chapter-{}\">
  {} »
  </a>
</p>",
                    i + 1,
                    i,
                    i + 1,
                    titles[i + 1]));
            }
        }
        
        let toc = self.toc.render();

        // If display_toc, display the toc inline
        if self.book.options.get_bool("display_toc").unwrap() {
            content = format!(
                "<h1>{}</h1>
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

        // Render the JS
        let template_js = mustache::compile_str(try!(self.book.get_template("html.script")).as_ref());
        let data = self.book.get_mapbuilder("none")
            .insert_str("book_svg", &book_svg)
            .insert_str("pages_svg", &pages_svg)
            .insert_bool("display_chapter", self.book.options.get_bool("html.display_chapter").unwrap())
            .build();
        let mut res:Vec<u8> = vec!();
        template_js.render_data(&mut res, &data);
        let js = String::from_utf8_lossy(&res);

        // Render the HTML document
        let mut mapbuilder = self.book.get_mapbuilder("none")
            .insert_str("content", content)
            .insert_str("toc", toc)
            .insert_str("script", js)
            .insert_bool(self.book.options.get_str("lang").unwrap(), true)
            .insert_bool("display_chapter", self.book.options.get_bool("html.display_chapter").unwrap())
            .insert_str("style", css.as_ref())
            .insert_str("print_style", self.book.get_template("html.print_css").unwrap())
            .insert_str("menu_svg", menu_svg)
            .insert_str("book_svg", book_svg)
            .insert_str("footer", self.get_footer())
            .insert_str("top", self.get_top())
            .insert_str("pages_svg", pages_svg);
        if self.book.options.get_bool("html.highlight_code") == Ok(true) {
            let highlight_js = try!(self.book.get_template("html.highlight.js"))
                .as_bytes()
                .to_base64(base64::STANDARD);
            let highlight_js = format!("data:text/javascript;base64,{}", highlight_js);
            mapbuilder = mapbuilder.insert_bool("highlight_code", true)
                .insert_str("highlight_css", try!(self.book.get_template("html.highlight.css")))
                .insert_str("highlight_js", highlight_js);
        }
        let data = mapbuilder.build();
        let template = mustache::compile_str(try!(self.book.get_template("html.template")).as_ref());        
        let mut res = vec!();
        template.render_data(&mut res, &data);
        Ok(String::from_utf8_lossy(&res).into_owned())
    }

    /// Renders a chapter to HTML
    pub fn render_html(&mut self, tokens: &[Token])-> Result<String> {
        let mut res = String::new();
        for token in tokens {
            res.push_str(&try!(self.render_token(&token)));
            self.render_side_notes(&mut res);
        }
        self.render_end_notes(&mut res);
        Ok(res)
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
        if self.book.options.get_bool("html.side_notes").unwrap() {
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
                if n == 1 {
                    this.as_mut().current_chapter_internal += 1;
                    this.as_mut().first_paragraph = true;
                }
                if this.as_ref().current_numbering >= n {
                    this.as_mut().inc_header(n - 1);
                }
                this.as_mut().link_number += 1;
                let s = if n == 1 && this.as_ref().current_numbering >= 1 {
                    let chapter = this.as_ref().current_chapter[0];
                    try!(this.as_ref().book.get_header(chapter, &try!(this.render_vec(vec))))
                } else if this.as_ref().current_numbering >= n {
                    format!("{} {}", this.as_ref().get_numbers(), try!(this.render_vec(vec)))
                } else {
                    try!(this.render_vec(vec))
                };
                if n <= this.as_ref().book.options.get_i32("numbering").unwrap() {
                    let url = if this.as_ref().add_script {
                        format!("{}#link-{}\" onclick = \"javascript:showChapter({})",
                                this.as_ref().filename,
                                this.as_ref().link_number,
                                this.as_ref().current_chapter_internal)
                    } else {
                        format!("{}#link-{}",
                                this.as_ref().filename,
                                this.as_ref().link_number)
                    };
                    this.as_mut().toc.add(n, url, s.clone());
                }
                if n == 1 && this.as_ref().current_hide {
                    Ok(format!("<h1 id = \"link-{}\"></h1>", this.as_ref().link_number))
                } else {
                    Ok(format!("<h{} id = \"link-{}\">{}</h{}>\n",
                            n, this.as_ref().link_number, s, n))
                }
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

    



