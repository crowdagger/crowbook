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

use book::{Book, Number};
use error::{Error,Result};
use token::Token;
use zipper::Zipper;
use escape::escape_tex;

use std::path::Path;
use std::iter::Iterator;

use mustache;

/// LaTeX renderer
pub struct LatexRenderer<'a> {
    book: &'a Book,
    current_chapter: Number,
}

impl<'a> LatexRenderer<'a> {
    /// Creates new LatexRenderer
    pub fn new(book: &'a Book) -> LatexRenderer<'a> {
        LatexRenderer {
            book: book,
            current_chapter: Number::Default,
        }
    }

    /// Render pdf in a file
    pub fn render_pdf(&mut self) -> Result<String> {
        if let Ok(pdf_file) = self.book.get_str("output.pdf") {
            let base_file = try!(Path::new(pdf_file).file_stem().ok_or(Error::Render("could not stem pdf filename")));
            let tex_file = format!("{}.tex", base_file.to_str().unwrap());
            let content = try!(self.render_book());
            let mut zipper = try!(Zipper::new(self.book.get_str("temp_dir").unwrap()));
            try!(zipper.write(&tex_file, &content.as_bytes(), false));
            zipper.generate_pdf(&self.book.get_str("tex.command").unwrap(), &tex_file, pdf_file)
        } else {
            Err(Error::Render("no output pdf file specified in book config"))
        }
    }

    /// Render latex in a string
    pub fn render_book(&mut self) -> Result<String> {
        let mut content = String::from("");

        // set tex numbering and toc display to book's parameters
        let numbering = self.book.get_i32("numbering").unwrap() - 1;
        content.push_str(&format!("\\setcounter{{tocdepth}}{{{}}}
\\setcounter{{secnumdepth}}{{{}}}\n",
                                  numbering, numbering));
        
        if self.book.get_bool("display_toc").unwrap() {
            content.push_str("\\tableofcontents\n");
        }
        
        for &(n, ref v) in &self.book.chapters {
            self.current_chapter = n;
            content.push_str(&self.render_vec(v, true));
        }
        

        let tex_lang = String::from(match self.book.get_str("lang").unwrap() {
            "en" => "english",
            "fr" => "francais",
            _ => {
                self.book.debug(&format!("Warning: can't find a tex equivalent for lang '{}', fallbacking on english", self.book.get_str("lang").unwrap()));
                "english"
            }
        });

        let template = mustache::compile_str(try!(self.book.get_template("tex.template")).as_ref());
        let data = self.book.get_mapbuilder("tex")
            .insert_str("content", content)
            .insert_str("tex_lang", tex_lang)
            .build();
        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated LaTeX was not valid utf-8")),
            Ok(res) => Ok(res)
        }
    }


    /// Transform a vector of `Token`s to LaTeX
    fn render_vec(&mut self, tokens: &[Token], escape: bool) -> String {
        let mut res = String::new();
        
        for token in tokens {
            res.push_str(&self.parse_token(&token, escape));
        }
        res
    }
    
    fn parse_token(&mut self, token: &Token, escape: bool) -> String {
        match *token {
            Token::Str(ref text) => if escape {escape_tex(text)} else {text.clone()},
            Token::Paragraph(ref vec) => format!("{}\n\n",
                                                 self.render_vec(vec, escape)),
            Token::Header(n, ref vec) => {
                let mut content = String::new();
                if n == 1 && self.current_chapter == Number::Hidden {
                    return String::new();
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
                content.push_str(&self.render_vec(vec, true));
                content.push_str("}\n");
                content
            },
            Token::Emphasis(ref vec) => format!("\\emph{{{}}}", self.render_vec(vec, escape)),
            Token::Strong(ref vec) => format!("\\textbf{{{}}}", self.render_vec(vec, escape)),
            Token::Code(ref vec) => format!("\\texttt{{{}}}", self.render_vec(vec, escape)),
            Token::BlockQuote(ref vec) => format!("\\begin{{quotation}}\n{}\\end{{quotation}}\n", self.render_vec(vec, escape)),
            Token::CodeBlock(_, ref vec) => format!("\\begin{{spverbatim}}{}\\end{{spverbatim}}\n\\vspace{{1em}}\n", self.render_vec(vec, false)),
            Token::Rule => String::from("\\HRule\n"),
            Token::SoftBreak => String::from(" "),
            Token::HardBreak => String::from("\n"),
            Token::List(ref vec) => format!("\\begin{{itemize}}\n{}\\end{{itemize}}", self.render_vec(vec, escape)),
            Token::OrderedList(_, ref vec) => format!("\\begin{{enumerate}}\n{}\\end{{enumerate}}\n", self.render_vec(vec, escape)),
            Token::Item(ref vec) => format!("\\item {}\n", self.render_vec(vec, escape)),
            Token::Link(ref url, _, ref vec) => {
                let content = self.render_vec(vec, escape);
                let url = escape_tex(url);
                if content == url {
                    format!("\\url{{{}}}", content)
                } else {
                    if self.book.get_bool("tex.links_as_footnotes").unwrap() {
                        format!("\\href{{{}}}{{{}}}\\footnote{{\\url{{{}}}}}", url, content, url)
                    } else {
                        format!("\\href{{{}}}{{{}}}", url, content)
                    }
                }
            },
            Token::Image(_, _, _) => {
                self.book.debug("warning: including images is not yet supported for tex output");
                String::from(" ")
            },
            Token::Footnote(ref vec) => format!("\\footnote{{{}}}", self.render_vec(vec, escape)),
            Token::Table(n, ref vec) => {
                let mut cols = String::new();
                for _ in 0..n {
                    cols.push_str("|c");
                }
                cols.push_str("|");
                format!("\\begin{{center}}
\\begin{{tabular}}{{{}}}
\\hline
{}
\\hline
\\end{{tabular}}
\\end{{center}}\n\n", cols, self.render_vec(vec, escape))
            },
            Token::TableRow(ref vec) | Token::TableHead(ref vec) => {
                let mut res:String = vec.iter().map(|v| {self.parse_token(v, escape)})
                    .collect::<Vec<_>>()
                    .join(" & ");
                res.push_str("\\\\ \n");
                if let Token::TableHead(_) = *token {
                    res.push_str("\\hline\n");
                }
                res
            }
            Token::TableCell(ref vec) => self.render_vec(vec, escape),
        }
    }
}
