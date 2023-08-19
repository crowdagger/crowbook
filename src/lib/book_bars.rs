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

// Progress bars implementation. Moved into a different file so it is possible
// to make some dependencies (incidacitf) optional.

use crate::book::{Book, Crowbar, CrowbarState};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rust_i18n::t;

use std::sync::Arc;
use std::time::Duration;

/// Store the progress bars needed for the book
pub struct Bars {
    /// Whether or not to use emoji
    pub emoji: bool,
    /// Container for the progress bars
    pub multibar: Option<Arc<MultiProgress>>,
    /// Main progress bar (actually a spinner)
    pub mainbar: Option<ProgressBar>,
    /// Secondary bar
    pub secondbar: Option<ProgressBar>,
    // /// Guard for thread
    // pub guard: Option<thread::JoinHandle<()>>,
    /// Spinners for each renderier
    pub spinners: Vec<ProgressBar>,
}

impl Bars {
    /// Create a new bars storage
    pub fn new() -> Bars {
        Bars {
            emoji: false,
            multibar: None,
            mainbar: None,
            secondbar: None,
            // guard: None,
            spinners: vec![],
        }
    }
}

impl Default for Bars {
    fn default() -> Self {
        Self::new()
    }
}

/// Return the style of a bar

impl Book<'_> {
    /// Adds a progress bar where where info should be written.
    ///
    /// See [indicatif doc](https://docs.rs/indicatif) for more information.
    pub fn private_add_progress_bar(&mut self, emoji: bool) {
        self.bars.emoji = emoji;
        let multibar = Arc::new(MultiProgress::new());
        self.bars.multibar = Some(multibar);
        let b = self
            .bars
            .multibar
            .as_ref()
            .unwrap()
            .add(ProgressBar::new_spinner());
        b.enable_steady_tick(Duration::from_millis(200));
        self.bars.mainbar = Some(b);
        self.bar_set_style(Crowbar::Main, CrowbarState::Running);
    }

    /// Sets a finished message to the progress bar, if it is set
    pub fn bar_finish(&self, bar: Crowbar, state: CrowbarState, msg: &str) {
        self.bar_set_style(bar, state);
        let pb = match bar {
            Crowbar::Main => {
                if let Some(ref bar) = self.bars.mainbar {
                    bar
                } else {
                    return;
                }
            }
            Crowbar::Second => {
                if let Some(ref bar) = self.bars.secondbar {
                    bar
                } else {
                    return;
                }
            }
            Crowbar::Spinner(i) => {
                if i < self.bars.spinners.len() {
                    &self.bars.spinners[i]
                } else {
                    return;
                }
            }
        };

        match bar {
            Crowbar::Second => pb.finish_and_clear(),
            _ => pb.finish_with_message(msg.to_owned()),
        };
    }

    /// Adds a secondary progress bar to display progress of book parsing
    pub fn add_second_bar(&mut self, msg: &str, len: u64) {
        if let Some(ref multibar) = self.bars.multibar {
            let bar = multibar.add(ProgressBar::new(len));
            self.bar_set_style(Crowbar::Second, CrowbarState::Running);
            bar.set_message(msg.to_owned());
            self.bars.secondbar = Some(bar);
        }
    }

    /// Increment second bar
    pub fn inc_second_bar(&self) {
        if let Some(ref bar) = self.bars.secondbar {
            bar.inc(1);
        }
    }

    /// Adds a spinner labeled key to the multibar, and set mainbar to "rendering"
    pub fn add_spinner_to_multibar(&mut self, key: &str) -> usize {
        if let Some(ref multibar) = self.bars.multibar {
            if let Some(ref mainbar) = self.bars.mainbar {
                mainbar.set_message(t!("ui.rendering"));
            }

            let bar = multibar.add(ProgressBar::new_spinner());
            bar.enable_steady_tick(Duration::from_millis(200));
            bar.set_message(t!("ui.waiting"));
            bar.set_prefix(format!("{key}:"));
            let i = self.bars.spinners.len();
            self.bars.spinners.push(bar);
            self.bar_set_style(Crowbar::Spinner(i), CrowbarState::Running);

            i
        } else {
            0
        }
    }

    pub fn bar_set_message(&self, bar: Crowbar, msg: &str) {
        let bar = match bar {
            Crowbar::Main => {
                if let Some(ref bar) = self.bars.mainbar {
                    bar
                } else {
                    return;
                }
            }
            Crowbar::Second => {
                if let Some(ref bar) = self.bars.secondbar {
                    bar
                } else {
                    return;
                }
            }
            Crowbar::Spinner(i) => {
                if i < self.bars.spinners.len() {
                    &self.bars.spinners[i]
                } else {
                    return;
                }
            }
        };
        bar.set_message(msg.to_owned());
    }

    /// Sets the style of a  bar
    fn bar_set_style(&self, bar: Crowbar, state: CrowbarState) {
        let pb = match bar {
            Crowbar::Main => {
                if let Some(ref bar) = self.bars.mainbar {
                    bar
                } else {
                    return;
                }
            }
            Crowbar::Second => {
                if let Some(ref bar) = self.bars.secondbar {
                    bar
                } else {
                    return;
                }
            }
            Crowbar::Spinner(i) => {
                if i < self.bars.spinners.len() {
                    &self.bars.spinners[i]
                } else {
                    return;
                }
            }
        };
        let emoji = self.bars.emoji;
        let mut style = match bar {
            Crowbar::Second => ProgressStyle::default_bar(),
            _ => ProgressStyle::default_spinner(),
        };

        let color = match state {
            CrowbarState::Running => "yellow",
            CrowbarState::Success => "cyan",
            CrowbarState::Error => "red",
        };
        let tick_chars = match (bar, emoji) {
            (Crowbar::Main, false) | (Crowbar::Spinner(_), false) => "-\\|/",
            (Crowbar::Main, true) => "🕛🕐🕑🕒🕓🕔🕔🕕🕖🕗🕘🕘🕙🕚",
            (Crowbar::Spinner(_), true) => "◐◓◑◒",
            (_, _) => "",
        };
        let end_tick = match (state, emoji) {
            (CrowbarState::Running, _) => "V",
            (CrowbarState::Success, true) => "✔",
            (CrowbarState::Error, true) => "❌",
            (CrowbarState::Success, false) => "V",
            (CrowbarState::Error, false) => "X",
        };
        match bar {
            Crowbar::Second => {
                style = style
                    .template("{bar:40.cyan/blue} {percent:>7} {wide_msg}")
                    .expect("Error in second progress bar style")
                    .progress_chars("##-");
            }
            bar => {
                style = style.tick_chars(&format!("{tick_chars}{end_tick}"));
                match bar {
                    Crowbar::Spinner(_) => {
                        style = style
                            .template(&format!(
                                "{{spinner:.bold.{color}}} {{prefix}} {{wide_msg}}"
                            ))
                            .expect("Error in spinner progress bar style");
                    }
                    _ => {
                        style = style
                            .template(&format!("{{spinner:.bold.{color}}} {{prefix}}{{wide_msg}}"))
                            .expect("Error in progress bar style");
                    }
                };
            }
        }
        pb.set_style(style);
    }
}

impl Drop for Book<'_> {
    fn drop(&mut self) {
        if let Some(ref bar) = self.bars.secondbar {
            bar.finish_and_clear();
        }
        if let Some(ref bar) = self.bars.mainbar {
            bar.finish();
            // let guard = mem::replace(&mut self.bars.guard, None);
            // guard.unwrap().join().unwrap();
        }
    }
}
