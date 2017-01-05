// Copyright (C) 2017 Élisabeth HENRY.
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

use crowbook_text_processing::escape;


#[cfg(feature="syntect")]
use syntect;

/// Wrapper around syntect, so it can be more easily optionally compiled.
#[cfg(feature="syntect")]
pub struct Syntax {
    syntax_set: syntect::parsing::SyntaxSet,
    theme_set: syntect::highlighting::ThemeSet, 
}

#[cfg(not(feature="syntect"))]
pub struct Syntax {}

#[cfg(feature="syntect")]
impl Syntax {
    /// Creates a new Syntax wrapper
    pub fn new() -> Syntax {
        Syntax {
            syntax_set: syntect::parsing::SyntaxSet::load_defaults_nonewlines(),
            theme_set: syntect::highlighting::ThemeSet::load_defaults(),
        }
    }

    /// Convert a string containing code to HTML
    pub fn to_html(&self, code: &str, language: &str) -> String {
        let syntax = self.syntax_set.find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let theme = &self.theme_set.themes["InspiredGitHub"];
        let mut h = syntect::easy::HighlightLines::new(syntax, theme);
        let regions = h.highlight(&code);
        format!("<pre>{}</pre>",
                syntect::html::styles_to_coloured_html(&regions[..],
                                                       syntect::html::IncludeBackground::No))
    }

    pub fn to_tex(&self, code: &str, language: &str) -> String {
        use syntect::highlighting::{FONT_STYLE_BOLD, FONT_STYLE_ITALIC, FONT_STYLE_UNDERLINE};
        let syntax = self.syntax_set.find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let theme = &self.theme_set.themes["InspiredGitHub"];
        let mut h = syntect::easy::HighlightLines::new(syntax, theme);
        let regions = h.highlight(&code);
        
        let mut result = String::with_capacity(code.len());
        for (style, text) in regions.into_iter() {
            let mut content = escape::tex(text).into_owned();
            content = content.replace('\n', "\\\\\n")
                .replace(' ', "\\hphantom{ }");
            content = format!("\\texttt{{{}}}", content);
            if style.font_style.contains(FONT_STYLE_BOLD) {
                content = format!("\\textbf{{{}}}", content);
            }
            if style.font_style.contains(FONT_STYLE_ITALIC) {
                content = format!("\\emph{{{}}}", content);
            }
            if style.font_style.contains(FONT_STYLE_UNDERLINE) {
                content = format!("\\underline{{{}}}", content);
            }
            result.push_str(&content);
        }
        format!("{{\\vspace{{1em}}}}
{{\\setlength{{\\parindent}}{{0cm}}{}}}",
                result)
    }
}

#[cfg(not(feature="syntect"))]
impl Syntax {
    pub fn new() -> Syntax {
        ::logger::Logger::display_error(lformat!("crowbook was compiled without syntect support, syntax highlighting will be disabled"));
        Syntax {}
    }

    pub fn to_html(&self, code: &str, language: &str) -> String {
        format!("<pre><code class = \"language-{lang}\">{code}</code></pre>",
                code = escape::html(code),
                lang = language)
    }

    pub fn to_tex(&self, code: &str, language: &str) -> String {
        format!("\\begin{{spverbatim}}{}\\end{{spverbatim}}\n",
                code)
    }
}
