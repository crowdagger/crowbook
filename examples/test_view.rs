extern crate crowbook;
use crowbook::{view_as_text, insert_at};
use crowbook::Parser;

fn main() {
    let s = "Un **petit** test avec du *formatage*";
          
    let mut ast = Parser::new().parse(s).unwrap();
    println!("{:?}", ast);
    println!("{}", view_as_text(&ast));
    insert_at(&mut ast, "(2)", 2);
    insert_at(&mut ast, "(3)", 3);
    insert_at(&mut ast, "(4)", 4);
    println!("{:?}", ast);
}
