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
use std::path::Path;
use std::io::Result;

/// Try to canonicalize a path using std::fs::canonicalize, and returns the
/// unmodified path if it fails (e.g. if the path doesn't exist (yet))
pub fn canonicalize<P: AsRef<Path>>(path: P) -> String {
    try_canonicalize(path.as_ref())
        .unwrap_or(format!("{}", path.as_ref().display()))
}


fn try_canonicalize<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = std::fs::canonicalize(path.as_ref())?;
    let cwd = std::env::current_dir()?;
    Ok(if let Ok(path) = path.strip_prefix(&cwd) {
        format!("{}", path.display())
    } else {
        format!("{}", path.display())
    })
}
