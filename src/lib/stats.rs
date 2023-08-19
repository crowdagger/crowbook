// Copyright (C) 2017, 2018, 2019 Falco Hirschenberger, Élisabeth Henry.
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

use crate::book::Book;
use crate::style;
use crate::text_view::view_as_text;

#[cfg(feature = "nightly")]
use hyphenation;
#[cfg(feature = "nightly")]
use hyphenation::{Hyphenator, Language, Load};
#[cfg(feature = "nightly")]
use punkt::params::Standard;
#[cfg(feature = "nightly")]
use punkt::{SentenceTokenizer, TrainingData};
use std::f64;
use std::fmt;
use rust_i18n::t;

/* Only collected on nightly */
struct AdvancedStats {
    pub sentence_count: usize,
    pub flesch_score: f64,
    pub syllable_count: usize,
}

struct ChapterStats {
    pub name: String,
    pub word_count: usize,
    pub char_count: usize,
    pub advanced: Option<AdvancedStats>,
}

impl ChapterStats {
    #[cfg(feature = "nightly")]
    pub fn fill_advanced(&mut self, lang: &str, text: &str) {
        let words: Vec<_> = text.split_whitespace().collect();
        let lang = Stats::language_from_str(lang);
        let corp = hyphenation::Standard::from_embedded(lang).unwrap();
        // Count the number of syllables for earch word.
        let syl = words
            .iter()
            .fold(0, |acc, w| acc + corp.opportunities(w).len() + 1);

        let (td, flesch_func) = Stats::language_data(lang);
        let sc = SentenceTokenizer::<Standard>::new(&text, &td).count();
        let stats = AdvancedStats {
            sentence_count: sc,
            flesch_score: f64::NAN,
            syllable_count: syl,
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
    pub fn fill_advanced(&mut self, _: &str, _: &str) {}
}

pub struct Stats {
    chapters: Vec<ChapterStats>,
    advanced: bool,
}

impl Stats {
    pub fn new(book: &Book, advanced: bool) -> Stats {
        let lang = book.options.get_str("lang").unwrap();

        let mut stats;

        if cfg!(not(feature = "nightly")) {
            if advanced {
                warn!("{}", t!("stats.no_advanced"));
            }
            stats = Stats {
                chapters: vec![],
                advanced: false,
            };
        } else {
            stats = Stats {
                chapters: vec![],
                advanced,
            };
            if !advanced {
                info!(
                    "{}",
                    t!("stats.avanced")
                );
            }
        }

        for c in &book.chapters {
            let name = c.filename.clone();
            let text = view_as_text(&c.content);
            let wc = text.split_whitespace().count();
            // Note: Don't count the bytes with `len()` count the actual (multibyte-)characters
            let cc = text.chars().count();

            let mut chapter_stats = ChapterStats {
                name,
                word_count: wc,
                char_count: cc,
                advanced: None,
            };
            if advanced {
                chapter_stats.fill_advanced(lang, &text);
            }
            stats.chapters.push(chapter_stats);
        }
        stats
    }

    // Returns the Language (defined by Hyphenation crate) according to the str code
    #[cfg(feature = "nightly")]
    fn language_from_str(lang: &str) -> Language {
        // FIXME: handle case where lang is e.g. fr_FR or en_GB
        match lang {
            "cz" => Language::Czech,
            "da" => Language::Danish,
            "nl" => Language::Dutch,
            "en" => Language::EnglishGB,
            "et" => Language::Estonian,
            "fi" => Language::Finnish,
            "fr" => Language::French,
            "de" => Language::German1996,
            "el" => Language::GreekPoly,
            "it" => Language::Italian,
            "no" => Language::NorwegianBokmal,
            "pl" => Language::Polish,
            "pt" => Language::Portuguese,
            "sl" => Language::Slovenian,
            "es" => Language::Spanish,
            "sv" => Language::Swedish,
            "tk" => Language::Turkish,
            _ => {
                warn!(
                    //FIXME: display localized warning (or use Result?)
                    "Unknown language: '{}' for text statistics, using 'en' default.",
                    lang
                );
                Language::EnglishGB
            }
        }
    }

    // The Flesch reading index formulae for different languages are from the `YoastSEO.js` text
    // analysis library. See: https://github.com/Yoast/YoastSEO.js/issues/267
    #[cfg(feature = "nightly")]
    fn language_data(lang: Language) -> (TrainingData, Option<Box<Fn(&ChapterStats) -> f64>>) {
        match lang {
            Language::Czech => (TrainingData::czech(), None),
            Language::Danish => (TrainingData::danish(), None),
            Language::Dutch => (
                TrainingData::dutch(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        206.84
                            - 77.0 * (adv.syllable_count as f64 / s.word_count as f64)
                            - 0.93 * (s.word_count as f64 / adv.sentence_count as f64)
                    } else {
                        unreachable!(
                            "If nightly build is set, it should always return Some(thing)"
                        );
                    }
                })),
            ),
            Language::EnglishGB => (
                TrainingData::english(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        206.835
                            - 0.77 * (adv.syllable_count as f64 * 100.0 / s.word_count as f64)
                            - 0.93 * (s.word_count as f64 / adv.sentence_count as f64)
                    } else {
                        unreachable!();
                    }
                })),
            ),
            Language::Estonian => (TrainingData::estonian(), None),
            Language::Finnish => (TrainingData::finnish(), None),
            Language::French => (
                TrainingData::french(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        207.0
                            - 1.015 * (s.word_count as f64 / adv.sentence_count as f64)
                            - 73.6 * (adv.syllable_count as f64 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }
                })),
            ),
            Language::German1996 => (
                TrainingData::german(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        180.0
                            - (s.word_count as f64 / adv.sentence_count as f64)
                            - 84.6 * (adv.syllable_count as f64 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }
                })),
            ),
            Language::GreekPoly => (TrainingData::greek(), None),
            Language::Italian => (
                TrainingData::italian(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        217.0
                            - 1.3 * (s.word_count as f64 / adv.sentence_count as f64)
                            - 60.0 * (adv.syllable_count as f64 * 100.0 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }
                })),
            ),
            Language::NorwegianBokmal => (TrainingData::norwegian(), None),
            Language::Polish => (TrainingData::polish(), None),
            Language::Portuguese => (TrainingData::portuguese(), None),
            Language::Slovenian => (TrainingData::slovene(), None),
            Language::Spanish => (
                TrainingData::spanish(),
                Some(Box::new(|s: &ChapterStats| {
                    if let Some(ref adv) = s.advanced {
                        206.84
                            - 1.02 * (s.word_count as f64 / adv.sentence_count as f64)
                            - 60.0 * (adv.syllable_count as f64 * 100.0 / s.word_count as f64)
                    } else {
                        unreachable!();
                    }
                })),
            ),
            Language::Swedish => (TrainingData::swedish(), None),
            Language::Turkish => (TrainingData::turkish(), None),
            _ => {
                unreachable!(
                    "language_data should handle all casess returned by language_from_str"
                );
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
            s if s < 80.0 => "Fairly easy",
            s if s < 90.0 => "Easy",
            _ => "Very Easy",
        })
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_chapter_length = self
            .chapters
            .iter()
            .max_by_key(|e| e.name.chars().count())
            .unwrap()
            .name
            .chars()
            .count()
            + 3;
        if self.advanced {
            write!(
                f,
                "{:<width$} {:>8} {:>10} {:>7} {:>11} {:>11} {:>16} {:>29}\n---------\n",
                style::header(&t!("stats.chapter")),
                style::header(&t!("stats.chars")),
                style::header(&t!("stats.syllables")),
                style::header(&t!("stats.words")),
                style::header(&t!("stats.sentences")),
                style::header(&t!("stats.chars_word")),
                style::header(&t!("stats.words_sentence")),
                style::header(&t!("stats.flesch")),
                width = max_chapter_length
            )?;
        } else {
            write!(
                f,
                "{:<width$} {:>8} {:>10} {:>11}\n---------\n",
                style::header(&t!("stats.chapter")),
                style::header(&t!("stats.chars")),
                style::header(&t!("stats.words")),
                style::header(&t!("stats.chars_word")),
                width = max_chapter_length
            )?;
        }
        for c in &self.chapters {
            if let Some(ref adv) = c.advanced {
                writeln!(
                    f,
                    "{:<width$} {:>8} {:>10} {:>7} {:>11} {:>11.2} {:>16.2} {:>8.1} => {:>17}",
                    style::element(&c.name),
                    c.char_count,
                    adv.syllable_count,
                    c.word_count,
                    adv.sentence_count,
                    c.char_count as f64 / c.word_count as f64,
                    c.word_count as f64 / adv.sentence_count as f64,
                    adv.flesch_score,
                    Self::flesch_text(adv.flesch_score),
                    width = max_chapter_length
                )?;
            } else {
                writeln!(
                    f,
                    "{:<width$} {:>8} {:>10} {:>11.2}",
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
            let syllable_count;
            if let Some(ref adv) = c.advanced {
                sentence_count = adv.sentence_count;
                flesch_score = adv.flesch_score;
                syllable_count = adv.syllable_count;
            } else {
                sentence_count = 0;
                flesch_score = f64::NAN;
                syllable_count = 0;
            };
            (
                acc.0 + c.char_count,
                acc.1 + syllable_count,
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
                style::element(&t!("stats.total")),
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
        } else {
            write!(
                f,
                "---------\n{:<width$} {:>8} {:>10} {:>11.2}\n",
                style::element(&t!("stats.total")),
                total.0,
                total.2,
                total.0 as f64 / total.2 as f64,
                width = max_chapter_length
            )
        }
    }
}
