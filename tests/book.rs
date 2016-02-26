extern crate crowbook;
use crowbook::Book;
use std::env;

#[test]
fn test_book() {
    let book = Book::new_from_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/test.book"), false).unwrap();
    book.render_all().unwrap();
}

#[test]
fn book_example() {
    let book = Book::new_from_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "book_example/config.book"), false).unwrap();
    book.render_all().unwrap();
}
