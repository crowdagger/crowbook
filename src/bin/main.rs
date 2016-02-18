extern crate crowbook;

use crowbook::{ast_to_html, Parser, French};


fn main() {
    let doc = "
Foo
===

« Oh la chevalier que voulez vous ? » 


```rust
fn min(x : &u32, y : u32) -> &u32 {
    if x < y { x } else { y }
}
```

Bar
---

Some paragraph

* a list
    * inside a list
* another item

3. three
4. four
5. five

[& some link](http://foo/bar?baz=42&coin=plop)
";

    let french = French::new('~');
    let mut parser = Parser::new().with_cleaner(Box::new(french));
    let v = parser.parse(doc).unwrap();
    println!("{:?}", &v);

    println!("{}", ast_to_html(v));
}
