use clap::{App, Arg, AppSettings, Format, ArgMatches};
use std::io::{self, Write};
use std::process::exit;
use std::fs;

/// Prints an error on stderr and exit the program
pub fn print_error(s: &str) -> ! {
    writeln!(&mut io::stderr(), "{} {}", Format::Error("error:"), s).unwrap();
    exit(0);
}

/// create a book file with the command line arguments
/// and exit the process at the end
pub fn create_book(matches: &ArgMatches) -> ! {
    if let Some(values) = matches.values_of("files") {
        let numbering = match matches.value_of("numbering") {
            Some("false") => false,
            _ => true,
        };
        
        let s = matches.value_of("BOOK").unwrap();
        if fs::metadata(s).is_ok() {
            print_error(&format!("Could not create file {}: it already exists!", s));
        } 
        let mut f = fs::File::create(s).unwrap();
        f.write_all(b"author: Your name\n").unwrap();
        f.write_all(b"title: Your title\n").unwrap();
        f.write_all(b"lang: en\n\n").unwrap();
        f.write_all(b"# Uncomment and fill to generate files\n").unwrap();
        f.write_all(b"# output.html: some_file.html\n").unwrap();
        f.write_all(b"# output.epub: some_file.epub\n").unwrap();
        f.write_all(b"# output.pdf: some_file.pdf\n\n").unwrap();
        f.write_all(b"# List of chapters\n").unwrap();
        for file in values {
            f.write_all(&format!("{} {}\n", if numbering {"+"} else {"-"}, file).as_bytes()).unwrap();
        }
        println!("Created {}, now you'll have to complete it!", s);
        exit(0);
    } else {
        print_error("--create must be used with a list of additonal files");
    }
}

pub fn create_matches<'a>() -> ArgMatches<'a> {
    let app = App::new("crowbook")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        //        .usage("crowbook [FLAGS] [OPTIONS] <BOOK> [FILE]")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Render a markdown book in Epub, PDF or HTML.")
        .after_help("Command line options allow to override options defined in <BOOK> configuration file. 
E.g., even if this file specifies 'verbose: false', calling 'crowbook --verbose <BOOK>' 
will activate verbose mode.

Note that Crowbook generates output files relatively to the directory where <BOOK> is:
$ crowbook foo/bar.book --to pdf --output baz.pdf
will thus generate baz.pdf in directory foo and not in current directory.")
        .arg_from_usage("-v, --verbose 'Activate verbose mode'")
        .arg(Arg::with_name("files")
             .value_name("FILES")
             .help("Files to put in book when using --create")
             .takes_value(true)
             .multiple(true)
             .index(2))
        .arg(Arg::with_name("create")
             .long("--create")
             .help("Creates a new book file with existing markdown files"))
        .arg(Arg::with_name("output")
             .long("--output")
             .short("-o")
             .value_name("FILE")
             .requires("to")
             .help("Specify output file"))
        .arg(Arg::with_name("autoclean")
             .long("--autoclean")
             .value_name("BOOL")
             .takes_value(true)
             .help("Try to clean input markdown")
             .possible_values(&["true", "false"]))
        .arg(Arg::with_name("numbering")
             .long("--numbering")
             .value_name("BOOL")
             .takes_value(true)
             .help("Number chapters or not")
             .possible_values(&["true", "false"]))
        .arg(Arg::with_name("to")
             .long("--to")
             .short("t")
             .takes_value(true)
             .possible_values(&["epub", "pdf", "html", "tex", "odt"])
             .value_name("FORMAT")
             .help("Generate specific format"))
        .arg(Arg::with_name("BOOK")
             .index(1)
             .required(true)
             .help("A file containing the book configuration"));

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
