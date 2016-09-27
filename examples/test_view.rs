extern crate crowbook;
use crowbook::{view_as_text, insert_annotation};
use crowbook::Parser;
use crowbook::Data;

fn main() {
    let s = "Un **petit** test avec du *formatage*";
          
    let mut ast = Parser::new().parse(s).unwrap();
    println!("{:?}", ast);
    println!("{}", view_as_text(&ast));
    insert_annotation(&mut ast, &Data::GrammarError("foo".to_owned()), 2, 1);
    insert_annotation(&mut ast, &Data::GrammarError("?".to_owned()), 3, 2);
    insert_annotation(&mut ast, &Data::GrammarError("!".to_owned()), 0, 10);
//    insert_annotation(&mut ast, "??", 3, 2);
//    insert_annotation(&mut ast, "??", 0, 10);
    
    // insert_at(&mut ast, "(2)", 2);
    // insert_at(&mut ast, "(3)", 3);
    // insert_at(&mut ast, "(4s)", 4);
    println!("{:?}", ast);
}
