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
use style;

use std::fmt;
use std::f64;
use punkt::{SentenceTokenizer, TrainingData};
use punkt::params::Standard;
use hyphenation;
use hyphenation::{Hyphenation, Language};

struct ChapterStats {
    pub name: String,
    pub word_count: usize,
    pub char_count: usize,
    pub sentence_count: usize,
    pub syllable_count: usize,
    pub flesch_score: f64,
}

pub struct Stats {
    chapters: Vec<ChapterStats>,
}

impl Stats {
    pub fn new(book: &Book) -> Stats {
        let mut stats = Stats { chapters: vec![] };

        let lang = book.options.get_str("lang").unwrap();
        let (td, hy, flesch_func) = Stats::language_data(lang);

        for c in &book.chapters {
            let name = c.filename.clone();
            let text = view_as_text(&c.content);
            let words: Vec<_> = text.split_whitespace().collect();
            let wc = words.len();
            // Note: Don't count the bytes with `len()` count the actual (multibyte-)characters
            let cc = text.chars().count();
            let sc = SentenceTokenizer::<Standard>::new(&text, &td).count();
            let corp = hyphenation::load(hy).unwrap();
            // Count the number of syllables for earch word.
            let syl = words
                .iter()
                .fold(0, |acc, w| acc + w.opportunities(&corp).len() + 1);
            let mut chapter_stats = ChapterStats {
                name: name,
                word_count: wc,
                char_count: cc,
                sentence_count: sc,
                syllable_count: syl,
                flesch_score: f64::NAN,
            };
            if let Some(ref f) = flesch_func {
                chapter_stats.flesch_score = f(&chapter_stats);
            }
            stats.chapters.push(chapter_stats);
        }
        stats
    }

    // The Flesch reading index formulae for different languages are from the `YoastSEO.js` text
    // analysis library. See: https://github.com/Yoast/YoastSEO.js/issues/267
    fn language_data(
        lang: &str,
    ) -> (
        TrainingData,
        Language,
        Option<Box<Fn(&ChapterStats) -> f64>>,
    ) {
        match lang {
            "cz" => (TrainingData::czech(), Language::Czech, None),
            "da" => (TrainingData::danish(), Language::Danish, None),
            "nl" => (
                TrainingData::dutch(),
                Language::Dutch,
                Some(Box::new(|s: &ChapterStats| {
                    206.84 - 77.0 * (s.syllable_count as f64 / s.word_count as f64)
                        - 0.93 * (s.word_count as f64 / s.sentence_count as f64)
                })),
            ),
            "en" => (
                TrainingData::english(),
                Language::English_GB,
                Some(Box::new(|s: &ChapterStats| {
                    206.835 - 0.77 * (s.syllable_count as f64 * 100.0 / s.word_count as f64)
                        - 0.93 * (s.word_count as f64 / s.sentence_count as f64)
                })),
            ),
            "et" => (TrainingData::estonian(), Language::Estonian, None),
            "fi" => (TrainingData::finnish(), Language::Finnish, None),
            "fr" => (
                TrainingData::french(),
                Language::French,
                Some(Box::new(|s: &ChapterStats| {
                    207.0 - 1.015 * (s.word_count as f64 / s.sentence_count as f64)
                        - 73.6 * (s.syllable_count as f64 / s.word_count as f64)
                })),
            ),
            "de" => (
                TrainingData::german(),
                Language::German_1996,
                Some(Box::new(|s: &ChapterStats| {
                    180.0 - (s.word_count as f64 / s.sentence_count as f64)
                        - 84.6 * (s.syllable_count as f64 / s.word_count as f64)
                })),
            ),
            "el" => (TrainingData::greek(), Language::Greek_Poly, None),
            "it" => (
                TrainingData::italian(),
                Language::Italian,
                Some(Box::new(|s: &ChapterStats| {
                    217.0 - 1.3 * (s.word_count as f64 / s.sentence_count as f64)
                        - 60.0 * (s.syllable_count as f64 * 100.0 / s.word_count as f64)
                })),
            ),
            "no" => (TrainingData::norwegian(), Language::Norwegian_Bokmal, None),
            "pl" => (TrainingData::polish(), Language::Polish, None),
            "pt" => (TrainingData::portuguese(), Language::Portuguese, None),
            "sl" => (TrainingData::slovene(), Language::Slovenian, None),
            "es" => (
                TrainingData::spanish(),
                Language::Spanish,
                Some(Box::new(|s: &ChapterStats| {
                    206.84 - 1.02 * (s.word_count as f64 / s.sentence_count as f64)
                        - 60.0 * (s.syllable_count as f64 * 100.0 / s.word_count as f64)
                })),
            ),
            "sv" => (TrainingData::swedish(), Language::Swedish, None),
            "tk" => (TrainingData::turkish(), Language::Turkish, None),
            _ => {
                warn!(
                    "Unknown language: '{}' for text statistics, using 'en' default.",
                    lang
                );
                (TrainingData::english(), Language::English_GB, None)
            }
        }
    }

    fn flesch_text(score: f64) -> String {
        String::from(match score {
            s if s.is_nan() => "Not availabe",
            s if s < 30.0 => "Very difficult",
            s if s < 50.0 => "Difficult",
            s if s < 60.0 => "Fairly difficult",
            s if s < 70.0 => "Standard",
            s if s < 80.0 => "Faily easy",
            s if s < 90.0 => "Easy",
            _ => "Very Easy",
        })
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_chapter_length = self.chapters
            .iter()
            .max_by_key(|e| e.name.chars().count())
            .unwrap()
            .name
            .chars()
            .count() + 3;
        write!(
            f,
            "{:<width$} {:>8} {:>10} {:>7} {:>11} {:>11} {:>16} {:>29}\n---------\n",
            style::header(&lformat!("Chapter")),
            style::header(&lformat!("Chars")),
            style::header(&lformat!("Syllables")),
            style::header(&lformat!("Words")),
            style::header(&lformat!("Sentences")),
            style::header(&lformat!("Chars/Word")),
            style::header(&lformat!("Words/Sentence")),
            style::header(&lformat!("Flesch reading ease index")),
            width = max_chapter_length
        )?;
        for c in &self.chapters {
            write!(
                f,
                "{:<width$} {:>8} {:>10} {:>7} {:>11} {:>11.2} {:>16.2} {:>8.1} => {:>17}\n",
                style::element(&c.name),
                c.char_count,
                c.syllable_count,
                c.word_count,
                c.sentence_count,
                c.char_count as f64 / c.word_count as f64,
                c.word_count as f64 / c.sentence_count as f64,
                c.flesch_score,
                Self::flesch_text(c.flesch_score),
                width = max_chapter_length
            )?;
        }
        let total = self.chapters.iter().fold((0, 0, 0, 0, 0.0, 0), |acc, c| {
            (
                acc.0 + c.char_count,
                acc.1 + c.syllable_count,
                acc.2 + c.word_count,
                acc.3 + c.sentence_count,
                acc.4 + c.flesch_score,
                acc.5 + 1,
            )
        });
        write!(
            f,
            "---------\n{:<width$} {:>8} {:>10} {:>7} {:>11} {:>11.2} {:>16.2} {:>8.1} => {:>17}\n",
            style::element(&lformat!("TOTAL:")),
            total.0,
            total.1,
            total.2,
            total.3,
            total.0 as f64 / total.2 as f64,
            total.2 as f64 / total.3 as f64,
            total.4 / total.5 as f64,
            Self::flesch_text(total.4 / total.5 as f64),
            width = max_chapter_length
        )
    }
}
