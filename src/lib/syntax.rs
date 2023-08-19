// Copyright (C) 2017-2023 Élisabeth HENRY.
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

use crate::error::Result;

use crowbook_text_processing::escape;
use rust_i18n::t;

#[cfg(feature = "syntect")]

/// Wrapper around syntect, so it can be more easily optionally compiled.
#[cfg(feature = "syntect")]
pub struct Syntax {
    syntax_set: syntect::parsing::SyntaxSet,
    theme: syntect::highlighting::Theme,
}

#[cfg(not(feature = "syntect"))]
pub struct Syntax {}

#[cfg(feature = "syntect")]
impl Syntax {
    /// Creates a new Syntax wrapper
    pub fn new(theme_name: &str) -> Syntax {
        let mut theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = match theme_set.themes.remove(theme_name) {
            Some(theme) => theme,
            None => {
                error!(
                    "{}",
                    t!("syntax.default_theme",
                        theme = theme_name
                    )
                );
                info!(
                    "{}",
                    t!("syntax valid_themes",
                        themes = theme_set
                            .themes
                            .keys()
                            .map(|s| s.to_owned())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                );
                theme_set.themes.remove("InspiredGitHub").unwrap()
            }
        };
        Syntax {
            syntax_set: syntect::parsing::SyntaxSet::load_defaults_nonewlines(),
            theme,
        }
    }

    /// Convert a string containing code to HTML
    pub fn to_html(&self, code: &str, language: &str) -> Result<String> {
        let language = strip_language(language);
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut h = syntect::easy::HighlightLines::new(syntax, &self.theme);
        let mut formatted_code = String::new();
        for line in code.split('\n') {
            let regions = h.highlight_line(line, &self.syntax_set)?;
            let bg = syntect::html::IncludeBackground::No;
            let res: String = syntect::html::styled_line_to_highlighted_html(&regions[..], bg)?;
            formatted_code.push_str(&res);
            formatted_code.push_str("\n");
        }
        Ok(format!("<pre>{formatted_code}</pre>"))
    }

    pub fn to_tex(&self, code: &str, language: &str) -> Result<String> {
        let language = strip_language(language);
        use crate::latex::insert_breaks;
        use syntect::highlighting::{Color, FontStyle};
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut h = syntect::easy::HighlightLines::new(syntax, &self.theme);

        let mut formatted_code = String::new();
        for line in code.split('\n') {
            let regions = h.highlight_line(line, &self.syntax_set)?;
            for (style, text) in regions {
                let mut content = escape::tex(text).into_owned();
                content = insert_breaks(&content);
                content = content
                    .replace('\n', "\\\\{}\n")
                    .replace(' ', "\\hphantom{ }\\allowbreak{}");
                content = format!("\\texttt{{{content}}}");
                if style.foreground != Color::BLACK {
                    let r = style.foreground.r as f32 / 255.0;
                    let g = style.foreground.g as f32 / 255.0;
                    let b = style.foreground.b as f32 / 255.0;
                    content = format!("\\textcolor[rgb]{{{r}, {g}, {b}}}{{{content}}}");
                }
                if style.font_style.contains(FontStyle::BOLD) {
                    content = format!("\\textbf{{{content}}}");
                }
                if style.font_style.contains(FontStyle::ITALIC) {
                    content = format!("\\emph{{{content}}}");
                }
                if style.font_style.contains(FontStyle::UNDERLINE) {
                    content = format!("\\underline{{{content}}}");
                }
                formatted_code.push_str(&content);
                formatted_code.push_str("\n");
            }
        }
        Ok(format!("{{\\sloppy {formatted_code}}}"))
    }
}

/// Strip language name of possible other infos, e.g. "rust,ignore" -> "rust"
/// Currently only ',' is done
fn strip_language(language: &str) -> &str {
    let splits: Vec<_> = language.split(|c: char| matches!(c, ',')).collect();
    splits[0].trim()
}

#[cfg(not(feature = "syntect"))]
impl Syntax {
    pub fn new(_: &str) -> Syntax {
        error!("{}", t!("syntax.no_support"));
        Syntax {}
    }

    pub fn to_html(&self, code: &str, language: &str) -> Result<String> {
        Ok(format!(
            "<pre><code class = \"language-{lang}\">{code}</code></pre>",
            code = escape::html(code),
            lang = language
        ))
    }

    pub fn to_tex(&self, code: &str, _: &str) -> Result<String> {
        Ok(format!(
            "\\begin{{spverbatim}}{}\\end{{spverbatim}}\n",
            code
        ))
    }
}
