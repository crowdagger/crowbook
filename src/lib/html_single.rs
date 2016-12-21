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

use error::{Error, Result, Source};
use html::HtmlRenderer;
use book::{Book, compile_str};
use token::Token;
use templates::img;
use renderer::Renderer;
use book_renderer::BookRenderer;
use parser::Parser;

use rustc_serialize::base64::{self, ToBase64};

use std::convert::{AsMut, AsRef};
use std::io::Write;

/// Single file HTML renderer
///
/// Renders a standalone, self-contained HTML file
pub struct HtmlSingleRenderer<'a> {
    html: HtmlRenderer<'a>,
}

impl<'a> HtmlSingleRenderer<'a> {
    /// Creates a new HtmlSingleRenderer
    pub fn new(book: &'a Book) -> HtmlSingleRenderer<'a> {
        let mut html = HtmlRenderer::new(book);
        html.handler.set_images_mapping(true);
        html.handler.set_base64(true);
        HtmlSingleRenderer { html: html }
    }

    /// Set aproofreading to true
    pub fn proofread(mut self) -> HtmlSingleRenderer<'a> {
        self.html.proofread = true;
        self
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
        let menu_svg = img::MENU_SVG.to_base64(base64::STANDARD);
        let menu_svg = format!("data:image/svg+xml;base64,{}", menu_svg);

        let book_svg = img::BOOK_SVG.to_base64(base64::STANDARD);
        let book_svg = format!("data:image/svg+xml;base64,{}", book_svg);

        let pages_svg = img::PAGES_SVG.to_base64(base64::STANDARD);
        let pages_svg = format!("data:image/svg+xml;base64,{}", pages_svg);

        let mut content = String::new();

        let mut titles = vec![];
        let mut chapters = vec![];
        let render_notes_chapter =
            self.html.book.options.get_bool("html_single.one_chapter").unwrap();

        for (i, chapter) in self.html.book.chapters.iter().enumerate() {
            self.html.handler.add_link(chapter.filename.as_ref(),
                                       format!("#chapter-{}", i));
        }
        
        for (i, chapter) in self.html.book.chapters.iter().enumerate() {
            let n = chapter.number;
            let v = &chapter.content;
            self.html.chapter_config(i, n, String::new());

            let mut title = String::new();
            for token in v {
                match *token {
                    Token::Header(1, ref vec) => {
                        if self.html.current_hide || self.html.current_numbering == 0 {
                            title = self.html.render_vec(vec)?;
                        } else {
                            title = self.html
                                .book
                                .get_chapter_header(self.html.current_chapter[1] + 1,
                                                    self.html.render_vec(vec)?,
                                                    |s| {
                                                        self.render_vec(&Parser::new()
                                                            .parse_inline(s)?)
                                                    })?;
                        }
                        break;
                    }
                    _ => {
                        continue;
                    }
                }
            }
            titles.push(title);

            chapters.push(format!("<div id = \"chapter-{}\" class = \"chapter\">
  {}
</div>",
                                  i,
                                  HtmlRenderer::render_html(self, v, render_notes_chapter)?));
        }
        self.html.source = Source::empty();

        for (i, chapter) in chapters.iter().enumerate() {
            if self.html.book.options.get_bool("html_single.one_chapter").unwrap() && i != 0 {
                content.push_str(&format!("<p onclick = \"javascript:showChapter({})\" class = \
                                           \"chapterControls prev_chapter chapter-{}\">
  <a \
                                           href = \"#chapter-{}\">
  « {}
  </a>
</p>",
                                          i - 1,
                                          i,
                                          i - 1,
                                          titles[i - 1]));
            }
            content.push_str(chapter);
            if self.html.book.options.get_bool("html_single.one_chapter").unwrap() &&
               i < titles.len() - 1 {
                content.push_str(&format!("<p onclick = \"javascript:showChapter({})\" class = \
                                           \"chapterControls next_chapter chapter-{}\">
  <a \
                                           href = \"#chapter-{}\">
  {} »
  </a>
</p>",
                                          i + 1,
                                          i,
                                          i + 1,
                                          titles[i + 1]));
            }
        }
        self.html.render_end_notes(&mut content);


        let toc = self.html.toc.render();
        // If display_toc, display the toc inline
        if self.html.book.options.get_bool("rendering.inline_toc").unwrap() {

            content = format!("<div id = \"toc\">
  <h1>{title}</h1>
  {toc}
</div>
{content}",
                              title = self.html.get_toc_name()?,
                              toc = &toc,
                              content = content);
        }

        // Render the CSS
        let template_css = compile_str(self.html.book.get_template("html.css")?
                                       .as_ref(),
                                       &self.html.book.source,
                                       lformat!("could not compile template 'html.css'"))?;
        let mut data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("colours",
                        self.html.book.get_template("html.css.colours")?);
        if let Ok(html_css_add) = self.html.book.options.get_str("html.css.add") {
            data = data.insert_str("additional_code", html_css_add);
        }
        if self.html.proofread && self.html.book.options.get_bool("proofread.nb_spaces").unwrap() {
            data = data.insert_bool("display_spaces", true);
        }
        let data = data.build();
        let mut res: Vec<u8> = vec![];
        template_css.render_data(&mut res, &data)?;
        let css = String::from_utf8_lossy(&res);

        // Render the JS
        let template_js =
            compile_str(self.html.book.get_template("html_single.js")?.as_ref(),
                        &self.html.book.source,
                        lformat!("could not compile template 'html_single.js'"))?;
        let data = self.html.book.get_metadata(|s| Ok(s.to_owned()))?
            .insert_str("book_svg", &book_svg)
            .insert_str("pages_svg", &pages_svg)
            .insert_bool("one_chapter",
                         self.html.book.options.get_bool("html_single.one_chapter").unwrap())
            .insert_str("common_script",
                        self.html.book.get_template("html.js").unwrap().as_ref())
            .build();
        let mut res: Vec<u8> = vec![];
        template_js.render_data(&mut res, &data)?;
        let js = String::from_utf8_lossy(&res);

        // Render the HTML document
        let mut mapbuilder = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("content", content)
            .insert_str("script", js)
            .insert_bool(self.html.book.options.get_str("lang").unwrap(), true)
            .insert_bool("one_chapter",
                         self.html.book.options.get_bool("html_single.one_chapter").unwrap())
            .insert_str("style", css.as_ref())
            .insert_str("print_style",
                        self.html.book.get_template("html.css.print").unwrap())
            .insert_str("menu_svg", menu_svg)
            .insert_str("book_svg", book_svg)
            .insert_str("pages_svg", pages_svg)
            .insert_str("footer", HtmlRenderer::get_footer(self)?)
            .insert_str("header", HtmlRenderer::get_header(self)?);
        if let Ok(favicon) = self.html.book.options.get_path("html.icon") {
                let favicon = self.html.handler.map_image(&self.html.book.source, favicon)?;
                mapbuilder = mapbuilder.insert_str("favicon", format!("<link rel = \"icon\" href = \"{}\">", favicon));
            }
        if !self.html.toc.is_empty() {
            mapbuilder = mapbuilder.insert_bool("has_toc", true);
            mapbuilder = mapbuilder.insert_str("toc", toc)
        }
        if self.html.book.options.get_bool("html.highlight_code") == Ok(true) {
            let highlight_js = self.html.book.get_template("html.highlight.js")?
                .as_bytes()
                .to_base64(base64::STANDARD);
            let highlight_js = format!("data:text/javascript;base64,{}", highlight_js);
            mapbuilder = mapbuilder.insert_bool("highlight_code", true)
                .insert_str("highlight_css",
                            self.html.book.get_template("html.highlight.css")?)
                .insert_str("highlight_js", highlight_js);
        }
        let data = mapbuilder.build();
        let template = compile_str(self.html.book.get_template("html_single.html")?
                                   .as_ref(),
                                   &self.html.book.source,
                                   lformat!("could not compile template 'html_single.html'"))?;
        let mut res = vec![];
        template.render_data(&mut res, &data)?;
        Ok(String::from_utf8_lossy(&res).into_owned())
    }
}

derive_html!{HtmlSingleRenderer<'a>, HtmlSingleRenderer::static_render_token}


pub struct HtmlSingle {}
pub struct ProofHtmlSingle {}

impl BookRenderer for HtmlSingle {
    fn render(&self, book: &Book, to: &mut Write) -> Result<()> {
        let mut html = HtmlSingleRenderer::new(book);
        let result = html.render_book()?;
        to.write_all(&result.as_bytes())
            .map_err(|e| {
                Error::render(&book.source,
                              lformat!("problem when writing HTML: {error}", error = e))
            })?;
        Ok(())
    }
}

impl BookRenderer for ProofHtmlSingle {
    fn render(&self, book: &Book, to: &mut Write) -> Result<()> {
        let mut html = HtmlSingleRenderer::new(book)
            .proofread();
        let result = html.render_book()?;
        to.write_all(&result.as_bytes())
            .map_err(|e| {
                Error::render(&book.source,
                              lformat!("problem when writing HTML: {error}", error = e))
            })?;
        Ok(())
    }
}

