// Copyright (C) 2017 Falco Hirschenberger, Élisabeth Henry.
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

struct ChapterStats {
    pub name: String,
    pub word_count: usize,
    pub char_count: usize,
}

pub struct Stats {
    chapters: Vec<ChapterStats>,
}

impl Stats {
    pub fn new(book: &Book) -> Stats {
        let mut stats = Stats{
            chapters: vec!(),
        };
        for c in &book.chapters {
            let name = c.filename.clone();
            let text = view_as_text(&c.content);
            let wc = text.split_whitespace().count();
            let cc = text.len();
            stats.chapters.push(ChapterStats {
                name: name,
                word_count: wc,
                char_count: cc
            });
        }
        stats
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<30} {:>6} {:>7}\n---------\n",
               lformat!("Chapter"),
               lformat!("Words"),
               lformat!("Chars"))?;
        for c in &self.chapters {
            write!(f, "{:<30} {:>6} {:>7}\n",
                   c.name,
                   c.word_count,
                   c.char_count)?;
        }
        let total = self.chapters
            .iter()
            .fold((0, 0), |acc, c| (acc.0 + c.word_count, acc.1 + c.char_count));
        write!(f, "---------\n{:<30} {:>6} {:>7}\n",
               lformat!("TOTAL:"),
               total.0,
               total.1)
    }
}
