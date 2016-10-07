use clap::{App, Arg, Format, ArgMatches};
use std::io::{self, Write};
use std::process::exit;
use std::fs;
use crowbook::Book;

/// Prints an error on stderr and exit the program
pub fn print_error(s: &str) -> ! {
    writeln!(&mut io::stderr(), "{} {}", Format::Error("Error:"), s).unwrap();
    exit(0);
}

/// Gets the book options in a (key, value) list, or print an error
pub fn get_book_options<'a>(matches: &'a ArgMatches) -> Vec<(&'a str, &'a str)> {
    let mut output = vec!();
    if let Some(iter) = matches.values_of("set") {
        let v:Vec<_> = iter.collect();
        if v.len() %2 != 0 {
            print_error("An odd number of arguments was passed to --set, but it takes a list of key value pairs.");
        }

        for i in 0..v.len()/2 {
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
            print_error(&format!("Error in setting key {}: {}", key, err));
        }
        output.push_str(&format!("{}: {}\n", key, value));
    }
    output
}

/// create a book file with the command line arguments
/// and exit the process at the end
pub fn create_book(matches: &ArgMatches) -> ! {
    let mut f:Box<Write> = if let Some(book) = matches.value_of("BOOK") {
        if fs::metadata(book).is_ok() {
            print_error(&format!("Could not create file {}: it already exists!", book));
        }
        Box::new(fs::File::create(book).unwrap())
    } else {
        Box::new(io::stdout())
    };
        
    if let Some(values) = matches.values_of("create") {
        if matches.is_present("set") {
            let mut book = Book::new(&[]);
            let s = set_book_options(&mut book, matches);
            f.write_all(&s.as_bytes()).unwrap();
        } else {
            f.write_all(b"author: Your name
title: Your title
lang: en

# Uncomment and fill to generate files
# output.html: some_file.html
# output.epub: some_file.epub
# output.pdf: some_file.pdf

# Uncomment and fill to set cover image (for Epub)
# cover: some_cover.png\n").unwrap();
        }
        f.write_all(b"\n# List of chapters\n").unwrap();
        for file in values {
            f.write_all(&format!("+ {}\n", file).as_bytes()).unwrap();
        }
        if let Some(s) = matches.value_of("BOOK") {
            println!("Created {}, now you'll have to complete it!", s);
        }
        exit(0);
    } else {
        unreachable!(); // because Clap takes care of it
    }
}

pub fn create_matches<'a>() -> ArgMatches<'a> {
    let app = App::new("crowbook")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Render a Markdown book in EPUB, PDF or HTML.")
        .arg_from_usage("-s, --single 'Use a single Markdown file instead of a book configuration file'")
        .arg_from_usage("-v, --verbose 'Print warnings in parsing/rendering'")
        .arg(Arg::from_usage("-q, --quiet 'Don't print info/error messages'")
             .conflicts_with("verbose")
             .conflicts_with("debug"))
        .arg_from_usage("-p, --proofread 'Enable roofreading'")
        .arg(Arg::from_usage("-d, --debug 'Print debugging information'")
             .conflicts_with("verbose")
             .hidden(true))
        .arg_from_usage("-c, --create [FILES]... 'Creates a new book with existing markdown files'")
        .arg(Arg::from_usage("-o, --output [FILE] 'Specifies output file'")
             .requires("to"))
        .arg(Arg::from_usage("-t, --to [FORMAT] 'Generate specific format'")
             .possible_values(&["epub", "pdf", "html", "tex", "odt", "proofread.html", "proofread.html_dir", "proofread.pdf", "proofread.tex"]))
        .arg(Arg::from_usage("--set [KEY_VALUES] 'Sets a list of book options'")
             .min_values(2))
        .arg_from_usage("-l --list-options 'Lists all possible options")
        .arg(Arg::from_usage("--list-options-md 'List all options, formatted in Markdown'")
             .hidden(true))
        .arg_from_usage("--print-template [TEMPLATE] 'Displays the default value of a template'")
        .arg(Arg::with_name("BOOK")
             .index(1)
             .help("File containing the book configuration, or a Markdown file when called with --single"));

    let matches = app.get_matches();

    pre_check(&matches);
    matches
}


/// Pre-check the matches to see if there isn't illegal options not detected by clap
fn pre_check(matches: &ArgMatches) {
    if matches.is_present("files") && !matches.is_present("create") {
        print_error("A list of additional files is only valid with the --create option.");
    }
}
