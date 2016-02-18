use escape::escape_html;
use token::Token;
use book::{Book, Number};
use parser::Parser;
use error::{Error,Result};

use mustache;
use mustache::MapBuilder;

/// Renderer for HTML.
///
/// Also used by Epub.
pub struct HtmlRenderer<'a> {
    current_chapter: i32,
    book: &'a Book,
    current_numbering: bool
}

impl<'a> HtmlRenderer<'a> {
    /// Creates a new HTML renderer
    pub fn new(book: &'a Book) -> HtmlRenderer<'a> {
        HtmlRenderer {
            book: book,
            current_chapter: 1,
            current_numbering: book.numbering,
        }
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        let mut parser = Parser::new();
        if let Some(cleaner) = self.book.get_cleaner() {
            parser = parser.with_cleaner(cleaner);
        }
        
        let mut content = String::new();

        for &(ref n, ref file) in &self.book.chapters {
            match n {
                &Number::Unnumbered => self.current_numbering = false,
                &Number::Default => self.current_numbering = self.book.numbering,
                &Number::Specified(n) => {
                    self.current_numbering = self.book.numbering;
                    self.current_chapter = n;
                }
            }
            let v = try!(parser.parse_file(file));
            for token in v {
                content.push_str(&self.parse_token(token));
            }
        }

        let template = mustache::compile_str(include_str!("../../templates/template.html"));
        
        let data = self.book.get_mapbuilder()
            .insert_str("content", content)
            .build();

        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated HTML was not utf-8 valid")),
            Ok(res) => Ok(res)
                          
        }
    }

    /// Transform a vector of `Token`s to HTML format.
    pub fn render_vec(&mut self, tokens: Vec<Token>) -> String {
        let mut res = String::new();
        
        for token in tokens {
            res.push_str(&self.parse_token(token));
        }
        res
    }
    
    fn parse_token(&mut self, token: Token) -> String {
        match token {
            Token::Str(text) => escape_html(&*text),
            Token::Paragraph(vec) => format!("<p>{}</p>\n", self.render_vec(vec)),
            Token::Header(n, vec) => {
                let s = if n == 1 && self.current_numbering {
                    let chapter = self.current_chapter;
                    self.current_chapter += 1;
                    format!("Chapitre {} : {}", chapter, self.render_vec(vec)) // todo: allow customization
                } else {
                    self.render_vec(vec)
                };
                format!("<h{}>{}</h{}>\n", n, s, n)
            },
            Token::Emphasis(vec) => format!("<em>{}</em>", self.render_vec(vec)),
            Token::Strong(vec) => format!("<b>{}</b>", self.render_vec(vec)),
            Token::Code(vec) => format!("<code>{}</code>", self.render_vec(vec)),
            Token::BlockQuote(vec) => format!("<blockquote>{}</blockquote>\n", self.render_vec(vec)),
            Token::CodeBlock(language, vec) => {
                let s = self.render_vec(vec);
                if language.is_empty() {
                    format!("<pre><code>\n{}</code></pre>\n", s)
                } else {
                    format!("<pre><code class = \"language-{}\">{}</code></pre>\n", language, s)
                }
            },
            Token::Rule => String::from("<p class = \"rule\">***</p>\n"),
            Token::SoftBreak => String::from(" "),
            Token::HardBreak => String::from("<br />\n"),
            Token::List(vec) => format!("<ul>\n{}</ul>\n", self.render_vec(vec)),
            Token::OrderedList(n, vec) => format!("<ol start = \"{}\">\n{}</ol>\n", n, self.render_vec(vec)),
            Token::Item(vec) => format!("<li>{}</li>\n", self.render_vec(vec)),
            Token::Link(url, title, vec) => format!("<a href = \"{}\"{}>{}</a>",
                                                    url,
                                                    if title.is_empty() {
                                                        String::new()
                                                    } else {
                                                        format!(" title = \"{}\"", title)
                                                    },
                                                    self.render_vec(vec)),
            Token::Image(url, title, alt) => format!("<img src = \"{}\" title = \"{}\" alt = \"{}\" />",
                                                     url,
                                                     title,
                                                     self.render_vec(alt))
                
        }
    }
}
    

