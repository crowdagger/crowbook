// Copyright (C) 2017-2023 Élisabeth HENRY.
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

// Progress bars non-implementation. Used when indicatif is not compiled.

use crate::book::{Book, Crowbar, CrowbarState};

/// Dummy bars implementation
pub struct Bars {}

impl Bars {
    pub fn new() -> Bars {
        Bars {}
    }
}

impl Book<'_> {
    pub fn private_add_progress_bar(&mut self, _: bool) {}

    /// Sets a finished message to the progress bar, if it is set
    pub fn bar_finish(&self, _: Crowbar, _: CrowbarState, _: &str) {}

    /// Adds a secondary progress bar to display progress of book parsing
    pub fn add_second_bar(&mut self, _: &str, _: u64) {}

    /// Increment second bar
    pub fn inc_second_bar(&self) {}

    /// Adds a spinner labeled key to the multibar, and set mainbar to "rendering"
    pub fn add_spinner_to_multibar(&mut self, _: &str) -> usize {
        0
    }

    pub fn bar_set_message(&self, _: Crowbar, _: &str) {}
}
