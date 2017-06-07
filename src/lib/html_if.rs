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

use error::{Error, Result, Source};
use html::HtmlRenderer;
use html::Highlight;
use book::{Book, compile_str};
use token::Token;
use renderer::Renderer;
use book_renderer::BookRenderer;
use parser::Parser;
use text_view::view_as_text;

use rustc_serialize::base64::{self, ToBase64};

use std::convert::{AsMut, AsRef};
use std::io;
use std::mem;

/// Interactive fiction HTML renderer
///
/// Renders a standalone, self-contained HTML file
pub struct HtmlIfRenderer<'a> {
    html: HtmlRenderer<'a>,
    n_fn: u32,
    curr_init: String,
    fn_defs: String,
}

impl<'a> HtmlIfRenderer<'a> {
    /// Creates a new HtmlIfRenderer
    pub fn new(book: &'a Book) -> Result<HtmlIfRenderer<'a>> {
        let mut html = HtmlRenderer::new(book,
                                         book.options
                                         .get_str("html.highlight.theme")
                                         .unwrap_or_else(|_| book.options.get_str("rendering.highlight.theme").unwrap()))?;
        html.handler.set_images_mapping(true);
        html.handler.set_base64(true);
        Ok(
            HtmlIfRenderer {
                html: html,
                n_fn: 0,
                curr_init: String::new(),
                fn_defs: String::new(),
            }
        )
    }

    /// Parse embedded javascript code
    pub fn parse_inner_code(&mut self, code: &str) -> Result<String> {
        let mut gen_code = String::new();
        let mut contains_md = false;
        let mut i = 0;
        let mut variables = vec![];
        
        while let Some(begin) = code[i..].find("@\"") {
            let begin = i + begin;
            if let Some(len) = code[begin..].find("\"@") {
                contains_md = true;
                let end = begin + len;
                gen_code.push_str(&code[i..begin]);
                
                let mut md_part = &code[begin+2..end];
                let rendered = self.render_vec(&Parser::new().parse(md_part)?)?;
                while let Some(b) = md_part.find("{{") {
                    md_part = &md_part[b+2..];
                    if let Some(l) = md_part.find("}}") {
                        variables.push(md_part[..l].to_owned());
                    }
                }
                gen_code.push_str("crowbook_return_variable += \"");
                    gen_code.push_str(&rendered 
                                  .replace('"', "\\\"")
                                  .replace('\n', "\\\n"));
                gen_code.push('"');
                for var in &variables {
                    gen_code.push_str(&format!(".replace(/{{{{{var}}}}}/, {var})",
                                               var = var));
                }
                gen_code.push(';');
                i = end + 2;
            }  else {
                gen_code.push_str(&code[i..begin+2]);
                i = begin + 2;
            }
        }
        gen_code.push_str(&code[i..]);
        if contains_md {
            gen_code = format!("var crowbook_return_variable = \"\";
{}
return crowbook_return_variable.replace(/<\\/ul><ul>/g, '');\n",
                                       gen_code);
        }
        let container = if !contains_md {
            "p"
        } else {
            "div"
        };
        let id = self.n_fn;
        self.fn_defs
            .push_str(&format!("function fn_{id}() {{
    {code}
}}\n",
                               id = id,
                               code = gen_code));
        self.curr_init
            .push_str(&format!("    result = fn_{id}();
    if (result != undefined) {{
        document.getElementById(\"result_{id}\").innerHTML = result;
    }}\n",
                               id = id));
        let content = format!("<{container} id = \"result_{id}\"></{container}>\n",
                              id = (self.n_fn),
                              container = container);
        self.n_fn += 1;
        Ok(content)
    }

    /// Renders a token
    ///
    /// Used by render_token implementation of Renderer trait. Separate function
    /// because we need to be able to call it from other renderers.
    ///
    /// See http://lise-henry.github.io/articles/rust_inheritance.html
    #[doc(hidden)]
    pub fn static_render_token<T>(this: &mut T, token: &Token) -> Result<String>
    where T: AsMut<HtmlIfRenderer<'a>>+AsRef<HtmlIfRenderer<'a>> +
        AsMut<HtmlRenderer<'a>>+AsRef<HtmlRenderer<'a>> + Renderer
    {
        match *token {
            Token::CodeBlock(ref language, ref v) if language == "" => {
                let mut html_if: &mut HtmlIfRenderer = this.as_mut();
                let code = view_as_text(v);
                let content = html_if.parse_inner_code(&code)?;
                Ok(content)
                
            },
            Token::CodeBlock(ref language, ref v) if language.starts_with(|c| c == '<' || c == '>') => {
                let mut html_if: &mut HtmlIfRenderer = this.as_mut();
                let code = format!("if (passageCount(state.current_id) {expr}) {{
    {code};
}}\n",
                                   code = view_as_text(v),
                                   expr = language);
                let content = html_if.parse_inner_code(&code)?;
                Ok(content)
            },
            Token::CodeBlock(ref language, ref v) if language.parse::<u32>().is_ok() => {
                let mut html_if: &mut HtmlIfRenderer = this.as_mut();
                let code = format!("if (passageCount(state.current_id) == {n}) {{
    {code};
}}\n",
                                   code = view_as_text(v),
                                   n = language.parse::<u32>().unwrap());
                let content = html_if.parse_inner_code(&code)?;
                Ok(content)
            },
            _ => HtmlRenderer::static_render_token(this, token)
        }
    }

    /// Render books as a standalone HTML file
    pub fn render_book(&mut self) -> Result<String> {
        let mut content = String::new();

        let mut titles = vec![];
        let mut chapters = vec![];
        let render_notes_chapter = true;

        for (i, chapter) in self.html.book.chapters.iter().enumerate() {
            self.html.handler.add_link(chapter.filename.as_ref(),
                                       format!("#chapter-{}", i));
        }

        let pre_code = self.html.book.options.get_str("html.if.new_turn")
            .unwrap_or("");
        let post_code = self.html.book.options.get_str("html.if.end_turn")
            .unwrap_or("");

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
                            title = self.html
                                .book
                                .get_chapter_header(self.html.current_chapter[1] + 1,
                                                    self.html.render_vec(vec)?,
                                                    |s| {
                                                        self.render_vec(&Parser::new()
                                                            .parse_inline(s)?)
                                                    })?
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

            let mut chapter_content = String::new();

            if !pre_code.is_empty() {
                chapter_content.push_str(&self.parse_inner_code(pre_code)?);
            }
            chapter_content.push_str(&HtmlRenderer::render_html(self, v, render_notes_chapter)?);
            if !post_code.is_empty() {
                chapter_content.push_str(&self.parse_inner_code(post_code)?);
            }
            
            chapters.push(format!("<div id = \"chapter-{}\" class = \"chapter\">
  {}
</div>",
                                  i,
                                  chapter_content));
            self.fn_defs.push_str(&format!("initFns.push(function () {{
    state.visited.push(state.current_id);
    {code}
}})\n",
                                           code = self.curr_init));
            self.curr_init = String::new();
        }
        
        self.html.source = Source::empty();

        for chapter in &chapters {
            content.push_str(chapter);
        }
        self.html.render_end_notes(&mut content);


        // Render the CSS
        let template_css = compile_str(self.html.book.get_template("html.css")?
                                       .as_ref(),
                                       &self.html.book.source,
                                       "html.css")?;
        let mut data = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("colours",
                        self.html.book.get_template("html.css.colours")?);
        if let Ok(html_css_add) = self.html.book.options.get_str("html.css.add") {
            data = data.insert_str("additional_code", html_css_add);
        }
        let data = data.build();
        let mut res: Vec<u8> = vec![];
        template_css.render_data(&mut res, &data)?;
        let css = String::from_utf8_lossy(&res);

        // Render the JS
        let template_js =
            compile_str(self.html.book.get_template("html.if.js")?.as_ref(),
                        &self.html.book.source,
                        "html.standalone.js")?;
        let data = self.html.book.get_metadata(|s| Ok(s.to_owned()))?
            .insert_bool("one_chapter", true)
            .insert_str("js_prelude", mem::replace(&mut self.fn_defs, String::new()))
            .insert_str("new_game", self.html.book.get_template("html.if.new_game").unwrap())
            .insert_str("common_script",
                        self.html.book.get_template("html.js").unwrap().as_ref())
            .build();
        let mut res: Vec<u8> = vec![];
        template_js.render_data(&mut res, &data)?;
        let js = String::from_utf8_lossy(&res);

        // Render the HTML document
        let mut mapbuilder = self.html
            .book
            .get_metadata(|s| self.render_vec(&Parser::new().parse_inline(s)?))?
            .insert_str("content", content)
            .insert_str("script", js)
            .insert_bool(self.html.book.options.get_str("lang").unwrap(), true)
            .insert_bool("one_chapter", true)
            .insert_str("style", css.as_ref())
            .insert_str("print_style",
                        self.html.book.get_template("html.css.print").unwrap())
            .insert_str("footer", HtmlRenderer::get_footer(self)?)
            .insert_str("header", HtmlRenderer::get_header(self)?)
            .insert_bool("has_toc", false);
        if let Ok(favicon) = self.html.book.options.get_path("html.icon") {
                let favicon = self.html.handler.map_image(&self.html.book.source, favicon)?;
                mapbuilder = mapbuilder.insert_str("favicon", format!("<link rel = \"icon\" href = \"{}\">", favicon));
            }
        if self.html.highlight == Highlight::Js {
            let highlight_js = self.html.book.get_template("html.highlight.js")?
                .as_bytes()
                .to_base64(base64::STANDARD);
            let highlight_js = format!("data:text/javascript;base64,{}", highlight_js);
            mapbuilder = mapbuilder.insert_bool("highlight_code", true)
                .insert_str("highlight_css",
                            self.html.book.get_template("html.highlight.css")?)
                .insert_str("highlight_js", highlight_js);
        }
        let data = mapbuilder.build();
        let template = compile_str(self.html.book.get_template("html.standalone.template")?
                                   .as_ref(),
                                   &self.html.book.source,
                                   "html.standalone.template")?;
        let mut res = vec![];
        template.render_data(&mut res, &data)?;
        Ok(String::from_utf8_lossy(&res).into_owned())
    }
}

derive_html!{HtmlIfRenderer<'a>, HtmlIfRenderer::static_render_token}


pub struct HtmlIf {}

impl BookRenderer for HtmlIf {
    fn render(&self, book: &Book, to: &mut io::Write) -> Result<()> {
        let mut html = HtmlIfRenderer::new(book)?;
        let result = html.render_book()?;
        to.write_all(result.as_bytes())
            .map_err(|e| {
                Error::render(&book.source,
                              lformat!("problem when writing interactive fiction: {error}", error = e))
            })?;
        Ok(())
    }
}
