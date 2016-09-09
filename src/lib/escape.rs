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

/// Escape characters <, >, and &
pub fn escape_html(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c),
        }
    }

    output
}

/// Escape characters for tex file
pub fn escape_tex(input: &str) -> String {
    let mut output = String::new();
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
    output
}
