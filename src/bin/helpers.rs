use crowbook::Book;
use clap::{App, Arg, Format, ArgMatches, AppSettings};

use std::io::{self, Write};
use std::process::exit;
use std::fs;
use std::env;


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

/// Prints an error on stderr and exit the program
pub fn print_error(s: &str) -> ! {
    writeln!(&mut io::stderr(),
             "{} {}",
             Format::Error(lformat!("Error:")),
             s)
        .unwrap();
    exit(0);
}

/// Gets the book options in a (key, value) list, or print an error
pub fn get_book_options<'a>(matches: &'a ArgMatches) -> Vec<(&'a str, &'a str)> {
    let mut output = vec![];
    if let Some(iter) = matches.values_of("set") {
        let v: Vec<_> = iter.collect();
        if v.len() % 2 != 0 {
            print_error(&lformat!("An odd number of arguments was passed to --set, but it takes \
                                   a list of key value pairs."));
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
/// the creation of a book to check that parameters are OK and t
/// then print them to file
pub fn set_book_options(book: &mut Book, matches: &ArgMatches) -> String {
    let mut output = String::new();
    let options = get_book_options(matches);

    for (key, value) in options {
        let res = book.options.set(key, value);
        if let Err(err) = res {
            print_error(&lformat!("Error in setting key {}: {}", key, err));
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
            print_error(&lformat!("Could not create file {}: it already exists!", book));
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

# Uncomment and fill to generate files
# output.html: some_file.html
# output.epub: some_file.epub
# output.pdf: some_file.pdf

# Uncomment and fill to set cover image (for Epub)
# cover: some_cover.png\n").as_bytes())
                .unwrap();
        }
        f.write_all(lformat!("\n# List of chapters\n").as_bytes()).unwrap();
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
        static ref VERBOSE: String = lformat!("Print warnings in parsing/rendering");
        static ref QUIET: String = lformat!("Don't print info/error messages");
        static ref PROOFREAD: String = lformat!("Enable proofreading");
        static ref CREATE: String = lformat!("Create a new book with existing Markdown files");
        static ref OUTPUT: String = lformat!("Specify output file");
        static ref LANG: String = lformat!("Set the runtime language used by Crowbook");
        static ref TO: String = lformat!("Generate specific format");
        static ref SET: String = lformat!("Set a list of book options");
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
        .arg(Arg::from_usage("-s, --single").help(SINGLE.as_str()))
        .arg(Arg::from_usage("-v, --verbose").help(VERBOSE.as_str()))
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
        print_error(&lformat!("A list of additional files is only valid with the --create \
                               option."));
    }
}
