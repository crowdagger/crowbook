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

use caribon::Parser;

use book::Book;
use text_view::view_as_text;
use text_view::insert_annotation;
use token::Token;
use token::Data;
use error::{Error, Result, Source};

/// Repetition detector
pub struct RepetitionDetector {
    lang: String,
    fuzzy: bool,
    fuzzy_threshold: f32,
    ignore_proper: bool,
    max_distance: i32,
    threshold: f32,
}

impl RepetitionDetector {
    /// Creates a new repetition detector
    pub fn new(book: &Book) -> RepetitionDetector {
        RepetitionDetector {
            lang: book.options.get_str("lang").unwrap().to_string(),
            fuzzy: book.options.get_bool("proofread.repetitions.fuzzy").unwrap(),
            fuzzy_threshold: book.options.get_f32("proofread.repetitions.fuzzy.threshold").unwrap(),
            ignore_proper: book.options.get_bool("proofread.repetitions.ignore_proper").unwrap(),
            max_distance: book.options.get_i32("proofread.repetitions.max_distance").unwrap(),
            threshold: book.options.get_f32("proofread.repetitions.threshold").unwrap(),
        }
    }

    /// Check repetitions in a vector of tokens.
    ///
    /// This modifies the AST
    pub fn check_chapter(&self, tokens: &mut Vec<Token>) -> Result<()> {
        let fuzzy = if self.fuzzy {
            Some(self.fuzzy_threshold)
        } else {
            None
        };
        let mut parser = Parser::new(&self.lang)
            .map_err(|err| Error::default(Source::empty(),
                                          lformat!("could not create caribon parser: {error}", error = err)))?
            .with_fuzzy(fuzzy)
            .with_html(false)
            .with_ignore_proper(self.ignore_proper)
            .with_max_distance(self.max_distance as u32);

        let mut ast = parser.tokenize(&view_as_text(tokens))
            .map_err(|err| Error::default(Source::empty(),
                                          lformat!("error detecting repetitions: {err}",
                                                   err = err)))?;
        
        parser.detect_local(&mut ast, self.threshold);
        let repetitions = parser.ast_to_repetitions(&ast);
        for repetition in repetitions.iter() {
            insert_annotation(tokens,
                              &Data::Repetition(repetition.colour.to_string()),
                              repetition.offset,
                              repetition.length);
        }

        Ok(())
    }
}
    
