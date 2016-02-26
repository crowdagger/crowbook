extern crate crowbook;
use crowbook::Book;
use std::env;
use std::io;

#[test]
fn test_book() {
    let book = Book::new_from_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/test.book"), false).unwrap();
    book.render_html(&mut io::sink()).unwrap();
    book.render_tex(&mut io::sink()).unwrap();
}

#[test]
fn book_example() {
    let book = Book::new_from_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "book_example/config.book"), false).unwrap();
    book.render_html(&mut io::sink()).unwrap();
    book.render_tex(&mut io::sink()).unwrap();
}
