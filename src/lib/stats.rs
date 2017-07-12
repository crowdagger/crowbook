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
use book::Book;
use text_view::view_as_text;

use std::fmt;

pub struct Stats {
  chapter_word_counts: Vec<(String, usize)>
}

impl Stats {
  pub fn new(book: &Book) -> Stats {
    let mut stats = Stats{ chapter_word_counts: vec!() };
    for c in &book.chapters {
      let count = view_as_text(&c.content).split_whitespace().count();
      stats.chapter_word_counts.push((c.filename.clone(), count));
    }
    stats
  }
}

impl fmt::Display for Stats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Chapter word counts:\n---------\n")?;
    for c in &self.chapter_word_counts {
      write!(f, "{:<30} {:>5}\n", c.0, c.1)?;
    }
    let total = self.chapter_word_counts.iter().fold(0, |acc, &(_, n)| acc +n);
    write!(f, "---------\nTOTAL:{:>30}\n", total)
  }
}
