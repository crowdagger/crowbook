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

use crowbook::{Book,BookOptions};
use clap::ArgMatches;
use std::process::exit;
use std::fs::File;
use std::io;


/// Render a book to specific format
fn render_format(book: &mut Book, matches: &ArgMatches, format: &str) -> ! {
    if let Some(file) = matches.value_of("output") {
        match format {
            "epub" => book.options.set("output.epub", file).unwrap(),
            "tex" => book.options.set("output.tex", file).unwrap(),
            "html" => book.options.set("output.html", file).unwrap(),
            "pdf" => book.options.set("output.pdf", file).unwrap(),
            "odt" => book.options.set("output.odt", file).unwrap(),
            _ => unreachable!()
        }
    }
    
    let option = match format {
        "epub" => book.options.get_path("output.epub"),
        "tex" => book.options.get_path("output.tex"),
        "html" => book.options.get_path("output.html"),
        "pdf" => book.options.get_path("output.pdf"),
        "odt" => book.options.get_path("output.odt"),
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

    if matches.is_present("list-options") {
        println!("{}", BookOptions::description(false));
        exit(0);
    }
    
    if matches.is_present("list-options-md") {
        println!("{}", BookOptions::description(true));
        exit(0);
    }

    if matches.is_present("print-template") {
        let template = matches.value_of("print-template").unwrap();
        let mut book = Book::new();
        set_book_options(&mut book, &matches);
        let result = book.get_template(template.as_ref());
        match result {
            Ok(s) => {
                println!("{}", s);
                exit(0);
            }
            Err(_) => print_error(&format!("{} is not a valid template name.", template)),
        }
    }

    if matches.is_present("create") {
        create_book(&matches);
    }

    if !matches.is_present("BOOK") {
        print_error(&format!("You must pass the file of a book configuration file.\n\n{}\n\nFor more information try --help.",
                            matches.usage()));
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
