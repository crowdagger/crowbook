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

use book::{Book, compile_str};
use number::Number;
use error::{Error, Result, Source};
use token::Token;
use token::Data;
use zipper::Zipper;
use resource_handler::ResourceHandler;
use renderer::Renderer;
use parser::Parser;

use crowbook_text_processing::escape;

use std::iter::Iterator;
use std::fs::File;
use std::io::Read;
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
}

impl<'a> LatexRenderer<'a> {
    /// Creates new LatexRenderer
    pub fn new(book: &'a Book) -> LatexRenderer<'a> {
        let mut handler = ResourceHandler::new(&book.logger);
        handler.set_images_mapping(true);
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
        }
    }

    /// Set proofreading to true
    #[doc(hidden)]
    pub fn proofread(mut self) -> Self {
        self.proofread = true;
        self
    }

    /// Render pdf in a file
    pub fn render_pdf(&mut self) -> Result<String> {
        let output = if self.proofread {
            "output.proofread.pdf"
        } else {
            "output.pdf"
        };
        if let Ok(pdf_file) = self.book.options.get_path(output) {
            let content = try!(self.render_book());
            let mut zipper =
                try!(Zipper::new(&self.book.options.get_path("crowbook.temp_dir").unwrap()));
            try!(zipper.write("result.tex", &content.as_bytes(), false));

            // write image files
            for (source, dest) in self.handler.images_mapping() {
                let mut f = try!(File::open(source).map_err(|_| {
                    Error::file_not_found(&self.source, lformat!("image"), source.to_owned())
                }));
                let mut content = vec![];
                try!(f.read_to_end(&mut content).map_err(|e| {
                    Error::render(&self.source,
                                  lformat!("error while reading image file: {error}", error = e))
                }));
                try!(zipper.write(dest, &content, true));
            }


            zipper.generate_pdf(&self.book.options.get_str("tex.command").unwrap(),
                                "result.tex",
                                &pdf_file)
        } else {
            Err(Error::render(&self.source,
                              lformat!("no output pdf file specified '{output}' in book config",
                                       output = output)))
        }
    }

    /// Render latex in a string
    pub fn render_book(&mut self) -> Result<String> {
        let mut content = String::from("");
        for (i, filename) in self.book.filenames.iter().enumerate() {
            self.handler.add_link(filename.clone(), format!("chapter-{}", i));
        }

        // set tex numbering and toc display to book's parameters
        let numbering = self.book.options.get_i32("rendering.num_depth").unwrap() - 1;
        content.push_str(&format!("\\setcounter{{tocdepth}}{{{}}}
\\setcounter{{secnumdepth}}{{{}}}\n",
                                  numbering,
                                  numbering));

        if self.book.options.get_bool("rendering.inline_toc").unwrap() {
            content.push_str("\\tableofcontents\n");
        }

        let mut i = 0;
        for &(n, ref v) in &self.book.chapters {
            self.source = Source::new(&self.book.filenames[i] as &str);
            content.push_str(&format!("\\label{{chapter-{}}}", i));
            self.current_chapter = n;
            content.push_str(&try!(self.render_vec(v)));
            i += 1;
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
                    .error(lformat!("LaTeX: can't find a tex equivalent for lang '{lang}', \
                                     fallbacking on english",
                                    lang = self.book.options.get_str("lang").unwrap()));
                "english"
            }
        });

        let template = try!(compile_str(try!(self.book.get_template("tex.template")).as_ref(),
                                        &self.book.source,
                                        lformat!("could not compile template 'tex.template'")));
        let mut data = try!(self.book.get_metadata(|s| {
                let tokens = try!(Parser::new().parse_inline(s));
                self.render_vec(&tokens)
            }))
            .insert_str("content", content)
            .insert_str("class", self.book.options.get_str("tex.class").unwrap())
            .insert_str("tex_lang", tex_lang);
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
        template.render_data(&mut res, &data);
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
                        let initial = try!(chars.next()
                            .ok_or(Error::parser(&self.book.source,
                                                 lformat!("empty str token, could not find \
                                                           initial"))));
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
                Ok(format!("{}\n\n", try!(self.render_vec(vec))))
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
                    } else {
                        if let Number::Specified(n) = self.current_chapter {
                            content.push_str(r"\setcounter{chapter}{");
                            content.push_str(&format!("{}", n - 1));
                            content.push_str("}\n");
                        }
                    }
                }
                match n {
                    1 => {
                        if !self.is_short {
                            content.push_str(r"\chapter");
                        } else {
                            // Chapters aren't handlled for class article
                            content.push_str(r"\section");
                        }
                    }
                    2 => content.push_str(r"\section"),
                    3 => content.push_str(r"\subsection"),
                    4 => content.push_str(r"\subsubsection"),
                    _ => content.push_str(r"\paragraph"),
                }
                if self.current_chapter == Number::Unnumbered {
                    content.push_str("*");
                }
                content.push_str(r"{");
                content.push_str(&try!(self.render_vec(vec)));
                content.push_str("}\n");
                Ok(content)
            }
            Token::Emphasis(ref vec) => Ok(format!("\\emph{{{}}}", try!(self.render_vec(vec)))),
            Token::Strong(ref vec) => Ok(format!("\\textbf{{{}}}", try!(self.render_vec(vec)))),
            Token::Code(ref vec) => Ok(format!("\\texttt{{{}}}", try!(self.render_vec(vec)))),
            Token::BlockQuote(ref vec) => {
                Ok(format!("\\begin{{quotation}}\n{}\\end{{quotation}}\n",
                           try!(self.render_vec(vec))))
            }
            Token::CodeBlock(_, ref vec) => {
                self.escape = false;
                let res = try!(self.render_vec(vec));
                self.escape = true;
                Ok(format!("\\begin{{spverbatim}}{}\\end{{spverbatim}}\n\\vspace{{1em}}\n",
                           res))
            }
            Token::Rule => Ok(String::from("\\HRule\n")),
            Token::SoftBreak => Ok(String::from(" ")),
            Token::HardBreak => Ok(String::from("\\\n")),
            Token::List(ref vec) => {
                Ok(format!("\\begin{{itemize}}\n{}\\end{{itemize}}",
                           try!(self.render_vec(vec))))
            }
            Token::OrderedList(_, ref vec) => {
                Ok(format!("\\begin{{enumerate}}\n{}\\end{{enumerate}}\n",
                           try!(self.render_vec(vec))))
            }
            Token::Item(ref vec) => Ok(format!("\\item {}\n", try!(self.render_vec(vec)))),
            Token::Link(ref url, _, ref vec) => {
                let content = try!(self.render_vec(vec));

                if ResourceHandler::is_local(url) {
                    Ok(format!("\\hyperref[{}]{{{}}}", self.handler.get_link(url), content))
                } else {
                    let url = escape::tex(url.as_ref());
                    if &content == &url {
                        Ok(format!("\\url{{{}}}", content))
                    } else {
                        if self.book.options.get_bool("tex.links_as_footnotes").unwrap() {
                            Ok(format!("\\href{{{}}}{{{}}}\\protect\\footnote{{\\url{{{}}}}}",
                                       url,
                                       content,
                                       url))
                        } else {
                            Ok(format!("\\href{{{}}}{{{}}}", url, content))
                        }
                    }
                }
            }
            Token::StandaloneImage(ref url, _, _) => {
                if ResourceHandler::is_local(url) {
                    let img = try!(self.handler.map_image(&self.source, url.as_ref()));
                    Ok(format!("\\begin{{center}}
  \\includegraphics[width=0.8\\linewidth]{{{}}}
\\end{{center}}",
                               img))

                } else {
                    self.book
                        .logger
                        .error(lformat!("LaTeX ({source}): image '{url}' doesn't seem to be \
                                         local; ignoring it.",
                                        source = self.source,
                                        url = url));
                    Ok(String::new())
                }
            }
            Token::Image(ref url, _, _) => {
                if ResourceHandler::is_local(url) {
                    Ok(format!("\\includegraphics{{{}}}",
                               try!(self.handler.map_image(&self.source, url.as_ref()))))
                } else {
                    self.book
                        .logger
                        .error(lformat!("LaTeX ({source}): image '{url}' doesn't seem to be \
                                         local; ignoring it.",
                                        source = self.source,
                                        url = url));
                    Ok(String::new())
                }
            }
            Token::Footnote(ref vec) => {
                Ok(format!("\\protect\\footnote{{{}}}", try!(self.render_vec(vec))))
            }
            Token::Table(n, ref vec) => {
                let mut cols = String::new();
                for _ in 0..n {
                    cols.push_str("|c");
                }
                cols.push_str("|");
                Ok(format!("\\begin{{center}}
\\begin{{tabular}}{{{}}}
\\hline
{}
\\hline
\\end{{tabular}}
\\end{{center}}\n\n",
                           cols,
                           try!(self.render_vec(vec))))
            }
            Token::TableRow(ref vec) |
            Token::TableHead(ref vec) => {
                let mut res: String = try!(vec.iter()
                        .map(|v| self.render_token(v))
                        .collect::<Result<Vec<_>>>())
                    .join(" & ");
                res.push_str("\\\\ \n");
                if let Token::TableHead(_) = *token {
                    res.push_str("\\hline\n");
                }
                Ok(res)
            }
            Token::TableCell(ref vec) => self.render_vec(vec),
            Token::Annotation(ref annotation, ref vec) => {
                let content = try!(self.render_vec(vec));
                if self.proofread {
                    match annotation {
                        &Data::GrammarError(ref s) => {
                            Ok(format!("\\underline{{{}}}\\protect\\footnote{{{}}}",
                                       content,
                                       escape::tex(s.as_str())))
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
