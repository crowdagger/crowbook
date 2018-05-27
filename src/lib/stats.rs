// Copyright (C) 2017, 2018 Falco Hirschenberger, Élisabeth Henry.
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
use hyphenation;
use hyphenation::{Hyphenation, Language};
#[cfg(feature = "nightly")]
use punkt::{SentenceTokenizer, TrainingData};
#[cfg(feature = "nightly")]
use punkt::params::Standard;

/* Only collected on nightly */
struct AdvancedStats {
    pub sentence_count: usize,
    pub flesch_score: f64,
}

struct ChapterStats {
    pub name: String,
    pub word_count: usize,
    pub char_count: usize,
    pub syllable_count: usize,
    pub advanced: Option<AdvancedStats>,
}

impl ChapterStats {
    #[cfg(feature = "nightly")]
    pub fn fill_advanced(&mut self, lang: Language, text: &str) {
        let (td, flesch_func) = Stats::language_data(lang);
        let sc = SentenceTokenizer::<Standard>::new(&text, &td).count();
        let stats = AdvancedStats {
            sentence_count: sc,
            flesch_score: f64::NAN,
        };
        self.advanced = Some(stats);
        let score = if let Some(ref f) = flesch_func {
            f(self)
        } else {
            f64::NAN
        };
        if let Some(ref mut adv) = self.advanced {
            adv.flesch_score = score;
        }
    }

    #[cfg(not(feature = "nightly"))]
    pub fn fill_advanced(&mut self, _: Language, _: &str) {
    }
}


pub struct Stats {
    chapters: Vec<ChapterStats>,
    advanced: bool,
}

impl Stats {
    pub fn new(book: &Book, advanced: bool) -> Stats {
        let lang = book.options.get_str("lang").unwrap();
        let lang = Stats::language_from_str(lang);
        let mut stats = Stats { chapters: vec![], advanced: advanced };

        for c in &book.chapters {
            let name = c.filename.clone();
            let text = view_as_text(&c.content);
            let words: Vec<_> = text.split_whitespace().collect();
            let wc = words.len();
            // Note: Don't count the bytes with `len()` count the actual (multibyte-)characters
            let cc = text.chars().count();
            let corp = hyphenation::load(lang).unwrap();
            // Count the number of syllables for earch word.
            let syl = words
                .iter()
                .fold(0, |acc, w| acc + w.opportunities(&corp).len() + 1);
            let mut chapter_stats = ChapterStats {
                name: name,
                word_count: wc,
                char_count: cc,
                syllable_count: syl,
                advanced: None,
            };
            if advanced {
                chapter_stats.fill_advanced(lang, &text);
            }
            stats.chapters.push(chapter_stats);
        }
        stats
    }


    // Returns the Languuage (defined by Hyphenation crate) according to the str code
    fn language_from_str(lang: &str) -> Language {
        // FIXME: handle case where lang is e.g. fr_FR or en_GB
        match lang {
            "cz" => Language::Czech,
            "da" => Language::Danish,
            "nl" => Language::Dutch,
            "en" => Language::English_GB,
            "et" => Language::Estonian,
            "fi" => Language::Finnish,
            "fr" => Language::French,
            "de" => Language::German_1996,
            "el" => Language::Greek_Poly,
            "it" => Language::Italian,
            "no" => Language::Norwegian_Bokmal,
            "pl" => Language::Polish,
            "pt" => Language::Portuguese,
            "sl" => Language::Slovenian,
            "es" => Language::Spanish,
            "sv" => Language::Swedish,
            "tk" => Language::Turkish,
            _ =>  {
                warn!(
                    //FIXME: display localized warning (or use Result?)
                    "Unknown language: '{}' for text statistics, using 'en' default.",
                    lang
                );
                Language::English_GB
            },
        }
    }
    
    // The Flesch reading index formulae for different languages are from the `YoastSEO.js` text
    // analysis library. See: https://github.com/Yoast/YoastSEO.js/issues/267
    #[cfg(feature = "nightly")]
    fn language_data(
        lang: Language
    ) -> (
        TrainingData,
        Option<Box<Fn(&ChapterStats) -> f64>>,
    ) {
        match lang {
            Language::Czech => (TrainingData::czech(), None),
            Language::Danish => (TrainingData::danish(), None),
            Language::Dutch => (
                TrainingData::dutch(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        206.84 - 77.0 * (s.syllable_count as f64 / s.word_count as f64)
                            - 0.93 * (s.word_count as f64 / adv.sentence_count as f64)
                    } else {
                        unreachable!("If nightly build is set, it should always return Some(thing)");
                    }
                })),
            ),
            Language::English_GB => (
                TrainingData::english(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                    206.835 - 0.77 * (s.syllable_count as f64 * 100.0 / s.word_count as f64)
                        - 0.93 * (s.word_count as f64 / adv.sentence_count as f64)
                    } else {
                        unreachable!();
                    }})),
            ),
            Language::Estonian => (TrainingData::estonian(), None),
            Language::Finnish => (TrainingData::finnish(), None),
            Language::French => (
                TrainingData::french(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        207.0 - 1.015 * (s.word_count as f64 / adv.sentence_count as f64)
                            - 73.6 * (s.syllable_count as f64 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }})),
            ),
            Language::German_1996 => (
                TrainingData::german(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        180.0 - (s.word_count as f64 / adv.sentence_count as f64)
                            - 84.6 * (s.syllable_count as f64 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }
                })),
            ),
            Language::Greek_Poly => (TrainingData::greek(), None),
            Language::Italian => (
                TrainingData::italian(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        217.0 - 1.3 * (s.word_count as f64 / adv.sentence_count as f64)
                            - 60.0 * (s.syllable_count as f64 * 100.0 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }
                })),
            ),
            Language::Norwegian_Bokmal => (TrainingData::norwegian(), None),
            Language::Polish => (TrainingData::polish(), None),
            Language::Portuguese => (TrainingData::portuguese(), None),
            Language::Slovenian => (TrainingData::slovene(), None),
            Language::Spanish => (
                TrainingData::spanish(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        206.84 - 1.02 * (s.word_count as f64 / adv.sentence_count as f64)
                        - 60.0 * (s.syllable_count as f64 * 100.0 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }})),
            ),
            Language::Swedish => (TrainingData::swedish(), None),
            Language::Turkish => (TrainingData::turkish(), None),
            _ => {
                unreachable!("language_data should handle all casess returned by language_from_str");
            },
        }
    }

    fn flesch_text(score: f64) -> String {
        String::from(match score {
            s if s.is_nan() => "Not availabe",
            s if s < 30.0 => "Very difficult",
            s if s < 50.0 => "Difficult",
            s if s < 60.0 => "Fairly difficult",
            s if s < 70.0 => "Standard",
            s if s < 80.0 => "Fairly easy",
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
        if self.advanced {
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
        } else {
            write!(
                f,
                "{:<width$} {:>8} {:>10} {:>11}\n---------\n",
                style::header(&lformat!("Chapter")),
                style::header(&lformat!("Chars")),
                style::header(&lformat!("Words")),
                style::header(&lformat!("Chars/Word")),
                width = max_chapter_length
            )?;
        }
        for c in &self.chapters {
            if let Some(ref adv) = c.advanced {
                write!(
                    f,
                    "{:<width$} {:>8} {:>10} {:>7} {:>11} {:>11.2} {:>16.2} {:>8.1} => {:>17}\n",
                    style::element(&c.name),
                    c.char_count,
                    c.syllable_count,
                    c.word_count,
                    adv.sentence_count,
                    c.char_count as f64 / c.word_count as f64,
                    c.word_count as f64 / adv.sentence_count as f64,
                    adv.flesch_score,
                    Self::flesch_text(adv.flesch_score),
                    width = max_chapter_length
                )?;
            } else {
                write!(
                    f,
                    "{:<width$} {:>8} {:>10} {:>11.2}\n",
                    style::element(&c.name),
                    c.char_count,
                    c.word_count,
                    c.char_count as f64 / c.word_count as f64,
                    width = max_chapter_length
                )?;
            }
        }
        let total = self.chapters.iter().fold((0, 0, 0, 0, 0.0, 0), |acc, c| {
            let sentence_count;
            let flesch_score;
            if let Some(ref adv) = c.advanced {
                sentence_count = adv.sentence_count;
                flesch_score = adv.flesch_score;
            } else {
                sentence_count = 0;
                flesch_score = f64::NAN;
            };
            (
                acc.0 + c.char_count,
                acc.1 + c.syllable_count,
                acc.2 + c.word_count,
                acc.3 + sentence_count,
                acc.4 + flesch_score,
                acc.5 + 1,
            )
        });
        if self.advanced {
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
                width = max_chapter_length
            )             
        } else {
            write!(
                f,
                "---------\n{:<width$} {:>8} {:>10} {:>11.2}\n",
                style::element(&lformat!("TOTAL:")),
                total.0,
                total.2,
                total.0 as f64 / total.2 as f64,
                width = max_chapter_length
            )
        }
    }
}
