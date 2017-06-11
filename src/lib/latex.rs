// Copyright (C) 2016, 2017 Élisabeth HENRY.
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

use book::{Book, compile_str};
use number::Number;
use error::{Error, Result, Source};
use token::Token;
use token::Data;
use zipper::Zipper;
use resource_handler::ResourceHandler;
use renderer::Renderer;
use parser::Parser;
use book_renderer::BookRenderer;
use syntax::Syntax;

use crowbook_text_processing::escape;

use std::iter::Iterator;
use std::fs::File;
use std::io;
use std::io::Read;
use std::fmt::Write;
use std::borrow::Cow;


/// LaTeX renderer
pub struct LatexRenderer<'a> {
    book: &'a Book,
    current_chapter: Number,
    handler: ResourceHandler<'a>,
    source: Source,
    escape: bool,
    first_letter: bool,
    first_paragraph: bool,
    is_short: bool,
    proofread: bool,
    syntax: Option<Syntax>,
    hyperref: bool,
}

impl<'a> LatexRenderer<'a> {
    /// Creates new LatexRenderer
    pub fn new(book: &'a Book) -> LatexRenderer<'a> {
        let mut handler = ResourceHandler::new(&book.logger);
        handler.set_images_mapping(true);
        let syntax = if book.options.get_str("rendering.highlight").unwrap() == "syntect"
            && book.features.codeblock {
            Some(Syntax::new(book,
                             book.options
                             .get_str("tex.highlight.theme")
                             .unwrap_or_else(|_| book.options.get_str("rendering.highlight.theme").unwrap())))
        } else {
            None
        };
        LatexRenderer {
            book: book,
            current_chapter: Number::Default,
            handler: handler,
            source: Source::empty(),
            escape: true,
            first_letter: false,
            first_paragraph: true,
            is_short: book.options.get_str("tex.class").unwrap() == "article",
            proofread: false,
            syntax: syntax,
            hyperref: book.options.get_bool("tex.hyperref").unwrap(),
        }
    }

    /// Set proofreading to true
    #[doc(hidden)]
    pub fn proofread(mut self) -> Self {
        self.proofread = true;
        self
    }

    /// Render pdf to a file
    pub fn render_pdf(&mut self, to: &mut io::Write) -> Result<String> {
        let content = self.render_book()?;
        let mut zipper = Zipper::new(&self.book.options.get_path("crowbook.temp_dir")
                                     .unwrap(),
                                     &self.book.logger)?;
        zipper.write("result.tex", content.as_bytes(), false)?;

        // write image files
        for (source, dest) in self.handler.images_mapping() {
            let mut f = File::open(source)
                .map_err(|_| {
                    Error::file_not_found(&self.source, lformat!("image"), source.to_owned())
                    })?;
            let mut content = vec![];
            f.read_to_end(&mut content)
                .map_err(|e| {
                    Error::render(&self.source,
                                  lformat!("error while reading image file: {error}", error = e))
                })?;
            zipper.write(dest, &content, true)?;
        }


        zipper.generate_pdf(self.book.options.get_str("tex.command").unwrap(),
                            "result.tex",
                            to)
    }

    /// Render latex in a string
    pub fn render_book(&mut self) -> Result<String> {
        let mut content = String::new();

        // set tex numbering and toc display to book's parameters
        let numbering = self.book.options.get_i32("rendering.num_depth").unwrap() - 1;
        write!(content,
               "\\setcounter{{tocdepth}}{{{}}}
\\setcounter{{secnumdepth}}{{{}}}\n",
               numbering,
               numbering)?;

        if self.book.options.get_bool("rendering.inline_toc").unwrap() {
            content.push_str("\\tableofcontents\n");
        }

        for (i, chapter) in self.book.chapters.iter().enumerate() {
            self.handler.add_link(chapter.filename.as_ref(), format!("chapter-{}", i));
        }
        
        for (i, chapter) in self.book.chapters.iter().enumerate() {
            let n = chapter.number;
            self.current_chapter = n;
            let v = &chapter.content;
            self.source = Source::new(chapter.filename.as_str());
            let mut offset = 0;
            if !v.is_empty() && v[0].is_header() {
                content.push_str(&self.render_token(&v[0])?);
                offset = 1;
            }
            write!(content,
                   "\\label{{chapter-{}}}\n",
                   i)?;
            content.push_str(&self.render_vec(&v[offset..])?);
        }
        self.source = Source::empty();


        let tex_lang = String::from(match self.book.options.get_str("lang").unwrap() {
            "af" => "afrikaans",
            "sq" => "albanian",
            "eu" => "basque",
            "bg" => "bulgarian",
            "ca" => "catalan",
            "hr" => "croatian",
            "cs" => "czech",
            "da" => "danish",
            "nl" => "dutch",
            "en" => "english",
            "eo" => "esperanto",
            "et" => "estonian",
            "fi" => "finnish",
            "fr" => "francais",
            "gl" => "galician",
            "el" => "greek",
            "de" => "ngerman",
            "he" => "hebrew",
            "hu" => "hungarian",
            "it" => "italian",
            "is" => "icelandic",
            "id" => "indonesian",
            "ga" => "irish",
            "la" => "latin",
            "ms" => "malay",
            "nn" => "norsk",
            "pl" => "polish",
            "pt" => "portuguese",
            "ro" => "romanian",
            "ru" => "russian",
            "gd" => "scottish",
            "sr" => "serbian",
            "sk" => "slovak",
            "sl" => "slovene",
            "es" => "spanish",
            "sw" => "swedish",
            "tr" => "turkish",
            "uk" => "ukrainian",
            "cy" => "welsh",
            _ => {
                self.book
                    .logger
                    .warning(lformat!("LaTeX: can't find a tex equivalent for lang '{lang}', \
                                     fallbacking on english",
                                    lang = self.book.options.get_str("lang").unwrap()));
                "english"
            }
        });

        let template = compile_str(self.book.get_template("tex.template")?.as_ref(),
                                   &self.book.source,
                                   "tex.template")?;
        let mut data = self.book.get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("content", content)
            .insert_str("class", self.book.options.get_str("tex.class").unwrap())
            .insert_bool("tex_title", self.book.options.get_bool("tex.title").unwrap())
            .insert_str("papersize", self.book.options.get_str("tex.paper_size").unwrap())
            .insert_bool("stdpage", self.book.options.get_bool("tex.stdpage").unwrap())
            .insert_bool("use_url", self.book.features.url)
            .insert_bool("use_tables", self.book.features.table)
            .insert_bool("use_codeblocks", self.book.features.codeblock)
            .insert_bool("use_images", self.book.features.image)
            .insert_str("tex_lang", tex_lang);
        if let Ok(tex_tmpl_add) = self.book.options.get_str("tex.template.add") {
            data = data.insert_str("additional_code", tex_tmpl_add);
        }
        if let Ok(tex_font_size) = self.book.options.get_i32("tex.font.size") {
            data = data
                .insert_bool("has_tex_size", true)
                .insert_str("tex_size", format!("{}", tex_font_size));
        }

        // If class isn't book, set open_any to true, so margins are symetric.
        if self.book.options.get_str("tex.class").unwrap() == "book" {
            data = data.insert_bool("book", true);
        }
        if self.book.options.get_bool("rendering.initials") == Ok(true) {
            data = data.insert_bool("initials", true);
        }
        // Insert xelatex if tex.command is set to xelatex
        if self.book.options.get_str("tex.command") == Ok("xelatex") {
            data = data.insert_bool("xelatex", true);
        }
        let data = data.build();
        let mut res: Vec<u8> = vec![];
        template.render_data(&mut res, &data)?;
        match String::from_utf8(res) {
            Err(_) => panic!(lformat!("generated LaTeX was not valid utf-8")),
            Ok(res) => Ok(res),
        }
    }
}

impl<'a> Renderer for LatexRenderer<'a> {
    fn render_token(&mut self, token: &Token) -> Result<String> {
        match *token {
            Token::Str(ref text) => {
                let content = if self.escape {
                    self.book.clean(escape::tex(text.as_ref()), true)
                } else {
                    Cow::Borrowed(text.as_ref())
                };
                if self.first_letter {
                    self.first_letter = false;
                    if self.book.options.get_bool("rendering.initials").unwrap() {
                        let mut chars = content.chars().peekable();
                        let initial = chars.next()
                            .ok_or_else(|| Error::parser(&self.book.source,
                                                    lformat!("empty str token, could not find \
                                                              initial")))?;
                        let mut first_word = String::new();
                        loop {
                            let c = if let Some(next_char) = chars.peek() {
                                *next_char
                            } else {
                                break;
                            };
                            if !c.is_whitespace() {
                                first_word.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }

                        let rest = chars.collect::<String>();

                        if initial.is_alphanumeric() {
                            Ok(format!("\\lettrine{{{}}}{{{}}}{}", initial, first_word, rest))
                        } else {
                            Ok(format!("{}{}{}", initial, first_word, rest))
                        }
                    } else {
                        Ok(content.into_owned())
                    }
                } else {
                    Ok(content.into_owned())
                }
            }
            Token::Paragraph(ref vec) => {
                if self.first_paragraph {
                    self.first_paragraph = false;
                    if !vec.is_empty() && vec[0].is_str() {
                        // Only use initials if first element is a Token::str
                        self.first_letter = true;
                    }
                }
                Ok(format!("{}\n\n", self.render_vec(vec)?))
            }
            Token::Header(n, ref vec) => {
                let mut content = String::new();
                if n == 1 {
                    self.first_paragraph = true;
                    if self.current_chapter == Number::Hidden {
                        if !self.is_short {
                            return Ok(r#"\chapter*{}"#.to_owned());
                        } else {
                            return Ok(r#"\section*{}"#.to_owned());
                        }
                    } else if let Number::Specified(n) = self.current_chapter {
                        content.push_str(r"\setcounter{chapter}{");
                        write!(content, "{}", n - 1)?;
                        content.push_str("}\n");
                    }
                }
                match n {
                    1 => {
                        if !self.is_short {
                            if self.current_chapter.is_part() {
                                if self.book.options.get_bool("rendering.part.reset_counter").unwrap() {
                                    content.push_str(r"\setcounter{chapter}{0}");
                                }
                                content.push_str(r"\part");
                            } else {
                                content.push_str(r"\chapter");
                            }
                        } else {
                            // Chapters or parts aren't handlled for class article
                            content.push_str(r"\section");
                        }
                    }
                    2 => content.push_str(r"\section"),
                    3 => content.push_str(r"\subsection"),
                    4 => content.push_str(r"\subsubsection"),
                    _ => content.push_str(r"\paragraph"),
                }
                if !self.current_chapter.is_numbered() {
                    content.push_str("*");
                }
                content.push_str(r"{");
                content.push_str(&self.render_vec(vec)?);
                content.push_str("}\n");
                Ok(content)
            }
            Token::Emphasis(ref vec) => Ok(format!("\\emph{{{}}}", self.render_vec(vec)?)),
            Token::Strong(ref vec) => Ok(format!("\\textbf{{{}}}", self.render_vec(vec)?)),
            Token::Code(ref vec) => Ok(format!("\\texttt{{{}}}",
                                               insert_breaks(&self.render_vec(vec)?))),
            Token::Superscript(ref vec) => Ok(format!("\\textsuperscript{{{}}}", self.render_vec(vec)?)),
            Token::Subscript(ref vec) => Ok(format!("\\textsubscript{{{}}}", self.render_vec(vec)?)),
            Token::BlockQuote(ref vec) => {
                Ok(format!("\\begin{{quotation}}{{\\itshape\n{}}}\\end{{quotation}}\n",
                           self.render_vec(vec)?))
            }
            Token::CodeBlock(ref language, ref vec) => {
                self.escape = false;
                let mut res = self.render_vec(vec)?;
                // Remove trailing newline
                if res.ends_with('\n') {
                    res.pop();
                }
                self.escape = true;
                res = if let Some(ref syntax) = self.syntax {
                    syntax.to_tex(&res, language)?
                } else {
                    format!("\\begin{{spverbatim}}
{code}
\\end{{spverbatim}}",
                            code = res)
                };
                res = format!("\\begin{{mdframed}}
{}
\\end{{mdframed}}", res);
                Ok(res)
            }
            Token::Rule => Ok(String::from("\\HRule\n")),
            Token::SoftBreak => Ok(String::from(" ")),
            Token::HardBreak => Ok(String::from("\\\n")),
            Token::List(ref vec) => {
                Ok(format!("\\begin{{itemize}}\n{}\\end{{itemize}}",
                           self.render_vec(vec)?))
            }
            Token::OrderedList(_, ref vec) => {
                Ok(format!("\\begin{{enumerate}}\n{}\\end{{enumerate}}\n",
                           self.render_vec(vec)?))
            }
            Token::Item(ref vec) => Ok(format!("\\item {}\n", self.render_vec(vec)?)),
            Token::Link(ref url, _, ref vec) => {
                let content = self.render_vec(vec)?;

                if self.hyperref && self.handler.contains_link(url) {
                    Ok(format!("\\hyperref[{}]{{{}}}", escape::tex(self.handler.get_link(url)), content))
                } else {
                    let url = escape::tex(url.as_ref());
                    if &content == &url {
                        Ok(format!("\\url{{{}}}", content))
                    } else if self.book.options.get_bool("tex.links_as_footnotes").unwrap() {
                        Ok(format!("\\href{{{}}}{{{}}}\\protect\\footnote{{\\url{{{}}}}}",
                                   url,
                                   content,
                                   url))
                    } else {
                        Ok(format!("\\href{{{}}}{{{}}}", url, content))
                    }
                }
            }
            Token::StandaloneImage(ref url, _, _) => {
                if ResourceHandler::is_local(url) {
                    let img = self.handler.map_image(&self.source, url.as_ref())?;
                    Ok(format!("\\begin{{center}}
  \\includegraphics[width=0.8\\linewidth]{{{}}}
\\end{{center}}",
                               img))

                } else {
                    self.book
                        .logger
                        .debug(lformat!("LaTeX ({source}): image '{url}' doesn't seem to be \
                                         local; ignoring it.",
                                        source = self.source,
                                        url = url));
                    Ok(String::new())
                }
            }
            Token::Image(ref url, _, _) => {
                if ResourceHandler::is_local(url) {
                    Ok(format!("\\includegraphics{{{}}}",
                               self.handler.map_image(&self.source, url.as_ref())?))
                } else {
                    self.book
                        .logger
                        .debug(lformat!("LaTeX ({source}): image '{url}' doesn't seem to be \
                                         local; ignoring it.",
                                        source = self.source,
                                        url = url));
                    Ok(String::new())
                }
            }
            Token::Footnote(ref vec) => {
                Ok(format!("\\protect\\footnote{{{}}}", self.render_vec(vec)?))
            }
            Token::Table(n, ref vec) => {
                let mut cols = String::new();
                for _ in 0..n {
                    cols.push_str("|X");
                }
                cols.push_str("|");
                Ok(format!("\\begin{{center}}
\\begin{{tabularx}}{{\\textwidth}}{{{}}}
\\hline
{}
\\hline
\\end{{tabularx}}
\\end{{center}}\n\n",
                           cols,
                           self.render_vec(vec)?))
            }
            Token::TableRow(ref vec) |
            Token::TableHead(ref vec) => {
                let mut res: String = vec.iter()
                    .map(|v| self.render_token(v))
                    .collect::<Result<Vec<_>>>()?
                    .join(" & ");
                res.push_str("\\\\ \n");
                if let Token::TableHead(_) = *token {
                    res.push_str("\\hline\n");
                }
                Ok(res)
            }
            Token::TableCell(ref vec) => self.render_vec(vec),
            Token::Annotation(ref annotation, ref vec) => {
                let content = self.render_vec(vec)?;
                if self.proofread {
                    match *annotation {
                        Data::GrammarError(ref s) => {
                            Ok(format!("\\underline{{{}}}\\protect\\footnote{{{}}}",
                                       content,
                                       escape::tex(s.as_str())))
                        },
                        Data::Repetition(ref colour) => {
                            if !self.escape && colour == "red" {
                                Ok(format!("\\underline{{{}}}",
                                           content))
                            } else {
                                Ok(content)
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    Ok(content)
                }
            }

            Token::__NonExhaustive => unreachable!(),
        }
    }
}

pub struct Latex;
pub struct ProofLatex;
pub struct Pdf;
pub struct ProofPdf;


impl BookRenderer for Latex {
    fn render(&self, book: &Book, to: &mut io::Write) -> Result<()> {
        let mut latex = LatexRenderer::new(book);
        let result = latex.render_book()?;
        to.write_all(result.as_bytes())
            .map_err(|e| {
                Error::render(&book.source,
                              lformat!("problem when writing LaTeX: {error}", error = e))
            })?;
        Ok(())
    }
}

impl BookRenderer for ProofLatex {
    fn render(&self, book: &Book, to: &mut io::Write) -> Result<()> {
        let mut latex = LatexRenderer::new(book).proofread();
        let result = latex.render_book()?;
        to.write_all(result.as_bytes())
            .map_err(|e| {
                Error::render(&book.source,
                              lformat!("problem when writing LaTeX: {error}", error = e))
            })?;
        Ok(())
    }
}

impl BookRenderer for Pdf {
    fn render(&self, book: &Book, to: &mut io::Write) -> Result<()> {
        LatexRenderer::new(book)
            .render_pdf(to)?;
        Ok(())
    }
}

impl BookRenderer for ProofPdf {
    fn render(&self, book: &Book, to: &mut io::Write) -> Result<()> {
        LatexRenderer::new(book)
            .proofread()
            .render_pdf(to)?;
        Ok(())
    }
}

/// Insert possible breaks after characters '-', '/', '_', '.', ... to avoid code exploding
/// the page
pub fn insert_breaks(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '.' | '_' | ')' | '(' | '-' | '/' | ':'  => {
                result.push(c);
                result.push_str("\\allowbreak{}");
            },
            _ => result.push(c),
        }
    }
    result
}
