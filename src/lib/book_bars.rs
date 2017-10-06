// Copyright (C) 2016, 2017 Élisabeth HENRY.
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

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::sync::Arc;
use std::thread;

impl Book {
    /// Adds a progress bar where where info should be written.
    ///
    /// See [indicatif doc](https://docs.rs/indicatif) for more information.
    pub fn private_add_progress_bar(&mut self) {
        let multibar = Arc::new(MultiProgress::new());
        self.multibar = Some(multibar.clone());
        let b = self.multibar
            .as_ref()
            .unwrap()
            .add(ProgressBar::new_spinner());
        let sty = ProgressStyle::default_spinner()
            .tick_chars("🕛🕐🕑🕒🕓🕔🕔🕕🕖🕗🕘🕘🕙🕚V")
//            .tick_chars("/|\\-V")
            .template("{spinner:.dim.bold.yellow} {prefix} {wide_msg}");
        b.set_style(sty);
        b.enable_steady_tick(200);
        self.mainbar = Some(b);
        self.guard = Some(thread::spawn(move || {
            if let Err(_) = multibar.join() {
                error!("{}", lformat!("could not display fancy UI, try running crowbook with --no-fancy"));
            }
        }));
    }

    /// Sets an error message to the progress bar, if it is set
    pub fn private_set_error(&self, msg: &str) {
        if let Some(ref mainbar) = self.mainbar {
            let sty = ProgressStyle::default_spinner()
            .tick_chars("/|\\-X")
                .template("{spinner:.dim.bold.red} {wide_msg}");
            mainbar.set_style(sty);
            mainbar.set_message(msg);
        }
    }


    /// Sets a finished message to the progress bar, if it is set
    pub fn set_finished(&self, msg: &str) {
        if let Some(ref bar) = self.mainbar {
            let sty = ProgressStyle::default_spinner()
                .tick_chars("/|\\-V")
                .template("{spinner:.dim.bold.cyan} {wide_msg}");
            bar.set_style(sty);
            bar.set_message(msg);
        }

    }

    /// Adds a secondary progress bar to display progress of book parsing
    pub fn add_second_bar(&mut self, msg: &str, len: u64)  {
        if let Some(ref multibar) = self.multibar {
            let bar = multibar.add(ProgressBar::new(len));
            bar.set_style(ProgressStyle::default_bar()
                          .template("{bar:40.cyan/blue} {percent:>7}% {msg}")
                          .progress_chars("##-"));
            bar.set_message(msg);
            self.secondbar = Some(bar);
        }
    }

    /// Finish secondary prograss bar
    pub fn finish_second_bar(&self) {
        if let Some(ref bar) = self.secondbar {
            bar.finish_and_clear();
        }
    }

    /// Increment second bar
    pub fn inc_second_bar(&self) {
        if let Some(ref bar) = self.secondbar {
            bar.inc(1);
        }
    }

    /// Adds a spinner labeled key to the multibar, and set mainbar to "rendering"
    pub fn add_spinner_to_multibar(&self, multibar: &MultiProgress, key: &str) -> ProgressBar {
        if let Some(ref mainbar) = self.mainbar {
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
        bar
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
