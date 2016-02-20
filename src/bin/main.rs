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

use crowbook::{Book};
use std::env;
use clap::{App,Arg};

fn main() {
    let mut app = App::new("crowbook")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Render a markdown book in Epub, PDF or HTML.")
        .after_help("Command line options allow to override options defined in <BOOK> configuration file. 
E.g., even if this file specifies 'verbose: false', calling 'crowbook --verbose <BOOK>' 
will activate verbose mode.

Note that Crowbook generates output files relatively to the directory where <BOOK> is:
$ crowbook foo/bar.book --to pdf --output baz.pdf
will thus generate baz.pdf in directory foo and not in current directory.")
        .arg_from_usage("-v, --verbose 'Activate verbose mode'")
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
             .possible_values(&["epub", "pdf", "html", "tex"])
             .value_name("FORMAT")
             .help("Generate specific format"))
        .arg(Arg::with_name("BOOK")
             .index(1)
             .required(true)
             .help("A file containing the book configuration"));

    let matches = app.get_matches();

    if let Some(s) = matches.value_of("BOOK") {
        match Book::new_from_file(s) {
            Ok(mut book) => {
                if matches.is_present("verbose") {
                    book.verbose = true;
                }
                if let Some(autoclean) = matches.value_of("autoclean") {
                    book.autoclean = match autoclean {
                        "true" => true,
                        "false" => false,
                        _ => unreachable!()
                    };
                }
                
                if let Some(numbering) = matches.value_of("numbering") {
                    book.numbering = match numbering {
                        "true" => true,
                        "false" => false,
                        _ => unreachable!()
                    };
                }

                if let Some(format) = matches.value_of("to") {
                    if let Some(file) = matches.value_of("output") {
                        let value = Some(file.to_owned());
                        match format {
                            "epub" => book.output_epub = value,
                            "tex" => book.output_tex = value,
                            "html" => book.output_html = value,
                            "pdf" => book.output_pdf = value,
                            _ => unreachable!()
                        }
                    }

                    if let &Some(ref file) = match format {
                        "epub" => &book.output_epub,
                        "tex" => &book.output_tex,
                        "html" => &book.output_html,
                        "pdf" => &book.output_pdf,
                        _ => unreachable!()
                    } {
                        if let Err(err) = match format {
                            "epub" => book.render_epub(file),
                            "tex" => book.render_tex(file),
                            "html" => book.render_html(file),
                            "pdf" => book.render_pdf(file),
                            _ => unreachable!()
                        } {
                            println!("{}", err);
                        }
                    }  else {
                        println!("No output file specified, and book doesn't specify an output file for {}", format);
                        return;
                    }
                } else {
                    if let Err(err) = book.render_all()  {
                    println!("{}", err);
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
