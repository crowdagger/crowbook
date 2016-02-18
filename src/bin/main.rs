extern crate crowbook;

use crowbook::{HtmlRenderer, Book};
use std::env;



fn main() {
    let config = "
author: Lizzie Crowdagger
title: Pas tout à fait des hommes
lang: fr
numbering: false
autoclean: true
cover: book-example/cover.png

+ book_example/chapitre_01.md
+ book_example/chapitre_02.md
+ book_example/chapitre_03.md
";

    let mut args = env::args();
    args.next(); //discard program name

    match args.next() {
        None => println!("Needs the name of a book config file"),
        Some(ref s) => {
            let mut book = Book::new_from_file(s).unwrap();
            let mut html = HtmlRenderer::new(&book);
            println!("{}", html.render_book().unwrap());
        }
    }
}
