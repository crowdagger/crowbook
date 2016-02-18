extern crate crowbook;

use crowbook::{HtmlRenderer, Parser, French, Book};



fn main() {
    let config = "
author: Lizzie Crowdagger
title: Pas tout à fait des hommes
lang: fr
numbering: true
autoclean: true
cover: book-example/cover.png
nb_char: '~'

+ book_example/chapitre_01.md
+ book_example/chapitre_02.md
+ book_example/chapitre_03.md
";

    let mut book = Book::new();
    book.set_from_config(config).unwrap();
    let mut html = HtmlRenderer::new(&book);
    println!("{}", html.render_book().unwrap());
}
