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

use book::Book;
use number::Number;
use error::{Error,Result, Source};
use token::Token;
use zipper::Zipper;
use escape::escape_tex;
use resource_handler::ResourceHandler;

use std::iter::Iterator;
use std::fs::File;
use std::borrow::Cow;
use std::io::Read;

use mustache;

/// LaTeX renderer
pub struct LatexRenderer<'a> {
    book: &'a Book,
    current_chapter: Number,
    handler: ResourceHandler<'a>,
    source: Source,
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
        }
    }

    /// Render pdf in a file
    pub fn render_pdf(&mut self) -> Result<String> {
        if let Ok(pdf_file) = self.book.options.get_path("output.pdf") {
            let content = try!(self.render_book());
            let mut zipper = try!(Zipper::new(&self.book.options.get_path("temp_dir").unwrap()));
            try!(zipper.write("result.tex", &content.as_bytes(), false));

            // write image files
            for (source, dest) in self.handler.images_mapping() {
                let mut f = try!(File::open(source).map_err(|_| Error::FileNotFound(self.source.clone(),
                                                                                    "image".to_owned(),
                                                                                    source.to_owned())));
                let mut content = vec!();
                try!(f.read_to_end(&mut content).map_err(|e| Error::Render(format!("error while reading image file: {}", e))));
                try!(zipper.write(dest, &content, true));
            }
        
            
            zipper.generate_pdf(&self.book.options.get_str("tex.command").unwrap(), "result.tex", &pdf_file)
        } else {
            Err(Error::Render("no output pdf file specified in book config".to_owned()))
        }
    }

    /// Render latex in a string
    pub fn render_book(&mut self) -> Result<String> {
        let mut content = String::from("");
        for (i, filename) in self.book.filenames.iter().enumerate() {
            self.handler.add_link(filename.clone(), format!("chapter-{}", i));
        }

        // set tex numbering and toc display to book's parameters
        let numbering = self.book.options.get_i32("numbering").unwrap() - 1;
        content.push_str(&format!("\\setcounter{{tocdepth}}{{{}}}
\\setcounter{{secnumdepth}}{{{}}}\n",
                                  numbering, numbering));
        
        if self.book.options.get_bool("display_toc").unwrap() {
            content.push_str("\\tableofcontents\n");
        }

        let mut i = 0;
        for &(n, ref v) in &self.book.chapters {
            self.source = Source::new(&self.book.filenames[i]);
            content.push_str(&format!("\\label{{chapter-{}}}", i));
            self.current_chapter = n;
            content.push_str(&try!(self.render_vec(v, true)));
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
                self.book.logger.warning(format!("LaTeX: can't find a tex equivalent for lang '{}', fallbacking on english", self.book.options.get_str("lang").unwrap()));
                "english"
            }
        });

        let template = mustache::compile_str(try!(self.book.get_template("tex.template")).as_ref());
        let mut data = self.book.get_mapbuilder("tex")
            .insert_str("content", content)
            .insert_str("tex_lang", tex_lang);
        if self.book.options.get_bool("tex.short") == Ok(true) {
            data = data.insert_bool("short", true);
        }
        let data = data.build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => panic!("generated LaTeX was not valid utf-8"),
            Ok(res) => Ok(res)
        }
    }


    /// Transform a vector of `Token`s to LaTeX
    fn render_vec(&mut self, tokens: &[Token], escape: bool) -> Result<String> {
        let mut res = String::new();
        
        for token in tokens {
            res.push_str(&try!(self.parse_token(&token, escape)));
        }
        Ok(res)
    }
    
    fn parse_token(&mut self, token: &Token, escape: bool) -> Result<String> {
        match *token {
            Token::Str(ref text) => if escape {
                Ok(self.book.clean(escape_tex(text), true))
            } else {
                Ok(text.clone())
            },
            Token::Paragraph(ref vec) => Ok(format!("{}\n\n",
                                                 try!(self.render_vec(vec, escape)))),
            Token::Header(n, ref vec) => {
                let mut content = String::new();
                if n == 1 && self.current_chapter == Number::Hidden {
                    return Ok(String::new());
                }
                if n == 1 {
                    if let Number::Specified(n) = self.current_chapter {
                        content.push_str(r"\setcounter{chapter}{");
                        content.push_str(&format!("{}", n - 1));
                        content.push_str("}\n");
                    }
                }
                match n {
                    1 => content.push_str(r"\chapter"),
                    2 => content.push_str(r"\section"),
                    3 => content.push_str(r"\subsection"),
                    4 => content.push_str(r"\subsubsection"),
                    _ => content.push_str(r"\paragraph"),
                }
                if self.current_chapter == Number::Unnumbered {
                    content.push_str("*");
                }
                content.push_str(r"{");
                content.push_str(&try!(self.render_vec(vec, true)));
                content.push_str("}\n");
                Ok(content)
            },
            Token::Emphasis(ref vec) => Ok(format!("\\emph{{{}}}", try!(self.render_vec(vec, escape)))),
            Token::Strong(ref vec) => Ok(format!("\\textbf{{{}}}", try!(self.render_vec(vec, escape)))),
            Token::Code(ref vec) => Ok(format!("\\texttt{{{}}}", try!(self.render_vec(vec, escape)))),
            Token::BlockQuote(ref vec) => Ok(format!("\\begin{{quotation}}\n{}\\end{{quotation}}\n", try!(self.render_vec(vec, escape)))),
            Token::CodeBlock(_, ref vec) => Ok(format!("\\begin{{spverbatim}}{}\\end{{spverbatim}}\n\\vspace{{1em}}\n", try!(self.render_vec(vec, false)))),
            Token::Rule => Ok(String::from("\\HRule\n")),
            Token::SoftBreak => Ok(String::from(" ")),
            Token::HardBreak => Ok(String::from("\n")),
            Token::List(ref vec) => Ok(format!("\\begin{{itemize}}\n{}\\end{{itemize}}", try!(self.render_vec(vec, escape)))),
            Token::OrderedList(_, ref vec) => Ok(format!("\\begin{{enumerate}}\n{}\\end{{enumerate}}\n", try!(self.render_vec(vec, escape)))),
            Token::Item(ref vec) => Ok(format!("\\item {}\n", try!(self.render_vec(vec, escape)))),
            Token::Link(ref url, _, ref vec) => {
                let content = try!(self.render_vec(vec, escape));

                if ResourceHandler::is_local(url) {
                    Ok(format!("\\hyperref[{}]{{{}}}",
                            self.handler.get_link(url),
                            content))
                }
                else {
                    let url = escape_tex(url);
                    if content == url {
                        Ok(format!("\\url{{{}}}", content))
                    } else {
                        if self.book.options.get_bool("tex.links_as_footnotes").unwrap() {
                            Ok(format!("\\href{{{}}}{{{}}}\\footnote{{\\url{{{}}}}}", url, content, url))
                        } else {
                            Ok(format!("\\href{{{}}}{{{}}}", url, content))
                        }
                    }
                }
            },
            Token::Image(ref url, _, _) => {
                if ResourceHandler::is_local(url) {
                    let img = try!(self.handler.map_image(&self.source,
                                                          Cow::Borrowed(url)));
                    Ok(format!("\\begin{{center}}
  \\includegraphics[width=0.8\\linewidth]{{{}}}
\\end{{center}}",
                            img))

                } else {
                    self.book.logger.warning(&format!("LaTeX: image '{}' doesn't seem to be local; ignoring it in Latex output.", url));
                    Ok(String::new())
                }
            },
            Token::StandaloneImage(ref url, _, _) => {
                if ResourceHandler::is_local(url) {
                    Ok(format!("\\includegraphics{{{}}}",
                            try!(self.handler.map_image(&self.source,
                                                        Cow::Borrowed(url)))))
                } else {
                    self.book.logger.warning(&format!("LaTeX: image '{}' doesn't seem to be local; ignoring it in Latex output.", url));
                    Ok(String::new())
                }                                
            },
            Token::Footnote(ref vec) => Ok(format!("\\footnote{{{}}}", try!(self.render_vec(vec, escape)))),
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
\\end{{center}}\n\n", cols, try!(self.render_vec(vec, escape))))
            },
            Token::TableRow(ref vec) | Token::TableHead(ref vec) => {
                let mut res:String = try!(vec.iter()
                    .map(|v| {self.parse_token(v, escape)})
                    .collect::<Result<Vec<_>>>())
                    .join(" & ");
                res.push_str("\\\\ \n");
                if let Token::TableHead(_) = *token {
                    res.push_str("\\hline\n");
                }
                Ok(res)
            }
            Token::TableCell(ref vec) => self.render_vec(vec, escape),
        }
    }
}
