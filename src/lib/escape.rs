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
use regex::Regex;

const NB_CHAR:char = ' '; // non breaking space
const NB_CHAR_NARROW:char = '\u{202F}'; // narrow non breaking space
const NB_CHAR_EM:char = '\u{2002}'; // demi em space




/// Escape non breaking spaces for HTML, so there is no problem for displaying them if the font or browser
/// doesn't know what to do with them
#[doc(hidden)]
pub fn escape_nb_spaces<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if let Some(first) = input.chars().position(|c| match c {
        NB_CHAR | NB_CHAR_NARROW | NB_CHAR_EM => true,
        _ => false
    }) {
        let mut chars = input.chars().collect::<Vec<_>>();
        let rest = chars.split_off(first);
        let mut output = chars.into_iter().collect::<String>();
        for c in rest {
            match c {
                NB_CHAR_NARROW  => output.push_str(r#"<span class = "nnbsp">&#8201;</span>"#),
                NB_CHAR_EM => output.push_str(r#"<span class = "ensp">&#8194;</span>"#),
                NB_CHAR => output.push_str(r#"<span class = "nbsp">&#160;</span>"#),
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
        lazy_static! {
        static ref REGEX: Regex = Regex::new("[<>&]").unwrap();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let len = input.len();
        let mut output = Vec::with_capacity(len + len /2);
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes();
        for c in rest {
            match c {
                b'<' => output.extend_from_slice(b"&lt;"),
                b'>' => output.extend_from_slice(b"&gt;"),
                b'&' => output.extend_from_slice(b"&amp;"),
                _ => output.push(c),
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}


/// Escape quotes
///
/// Replace `"` by `'`
#[doc(hidden)]
pub fn escape_quotes<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if input.contains('"') {
        let mut output = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '"' => output.push('\''),
                _ => output.push(c)
            }
        }
        Cow::Owned(output)
    } else {
        input
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
    const REGEX_LITERAL:&'static str = r"[&%$#_\x7E\x2D\{\}\^\\]";
    lazy_static! {
       static ref REGEX: Regex = Regex::new(REGEX_LITERAL).unwrap();
        
    }
    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let len = input.len();
        let mut output = Vec::with_capacity(len + len/2);
        output.extend_from_slice(input[0..first].as_bytes());
        let mut bytes:Vec<_> = input[first..].bytes().collect();
        bytes.push(b' '); // add a dummy char for call to .windows()
        // for &[c, next] in chars.windows(2) { // still experimental, uncomment when stable
        for win in bytes.windows(2) { 
            let c = win[0];
            let next = win[1];
            match c {
                b'-' => {
                    if next == b'-' {
                        output.extend_from_slice(br"-{}"); // if next char is also a -, to avoid tex ligatures
                    } else {
                        output.push(c);
                    }
                },
                b'&' => output.extend_from_slice(br"\&"),
                b'%' => output.extend_from_slice(br"\%"),
                b'$' => output.extend_from_slice(br"\$"),
                b'#' => output.extend_from_slice(br"\#"),
                b'_' => output.extend_from_slice(br"\_"),
                b'{' => output.extend_from_slice(br"\{"),
                b'}' => output.extend_from_slice(br"\}"),
                b'~' => output.extend_from_slice(br"\textasciitilde{}" ),
                b'^' => output.extend_from_slice(br"\textasciicircum{}"),
                b'\\' => output.extend_from_slice(br"\textbackslash{}"),
                _  => output.push(c)
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}
