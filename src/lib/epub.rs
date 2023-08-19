// Copyright (C) 2016-2023 Élisabeth HENRY.
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

use crate::book::Header;
use crate::book::Book;
use crate::book_renderer::BookRenderer;
use crate::error::{Error, Result, Source};
use crate::html::HtmlRenderer;
use crate::lang;
use crate::parser::Parser;
use crate::renderer::Renderer;
use crate::resource_handler;
use crate::templates::epub::*;
use crate::templates::epub3;
use crate::text_view::view_as_text;
use crate::token::Token;

use crowbook_text_processing::escape;
use epub_builder::{
    EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipCommand, ZipCommandOrLibrary,
    ZipLibrary,
};
use upon::Template;
use rust_i18n::t;

use std::borrow::Cow;
use std::convert::{AsMut, AsRef};
use std::fs;
use std::fs::File;
use std::io::Write;

use std::path::Path;

/// Renderer for Epub
///
/// Uses part of the HTML renderer
pub struct EpubRenderer<'a> {
    toc: Vec<String>,
    html: HtmlRenderer<'a>,
    chapter_title: String,
    chapter_title_raw: String,
}

impl<'a> EpubRenderer<'a> {
    /// Creates a new Epub renderer
    pub fn new(book: &'a Book) -> Result<EpubRenderer<'a>> {
        let mut html = HtmlRenderer::new(
            book,
            book.options
                .get_str("epub.highlight.theme")
                .unwrap_or_else(|_| book.options.get_str("rendering.highlight.theme").unwrap()),
        )?;
        html.handler.set_images_mapping(true);
        html.handler.set_base64(false);
        Ok(EpubRenderer {
            html,
            toc: vec![],
            chapter_title: String::new(),
            chapter_title_raw: String::new(),
        })
    }

    /// Render a book
    pub fn render_book(&mut self, to: &mut dyn Write) -> Result<String> {
        // Initialize the EPUB builder
        let mut zip = ZipCommand::new_in(self.html.book.options.get_path("crowbook.temp_dir")?)
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        zip.command(
            self.html
                .book
                .options
                .get_str("crowbook.zip.command")
                .unwrap(),
        );
        let wrapper = if zip.test().is_ok() {
            ZipCommandOrLibrary::Command(zip)
        } else {
            warn!(
                "{}",
                t!("epub.zip_command")
            );
            ZipCommandOrLibrary::Library(ZipLibrary::new()
                .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?)
        };
        let mut maker = EpubBuilder::new(wrapper)
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        maker.escape_html(false);
        if self.html.book.options.get_i32("epub.version").unwrap() == 3 {
            maker.epub_version(EpubVersion::V30);
        }

        let lang = self.html.book.options.get_str("lang").unwrap();
        let toc_extras = self.html.book.options.get_bool("epub.toc.extras").unwrap();
        maker.metadata("lang", lang)
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        maker.metadata(
            "author",
            self.html.book.options.get_str("author").unwrap(),
        )
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        maker.metadata(
            "title",
            escape::html(self.html.book.options.get_str("title").unwrap()),
        )
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        maker.metadata("generator", "crowbook")
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        maker.metadata("toc_name", lang::get_str(lang, "toc"))
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        if let Ok(subject) = self.html.book.options.get_str("subject") {
            maker.metadata("subject", subject)
                .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        }
        if let Ok(description) = self.html.book.options.get_str("description") {
            maker.metadata("description", description)
                .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        }
        if let Ok(license) = self.html.book.options.get_str("license") {
            maker.metadata("license", license)
                .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        }

        // if self.html.book.options.get_bool("epub.toc.extras").unwrap() == true {
        //     if self.html.book.options.get("cover").is_ok() {
        //         self.html.toc.add(1,
        //                           String::from("cover.xhtml"),
        //                           lang::get_str(lang, "cover"));
        //     }
        //     self.html.toc.add(1,
        //                       String::from("title_page.xhtml"),
        //                       lang::get_str(lang, "title"));

        // }

        // /* If toc will be rendered inline, add it... to the toc (yeah it's meta) */
        // if self.html.book.options.get_bool("rendering.inline_toc").unwrap() == true {
        //     self.html.toc.add(1,
        //                       String::from("toc.xhtml"),
        //                       lang::get_str(self.html.book.options.get_str("lang").unwrap(),
        //                                     "toc"));
        // }

        for (i, chapter) in self.html.book.chapters.iter().enumerate() {
            self.html
                .handler
                .add_link(chapter.filename.as_str(), filenamer(i));
        }

        // Write cover.xhtml (if needs be)
        if self.html.book.options.get_path("cover").is_ok() {
            let cover = self.render_cover()?;
            let mut content =
                EpubContent::new("cover.xhtml", cover.as_bytes()).reftype(ReferenceType::Cover);
            if toc_extras {
                content = content.title(lang::get_str(lang, "cover"));
            }
            maker.add_content(content)
                .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        }

        // Write titlepage
        {
            let title_page = self.render_titlepage()?;
            let mut content = EpubContent::new("title_page.xhtml", title_page.as_bytes())
                .reftype(ReferenceType::TitlePage);
            if toc_extras {
                content = content.title(lang::get_str(lang, "title"));
            }
            maker.add_content(content)
                .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        }

        if self
            .html
            .book
            .options
            .get_bool("rendering.inline_toc")
            .unwrap()
        {
            maker.inline_toc();
        }

        // Write chapters
        let template_chapter_src = self.html.book.get_template("epub.chapter.xhtml")?;
        let template_chapter = self.html.book.compile_str(
            template_chapter_src.as_ref(),
            &self.html.book.source,
            "epub.chapter.xhtml",
        )?;
        let mut rendered = vec![];
        for (i, chapter) in self.html.book.chapters.iter().enumerate() {
            let n = chapter.number;
            let v = &chapter.content;
            self.html.chapter_config(i, n, filenamer(i));
            let this_chapter = self.render_chapter(v, &template_chapter)?;
            rendered.push(this_chapter);
        }

        for (i, (rendered_chapter, raw_title)) in rendered.into_iter().enumerate() {
            let mut content = EpubContent::new(filenamer(i), rendered_chapter.as_bytes());
            if i == 0 {
                content = content.reftype(ReferenceType::Text);
            }

            // horrible hack to add subtoc of this chapter to epub's toc
            // todo: find cleaner way
            for element in &self.html.toc.elements {
                if element.url.contains(&filenamer(i)) {
                    content = content.title(escape::html(&raw_title));
                    content.toc.children = element.children.clone();
                    break;
                }
            }
            maker.add_content(content)
                .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
        }
        self.html.source = Source::empty();

        // Render the CSS file and write it
        let template_css_src = self.html.book.get_template("epub.css").unwrap();
        let template_css = self.html.book.compile_str(
            template_css_src.as_ref(),
            &self.html.book.source,
            "epub.css",
        )?;
        let mut data = self
            .html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?;
        data.insert(self.html.book.options.get_str("lang").unwrap().into(), true.into());
        let epub_css_add = self.html.book.options.get_str("epub.css.add").unwrap_or("".into()); 
        data.insert("additional_code".into(), epub_css_add.into());
        
        let css = template_css.render(&data).to_string()?;
        maker.stylesheet(css.as_bytes())
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;

        // Write all images (including cover)
        let cover = self.html.book.options.get_path("cover");
        for (source, dest) in self.html.handler.images_mapping() {
            let f = fs::canonicalize(source).and_then(File::open).map_err(|_| {
                Error::file_not_found(
                    &self.html.source,
                    t!("epub.image_or_cover"),
                    source.to_owned(),
                )
            })?;
            if cover.as_ref() == Ok(source) {
                // Treat cover specially so it is properly tagged
                maker.add_cover_image(dest, &f, self.get_format(dest))
                    .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
            } else {
                maker.add_resource(dest, &f, self.get_format(dest))
                    .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
            }
        }

        // Write additional resources
        if let Ok(list) = self.html.book.options.get_str_vec("resources.files") {
            let base_path_files = self
                .html
                .book
                .options
                .get_path("resources.base_path.files")
                .unwrap();
            let list = resource_handler::get_files(list, &base_path_files)?;
            let data_path = Path::new(
                self.html
                    .book
                    .options
                    .get_relative_path("resources.out_path")?,
            );
            for path in list {
                let abs_path = Path::new(&base_path_files).join(&path);
                let f = fs::canonicalize(&abs_path)
                    .and_then(File::open)
                    .map_err(|_| {
                        Error::file_not_found(
                            &self.html.book.source,
                            t!("epub.resources"),
                            abs_path.to_string_lossy().into_owned(),
                        )
                    })?;
                maker.add_resource(data_path.join(&path), &f, self.get_format(path.as_ref()))
                    .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;
            }
        }

        maker.generate(to)
            .map_err(|err| Error::render(Source::empty(), format!("{}", err)))?;

        Ok(String::new())
    }

    /// Render the titlepgae
    fn render_titlepage(&mut self) -> Result<String> {
        let template_src = self.html.book.get_template("epub.titlepage.xhtml")?;
        let template = self.html.book.compile_str(
            template_src.as_ref(),
            &self.html.book.source,
            "epub.titlepage.xhtml",
        )?;
        let data = self
            .html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?;
        Ok(template.render(&data).to_string()?)
    }

    /// Render cover.xhtml
    fn render_cover(&mut self) -> Result<String> {
        if let Ok(cover) = self.html.book.options.get_path("cover") {
            // Check that cover can be found
            if fs::metadata(&cover).is_err() {
                return Err(Error::file_not_found(
                    &self.html.book.source,
                    t!("epub.cover"),
                    cover,
                ));
            }
            let epub3 = self.html.book.options.get_i32("epub.version").unwrap() == 3;
            let template = self.html.book.compile_str(
                if epub3 { epub3::COVER } else { COVER },
                &self.html.book.source,
                "cover.xhtml",
            )?;
            let mut data = self
                .html
                .book
                .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?;
            data.insert(
                    "cover".into(),
                    self.html
                        .handler
                        .map_image(&self.html.source, Cow::Owned(cover))?
                        .into());
            Ok(template.render(&data).to_string()?)
        } else {
            unreachable!();
        }
    }

    /// Render a chapter
    ///
    /// Return chapter content and raw title
    pub fn render_chapter(&mut self, v: &[Token], template: &Template) -> Result<(String, String)> {
        let mut content = String::new();

        for token in v {
            content.push_str(&self.render_token(token)?);
            self.html.render_side_notes(&mut content);
        }

        let epub3 = self.html.book.options.get_i32("epub.version").unwrap() == 3;
        if epub3 {
            self.html.render_end_notes(&mut content, "section", "epub:type=\"footnotes\"");
        } else {
            self.html.render_end_notes(&mut content, "div", "");
        }

        if self.chapter_title.is_empty() && self.html.current_numbering >= 1 {
            let number;
            let header;
            if self.html.current_part {
                number = self.html.current_chapter[0] + 1;
                header = Header::Part;
            } else {
                number = self.html.current_chapter[1] + 1;
                header = Header::Chapter;
            }

            self.chapter_title = self
                .html
                .book
                .get_header(header, number, "".to_owned(), |s| {
                    self.render_vec(&Parser::new().parse_inline(s)?)
                })?
                .text;
            self.chapter_title_raw = self
                .html
                .book
                .get_header(header, number, "".to_owned(), |s| {
                    Ok(view_as_text(&Parser::new().parse_inline(s)?))
                })?
                .text;
        }
        self.toc.push(self.chapter_title.clone());

        let mut data = self
            .html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?;
        data.insert("content".into(), content.into());
        data.insert("chapter_title_raw".into(), self.chapter_title_raw.clone(). into());
        data.insert("chapter_title".into(), std::mem::take(&mut self.chapter_title).into());
        Ok((template.render(&data).to_string()?,
            std::mem::take(&mut self.chapter_title_raw)))
    }

    /// Renders the header section of the book, finding the title of the chapter
    fn find_title(&mut self, vec: &[Token]) -> Result<()> {
        if self.html.current_hide || self.html.current_numbering == 0 {
            if self.chapter_title.is_empty() {
                self.chapter_title = self.html.render_vec(vec)?;
                self.chapter_title_raw = view_as_text(vec);
            } else {
                warn!(
                    "{}",
                    t!(
                        "epub.ambiguous_invisible",
                        source = self.html.source
                    )
                );
            }
        } else {
            let header;
            let number;
            if self.html.current_part {
                header = Header::Part;
                number = self.html.current_chapter[0] + 1;
            } else {
                header = Header::Chapter;
                number = self.html.current_chapter[1] + 1;
            };
            let res = self
                .html
                .book
                .get_header(header, number, self.html.render_vec(vec)?, |s| {
                    self.render_vec(&(Parser::new().parse_inline(s)?))
                });
            let s = res?;
            if self.chapter_title.is_empty() {
                self.chapter_title = s.text;
                self.chapter_title_raw = self
                    .html
                    .book
                    .get_header(header, number, view_as_text(vec), |s| {
                        Ok(view_as_text(&(Parser::new().parse_inline(s)?)))
                    })?
                    .text;
            } else {
                warn!(
                    "{}",
                    t!(
                        "epub.ambiguous",
                        source = self.html.source
                    )
                );
                warn!(
                    "{}",
                    t!(
                        "epub.title_conflict",
                        source = self.html.source,
                        title1 = self.chapter_title,
                        title2 = s
                    )
                );
            }
        }
        Ok(())
    }

    // Get the format of a file, based on its extension
    fn get_format(&self, s: &str) -> String {
        let opt = mime_guess::from_path(s).first();
        match opt {
            Some(s) => s.to_string(),
            None => {
                error!(
                    "{}",
                    t!(
                        "epub.guess",
                        file = s
                    )
                );
                String::from("png")
            }
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
    where
        T: AsMut<EpubRenderer<'a>>
            + AsRef<EpubRenderer<'a>>
            + AsMut<HtmlRenderer<'a>>
            + AsRef<HtmlRenderer<'a>>
            + Renderer,
    {
        match *token {
            Token::Str(ref text) => {
                let html: &mut HtmlRenderer = this.as_mut();
                let content = if html.verbatim {
                    Cow::Borrowed(text.as_ref())
                } else {
                    escape::html(html.book.clean(text.as_str()))
                };
                let mut content = if html.first_letter {
                    html.first_letter = false;
                    if html.book.options.get_bool("rendering.initials").unwrap() {
                        // Use initial
                        let mut chars = content.chars();
                        let initial = chars.next().ok_or_else(|| {
                            Error::parser(
                                &html.book.source,
                                t!("error.initial"),
                            )
                        })?;
                        let mut new_content = if initial.is_alphanumeric() {
                            format!("<span class = \"initial\">{initial}</span>")
                        } else {
                            format!("{initial}")
                        };
                        for c in chars {
                            new_content.push(c);
                        }
                        Cow::Owned(new_content)
                    } else {
                        content
                    }
                } else {
                    content
                };

                if html.book.options.get_bool("epub.escape_nb_spaces").unwrap() {
                    content = escape::nb_spaces_html(content);
                }
                Ok(content.into_owned())
            }
            Token::Header(1, ref vec) => {
                {
                    let epub: &mut EpubRenderer = this.as_mut();
                    epub.find_title(vec)?;
                }
                HtmlRenderer::static_render_token(this, token)
            }
            Token::FootnoteReference(ref reference) => {
                let epub3 = (this.as_ref() as &HtmlRenderer)
                    .book
                    .options
                    .get_i32("epub.version")
                    .unwrap()
                    == 3;

                Ok(format!(
                    "<a {} href = \"#note-dest-{reference}\"><sup id = \
                            \"note-source-{reference}\">[{reference}]</sup></a>",
                    if epub3 { "epub:type = \"noteref\"" } else { "" },
                ))
            }
            Token::FootnoteDefinition(ref reference, ref vec) => {
                let epub3 = (this.as_ref() as &HtmlRenderer)
                    .book
                    .options
                    .get_i32("epub.version")
                    .unwrap()
                    == 3;
                let inner_content = this.render_vec(vec)?;
                let html: &mut HtmlRenderer = this.as_mut();
                let note_number = format!(
                    "<p class = \"note-number\">
  <a href = \"#note-source-{reference}\">[{reference}]</a>
</p>\n",
                );
                let inner = if epub3 {
                    format!(
                        "<aside epub:type = \"footnote\" id = \"note-dest-{reference}\">{inner_content}</aside>"
                    )
                } else {
                    format!("<a id = \"note-dest-{reference}\" />{inner_content}")
                };
                html.add_footnote(note_number, inner);

                Ok(String::new())
            }
            _ => HtmlRenderer::static_render_token(this, token),
        }
    }
}

/// Generate a file name given an int
fn filenamer(i: usize) -> String {
    format!("chapter_{i:03}.xhtml")
}

derive_html! {EpubRenderer<'a>, EpubRenderer::static_render_token}

pub struct Epub {}

impl BookRenderer for Epub {
    fn auto_path(&self, book_name: &str) -> Result<String> {
        Ok(format!("{book_name}.epub"))
    }

    fn render(&self, book: &Book, to: &mut dyn Write) -> Result<()> {
        EpubRenderer::new(book)?.render_book(to)?;
        Ok(())
    }
}
