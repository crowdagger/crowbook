// Copyright (C) 2017 Élisabeth HENRY.
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

//! Functions for not setting styles on text, not using console to make it look prettier.
//! (stub implementation when optional dependency disabled)

pub fn header(msg: &str) -> &str { msg }
pub fn element(msg: &str) -> &str { msg }
pub fn field(msg: &str) -> &str { msg }
pub fn tipe(msg: &str) -> &str { msg }
pub fn value(msg: &str) -> &str { msg }
pub fn fill(msg: &str, indent: &str) -> String { format!("{}{}", indent, msg) }
