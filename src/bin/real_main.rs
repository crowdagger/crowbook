// Copyright (C) 2016-2023 Élisabeth HENRY.
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

use crate::helpers::*;

use crowbook::Stats;
use crowbook::{Book, BookOptions, Result};

use clap::ArgMatches;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger, TermLogger, WriteLogger};
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::exit;
use yaml_rust::Yaml;
use rust_i18n::t;


/// Render a book to specific format
fn render_format(book: &mut Book, emoji: bool, matches: &ArgMatches, format: &str) {
    let mut key = String::from("output.");
    key.push_str(format);

    let mut stdout = false;
    let mut file = None;

    if let Some(f) = matches.get_one::<String>("output") {
        if f.as_str() == "-" {
            stdout = true;
        } else {
            file = Some(String::from(f));
        }
    }

    let res = book.options.get_path(&key);

    let result = match (file, res, stdout) {
        (Some(file), _, _) | (None, Ok(file), false) => book.render_format_to_file(format, file),

        (None, Err(_), _) | (None, _, true) => book.render_format_to(format, &mut io::stdout()),
    };

    if let Err(err) = result {
        print_error(&format!("{err}"), emoji)
    }
}

pub fn try_main() -> Result<()> {
    let lang = get_lang().or_else(|| match env::var("LANG") {
        Ok(val) => Some(val),
        Err(_) => None,
    });
    if let Some(val) = lang {
        if val.starts_with("fr") {
            rust_i18n::set_locale("fr");
        } else {
            rust_i18n::set_locale("en");
        }
    }

    let mut fancy_ui = true;
    let mut emoji = console::Term::stderr().features().wants_emoji();

    let matches = create_matches();

    if matches.get_flag("force-emoji") {
        emoji = true;
    }

    if !matches.get_flag("quiet") {
        display_header(emoji);
    }

    if matches.get_flag("list-options") {
        println!("{}", BookOptions::description(false));
        exit(0);
    }

    if matches.get_flag("no-fancy") || matches.get_flag("stats") {
        fancy_ui = false;
        emoji = false;
    }

    if matches.get_flag("list-options-md") {
        println!("{}", BookOptions::description(true));
        exit(0);
    }

    if let Some(template) = matches.get_one::<String>("print-template") {
        let mut book = Book::new();
        set_book_options(&mut book, &matches);
        let result = book.get_template(template.as_ref());
        match result {
            Ok(s) => {
                println!("{s}");
                exit(0);
            }
            Err(_) => print_error_and_exit(
                &t!("error.invalid_template", template = template),
                emoji,
            ),
        }
    }

    if matches.get_many::<String>("files").is_some() {
        create_book(&matches);
    }
    let book = matches.get_one::<String>("BOOK");
    if book.is_none() {
        print_error_and_exit(
            &t!("error.no_file"),
            emoji,
        );
    }

    // ok to unwrap since clap checks it's there
    let &s = book.as_ref().unwrap();

    // Initalize logger
    let mut builder = ConfigBuilder::new();
    builder.set_target_level(LevelFilter::Off);
    builder.set_location_level(LevelFilter::Off);
    builder.set_time_level(LevelFilter::Off);
    let verbosity = if matches.get_flag("verbose") && !matches.get_flag("stats") {
        builder.set_time_level(LevelFilter::Error);
        builder.set_target_level(LevelFilter::Error);
        fancy_ui = false;
        LevelFilter::Debug
    } else if matches.get_flag("quiet") {
        fancy_ui = false;
        LevelFilter::Error
    } else if fancy_ui {
        LevelFilter::Warn
    } else {
        LevelFilter::Info
    };
    let log_config = builder.build();

    let error_dir = tempfile::tempdir().expect("Could not create temporary directory");
    let error_path = "error.log";
    if fancy_ui {
        let errors = File::create(error_dir.path().join(error_path)).unwrap();
        let _ = WriteLogger::init(verbosity, log_config, errors);
    } else if TermLogger::init(
        verbosity,
        log_config.clone(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .is_err()
    {
        // If it failed, not much we can do, we just won't display log
        let _ = SimpleLogger::init(verbosity, log_config);
    }

    {
        let mut book = Book::new();
        if matches.get_flag("autograph") {
            println!("{}", &t!("msg.autograph"));
            let mut autograph = String::new();
            match io::stdin().read_to_string(&mut autograph) {
                Ok(_) => {
                    book.options
                        .set_yaml(
                            Yaml::String("autograph".to_string()),
                            Yaml::String(autograph),
                        )
                        .unwrap();
                }
                Err(_) => print_error(&t!("error.autograph") , emoji),
            }
        }

        if fancy_ui {
            book.add_progress_bar(emoji);
        }
        book.set_options(&get_book_options(&matches));

        {
            let res = if matches.get_flag("single") {
                if s != "-" {
                    book.load_markdown_file(s)
                } else {
                    book.read_markdown_config(io::stdin())
                }
            } else if s != "-" {
                book.load_file(s)
            } else {
                book.read_config(io::stdin())
            }
            .map(|_| ());

            match res {
                Ok(..) => {}
                Err(err) => {
                    book.set_error(&format!("{err}"));
                    return Err(err);
                }
            }
        }

        set_book_options(&mut book, &matches);

        if matches.get_flag("stats") {
            let stats = Stats::new(&book, matches.get_flag("verbose"));
            println!("{stats}");
            exit(0);
        }

        if let Some(format) = matches.get_one::<String>("to") {
            render_format(&mut book, emoji, &matches, format);
        } else {
            book.render_all();
        }
    }
    if fancy_ui {
        let mut errors = String::new();
        let mut file = File::open(error_dir.path().join(error_path)).unwrap();
        file.read_to_string(&mut errors).unwrap();
        if !errors.is_empty() {
            print_warning(
                &t!("error.occurred"),
                emoji,
            );
            // Non-efficient dedup algorithm but we need to keep the order
            let mut lines: Vec<String> = vec![];
            for line in errors.lines() {
                let mut contains = false;
                for l in &lines {
                    if l == line {
                        contains = true;
                        break;
                    }
                }
                if !contains {
                    lines.push(line.to_string());
                }
            }
            for line in &lines {
                if line.starts_with("[ERROR]") {
                    let line = &line[8..];
                    print_error(line, emoji);
                } else if line.starts_with("[WARN]") {
                    let line = &line[7..];
                    print_warning(line, emoji);
                }
            }
        }
    }

    Ok(())
}

pub fn real_main() {
    if let Err(err) = try_main() {
        print_error_and_exit(&format!("{err}"), false);
    }
}
