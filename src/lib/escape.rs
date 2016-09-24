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

//! Provides utility functions for escaping text to HTML or LaTeX

use std::borrow::Cow;

const NB_CHAR:char = ' '; // non breaking space
const NB_CHAR_NARROW:char = '\u{202F}'; // narrow non breaking space
const NB_CHAR_EM:char = '\u{2002}'; // demi em space


/// Escape non breaking spaces for HTML, so they are visible.
#[doc(hidden)]
pub fn escape_nb_spaces<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if input.contains(|c| match c {
        NB_CHAR | NB_CHAR_NARROW | NB_CHAR_EM => true,
        _ => false
    }) {
        let mut output = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                NB_CHAR_NARROW
                    | NB_CHAR_EM
                    | NB_CHAR
                    => output.push_str(&format!("<span style = \"background-color: {}\">{}</span>",
                                                match c {
                                                    NB_CHAR => "#ffff66",
                                                    NB_CHAR_NARROW => "#9999ff",
                                                    NB_CHAR_EM => "#ff9999",
                                                    _ => unreachable!()
                                                },
                                                c)),
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input.into()
    }
}

/// Escape characters `<`, `>`, and `&`
///
/// # Examples
///
/// ```
/// use crowbook::escape::escape_html;
/// let s = escape_html("<foo> & <bar>");
/// assert_eq!(&s, "&lt;foo&gt; &amp; &lt;bar&gt;");
/// ```
pub fn escape_html<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if input.contains(|c| match c {
        '<'|'>'|'&' => true,
        _ => false
    }) {
        let mut output = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '<' => output.push_str("&lt;"),
                '>' => output.push_str("&gt;"),
                '&' => output.push_str("&amp;"),
                _ => output.push(c),
            }
        }

        Cow::Owned(output)
    } else {
        input.into()
    }
}

/// Escape characters for LaTeX
///
/// # Examples
///
/// ```
/// use crowbook::escape::escape_tex;
/// let s = escape_tex("command --foo # calls command with option foo");
/// assert_eq!(&s, r"command -{}-foo \# calls command with option foo");
/// ```
pub fn escape_tex<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if input.contains(|c| match c {
        '&'|'%'|'$'|'#'|'_'|'{'|'}'|'~'|'^'|'\\'|'-' => true,
        _ => false
    }) {
        let mut output = String::with_capacity(input.len());
        let mut chars:Vec<char> = input.chars().collect();
        chars.push(' '); // add a dummy char for call to .windows()
        // for &[c, next] in chars.windows(2) { // still experimental, uncomment when stable
        for win in chars.windows(2) { 
            let c = win[0];
            let next = win[1];
            match c {
                '-' => {
                    if next == '-' {
                        output.push_str(r"-{}"); // if next char is also a -, to avoid tex ligatures
                    } else {
                        output.push(c);
                    }
                },
                '&' => output.push_str(r"\&"),
                '%' => output.push_str(r"\%"),
                '$' => output.push_str(r"\$"),
                '#' => output.push_str(r"\#"),
                '_' => output.push_str(r"\_"),
                '{' => output.push_str(r"\{"),
                '}' => output.push_str(r"\}"),
                '~' => output.push_str(r"\textasciitilde{}" ),
                '^' => output.push_str(r"\textasciicircum{}"),
                '\\' => output.push_str(r"\textbackslash{}"),
                _  => output.push(c)
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}
