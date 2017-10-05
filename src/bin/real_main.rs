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

use helpers::*;

use crowbook::{Result, Book, BookOptions};
use crowbook_intl_runtime::set_lang;
use crowbook::Stats;
use tempdir::TempDir;
use clap::ArgMatches;
use std::process::exit;
use std::io;
use std::io::Read;
use std::env;
use std::fs::File;
use simplelog::{Config, TermLogger, LogLevel, LogLevelFilter, SimpleLogger, WriteLogger};

/// Render a book to specific format
fn render_format(book: &mut Book, matches: &ArgMatches, format: &str) {
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
        Ok(_) => {}
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

    let mut fancy_ui = true;

    let (matches, help, version) = create_matches();

    if matches.is_present("list-options") {
        println!("{}", BookOptions::description(false));
        exit(0);
    }

    if matches.is_present("no-fancy") {
        fancy_ui = false;
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
            Err(_) => print_error_and_exit(&lformat!("{} is not a valid template name.", template)),
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
        print_error_and_exit(&lformat!("You must pass the file of a book configuration \
                               file.\n\n{}\n\nFor more information try --help.",
                              matches.usage()));
    }



    // ok to unwrap since clap checks it's there
    let s = matches.value_of("BOOK").unwrap();

    // Initalize logger
    let mut log_config = Config::default();
    log_config.target = None;
    log_config.location = None;
    log_config.time = None;
    let verbosity = if matches.is_present("verbose") {
        log_config.time = Some(LogLevel::Error);
        log_config.target = Some(LogLevel::Error);
        fancy_ui = false;
        LogLevelFilter::Debug
    } else if matches.is_present("quiet") {
        fancy_ui = false;
        LogLevelFilter::Error
    } else if fancy_ui {
        LogLevelFilter::Error
    } else {
        LogLevelFilter::Info
    };


    let error_dir = TempDir::new("crowbook").unwrap();
    let error_path = "error.log";
    if fancy_ui {
        let errors = File::create(error_dir.path().join(error_path)).unwrap();
        let _ = WriteLogger::init(verbosity, log_config, errors);
    } else {
        if TermLogger::init(verbosity, log_config).is_err() {
            // If it failed, not much we can do, we just won't display log
            let _ = SimpleLogger::init(verbosity, log_config);
        }
    }

    {
        let mut book = Book::new();
        if fancy_ui {
            book.add_progress_bar();
        }
        book.set_options(&get_book_options(&matches));
        
        {
            let res = if matches.is_present("single") {
                if s != "-" {
                    book.load_markdown_file(s)
                } else {
                    book.read_markdown_config(io::stdin())
                }
            } else if s != "-" {
                book.load_file(s)
            } else {
                book.read_config(io::stdin())
            }.map(|_| ());
            
            match res {
                Ok(..) => {},
                Err(err) => {
                    book.set_error(&format!("{}", err));
                    return Err(err);
                }
            }
        }
        
        set_book_options(&mut book, &matches);
        
        if matches.is_present("stats") {
            let stats = Stats::new(&book);
            println!("{}", stats);
            exit(0);
        }
        
        if let Some(format) = matches.value_of("to") {
            render_format(&mut book, &matches, format);
        } else {
            book.render_all();
        }
    }
    if fancy_ui {
        let mut errors = String::new();
        let mut file = File::open(error_dir.path().join(error_path)).unwrap();
        file.read_to_string(&mut errors).unwrap();
        if !errors.is_empty() {
            print_warning();
            for line in errors.lines() {
                print_error(line);
            }
        }
    }

    
    Ok(())
}

pub fn real_main() {
    display_header();
    if let Err(err) = try_main() {
        print_error_and_exit(&format!("{}", err));
    }
}
