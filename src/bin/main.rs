extern crate crowbook;

use crowbook::{ast_to_html, Parser, French, Book};


fn main() {
    let config = "
author: Lizzie Crowdagger
title: Boum!
lang: fr
numbering: true
autoclean: false
cover: cover.png

toto: tutu
- preface.md
-    intro.md
16. chapitre_16.md
1. chapitre_1.md
+ chapitre_2.md
";

    let doc = "
Foo
===

Bar";

    let french = French::new('~');
    let mut parser = Parser::new().with_cleaner(Box::new(french));
    let v = parser.parse(doc).unwrap();
    println!("{:?}", &v);

    println!("{}", ast_to_html(v));

    println!("");
    let mut book = Book::new();
    let res = book.set_from_config(config);
    match res {
        Ok(_) => println!("Book configured succesfully"),
        Err(err) => println!("{}", err)
    }
    println!("{:?}", &book);
}
