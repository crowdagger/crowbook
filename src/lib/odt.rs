use escape::escape_html;
use token::Token;
use book::{Book, Number};
use error::{Error,Result};
use templates::odt;
use zipper::Zipper;

use mustache;

/// Rendererer for ODT
/// (Experimental)
pub struct OdtRenderer<'a> {
    book: &'a Book,
    current_numbering: bool,
    current_hide: bool,
    current_chapter: i32,
    automatic_styles: String,
}

impl<'a> OdtRenderer<'a> {
    /// Create a new OdtRenderer
    pub fn new(book: &'a Book) -> OdtRenderer {
        OdtRenderer {
            book: book,
            current_chapter: 1,
            current_numbering: book.numbering,
            current_hide: false,
            automatic_styles: String::from("
<style:style style:name=\"T1\" style:family=\"text\">
  <style:text-properties fo:font-style=\"italic\" style:font-style-asian=\"italic\" style:font-style-complex=\"italic\"/>
</style:style>
<style:style style:name=\"T2\" style:family=\"text\">
  <style:text-properties fo:font-weight=\"bold\" style:font-weight-asian=\"bold\" style:font-weight-complex=\"bold\"/>
</style:style>"),
        }
    }

    /// Render book
    pub fn render_book(&mut self) -> Result<String> {
        let content = try!(self.render_content());
        
        let mut zipper = try!(Zipper::new(&self.book.temp_dir));

        // Write template.odt there
        try!(zipper.write("template.odt", odt::ODT, false));
        // unzip it
        try!(zipper.unzip("template.odt"));
        // Complete it with content.xml
        try!(zipper.write("content.xml", &content.as_bytes(), false));
        // Zip and copy
        if let Some(ref file) = self.book.output_odt {
            zipper.generate_odt(file)
        } else {
            panic!("odt.render_book called while book.output_odt is not set");
        }
    }

    /// Render content.xml
    pub fn render_content(&mut self) -> Result<String> {
        let mut content = String::new();

        for &(n, ref v) in &self.book.chapters {
            self.current_hide = false;
            match n {
                Number::Unnumbered => self.current_numbering = false,
                Number::Default => self.current_numbering = self.book.numbering,
                Number::Specified(n) => {
                    self.current_numbering = self.book.numbering;
                    self.current_chapter = n;
                },
                Number::Hidden => {
                    self.current_numbering = false;
                    self.current_hide = true;
                },
            }
            for token in v {
                content.push_str(&self.parse_token(token));
            }
        }
        
        let template = mustache::compile_str(odt::CONTENT);
        let data = self.book.get_mapbuilder("none")
            .insert_str("content", content)
            .insert_str("automatic_styles", &self.automatic_styles)
            .build();

        let mut res:Vec<u8> = vec!();
        template.render_data(&mut res, &data);
        match String::from_utf8(res) {
            Err(_) => Err(Error::Render("generated content.xml was not utf-8 valid")),
            Ok(res) => Ok(res)
        }
    }

    /// Transform a vector of `Token`s to Odt format
    pub fn render_vec(&mut self, tokens:&[Token]) -> String {
        let mut res = String::new();

        for token in tokens {
            res.push_str(&self.parse_token(&token));
        }
        res
    }
    
    pub fn parse_token(&mut self, token: &Token) -> String {
        match *token {
            Token::Str(ref text) => escape_html(&*text),
            Token::Paragraph(ref vec) => format!("<text:p text:style-name=\"Text_20_body\">{}</text:p>\n", self.book.clean(self.render_vec(vec))),
            Token::Header(n, ref vec) => {
                if n == 1 && self.current_hide {
                    return String::new();
                }
                let s = if n == 1 && self.current_numbering {
                    let chapter = self.current_chapter;
                    self.current_chapter += 1;
                    self.book.get_header(chapter, &self.render_vec(vec)).unwrap()
                } else {
                    self.render_vec(vec)
                };
                format!("<text:h text:style-name=\"Heading_20_{}\">\n{}</text:h>\n",
                        n, escape_html(&self.book.clean(s)))
            },
            Token::Emphasis(ref vec) => format!("<text:span text:style-name=\"T1\">{}</text:span>", self.render_vec(vec)),
            Token::Strong(ref vec) => format!("<text:span text:style-name=\"T2\">{}</text:span>", self.render_vec(vec)),
            Token::List(ref vec) => format!("<text:list>\n{}</text:list>\n", self.render_vec(vec)),
            Token::OrderedList(_, ref vec) => {
                self.book.debug("Ordered list not currently implemented for ODT, fallbacking to unordered one");
                format!("<text:list>\n{}</text:list>\n", self.render_vec(vec))
            },
            Token::Item(ref vec) => format!("<text:list-item>\n<text:p>{}</text:p></text:list-item>", self.book.clean(self.render_vec(vec))),
            Token::Link(ref url, _, ref vec) => format!("<text:a xlink:type=\"simple\"  xlink:href=\"{}\">{}</text:a>", url, self.render_vec(vec)),
            Token::Code(ref vec) => format!("<text:span text:style-name=\"Preformatted_20_Text\">{}</text:span>", self.render_vec(vec)),
            Token::BlockQuote(ref vec) | Token::CodeBlock(_, ref vec) => {
                self.book.debug("warning: block quote and codeblocks are not currently implemented in ODT");
                format!("<text:p text:style-name=\"Text_20_Body\">{}</text:p>\n", self.book.clean(self.render_vec(vec)))
            },
            Token::SoftBreak | Token::HardBreak => String::from(" "),
            Token::Rule => format!("<text:p /><text:p>***</text:p><text:p />"),
            Token::Image(_,_,_) => {
                self.book.debug("warning: images not currently implemented for odt");
                String::from(" ")
            },
            Token::Table(_,_) | Token::TableHead(_) | Token::TableRow(_) | Token::TableCell(_) => {
                self.book.debug("warning: tables are not currently implemented for odt");
                String::from(" ")
            },
            Token::Footnote(_) => panic!("footnotes are not implemented yet"),
        }
    }
}

