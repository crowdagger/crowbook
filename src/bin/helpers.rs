// Copyright (C) 2016-2023Élisabeth HENRY.
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

use clap::{Arg, ArgAction, ArgMatches, Command};
use console::style;
use crowbook::Book;
use rust_i18n::t;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

static BIRD: &str = "🐦 ";
static ERROR: &str = "💣 ";
static WARNING: &str = "⚠️ ";
static BOOK: &str = "📚 ";

pub fn print_warning(msg: &str, emoji: bool) {
    if emoji {
        eprint!("{}", style(WARNING).yellow());
    }
    eprintln!("{} {}", style(t!("error.warning")).bold().yellow(), msg);
}

/// Prints an error
pub fn print_error(s: &str, emoji: bool) {
    if emoji {
        eprint!("{}", style(ERROR).red());
    }
    eprintln!("{} {}", style(t!("error.error")).bold().red(), s);
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
            return Some(arg);
        } else if arg == "--lang" || arg == "-L" {
            found = true;
        }
    }
    None
}

/// Gets the book options in a (key, value) list, or print an error
pub fn get_book_options(matches: &ArgMatches) -> Vec<(&str, &str)> {
    let mut output = vec![];
    if let Some(iter) = matches.get_many::<String>("set") {
        let v: Vec<_> = iter.collect();
        if v.len() % 2 != 0 {
            print_error_and_exit(
                &t!("error.odd_number"),
                false,
            );
        }

        for i in 0..v.len() / 2 {
            let key = v[i * 2];
            let value = v[i * 2 + 1];
            output.push((key.as_str(), value.as_str()));
        }
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
            print_error_and_exit(&t!("error.set_key", key = key, error = err), false);
        }
        output.push_str(&format!("{key}: {value}\n"));
    }
    output
}

/// create a book file with the command line arguments
/// and exit the process at the end
pub fn create_book(matches: &ArgMatches) -> ! {
    let mut f: Box<dyn Write> = if let Some(book) = matches.get_one::<String>("BOOK") {
        if fs::metadata(book).is_ok() {
            print_error_and_exit(
                &t!("error.create", file = book),
                false,
            );
        }
        Box::new(fs::File::create(book).unwrap())
    } else {
        Box::new(io::stdout())
    };

    if let Some(values) = matches.get_many::<String>("files") {
        if matches.get_many::<String>("set").is_some() {
            let mut book = Book::new();
            let s = set_book_options(&mut book, matches);
            f.write_all(s.as_bytes()).unwrap();
        } else {
            f.write_all(
                t!("msg.default_book")
                .as_bytes(),
            )
            .unwrap();
        }
        f.write_all(t!("msg.chapter_list").as_bytes())
            .unwrap();
        for file in values {
            f.write_all(format!("+ {file}\n").as_bytes()).unwrap();
        }
        if let Some(s) = matches.get_one::<String>("BOOK") {
            println!(
                "{}",
                t!("msg.created", file = s)
            );
        }
        exit(0);
    } else {
        unreachable!(); // because Clap takes care of it
    }
}

pub fn create_matches() -> ArgMatches {
    app().get_matches()
}

// in its own function for testing purpose
fn app() -> clap::Command {
    lazy_static! {
        static ref ABOUT: String = t!("cmd.about");
        static ref SINGLE: String = t!("cmd.single");
        static ref EMOJI: String = t!("cmd.emoji");
        static ref VERBOSE: String = t!("cmd.verbose");
        static ref QUIET: String = t!("cmd.quiet");
        static ref CREATE: String = t!("cmd.create");
        static ref AUTOGRAPH: String = t!("cmd.autograph");
        static ref OUTPUT: String = t!("cmd.output");
        static ref LANG: String = t!("cmd.lang");
        static ref TO: String = t!("cmd.to");
        static ref SET: String = t!("cmd.set");
        static ref NO_FANCY: String = t!("cmd.no_fancy");
        static ref LIST_OPTIONS: String = t!("cmd.list_options");
        static ref LIST_OPTIONS_MD: String = t!("cmd.list_options_md");
        static ref PRINT_TEMPLATE: String = t!("cmd.template");
        static ref BOOK: String = t!("cmd.book");
        static ref STATS: String = t!("cmd.stats");
        static ref TEMPLATE: String = t!("clap.template");
    }

    let app = Command::new("crowbook")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Élisabeth Henry <liz.henry@ouvaton.org>")
        .hide_possible_values(true)
        .about(ABOUT.as_str())
        .arg(
            Arg::new("force-emoji")
                .short('f')
                .long("force-emoji")
                .action(ArgAction::SetTrue)
                .help(EMOJI.as_str()),
        )
        .arg(
            Arg::new("single")
                .short('s')
                .long("single")
                .action(ArgAction::SetTrue)
                .help(SINGLE.as_str()),
        )
        .arg(
            Arg::new("no-fancy")
                .short('n')
                .long("no-fancy")
                .action(ArgAction::SetTrue)
                .help(NO_FANCY.as_str()),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help(VERBOSE.as_str()),
        )
        .arg(
            Arg::new("autograph")
                .short('a')
                .long("autograph")
                .action(ArgAction::SetTrue)
                .help(AUTOGRAPH.as_str()),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help(QUIET.as_str())
                .conflicts_with("verbose"),
        )
        .arg(
            Arg::new("files")
                .short('c')
                .long("create")
                .action(ArgAction::Set)
                .num_args(1..)
                .help(CREATE.as_str()),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .action(ArgAction::Set)
                .num_args(1)
                .help(OUTPUT.as_str())
                .requires("to"),
        )
        .arg(
            Arg::new("to")
                .short('t')
                .long("to")
                .action(ArgAction::Set)
                .value_parser([
                    "epub",
                    "pdf",
                    "html",
                    "tex",
                    "odt",
                    "html.dir",
                ])
                .help(TO.as_str()),
        )
        .arg(
            Arg::new("set")
                .long("set")
                .action(ArgAction::Set)
                .num_args(2..)
                .help(SET.as_str()),
        )
        .arg(
            Arg::new("list-options")
                .short('l')
                .long("list-options")
                .action(ArgAction::SetTrue)
                .help(LIST_OPTIONS.as_str()),
        )
        .arg(
            Arg::new("list-options-md")
                .long("list-options-md")
                .action(ArgAction::SetTrue)
                .help(LIST_OPTIONS_MD.as_str())
                .hide(true),
        )
        .arg(
            Arg::new("lang")
                .short('L')
                .long("lang")
                .action(ArgAction::Set)
                .num_args(1)
                .help(LANG.as_str()),
        )
        .arg(
            Arg::new("print-template")
                .long("print-template")
                .action(ArgAction::Set)
                .num_args(1)
                .help(PRINT_TEMPLATE.as_str()),
        )
        .arg(
            Arg::new("stats")
                .short('S')
                .long("stats")
                .action(ArgAction::SetTrue)
                .help(STATS.as_str()),
        )
        .arg(
            Arg::new("BOOK")
                .index(1)
                .action(ArgAction::Set)
                .help(BOOK.as_str()),
        )
        .help_template(TEMPLATE.as_str());

    app
}

#[cfg(test)]
mod tests {
    use super::app;

    #[test]
    fn verify_app() {
        app().debug_assert();
    }
}
