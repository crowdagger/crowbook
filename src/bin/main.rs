extern crate crowbook;
extern crate zip;

use crowbook::{HtmlRenderer, Book, EpubRenderer, LatexRenderer};
use std::env;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut args = env::args();
    args.next(); //discard program name

    match args.next() {
        None => println!("Needs the name of a book config file"),
        Some(ref s) => {
            let book = Book::new_from_file(s).unwrap();
            let mut html = HtmlRenderer::new(&book);
            let result = html.render_book().unwrap();

            let mut epub = EpubRenderer::new(&book);
            let buf = epub.render_book().unwrap();

            let mut f = File::create("test.epub").unwrap();
            f.write(&buf);

            let mut latex = LatexRenderer::new(&book);
            let s = latex.render_book().unwrap();
            println!("{}", s);
        }
    }
}
