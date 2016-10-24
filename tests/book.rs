extern crate crowbook;
use crowbook::{Book, InfoLevel};
use std::io;

#[test]
fn test_book() {
    let book = Book::new_from_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/test.book"), InfoLevel::Error, &[]).unwrap();
    book.render_html(&mut io::sink()).unwrap();
    book.render_tex(&mut io::sink()).unwrap();
}

#[test]
fn book_example() {
    let book = Book::new_from_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "guide.book"), InfoLevel::Error, &[]).unwrap();
    book.render_html(&mut io::sink()).unwrap();
    book.render_tex(&mut io::sink()).unwrap();
}
