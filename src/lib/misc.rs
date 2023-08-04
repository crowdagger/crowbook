// Copyright (C) 2016-2023 Élisabeth HENRY.
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

//! Misc utility functions used across crowbook

use crate::token::Token;

use std::io::Result;
use std::path::{Path, PathBuf};
use base64::Engine;


/// Try to canonicalize a path using std::fs::canonicalize, and returns the
/// unmodified path if it fails (e.g. if the path doesn't exist (yet))
pub fn normalize<P: AsRef<Path>>(path: P) -> String {
    try_normalize(path.as_ref()).unwrap_or_else(|_| format!("{}", path.as_ref().display()))
}

fn try_normalize<P: AsRef<Path>>(path: P) -> Result<String> {
    let full_path = std::fs::canonicalize(path.as_ref())?;
    let mut cwd = std::env::current_dir()?;
    let mut ups = 0;

    loop {
        if let Ok(path) = full_path.strip_prefix(&cwd.clone()) {
            let mut new_path = PathBuf::new();
            for _ in 0..ups {
                new_path.push("../");
            }
            new_path.push(path);
            return Ok(format!("{}", new_path.display()));
        } else if !cwd.pop() {
            return Ok(format!("{}", full_path.display()));
        } else {
            ups += 1;
        }
    }
}

/// Insert a title (if there is none) to a vec of tokens
pub fn insert_title(tokens: &mut Vec<Token>) {
    for token in tokens.iter() {
        if let &Token::Header(1, _) = token {
            return;
        }
    }
    tokens.insert(0, Token::Header(1, vec![]));
}

/// Convert to base 64
pub fn u8_to_base64(s: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(s)
}
