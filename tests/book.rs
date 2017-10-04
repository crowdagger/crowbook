extern crate crowbook;
use crowbook::Book;
use std::io;

#[test]
fn test_book() {
    let mut book = Book::new();
    book.load_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/test.book"))
        .unwrap();
    book.render_format_to("html", &mut io::sink()).unwrap();
    book.render_format_to("tex", &mut io::sink()).unwrap();
}

#[test]
fn book_example() {
    let mut book = Book::new();
    book.load_file(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "guide.book"))
        .unwrap();
    book.render_format_to("html", &mut io::sink()).unwrap();
    book.render_format_to("tex", &mut io::sink()).unwrap();
}
