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

use error::{Result, Source};
use html::HtmlRenderer;
use book::{Book, compile_str};
use token::Token;
use templates::{html};
use renderer::Renderer;
use parser::Parser;

use rustc_serialize::base64::{self, ToBase64};

use std::convert::{AsMut, AsRef};


/// Single file HTML renderer
///
/// Renders a standalone, self-contained HTML file
pub struct HtmlSingleRenderer<'a> {
    html: HtmlRenderer<'a>,
}

impl<'a> HtmlSingleRenderer<'a> {
    /// Cretaes a new HtmlSingleRenderer
    pub fn new(book: &'a Book) -> HtmlSingleRenderer<'a> {
        let mut html = HtmlRenderer::new(book);
        html.handler.set_images_mapping(true);
        html.handler.set_base64(true);
        HtmlSingleRenderer {
            html: html,
        }
    }

    /// Renders a token
    ///
    /// Used by render_token implementation of Renderer trait. Separate function
    /// because we need to be able to call it from other renderers.
    ///
    /// See http://lise-henry.github.io/articles/rust_inheritance.html
    #[doc(hidden)]
    pub fn static_render_token<T>(this: &mut T, token: &Token) -> Result<String>
    where T: AsMut<HtmlSingleRenderer<'a>>+AsRef<HtmlSingleRenderer<'a>> +
        AsMut<HtmlRenderer<'a>>+AsRef<HtmlRenderer<'a>> + Renderer
    {
        HtmlRenderer::static_render_token(this, token)
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        let menu_svg = html::MENU_SVG.to_base64(base64::STANDARD);
        let menu_svg = format!("data:image/svg+xml;base64,{}", menu_svg);

        let book_svg = html::BOOK_SVG.to_base64(base64::STANDARD);
        let book_svg = format!("data:image/svg+xml;base64,{}", book_svg);

        let pages_svg = html::PAGES_SVG.to_base64(base64::STANDARD);
        let pages_svg = format!("data:image/svg+xml;base64,{}", pages_svg);

        for (i, filename) in self.html.book.filenames.iter().enumerate() {
            self.html.handler.add_link(filename.clone(), format!("#chapter-{}", i));
        }
        let mut content = String::new();

        let mut titles = vec!();
        let mut chapters = vec!();

        for (i, &(n, ref v)) in self.html.book.chapters.iter().enumerate() {
            self.html.chapter_config(i, n, String::new());
            
            let mut title = String::new();
            for token in v {
                match *token {
                    Token::Header(1, ref vec) => {
                        if self.html.current_hide || self.html.current_numbering == 0 {
                            title = try!(self.html.render_vec(vec));
                        } else {
                            title = try!(self.html.book.get_header(
                                self.html.current_chapter[0] + 1,
                                &try!(self.html.render_vec(vec))));
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
                try!(HtmlRenderer::render_html(self, v))));
        }
        self.html.source = Source::empty();

        for (i, chapter) in chapters.iter().enumerate() {
            if self.html.book.options.get_bool("html_single.one_chapter").unwrap()
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
            if self.html.book.options.get_bool("html_single.one_chapter").unwrap()
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
        
        let toc = self.html.toc.render();

        // If display_toc, display the toc inline
        if self.html.book.options.get_bool("rendering.inline_toc").unwrap() {
            content = format!(
                "<h1>{}</h1>
<div id = \"toc\">
{}
</div>
{}",
                self.html.book.options.get_str("rendering.inline_toc.name").unwrap(),
                &toc,
                content);
        }

        // Render the CSS
        let template_css = try!(compile_str(try!(self.html.book.get_template("html.css")).as_ref(),
                                            &self.html.book.source,
                                            "could not compile template 'html.css'"));
        let data = try!(self.html.book.get_metadata(|s| self.render_vec(&try!(Parser::new().parse_inline(s)))))
            .insert_bool(self.html.book.options.get_str("lang").unwrap(), true)
            .build();
        let mut res:Vec<u8> = vec!();
        template_css.render_data(&mut res, &data);
        let css = String::from_utf8_lossy(&res);

        // Render the JS
        let template_js = try!(compile_str(try!(self.html.book.get_template("html.script")).as_ref(),
                                           &self.html.book.source,
                                           "could not compile template 'html.script'"));
        let data = try!(self.html.book.get_metadata(|s| Ok(s.to_owned())))
            .insert_str("book_svg", &book_svg)
            .insert_str("pages_svg", &pages_svg)
            .insert_bool("display_chapter", self.html.book.options.get_bool("html_single.one_chapter").unwrap())
            .build();
        let mut res:Vec<u8> = vec!();
        template_js.render_data(&mut res, &data);
        let js = String::from_utf8_lossy(&res);

        // Render the HTML document
        let mut mapbuilder = try!(self.html.book.get_metadata(|s| self.render_vec(&try!(Parser::new().parse_inline(s)))))
            .insert_str("content", content)
            .insert_str("toc", toc)
            .insert_str("script", js)
            .insert_bool(self.html.book.options.get_str("lang").unwrap(), true)
            .insert_bool("display_chapter", self.html.book.options.get_bool("html_single.one_chapter").unwrap())
            .insert_str("style", css.as_ref())
            .insert_str("print_style", self.html.book.get_template("html.print_css").unwrap())
            .insert_str("menu_svg", menu_svg)
            .insert_str("book_svg", book_svg)
            .insert_str("footer", try!(self.html.get_footer()))
            .insert_str("top", try!(self.html.get_header()))
            .insert_str("pages_svg", pages_svg);
        if self.html.book.options.get_bool("html.highlight_code") == Ok(true) {
            let highlight_js = try!(self.html.book.get_template("html.highlight.js"))
                .as_bytes()
                .to_base64(base64::STANDARD);
            let highlight_js = format!("data:text/javascript;base64,{}", highlight_js);
            mapbuilder = mapbuilder.insert_bool("highlight_code", true)
                .insert_str("highlight_css", try!(self.html.book.get_template("html.highlight.css")))
                .insert_str("highlight_js", highlight_js);
        }
        let data = mapbuilder.build();
        let template = try!(compile_str(try!(self.html.book.get_template("html.template")).as_ref(),
                                        &self.html.book.source,
                                        "could not compile template 'html.template'"));
        let mut res = vec!();
        template.render_data(&mut res, &data);
        Ok(String::from_utf8_lossy(&res).into_owned())
    }
}

derive_html!{HtmlSingleRenderer<'a>, HtmlSingleRenderer::static_render_token}

