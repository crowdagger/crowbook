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

extern crate clap;

use helpers::*;

use crowbook::{Book, BookOptions, InfoLevel, set_lang};
use clap::ArgMatches;
use std::process::exit;
use std::fs::File;
use std::io;
use std::env;


/// Render a book to specific format
fn render_format(book: &mut Book, matches: &ArgMatches, format: &str) -> ! {
    if let Some(file) = matches.value_of("output") {
        match format {
            "epub" => book.options.set("output.epub", file).unwrap(),
            "tex" => book.options.set("output.tex", file).unwrap(),
            "html" => book.options.set("output.html", file).unwrap(),
            "pdf" => book.options.set("output.pdf", file).unwrap(),
            "odt" => book.options.set("output.odt", file).unwrap(),
            "proofread.html" => book.options.set("output.proofread.html", file).unwrap(),
            "proofread.pdf" => book.options.set("output.proofread.pdf", file).unwrap(),
            "proofread.tex" => book.options.set("output.proofead.tex", file).unwrap(),
            "proofread.hml_dir" => book.options.set("output.proofread.html_dir", file).unwrap(),
            _ => unreachable!(),
        };
    }

    let option = match format {
        "epub" => book.options.get_path("output.epub"),
        "tex" => book.options.get_path("output.tex"),
        "html" => book.options.get_path("output.html"),
        "pdf" => book.options.get_path("output.pdf"),
        "odt" => book.options.get_path("output.odt"),
        "proofread.html" => book.options.get_path("output.proofread.html"),
        "proofread.html_dir" => book.options.get_path("output.proofread.html_dir"),
        "proofread.pdf" => book.options.get_path("output.proofread.pdf"),
        "proofread.tex" => book.options.get_path("output.proofread.tex"),
        _ => unreachable!(),
    };
    let result = match option {
        Err(_) => {
            match format {
                "html" => book.render_html(&mut io::stdout()),
                "proofread.html" => book.render_proof_html(&mut io::stdout()),
                "tex" => book.render_tex(&mut io::stdout()),
                "proofread.tex" => book.render_tex(&mut io::stdout()),
                _ => {
                    print_error(&lformat!("No output file specified, and book doesn't specify an \
                                           output file for {}",
                                          format))
                }
            }
        }
        Ok(file) => {
            match format {
                "epub" => book.render_epub(),
                "tex" => {
                    if let Ok(mut f) = File::create(&file) {
                        book.render_tex(&mut f)
                    } else {
                        print_error(&lformat!("Could not create file '{}'", file));
                    }
                }
                "proofread.tex" => {
                    if let Ok(mut f) = File::create(&file) {
                        book.render_proof_tex(&mut f)
                    } else {
                        print_error(&lformat!("Could not create file '{}'", file));
                    }
                }
                "html" => {
                    if let Ok(mut f) = File::create(&file) {
                        book.render_html(&mut f)
                    } else {
                        print_error(&lformat!("Could not create file '{}'", file));
                    }
                }
                "proofread.html" => {
                    if let Ok(mut f) = File::create(&file) {
                        book.render_proof_html(&mut f)
                    } else {
                        print_error(&lformat!("Could not create file '{}'", file));
                    }
                }
                "pdf" => book.render_pdf(),
                "proofread.pdf" => book.render_proof_pdf(),
                "odt" => book.render_odt(),
                _ => unreachable!(),
            }
        }
    };
    match result {
        Err(err) => print_error(&format!("{}", err)),
        Ok(_) => {
            exit(0);
        }
    }
}

pub fn real_main() {
    match env::var("LANG") {
        Ok(val) => {
            if val.starts_with("fr") {
                set_lang("fr");
            } else {
                set_lang("en");
            }
        }
        Err(_) => (),
    }

    let (matches, help, version) = create_matches();

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
        let mut book = Book::new(&[]);
        set_book_options(&mut book, &matches);
        let result = book.get_template(template.as_ref());
        match result {
            Ok(s) => {
                println!("{}", s);
                exit(0);
            }
            Err(_) => print_error(&lformat!("{} is not a valid template name.", template)),
        }
    }

    if matches.is_present("help") {
        println!("{}", help);
        exit(0);
    }

    if matches.is_present("version") {
        println!("{}", version);
        exit(0);
    }

    if matches.is_present("create") {
        create_book(&matches);
    }

    if !matches.is_present("BOOK") {
        print_error(&lformat!("You must pass the file of a book configuration \
                               file.\n\n{}\n\nFor more information try --help.",
                              matches.usage()));
    }



    // ok to unwrap since clap checks it's there
    let s = matches.value_of("BOOK").unwrap();
    let verbosity = if matches.is_present("debug") {
        InfoLevel::Debug
    } else if matches.is_present("verbose") {
        InfoLevel::Warning
    } else if matches.is_present("quiet") {
        InfoLevel::Quiet
    } else {
        InfoLevel::Info
    };

    let book_res = if matches.is_present("single") {
        Book::new_from_markdown_file(s, verbosity, &get_book_options(&matches))
    } else {
        Book::new_from_file(s, verbosity, &get_book_options(&matches))
    };

    match book_res {
        Err(err) => print_error(&format!("{}", err)),
        Ok(mut book) => {
            set_book_options(&mut book, &matches);

            if let Some(format) = matches.value_of("to") {
                render_format(&mut book, &matches, format);
            } else {
                book.render_all();
            }
        }
    }
}
