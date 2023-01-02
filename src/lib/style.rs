// Copyright (C) 2017-2022 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Crowbook is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

//! Functions for setting styles on text, using console to make it look prettier.

use console::{style, StyledObject, Term};

/// Displays a string as some header
pub fn header(msg: &str) -> StyledObject<&str> {
    style(msg).magenta().bold()
}

/// Displays a string as some element
pub fn element(msg: &str) -> StyledObject<&str> {
    style(msg).yellow().bold()
}

pub fn field(msg: &str) -> StyledObject<&str> {
    style(msg).cyan().bold()
}

pub fn tipe(msg: &str) -> StyledObject<&str> {
    style(msg).cyan()
}

pub fn value(msg: &str) -> StyledObject<&str> {
    style(msg).yellow()
}

pub fn fill(msg: &str, indent: &str) -> String {
    let (_, width) = Term::stdout().size();
    let options = textwrap::Options::new(width.into())
        .initial_indent(indent)
        .subsequent_indent(indent);
    textwrap::fill(msg, options)
}
