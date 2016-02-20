extern crate crowbook;
extern crate clap;

use crowbook::{Book};
use std::env;
use clap::{App,Arg};

fn main() {
    let mut app = App::new("crowbook")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Élisabeth Henry")
        .about("Render a markdown book in Epub, PDF or HTML")
        .arg_from_usage("-v, --verbose 'Activate verbose mode'")
        .arg_from_usage("-o, --output=[FILE] 'Specifies an output file'")
        .arg(Arg::with_name("book")
             .index(1)
             .required(true)
             .help("A file containing the book configuration"));

    let matches = app.get_matches();

    if let Some(s) = matches.value_of("book") {
        match Book::new_from_file(s) {
            Ok(book) => {
                if let Err(err) = book.render_all()  {
                    println!("{}", err);
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
