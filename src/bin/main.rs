extern crate crowbook;

use crowbook::{Book};
use std::env;

fn main() {
    let mut args = env::args();
    args.next(); //discard program name

    match args.next() {
        None => println!("Needs the name of a book config file"),
        Some(ref s) => {
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
}
