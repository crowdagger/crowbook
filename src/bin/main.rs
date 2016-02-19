extern crate crowbook;

use crowbook::Book;
use std::env;

fn main() {
    let mut args = env::args();
    args.next(); //discard program name

    match args.next() {
        None => println!("Needs the name of a book config file"),
        Some(ref s) => {
            let book = Book::new_from_file(s).unwrap();
            book.render_all().unwrap();
        }
    }
}
