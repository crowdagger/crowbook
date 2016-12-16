// Copyright (C) 2016 Élisabeth HENRY.
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

use std;
use std::path::{Path, PathBuf};
use std::io::Result;

/// Try to canonicalize a path using std::fs::canonicalize, and returns the
/// unmodified path if it fails (e.g. if the path doesn't exist (yet))
pub fn normalize<P: AsRef<Path>>(path: P) -> String {
    try_normalize(path.as_ref())
        .unwrap_or(format!("{}", path.as_ref().display()))
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
        } else {
            if !cwd.pop() {
                return Ok(format!("{}", full_path.display()));
            } else {
                ups += 1;
            }
        }
    }
}
