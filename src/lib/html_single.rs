// Copyright (C) 2016-2023 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Crowbook is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use crate::book::Book;
use crate::book_renderer::BookRenderer;
use crate::error::{Error, Result, Source};
use crate::html::Highlight;
use crate::html::HtmlRenderer;
use crate::parser::Parser;
use crate::renderer::Renderer;
use crate::templates::img;
use crate::token::Token;
use crate::misc;

use std::convert::{AsMut, AsRef};
use std::fmt::Write;
use std::io;
use rust_i18n::t;

/// Single file HTML renderer
///
/// Renders a standalone, self-contained HTML file
pub struct HtmlSingleRenderer<'a> {
    html: HtmlRenderer<'a>,
}

impl<'a> HtmlSingleRenderer<'a> {
    /// Creates a new HtmlSingleRenderer
    pub fn new(book: &'a Book) -> Result<HtmlSingleRenderer<'a>> {
        let mut html = HtmlRenderer::new(
            book,
            book.options
                .get_str("html.highlight.theme")
                .unwrap_or_else(|_| book.options.get_str("rendering.highlight.theme").unwrap()),
        )?;
        html.handler.set_images_mapping(true);
        html.handler.set_base64(true);
        Ok(HtmlSingleRenderer { html })
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
    where
        T: AsMut<HtmlSingleRenderer<'a>>
            + AsRef<HtmlSingleRenderer<'a>>
            + AsMut<HtmlRenderer<'a>>
            + AsRef<HtmlRenderer<'a>>
            + Renderer,
    {
        HtmlRenderer::static_render_token(this, token)
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        let menu_svg = misc::u8_to_base64(&img::MENU_SVG);
        let menu_svg = format!("data:image/svg+xml;base64,{menu_svg}");

        let book_svg = misc::u8_to_base64(&img::BOOK_SVG);
        let book_svg = format!("data:image/svg+xml;base64,{book_svg}");

        let pages_svg = misc::u8_to_base64(&img::PAGES_SVG);
        let pages_svg = format!("data:image/svg+xml;base64,{pages_svg}");

        let mut content = String::new();

        let mut titles = vec![];
        let mut chapters = vec![];
        let render_notes_chapter = self
            .html
            .book
            .options
            .get_bool("html.standalone.one_chapter")
            .unwrap();

        for (i, chapter) in self.html.book.chapters.iter().enumerate() {
            self.html
                .handler
                .add_link(chapter.filename.as_str(), format!("#chapter-{i}"));
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
                            title = self
                                .html
                                .book
                                .get_chapter_header(
                                    self.html.current_chapter[1] + 1,
                                    self.html.render_vec(vec)?,
                                    |s| self.render_vec(&Parser::new().parse_inline(s)?),
                                )?
                                .text;
                        }
                        break;
                    }
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
                HtmlRenderer::render_html(self, v, render_notes_chapter)?
            ));
        }
        self.html.source = Source::empty();

        for (i, chapter) in chapters.iter().enumerate() {
            if self
                .html
                .book
                .options
                .get_bool("html.standalone.one_chapter")
                .unwrap()
                && i != 0
            {
                write!(
                    content,
                    "<p onclick = \"javascript:showChapter({})\" class = \
                        \"chapterControls prev_chapter chapter-{}\">
  <a href = \"#chapter-{}\">
  « {}
  </a>
</p>",
                    i - 1,
                    i,
                    i - 1,
                    titles[i - 1]
                )?;
            }
            content.push_str(chapter);
            if self
                .html
                .book
                .options
                .get_bool("html.standalone.one_chapter")
                .unwrap()
                && i < titles.len() - 1
            {
                write!(
                    content,
                    "<p onclick = \"javascript:showChapter({})\" class = \
                           \"chapterControls next_chapter chapter-{}\">
  <a href = \"#chapter-{}\">
  {} »
  </a>
</p>",
                    i + 1,
                    i,
                    i + 1,
                    titles[i + 1]
                )?;
            }
        }
        self.html.render_end_notes(&mut content, "section", "");

        let toc = self.html.toc.render(false, false);
        // If display_toc, display the toc inline
        if self
            .html
            .book
            .options
            .get_bool("rendering.inline_toc")
            .unwrap()
        {
            content = format!(
                "<div id = \"toc\">
  <h1>{title}</h1>
  {toc}
</div>
{content}",
                title = self.html.get_toc_name()?,
                toc = &toc,
                content = content
            );
        }

        // Render the CSS
        let template_css_src = self.html.book.get_template("html.css")?;
        let template_css = self.html.book.compile_str(
            template_css_src.as_ref(),
            &self.html.book.source,
            "html.css",
        )?;
        let mut data = self
            .html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?;
        data.insert("colors".into(), self.html.book.get_template("html.css.colors")?.into());
        if let Ok(html_css_add) = self.html.book.options.get_str("html.css.add") {
            data.insert("additional_code".into(), html_css_add.into());
        } else {
            data.insert("additional_code".into(), "".into());
        }
        let css = template_css.render(&data).to_string()?;


        // Render the JS
        let template_js_src = self.html.book.get_template("html.standalone.js")?;
        let template_js = self.html.book.compile_str(
            template_js_src.as_ref(),
            &self.html.book.source,
            "html.standalone.js",
        )?;
        let mut data = self
            .html
            .book
            .get_metadata(|s| Ok(s.to_owned()))?;
        data.insert("book_svg".into(), book_svg.clone().into());
        data.insert("pages_svg".into(), pages_svg.clone().into());
        data.insert(
                "one_chapter".into(),
                self.html
                    .book
                    .options
                    .get_bool("html.standalone.one_chapter")
                    .unwrap()
                    .into(),
        );
        data.insert(
            "common_script".into(),
            self.html.book.get_template("html.js").unwrap().into(),
        );
        let js = template_js.render(&data).to_string()?;

        // Render the HTML document
        let mut data = self
            .html
            .get_metadata()?;
        data.insert("content".into(), content.into());
        data.insert(
                "one_chapter".into(),
                self.html
                    .book
                    .options
                    .get_bool("html.standalone.one_chapter")
                    .unwrap()
                    .into(),
        );
        data.insert("style".into(), css.into());
        data.insert("script".into(), js.into()); // Need to override this for html_single
        data.insert(
                "print_style".into(),
                self.html.book.get_template("html.css.print").unwrap().into(),
        );
        data.insert("menu_svg".into(), menu_svg.clone().into());
        data.insert("book_svg".into(), book_svg.clone().into());
        data.insert("pages_svg".into(), pages_svg.clone().into());
        if let Ok(favicon) = self.html.book.options.get_path("html.icon") {
            let favicon = self
                .html
                .handler
                .map_image(&self.html.book.source, favicon)?;
            data.insert(
                "favicon".into(),
                format!("<link rel = \"icon\" href = \"{favicon}\">").into(),
            );
        } else {
            data.insert("favicon".into(), "".into()); 
        }
        if !self.html.toc.is_empty() {
            data.insert("has_toc".into(), true.into());
            data.insert("toc".into(), toc.into());
        } else {
            data.insert("has_toc".into(), false.into());
        }
        if self.html.highlight == Highlight::Js {
            let highlight_js = misc::u8_to_base64(&self
                .html
                .book
                .get_template("html.highlight.js")?
                .as_bytes());
            let highlight_js = format!("data:text/javascript;base64,{highlight_js}");
            data.insert("highlight_code".into(), true.into());
            data.insert(
                    "highlight_css".into(),
                    self.html.book.get_template("html.highlight.css")?.into(),
            );
            data.insert("highlight_js".into(), highlight_js.into());
        } 
        let template_src = self.html.book.get_template("html.standalone.template")?;
        let template = self.html.book.compile_str(
            template_src.as_ref(),
            &self.html.book.source,
            "html.standalone.template",
        )?;
        Ok(template.render(&data).to_string()?)
    }
}

derive_html! {HtmlSingleRenderer<'a>, HtmlSingleRenderer::static_render_token}

pub struct HtmlSingle {}
pub struct ProofHtmlSingle {}

impl BookRenderer for HtmlSingle {
    fn auto_path(&self, book_name: &str) -> Result<String> {
        Ok(format!("{book_name}.html"))
    }

    fn render(&self, book: &Book, to: &mut dyn io::Write) -> Result<()> {
        let mut html = HtmlSingleRenderer::new(book)?;
        let result = html.render_book()?;
        to.write_all(result.as_bytes()).map_err(|e| {
            Error::render(
                &book.source,
                t!("html.write_error", error = e),
            )
        })?;
        Ok(())
    }
}

impl BookRenderer for ProofHtmlSingle {
    fn auto_path(&self, book_name: &str) -> Result<String> {
        Ok(format!("{book_name}.proof.html"))
    }

    fn render(&self, book: &Book, to: &mut dyn io::Write) -> Result<()> {
        let mut html = HtmlSingleRenderer::new(book)?.proofread();
        let result = html.render_book()?;
        to.write_all(result.as_bytes()).map_err(|e| {
            Error::render(
                &book.source,
                t!("html.write_error", error = e),
            )
        })?;
        Ok(())
    }
}
