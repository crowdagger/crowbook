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

// Progress bars implementation. Moved into a different file so it is possible
// to make some dependencies (incidacitf) optional.

use book::Book;
use error::{Result, Error, Source};
use misc;

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

use std::sync::Arc;
use std::thread;
use std::mem;
use std::path::{Path, PathBuf};

/// Store the progress bars needed for the book
pub struct Bars {
    /// Container for the progress bars
    pub multibar: Option<Arc<MultiProgress>>,
    /// Main progress bar (actually a spinner)
    pub mainbar: Option<ProgressBar>,
    /// Secondary bar
    pub secondbar: Option<ProgressBar>,
    /// Guard for thread 
    pub guard: Option<thread::JoinHandle<()>>,
    /// Spinners for each renderier
    pub spinners: Vec<ProgressBar>,
}

impl Bars {
    /// Create a new bars storage
    pub fn new() -> Bars {
        Bars {
            multibar: None,
            mainbar: None,
            secondbar: None,
            guard: None,
            spinners: vec![],
        }
    }
}

impl Book {
    /// Adds a progress bar where where info should be written.
    ///
    /// See [indicatif doc](https://docs.rs/indicatif) for more information.
    pub fn private_add_progress_bar(&mut self) {
        let multibar = Arc::new(MultiProgress::new());
        self.bars.multibar = Some(multibar.clone());
        let b = self.bars.multibar
            .as_ref()
            .unwrap()
            .add(ProgressBar::new_spinner());
        let sty = ProgressStyle::default_spinner()
            .tick_chars("🕛🕐🕑🕒🕓🕔🕔🕕🕖🕗🕘🕘🕙🕚V")
//            .tick_chars("/|\\-V")
            .template("{spinner:.dim.bold.yellow} {prefix} {wide_msg}");
        b.set_style(sty);
        b.enable_steady_tick(200);
        self.bars.mainbar = Some(b);
        self.bars.guard = Some(thread::spawn(move || {
            if let Err(_) = multibar.join() {
                error!("{}", lformat!("could not display fancy UI, try running crowbook with --no-fancy"));
            }
        }));
    }

    /// Sets an error message to the progress bar, if it is set
    pub fn private_set_error(&self, msg: &str) {
        if let Some(ref mainbar) = self.bars.mainbar {
            let sty = ProgressStyle::default_spinner()
            .tick_chars("/|\\-X")
                .template("{spinner:.dim.bold.red} {wide_msg}");
            mainbar.set_style(sty);
            mainbar.set_message(msg);
        }
    }


    /// Sets a finished message to the progress bar, if it is set
    pub fn set_finished(&self, msg: &str) {
        if let Some(ref bar) = self.bars.mainbar {
            let sty = ProgressStyle::default_spinner()
                .tick_chars("/|\\-V")
                .template("{spinner:.dim.bold.cyan} {wide_msg}");
            bar.set_style(sty);
            bar.set_message(msg);
        }

    }

    /// Adds a secondary progress bar to display progress of book parsing
    pub fn add_second_bar(&mut self, msg: &str, len: u64)  {
        if let Some(ref multibar) = self.bars.multibar {
            let bar = multibar.add(ProgressBar::new(len));
            bar.set_style(ProgressStyle::default_bar()
                          .template("{bar:40.cyan/blue} {percent:>7}% {msg}")
                          .progress_chars("##-"));
            bar.set_message(msg);
            self.bars.secondbar = Some(bar);
        }
    }

    /// Finish secondary prograss bar
    pub fn finish_second_bar(&self) {
        if let Some(ref bar) = self.bars.secondbar {
            bar.finish_and_clear();
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
                mainbar.set_message(&lformat!("Rendering..."));
            }
            
            let bar = multibar.add(ProgressBar::new_spinner());
            let sty = ProgressStyle::default_spinner()
                .tick_chars("/|\\-X")
                .template(&format!("{{spinner:.dim.bold.yellow}} {format}: {{wide_msg:.yellow}}",
                                   format = key));
            bar.set_style(sty);
            bar.enable_steady_tick(200);
            bar.set_message(&lformat!("waiting..."));
            bar.tick();
            self.bars.spinners.push(bar);
            return self.bars.spinners.len() - 1;
        } else {
            0
        }
    }

    pub fn mainbar_set_message(&self, msg: &str) {
        if let Some(ref bar) = self.bars.mainbar {
            bar.set_message(msg);
            bar.tick();
        } 
    }

    pub fn secondbar_set_message(&self, msg: &str) {
        if let Some(ref bar) = self.bars.secondbar {
            bar.set_message(msg);
            bar.tick();
        } 
    }

    pub fn nth_bar_set_message(&self, bar: usize, msg: &str) {
        if bar < self.bars.spinners.len() {
            let bar = &self.bars.spinners[bar];
            bar.set_message(msg);
            bar.tick();
        }
    }

    // Finish a spinner with an error message
    pub fn finish_nth_spinner_error(&self, bar: usize, key: &str, msg: &str) {
        if bar < self.bars.spinners.len() {
            let bar = &self.bars.spinners[bar];
            bar.set_style(ProgressStyle::default_spinner()
                          .tick_chars("/|\\-X")
                          .template(&format!("{{spinner:.dim.bold.red}} {format}: {{wide_msg:.red}}",
                                             format = key)));
            bar.finish_with_message(msg);
        }
    }

    // Finish a spinner with success message
    pub fn finish_nth_spinner_success(&self, bar: usize, key: &str, msg: &str) {
        if bar < self.bars.spinners.len() {
            let bar = &self.bars.spinners[bar];
            let sty = ProgressStyle::default_spinner()
                .tick_chars("/|\\-V")
                .template(&format!("{{spinner:.dim.bold.cyan}} {format}: {{wide_msg:.cyan}}",
                                   format = key));
            bar.set_style(sty);
            bar.finish_with_message(msg);
        }
    }


    // Finish a spinner with an error message
    pub fn finish_spinner_error(&self, bar: &ProgressBar, key: &str, msg: &str) {
        bar.set_style(ProgressStyle::default_spinner()
                      .tick_chars("/|\\-X")
                      .template(&format!("{{spinner:.dim.bold.red}} {format}: {{wide_msg:.red}}",
                                         format = key)));
        bar.finish_with_message(msg);
    }

    
    // Finish a spinner with success message
    pub fn finish_spinner_success(&self, bar: &ProgressBar, key: &str, msg: &str) {
        let sty = ProgressStyle::default_spinner()
            .tick_chars("/|\\-V")
            .template(&format!("{{spinner:.dim.bold.cyan}} {format}: {{wide_msg:.cyan}}",
                               format = key));
        bar.set_style(sty);
        bar.finish_with_message(msg);
    }
}

impl Drop for Book {
    fn drop(&mut self) {
        if let Some(ref bar) = self.bars.mainbar {
            bar.finish();
            let guard = mem::replace(&mut self.bars.guard, None);
            guard.unwrap()
                .join()
                .unwrap();
        }
    }
}
