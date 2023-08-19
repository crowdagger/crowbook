
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

use crate::book::Book;
use crate::error::{Error, Result, Source};
use crate::token::Token;

use std::convert::AsRef;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::BitOr;
use std::path::Path;

use comrak::nodes::{AstNode, ListType, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use rust_i18n::t;

#[derive(Debug, Copy, Clone, PartialEq)]
/// The list of features used in a document.
///
/// This is used by the renderers to only require some packages if they
/// are needed.
pub struct Features {
    pub image: bool,
    pub footnote: bool,
    pub blockquote: bool,
    pub codeblock: bool,
    pub ordered_list: bool,
    pub table: bool,
    pub url: bool,
    pub subscript: bool,
    pub superscript: bool,
    pub strikethrough: bool,
    pub taskitem: bool,
}

impl Features {
    /// Creates a new set of features where all are set to false
    pub fn new() -> Features {
        Features {
            image: false,
            blockquote: false,
            codeblock: false,
            ordered_list: false,
            footnote: false,
            table: false,
            url: false,
            subscript: false,
            superscript: false,
            strikethrough: false,
            taskitem: false,
        }
    }
}

impl Default for Features {
    fn default() -> Self {
        Self::new()
    }
}

impl BitOr for Features {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Features {
            image: self.image | rhs.image,
            blockquote: self.blockquote | rhs.blockquote,
            codeblock: self.codeblock | rhs.codeblock,
            ordered_list: self.ordered_list | rhs.ordered_list,
            footnote: self.footnote | rhs.footnote,
            table: self.table | rhs.table,
            url: self.url | rhs.url,
            subscript: self.subscript | rhs.subscript,
            superscript: self.superscript | rhs.superscript,
            strikethrough: self.strikethrough | rhs.strikethrough,
            taskitem: self.taskitem | rhs.taskitem,
        }
    }
}

/// A parser that reads markdown and convert it to AST (a vector of `Token`s)
///
/// This AST can then be used by various renderes.
///
/// As this Parser uses Pulldown-cmark's one, it should be able to parse most
/// *valid* CommonMark variant of Markdown.
///
/// Compared to other Markdown parser, it might fail more often on invalid code, e.g.
/// footnotes references that are not defined anywhere.
///
/// # Examples
///
/// ```
/// use crowbook::Parser;
/// let mut parser = Parser::new();
/// let result = parser.parse("Some *valid* Markdown[^1]\n\n[^1]: with a valid footnote");
/// assert!(result.is_ok());
/// ```
///
/// ```
/// use crowbook::Parser;
/// let mut parser = Parser::new();
/// let result = parser.parse("Some footnote pointing to nothing[^1] ");
/// assert!(result.is_err());
/// ```
pub struct Parser {
    source: Source,
    features: Features,
    ignore_paragraphs: bool,

    html_as_text: bool,
    superscript: bool,
    parse_frontmatter: bool,
}

impl Parser {
    /// Creates a parser
    pub fn new() -> Parser {
        Parser {
            source: Source::empty(),
            features: Features::new(),
            ignore_paragraphs: false,
            html_as_text: true,
            superscript: false,
            parse_frontmatter: false,
        }
    }

    /// Creates a parser with options from a book configuration file
    pub fn from(book: &Book) -> Parser {
        let mut parser = Parser::new();
        parser.html_as_text = book.options.get_bool("crowbook.html_as_text").unwrap();
        parser.parse_frontmatter = book.options.get_bool("input.yaml_blocks").unwrap();
        parser.superscript = book
            .options
            .get_bool("crowbook.markdown.superscript")
            .unwrap();
        parser
    }

    /// Enable/disable HTML as text
    pub fn html_as_text(&mut self, b: bool) {
        self.html_as_text = b;
    }

    /// Sets a parser's source file
    pub fn set_source_file(&mut self, s: &str) {
        self.source = Source::new(s);
    }

    /// Parse a file and returns an AST or  an error
    pub fn parse_file<P: AsRef<Path>>(&mut self, filename: P, yaml_block: Option<&mut String>) -> Result<Vec<Token>> {
        let path: &Path = filename.as_ref();
        let mut f = File::open(path).map_err(|_| {
            Error::file_not_found(
                &self.source,
                t!("format.markdown"),
                format!("{}", path.display()),
            )
        })?;
        let mut s = String::new();

        f.read_to_string(&mut s).map_err(|_| {
            Error::parser(
                &self.source,
                t!("error.utf8",
                   file = path.display()
                ),
            )
        })?;
        self.parse(&s, yaml_block)
    }

    /// Parse a string and returns an AST or an Error.
    /// If yaml is set to some string, fill it with frontmatter if it is found
    pub fn parse(&mut self, s: &str, mut yaml: Option<&mut String>) -> Result<Vec<Token>> {
        let arena = Arena::new();

        // Set options for comrak
        let mut options = ComrakOptions::default();
        options.render.hardbreaks = false;
        options.parse.smart = false;
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.tasklist = true;
        options.extension.superscript = self.superscript;
        options.extension.footnotes = true;
        options.extension.description_lists = true;
        if self.parse_frontmatter {
            options.extension.front_matter_delimiter = Some("---".to_owned());
        }

        let root = parse_document(&arena, s, &options);

        let mut res = self.parse_node(root, &mut yaml)?;

        collapse(&mut res);

        find_standalone(&mut res);

        Ok(res)
    }

    /// Parse an inline string and returns a list of `Token`.
    ///
    /// This function removes the outermost `Paragraph` in most of the
    /// cases, as it is meant to be used for an inline string (e.g. metadata)
    pub fn parse_inline(&mut self, s: &str) -> Result<Vec<Token>> {
        let mut tokens = self.parse(s, None)?;
        // Unfortunately, parser will put all this in a paragraph, so we might need to remove it.
        if tokens.len() == 1 {
            let res = match tokens[0] {
                Token::Paragraph(ref mut v) => Some(std::mem::take(v)),
                _ => None,
            };
            match res {
                Some(tokens) => Ok(tokens),
                _ => Ok(tokens),
            }
        } else {
            Ok(tokens)
        }
    }

    /// Returns the list of features used by this parser
    pub fn features(&self) -> Features {
        self.features
    }

    fn parse_node<'a>(&mut self, node: &'a AstNode<'a>, yaml_block: &mut Option<&mut String>) -> Result<Vec<Token>> {
        let mut inner = vec![];

        // Some special cases where we need to modifiy a bit the state of the parser between parsing inner content
        if let NodeValue::DescriptionTerm = node.data.borrow().value {
            self.ignore_paragraphs = true;
        }
        for c in node.children() {
            let mut v = self.parse_node(c, yaml_block)?;
            inner.append(&mut v);
        }
        // Reset state after special cases shenanigans
        if let NodeValue::DescriptionTerm = node.data.borrow().value {
            // There should be no paragraphs inside description terms
            self.ignore_paragraphs = false;
        }

        inner = match node.data.borrow().value {
            NodeValue::Document => inner,
            NodeValue::BlockQuote => {
                self.features.blockquote = true;
                vec![Token::BlockQuote(inner)]
            }
            NodeValue::FrontMatter(ref v) => {
                if let Some(yaml) = yaml_block {
                    // We can add the frontmatter to the yaml block
                    yaml.push_str(v);
                }
                vec![]
            },
            NodeValue::List(ref list) => {
                // Todo: use "tight" feature?
                match list.list_type {
                    ListType::Bullet => vec![Token::List(inner)],
                    ListType::Ordered => vec![Token::OrderedList(list.start, inner)],
                }
            }
            NodeValue::Item(_) => vec![Token::Item(inner)],
            NodeValue::DescriptionList => vec![Token::DescriptionList(inner)],
            NodeValue::DescriptionItem(_) => vec![Token::DescriptionItem(inner)],
            NodeValue::DescriptionTerm => vec![Token::DescriptionTerm(inner)],
            NodeValue::DescriptionDetails => vec![Token::DescriptionDetails(inner)],
            NodeValue::CodeBlock(ref block) => {
                let info = block.info.clone();
                let code = block.literal.clone();
                self.features.codeblock = true;
                vec![Token::CodeBlock(info, code)]
            }
            NodeValue::HtmlBlock(ref block) => {
                let text = block.literal.clone();
                if self.html_as_text {
                    vec![Token::Str(text)]
                } else {
                    debug!("{}", t!("parser.ignore_html", block = text));
                    vec![]
                }
            }
            NodeValue::HtmlInline(ref html) => {
                let text = html.clone();
                if self.html_as_text {
                    vec![Token::Str(text)]
                } else {
                    debug!("{}", t!("parser.ignore_html", block = text));
                    vec![]
                }
            }
            NodeValue::Paragraph => {
                if !self.ignore_paragraphs {
                    vec![Token::Paragraph(inner)]
                } else {
                    inner
                }
            }
            NodeValue::Heading(ref heading) => vec![Token::Header(heading.level as i32, inner)],
            NodeValue::ThematicBreak => vec![Token::Rule],
            NodeValue::FootnoteDefinition(ref def) => {
                let reference = def.clone();
                vec![Token::FootnoteDefinition(reference, inner)]
            }
            NodeValue::Text(ref text) => {
                let text = text.clone();
                vec![Token::Str(text)]
            }
            NodeValue::Code(ref code) => {
                let text = code.literal.clone();
                vec![Token::Code(text)]
            }
            NodeValue::SoftBreak => vec![Token::SoftBreak],
            NodeValue::LineBreak => vec![Token::HardBreak],
            NodeValue::Emph => vec![Token::Emphasis(inner)],
            NodeValue::TaskItem(c) => {
                self.features.taskitem = true;
                let checked = if c.is_some() { true } else { false };
                vec![Token::TaskItem(checked, inner)]
            }
            NodeValue::Strong => vec![Token::Strong(inner)],
            NodeValue::Strikethrough => {
                self.features.strikethrough = true;
                vec![Token::Strikethrough(inner)]
            }
            NodeValue::Superscript => vec![Token::Superscript(inner)],
            NodeValue::Link(ref link) => {
                self.features.url = true;
                let url = link.url.clone();
                let title = link.title.clone();
                vec![Token::Link(url, title, inner)]
            }
            NodeValue::Image(ref link) => {
                self.features.image = true;
                let url = link.url.clone();
                let title = link.title.clone();
                vec![Token::Image(url, title, inner)]
            }
            NodeValue::FootnoteReference(ref name) => {
                let name = name.clone();

                vec![Token::FootnoteReference(name)]
            }
            NodeValue::TableCell => vec![Token::TableCell(inner)],
            NodeValue::TableRow(header) => {
                if header {
                    vec![Token::TableHead(inner)]
                } else {
                    vec![Token::TableRow(inner)]
                }
            }
            NodeValue::Table(ref aligns) => {
                self.features.table = true;
                // TODO: actually use alignements)
                vec![Token::Table(aligns.len() as i32, inner)]
            }
        };
        Ok(inner)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// Replace consecutives Strs by a Str of both, collapse soft breaks to previous std and so on
fn collapse(ast: &mut Vec<Token>) {
    let mut i = 0;
    while i < ast.len() {
        if ast[i].is_str() && i + 1 < ast.len() {
            if ast[i + 1].is_str() {
                // Two consecutives Str, concatenate them
                let token = ast.remove(i + 1);
                if let (&mut Token::Str(ref mut dest), Token::Str(ref source)) =
                    (&mut ast[i], token)
                {
                    //                        dest.push(' ');
                    dest.push_str(source);
                    continue;
                } else {
                    unreachable!();
                }
            } else if ast[i + 1] == Token::SoftBreak {
                ast.remove(i + 1);
                if let &mut Token::Str(ref mut dest) = &mut ast[i] {
                    dest.push(' ');
                    continue;
                } else {
                    unreachable!();
                }
            }
        }

        // If token is containing others, recurse into them
        if let Some(ref mut inner) = ast[i].inner_mut() {
            collapse(inner);
        }
        i += 1;
    }
}

/// Replace images which are alone in a paragraph by standalone images
fn find_standalone(ast: &mut Vec<Token>) {
    for token in ast {
        let res = if let &mut Token::Paragraph(ref mut inner) = token {
            if inner.len() == 1 {
                if inner[0].is_image() {
                    if let Token::Image(source, title, inner) =
                        mem::replace(&mut inner[0], Token::Rule)
                    {
                        Token::StandaloneImage(source, title, inner)
                    } else {
                        unreachable!();
                    }
                } else {
                    // If paragraph only contains a link only containing an image, ok too
                    // Fixme: messy code and unnecessary clone
                    if let Token::Link(ref url, ref alt, ref mut inner) = inner[0] {
                        if inner[0].is_image() {
                            if let Token::Image(source, title, inner) =
                                mem::replace(&mut inner[0], Token::Rule)
                            {
                                Token::Link(
                                    url.clone(),
                                    alt.clone(),
                                    vec![Token::StandaloneImage(source, title, inner)],
                                )
                            } else {
                                unreachable!();
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
            } else {
                continue;
            }
        } else {
            continue;
        };

        *token = res;
    }
}
