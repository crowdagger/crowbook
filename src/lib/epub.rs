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
use token::Token;
use html::HtmlRenderer;
use book::{Book, compile_str};
use templates::epub::*;
use templates::epub3;
use resource_handler;
use renderer::Renderer;
use parser::Parser;
use lang;
use book_renderer::BookRenderer;

use mustache::Template;
use crowbook_text_processing::escape;
use epub_builder::EpubBuilder;
use epub_builder::EpubVersion;
use epub_builder::EpubContent;
use epub_builder::ZipCommand;
use epub_builder::ReferenceType;

use std::io::Write;
use std::convert::{AsRef, AsMut};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::borrow::Cow;
use std::mem;
use mime_guess::guess_mime_type_opt;

/// Renderer for Epub
///
/// Uses part of the HTML renderer
pub struct EpubRenderer<'a> {
    toc: Vec<String>,
    html: HtmlRenderer<'a>,
    chapter_title: String,
}

impl<'a> EpubRenderer<'a> {
    /// Creates a new Epub renderer
    pub fn new(book: &'a Book) -> EpubRenderer<'a> {
        let mut html = HtmlRenderer::new(book,
                                         book.options
                                         .get_str("epub.highlight.theme")
                                         .unwrap_or_else(|_| book.options.get_str("rendering.highlight.theme").unwrap()));
        html.handler.set_images_mapping(true);
        html.handler.set_base64(false);
        EpubRenderer {
            html: html,
            toc: vec![],
            chapter_title: String::new(),
        }
    }

    /// Render a book
    pub fn render_book(&mut self, to: &mut Write) -> Result<String> {
        // Initialize the EPUB builder
        let mut zip = ZipCommand::new_in(self.html.book.options.get_path("crowbook.temp_dir")?)?;
        zip.command(self.html.book.options.get_str("crowbook.zip.command")
                    .unwrap());
        let mut maker = EpubBuilder::new(zip)?;
        if self.html.book.options.get_i32("epub.version").unwrap() == 3 {
            maker.epub_version(EpubVersion::V30);
        }
        
        let lang = self.html.book.options.get_str("lang").unwrap();
        let toc_extras = self.html.book.options.get_bool("epub.toc.extras").unwrap();
        maker.metadata("lang", lang)?;
        maker.metadata("author", escape::html(self.html.book.options.get_str("author").unwrap()))?;
        maker.metadata("title", escape::html(self.html.book.options.get_str("title").unwrap()))?;
        maker.metadata("generator", "crowbook")?;
        maker.metadata("toc_name", lang::get_str(lang,
                                                 "toc"))?;
        if let Ok(subject) = self.html.book.options.get_str("subject") {
            maker.metadata("subject", subject)?;
        }
        if let Ok(description) = self.html.book.options.get_str("description") {
            maker.metadata("description", description)?;
        }
        if let Ok(license) = self.html.book.options.get_str("license") {
            maker.metadata("license", license)?;
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
            self.html.handler.add_link(chapter.filename.as_str(), filenamer(i));
        }

        // Write cover.xhtml (if needs be)
        if self.html.book.options.get_path("cover").is_ok() {
            let cover = self.render_cover()?;
            let mut content = EpubContent::new("cover.xhtml", cover.as_bytes())
                .reftype(ReferenceType::Cover);
            if toc_extras {
                content = content.title(lang::get_str(lang, "cover"));
            }
            maker.add_content(content)?;
        }

        // Write titlepage
        {
            let title_page = self.render_titlepage()?;
            let mut content = EpubContent::new("title_page.xhtml", title_page.as_bytes())
                .reftype(ReferenceType::TitlePage);
            if toc_extras {
                content = content.title(lang::get_str(lang, "title"));
            }
            maker.add_content(content)?;
        }

        if self.html.book.options.get_bool("rendering.inline_toc").unwrap() {
            maker.inline_toc();
        }

        
        // Write chapters
        let template_chapter =
            compile_str(self.html.book.get_template("epub.chapter.xhtml")?.as_ref(),
                        &self.html.book.source,
                        "epub.chapter.xhtml")?;
        for (i, chapter) in self.html.book.chapters.iter().enumerate() {
            let n = chapter.number;
            let v = &chapter.content;
            self.html.chapter_config(i, n, filenamer(i));
            let chapter = self.render_chapter(v, &template_chapter)?;

            let mut content = EpubContent::new(filenamer(i), chapter.as_bytes());
            if i == 0 {
                content = content.reftype(ReferenceType::Text);
            }
            // horrible hack
            // todo: find cleaner way
            for element in &self.html.toc.elements {
                if element.url.contains(&filenamer(i)) {
                    content = content.title(element.title.as_ref());
                    content.toc.children = element.children.clone();
                    break;
                }
            }
            maker.add_content(content)?;
        }
        self.html.source = Source::empty();

        // Render the CSS file and write it
        let template_css =
            compile_str(self.html.book.get_template("epub.css").unwrap().as_ref(),
                        &self.html.book.source,
                        "epub.css")?;
        let mut data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_bool(self.html.book.options.get_str("lang").unwrap(), true);
        if let Ok(epub_css_add) = self.html.book.options.get_str("epub.css.add") {
            data = data.insert_str("additional_code", epub_css_add);
        }
        let data = data.build();
        let mut res: Vec<u8> = vec![];
        template_css.render_data(&mut res, &data)?;
        let css = String::from_utf8_lossy(&res);
        maker.stylesheet(css.as_bytes())?;

        // Write all images (including cover)
        let cover = self.html.book.options.get_path("cover");
        for (source, dest) in self.html.handler.images_mapping() {
            let f = File::open(source).map_err(|_| {
                Error::file_not_found(&self.html.source,
                                      lformat!("image or cover"),
                                      source.to_owned())
            })?;
            if cover.as_ref() == Ok(source) {
                // Treat cover specially so it is properly tagged
                maker.add_cover_image(dest, &f, self.get_format(dest))?;
            } else {
                maker.add_resource(dest, &f, self.get_format(dest))?;
            }
        }

        // Write additional resources
        if let Ok(list) = self.html.book.options.get_paths_list("resources.files") {
            let base_path_files =
                self.html.book.options.get_path("resources.base_path.files").unwrap();
            let list = resource_handler::get_files(list, &base_path_files)?;
            let data_path = Path::new(self.html.book.options.get_relative_path("resources.out_path")?);
            for path in list {
                let abs_path = Path::new(&base_path_files).join(&path);
                let f = File::open(&abs_path).map_err(|_| {
                    Error::file_not_found(&self.html.book.source,
                                          lformat!("additional resource from resources.files"),
                                          abs_path.to_string_lossy().into_owned())
                })?;
                maker.add_resource(data_path.join(&path), &f, self.get_format(path.as_ref()))?;
            }
        }

        maker.generate(to)?;
    
        Ok(String::new())
    }

    /// Render the titlepgae
    fn render_titlepage(&mut self) -> Result<String> {
        let epub3 = self.html.book.options.get_i32("epub.version").unwrap() == 3;
        let template = compile_str(if epub3 { epub3::TITLE } else { TITLE },
                                   &self.html.book.source,
                                   "title page")?;
        let data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
        .build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!("generated HTML in titlepage was not utf-8 valid"),
            Ok(res) => Ok(res),
        }
    }

    /// Render cover.xhtml
    fn render_cover(&mut self) -> Result<String> {
        if let Ok(cover) = self.html.book.options.get_path("cover") {
            // Check that cover can be found
            if fs::metadata(&cover).is_err() {
                return Err(Error::file_not_found(&self.html.book.source, lformat!("cover"), cover));

            }
            let epub3 = self.html.book.options.get_i32("epub.version").unwrap() == 3;
            let template = compile_str(if epub3 { epub3::COVER } else { COVER },
                                       &self.html.book.source,
                                       "cover.xhtml")?;
            let data = self.html
                .book
                .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
                .insert_str("cover",
                            self.html
                            .handler
                            .map_image(&self.html.source, Cow::Owned(cover))?
                            .into_owned())
                .build();
            let mut res: Vec<u8> = vec![];
            template.render_data(&mut res, &data)?;
            match String::from_utf8(res) {
                Err(_) => panic!(lformat!("generated HTML for cover.xhtml was not utf-8 valid")),
                Ok(res) => Ok(res),
            }
        } else {
            panic!(lformat!("Why is this method called if cover is None???"));
        }
    }


    /// Render a chapter
    pub fn render_chapter(&mut self, v: &[Token], template: &Template) -> Result<String> {
        let mut content = String::new();

        for token in v {
            content.push_str(&self.render_token(token)?);
            self.html.render_side_notes(&mut content);
        }
        self.html.render_end_notes(&mut content);

        if self.chapter_title.is_empty() && self.html.current_numbering >= 1 {
            let number = self.html.current_chapter[1] + 1;
            self.chapter_title = self.html.book.get_chapter_header(number, "".to_owned(), |s| {
                    self.render_vec(&Parser::new().parse_inline(s)?)
                })?;
        }
        self.toc.push(self.chapter_title.clone());

        let data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("content", content)
            .insert_str("chapter_title",
                        mem::replace(&mut self.chapter_title, String::new()))
            .build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!(lformat!("generated HTML was not utf-8 valid")),
            Ok(res) => Ok(res),
        }
    }

    /// Renders the header section of the book, finding the title of the chapter
    fn find_title(&mut self, vec: &[Token]) -> Result<()> {
        if self.html.current_hide || self.html.current_numbering == 0 {
            if self.chapter_title.is_empty() {
                self.chapter_title = self.html.render_vec(vec)?;
            } else {
                self.html
                    .book
                    .logger
                    .warning(lformat!("EPUB ({source}): detected two chapter titles inside the \
                                       same markdown file, in a file where chapter titles are \
                                       not even rendered.",
                                      source = self.html.source));
            }
        } else {
            let res = self.html.book.get_chapter_header(self.html.current_chapter[1] + 1,
                                                        self.html.render_vec(vec)?,
                                                        |s| {
                                                            self.render_vec(&(Parser::new()
                                                                .parse_inline(s)?))
                                                        });
            let s = res.unwrap();
            if self.chapter_title.is_empty() {
                self.chapter_title = s;
            } else {
                self.html
                    .book
                    .logger
                    .warning(lformat!("EPUB ({source}): detected two chapters inside the same \
                                       markdown file.",
                                      source = self.html.source));
                self.html
                    .book
                    .logger
                    .warning(lformat!("EPUB ({source}): conflict between: {title1} and {title2}",
                                      source = self.html.source,
                                      title1 = self.chapter_title,
                                      title2 = s));
            }
        }
        Ok(())
    }

    // Get the format of a file, based on its extension
    fn get_format(&self, s: &str) -> String {
        let opt = guess_mime_type_opt(s);
        match opt {
            Some(s) => s.to_string(),
            None => {
                self.html
                    .book
                    .logger
                    .error(lformat!("EPUB: could not guess the format of {file} based on \
                                     extension. Assuming png.",
                                    file = s));
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
    where T: AsMut<EpubRenderer<'a>>+AsRef<EpubRenderer<'a>> +
        AsMut<HtmlRenderer<'a>>+AsRef<HtmlRenderer<'a>> + Renderer
    {
        match *token {
            Token::Str(ref text) => {
                let html: &mut HtmlRenderer = this.as_mut();
                let content = if html.verbatim {
                    Cow::Borrowed(text.as_ref())
                } else {
                    escape::html(html.book.clean(text.as_ref(), false))
                };
                let mut content = if html.first_letter {
                    html.first_letter = false;
                    if html.book.options.get_bool("rendering.initials").unwrap() {
                        // Use initial
                        let mut chars = content.chars();
                        let initial = chars.next()
                            .ok_or_else(|| Error::parser(&html.book.source,
                                                         lformat!("empty str token, could not find \
                                                                   initial")))?;
                        let mut new_content = if initial.is_alphanumeric() {
                            format!("<span class = \"initial\">{}</span>", initial)
                        } else {
                            format!("{}", initial)
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
                    content = escape::nnbsp(content);
                }
                Ok(content.into_owned())
            },
            Token::Header(1, ref vec) => {
                {
                    let epub: &mut EpubRenderer = this.as_mut();
                    epub.find_title(vec)?;
                }
                HtmlRenderer::static_render_token(this, token)
            }
            Token::Footnote(ref vec) => {
                let epub3 = (this.as_ref() as &HtmlRenderer)
                    .book
                    .options
                    .get_i32("epub.version")
                    .unwrap() == 3;
                let inner_content = this.render_vec(vec)?;
                let html: &mut HtmlRenderer = this.as_mut();
                html.footnote_number += 1;
                let number = html.footnote_number;
                let note_number = format!("<p class = \"note-number\">
  <a href = \"#note-source-{}\">[{}]</a>
</p>\n",
                                          number,
                                          number);
                let inner = if epub3 {
                    format!("<aside epub:type = \"footnote\" id = \"note-dest-{}\">{}</aside>",
                            number,
                            inner_content)
                } else {
                    format!("<a id = \"note-dest-{}\" />{}", number, inner_content)
                };
                html.add_footnote(note_number, inner);

                Ok(format!("<a {} href = \"#note-dest-{}\"><sup id = \
                            \"note-source-{}\">[{}]</sup></a>",
                           if epub3 { "epub:type = \"noteref\"" } else { "" },
                           number,
                           number,
                           number))
            }
            _ => HtmlRenderer::static_render_token(this, token),
        }
    }
}


/// Generate a file name given an int
fn filenamer(i: usize) -> String {
    format!("chapter_{:03}.xhtml", i)
}


derive_html!{EpubRenderer<'a>, EpubRenderer::static_render_token}

pub struct Epub {}

impl BookRenderer for Epub {
    fn render(&self, book: &Book, to: &mut Write) -> Result<()> {
        EpubRenderer::new(book)
            .render_book(to)?;
        Ok(())
    }
}
