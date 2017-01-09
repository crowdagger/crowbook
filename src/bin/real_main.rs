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

use crowbook::{Result, Book, BookOptions, InfoLevel};
use crowbook_intl_runtime::set_lang;
use clap::ArgMatches;
use std::process::exit;
use std::io;
use std::env;


/// Render a book to specific format
fn render_format(book: &mut Book, matches: &ArgMatches, format: &str) -> ! {
    let mut key = String::from("output.");
    key.push_str(format);

    let mut stdout = false;
    let mut file = None;
    
    if let Some(f) = matches.value_of("output") {
        if f == "-" {
            stdout = true;
        } else {
            file = Some(String::from(f));
        }
    }

    let res = book.options.get_path(&key);

    let result = match(file, res, stdout) {
        (Some(file), _, _) |
        (None, Ok(file), false) => book.render_format_to_file(format, file),

        (None, Err(_), _) |
        (None, _, true)
        => book.render_format_to(format, &mut io::stdout()),
    };
    
    match result {
        Err(err) => print_error(&format!("{}", err)),
        Ok(_) => {
            exit(0);
        }
    }
}

pub fn try_main() -> Result<()> {
    let lang = get_lang()
        .or_else(|| {
            match env::var("LANG") {
                Ok(val) => {
                    Some(val)
                },
                Err(_) => None,
            }
        });
    if let Some(val) = lang {
        if val.starts_with("fr") {
            set_lang("fr");
        } else {
            set_lang("en");
        }
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
        let mut book = Book::new();
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

    let mut book = Book::new();
    book.set_verbosity(verbosity)
        .set_options(&get_book_options(&matches));

    if matches.is_present("single") {
        if s != "-" {
            book.load_markdown_file(s)?;
        } else {
            book.read_markdown_config(io::stdin())?;
        }
    } else if s != "-" {
        book.load_file(s)?;
    } else {
        book.read_config(io::stdin())?;
    }

    set_book_options(&mut book, &matches);
    if let Some(format) = matches.value_of("to") {
        render_format(&mut book, &matches, format);
    } else {
        book.render_all();
    }

    Ok(())
}

pub fn real_main() {
    if let Err(err) = try_main() {
        print_error(&format!("{}", err));
    }
}
