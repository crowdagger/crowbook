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

use crowbook::Book;
use clap::{App, Arg,  ArgMatches, AppSettings};
use console::style;

use std::io::{self, Write};
use std::process::exit;
use std::fs;
use std::env;


static BIRD: &str = "🐦 ";
static ERROR: &str = "💣 ";
static WARNING: &str = "⚠️ ";
static BOOK: &str = "📚 ";

pub fn print_warning(msg: &str, emoji: bool) {
    if emoji {
        eprint!("{}", style(WARNING).yellow());
    }
    eprintln!("{} {}",
              style(lformat!("WARNING")).bold().yellow(),
              msg);
}

/// Prints an error
pub fn print_error(s: &str, emoji: bool) {
    if emoji {
        eprint!("{}", style(ERROR).red());
    }
    eprintln!("{} {}",
              style(lformat!("ERROR")).bold().red(),
              s);
}

/// Prints an error on stderr and exit the program
pub fn print_error_and_exit(s: &str, emoji: bool) -> ! {
    print_error(s, emoji);
    exit(0);
}


/// Display version number
pub fn display_header(emoji: bool) {
    if emoji {
        eprint!("{}", style(BIRD).magenta());
    }
    eprint!("{}", style("CROWBOOK ").magenta().bold());
    if emoji {
        eprint!("{}", style(BOOK).magenta());
    }
    eprintln!("{}", style(env!("CARGO_PKG_VERSION")).blue());
}

/// Return the --lang option, if it is set
pub fn get_lang() -> Option<String> {
    let mut found = false;
    for arg in env::args() {
        if found {
            return Some(arg.clone());
        } else if arg == "--lang" || arg == "-L" {
            found = true;
        }
    }
    None
}


/// Gets the book options in a (key, value) list, or print an error
pub fn get_book_options<'a>(matches: &'a ArgMatches) -> Vec<(&'a str, &'a str)> {
    let mut output = vec![];
    if let Some(iter) = matches.values_of("set") {
        let v: Vec<_> = iter.collect();
        if v.len() % 2 != 0 {
            print_error_and_exit(&lformat!("An odd number of arguments was passed to --set, but it takes \
                                   a list of key value pairs."), false);
        }

        for i in 0..v.len() / 2 {
            let key = v[i * 2];
            let value = v[i * 2 + 1];
            output.push((key, value));
        }
    }
    if matches.is_present("proofread") {
        output.push(("proofread", "true"));
    }
    output
}


/// Sets the book options according to command line arguments
/// Also print these options to a string, so it can be used at
/// the creation of a book to check that parameters are OK and 
/// then print them to file
pub fn set_book_options(book: &mut Book, matches: &ArgMatches) -> String {
    let mut output = String::new();
    let options = get_book_options(matches);

    for (key, value) in options {
        let res = book.options.set(key, value);
        if let Err(err) = res {
            print_error_and_exit(&lformat!("Error in setting key {}: {}", key, err), false);
        }
        output.push_str(&format!("{}: {}\n", key, value));
    }
    output
}

/// create a book file with the command line arguments
/// and exit the process at the end
pub fn create_book(matches: &ArgMatches) -> ! {
    let mut f: Box<Write> = if let Some(book) = matches.value_of("BOOK") {
        if fs::metadata(book).is_ok() {
            print_error_and_exit(&lformat!("Could not create file {}: it already exists!", book), false);
        }
        Box::new(fs::File::create(book).unwrap())
    } else {
        Box::new(io::stdout())
    };

    if let Some(values) = matches.values_of("create") {
        if matches.is_present("set") {
            let mut book = Book::new();
            let s = set_book_options(&mut book, matches);
            f.write_all(s.as_bytes()).unwrap();
        } else {
            f.write_all(lformat!("author: Your name
title: Your title
lang: en

## Output formats

# Uncomment and fill to generate files
# output.html: some_file.html
# output.epub: some_file.epub
# output.pdf: some_file.pdf

# Or uncomment the following to generate PDF, HTML and EPUB files based on this file's name
# output: [pdf, epub, html]

# Uncomment and fill to set cover image (for EPUB)
# cover: some_cover.png\n").as_bytes())
                .unwrap();
        }
        f.write_all(lformat!("\n## List of chapters\n").as_bytes()).unwrap();
        for file in values {
            f.write_all(format!("+ {}\n", file).as_bytes()).unwrap();
        }
        if let Some(s) = matches.value_of("BOOK") {
            println!("{}",
                     lformat!("Created {}, now you'll have to complete it!", s));
        }
        exit(0);
    } else {
        unreachable!(); // because Clap takes care of it
    }
}

pub fn create_matches<'a>() -> (ArgMatches<'a>, String, String) {
    lazy_static! {
        static ref HELP: String = lformat!("Print help information");
        static ref VERSION: String = lformat!("Print version information");
        static ref ABOUT: String = lformat!("Render a Markdown book in EPUB, PDF or HTML.");
        static ref SINGLE: String = lformat!("Use a single Markdown file instead of a book configuration file");
        static ref EMOJI: String = lformat!("Force emoji usage even if it might not work on your system");
        static ref VERBOSE: String = lformat!("Print warnings in parsing/rendering");
        static ref QUIET: String = lformat!("Don't print info/error messages");
        static ref PROOFREAD: String = lformat!("Enable proofreading");
        static ref CREATE: String = lformat!("Create a new book with existing Markdown files");
        static ref AUTOGRAPH: String = lformat!("Prompts for an autograph for this book");
        static ref OUTPUT: String = lformat!("Specify output file");
        static ref LANG: String = lformat!("Set the runtime language used by Crowbook");
        static ref TO: String = lformat!("Generate specific format");
        static ref SET: String = lformat!("Set a list of book options");
        static ref NO_FANCY: String = lformat!("Disably fancy UI");
        static ref LIST_OPTIONS: String = lformat!("List all possible options");
        static ref LIST_OPTIONS_MD: String = lformat!("List all possible options, formatted in Markdown");
        static ref PRINT_TEMPLATE: String = lformat!("Prints the default content of a template");
        static ref BOOK: String = lformat!("File containing the book configuration file, or a Markdown file when called with --single");
        static ref STATS: String = lformat!("Print some project statistics");
        static ref TEMPLATE: String = lformat!("\
{{bin}} {{version}} by {{author}}
{{about}}

USAGE:
    {{usage}}

FLAGS:
{{flags}}

OPTIONS:
{{options}}

ARGS:
{{positionals}}
");
    }


    let app = App::new("crowbook")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Élisabeth Henry <liz.henry@ouvaton.org>")
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::HidePossibleValuesInHelp)
        .about(ABOUT.as_str())
        .arg(Arg::from_usage("-☺️, --force-emoji").help(EMOJI.as_str()))
        .arg(Arg::from_usage("-s, --single").help(SINGLE.as_str()))
        .arg(Arg::from_usage("-n, --no-fancy").help(NO_FANCY.as_str()))
        .arg(Arg::from_usage("-v, --verbose").help(VERBOSE.as_str()))
        .arg(Arg::from_usage("-a, --autograph").help(AUTOGRAPH.as_str()))
        .arg(Arg::from_usage("-q, --quiet")
            .help(QUIET.as_str())
            .conflicts_with("verbose"))
        .arg(Arg::from_usage("-h, --help").help(HELP.as_str()))
        .arg(Arg::from_usage("-V, --version").help(VERSION.as_str()))
        .arg(Arg::from_usage("-p, --proofread").help(PROOFREAD.as_str()))
        .arg(Arg::from_usage("-c, --create [FILES]...").help(CREATE.as_str()))
        .arg(Arg::from_usage("-o, --output [FILE]")
            .help(OUTPUT.as_str())
            .requires("to"))
        .arg(Arg::from_usage("-t, --to [FORMAT]")
            .help(TO.as_str())
            .possible_values(&["epub",
                               "pdf",
                               "html",
                               "tex",
                               "odt",
                               "html.dir",
                               "proofread.html",
                               "proofread.html.dir",
                               "proofread.pdf",
                               "proofread.tex"]))
        .arg(Arg::from_usage("--set [KEY_VALUES]")
            .help(SET.as_str())
            .min_values(2))
        .arg(Arg::from_usage("-l --list-options").help(LIST_OPTIONS.as_str()))
        .arg(Arg::from_usage("--list-options-md")
            .help(LIST_OPTIONS_MD.as_str())
             .hidden(true))
        .arg(Arg::from_usage("-L --lang [LANG]")
             .help(LANG.as_str()))
        .arg(Arg::from_usage("--print-template [TEMPLATE]").help(PRINT_TEMPLATE.as_str()))
        .arg(Arg::from_usage("--stats -S").help(STATS.as_str()))
        .arg(Arg::with_name("BOOK")
            .index(1)
            .help(BOOK.as_str()))
        .template(TEMPLATE.as_str());

    // Write help and version now since it `app` is moved when `get_matches` is run
    let mut help = vec![];
    app.write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();
    let mut version = vec![];
    app.write_version(&mut version).unwrap();
    let version = String::from_utf8(version).unwrap();

    let matches = app.get_matches();

    pre_check(&matches);
    (matches, help, version)
}


/// Pre-check the matches to see if there isn't illegal options not detected by clap
fn pre_check(matches: &ArgMatches) {
    if matches.is_present("files") && !matches.is_present("create") {
        print_error_and_exit(&lformat!("A list of additional files is only valid with the --create \
                               option."), false);
    }
}
