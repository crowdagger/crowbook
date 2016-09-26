extern crate crowbook;
use crowbook::{view_as_text, insert_at};
use crowbook::Parser;

fn main() {
    let s = "Un **petit** test avec du *formatage*";
          
    let mut ast = Parser::new().parse(s).unwrap();
    println!("{:?}", ast);
    println!("{}", view_as_text(&ast));
    insert_at(&mut ast, "!!", 5);
    println!("{:?}", ast);
}
