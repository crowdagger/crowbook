extern crate crowbook;

use crowbook::{ast_to_html, Parser, French, Book};


fn main() {
    let config = "
author: Lizzie Crowdagger
title: Boum!
lang: fr
numbering: false
autoclean: ok
cover: cover.png

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
    book.set_from_config(config).unwrap();
    println!("{:?}", &book);
}
