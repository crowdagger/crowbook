// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

extern crate crowbook;
extern crate clap;

mod helpers;
use helpers::*;

use crowbook::{Book};
use clap::ArgMatches;
use std::process::exit;
use std::fs::File;
use std::io;


/// Render a book to specific format
fn render_format(book: &mut Book, matches: &ArgMatches, format: &str) -> ! {
    if let Some(file) = matches.value_of("output") {
        match format {
            "epub" => book.set_option("output.epub", file).unwrap(),
            "tex" => book.set_option("output.tex", file).unwrap(),
            "html" => book.set_option("output.html", file).unwrap(),
            "pdf" => book.set_option("output.pdf", file).unwrap(),
            "odt" => book.set_option("output.odt", file).unwrap(),
            _ => unreachable!()
        }
    }
    
    let option = match format {
        "epub" => book.get_str("output.epub"),
        "tex" => book.get_str("output.tex"),
        "html" => book.get_str("output.html"),
        "pdf" => book.get_str("output.pdf"),
        "odt" => book.get_str("output.odt"),
        _ => unreachable!()
    };
    let result = match option {
        Err(_) => {
            match format {
                "html" => book.render_html(&mut io::stdout()),
                "tex" => book.render_tex(&mut io::stdout()),
                _ => print_error(&format!("No output file specified, and book doesn't specify an output file for {}", format)),
            }
        },
        Ok(file) => {
            match format {
                "epub" => book.render_epub(),
                "tex" => {
                    let mut f = File::create(file).unwrap();
                    book.render_tex(&mut f)
                },
                "html" => {
                    let mut f = File::create(file).unwrap();
                    book.render_html(&mut f)
                },
                "pdf" => book.render_pdf(),
                "odt" => book.render_odt(),
                _ => unreachable!()
            }
        }
    };
    match result {
        Err(err) => print_error(&format!("{}", err)),
        Ok(_) => {
                    println!("crowbook terminated successfully");
            exit(0);
        },
    }
}


fn main() {
    let matches = create_matches();

    if matches.is_present("create") {
        create_book(&matches);
    }

    // ok to unwrap since clap checks it's there
    let s = matches.value_of("BOOK").unwrap();
    let verbose = matches.is_present("verbose");
    match Book::new_from_file(s, verbose) {
        Err(err) => print_error(&format!("{}", err)),
        Ok(mut book) => {
            set_book_options(&mut book, &matches);
            
            if let Some(format) = matches.value_of("to") {
                render_format(&mut book, &matches, format);
            } else {
                match book.render_all() {
                    Err(err) => print_error(&format!("{}", err)),
                    Ok(_) => {
                        println!("crowbook terminated successfully");
                        exit(0);
                    }
                }
            }
        }
    }
}
