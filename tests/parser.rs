extern crate crowbook;

use self::crowbook::{Parser, Token};

fn parse_from_str(doc: &str) -> Vec<Token> {
    let mut parser = Parser::new();
    parser.parse(doc).unwrap()
}

#[test]
fn h_p_em() {
    let doc = "
Test
====

some *emphasis* required
";
    let mut parser = Parser::new();
    let res = parser.parse(doc).unwrap();
    
    assert_eq!(res, vec!(
        Token::Header(1, vec!(
            Token::Str(String::from("Test")))),
        Token::Paragraph(vec!(
            Token::Str(String::from("some ")),
            Token::Emphasis(vec!(
                Token::Str(String::from("emphasis")))),
            Token::Str(String::from(" required"))))));
}

#[test]
fn link_inline() {
    let doc = "[a link](http://foo.bar)";
    let mut parser = Parser::new();
    let res = parser.parse(doc).unwrap();

    assert_eq!(res,
               vec!(
                   Token::Paragraph(vec!(
                       Token::Link(String::from("http://foo.bar"),
                                   String::from(""),
                                   vec!(
                                       Token::Str(String::from("a link"))))))));
}

#[test]
fn reference_link() {
    let doc = "
[reference link][1]

[1]: http://foo.bar
";
    let expected = r#"[Paragraph([Link("http://foo.bar", "", [Str("reference link")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(&result, expected);    
}

#[test]
fn rule() {
    let doc = "a paragraph
****
another one";
    let expected = r#"[Paragraph([Str("a paragraph")]), Rule, Paragraph([Str("another one")])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(&result, expected);
}
        
#[test]
fn lists() {
    let doc = "
* banana
    3. 3
    -  4
* apple
* orange
";
    let expected = r#"[List([Item([Str("banana"), OrderedList(3, [Item([Str("3")])]), List([Item([Str("4")])])]), Item([Str("apple")]), Item([Str("orange")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(result, expected);
}

#[test]
fn blockquote() {
    let doc = "
normal paragraph

> some
> blockquote
";
    let expected = "[Paragraph([Str(\"normal paragraph\")]), BlockQuote([Paragraph([Str(\"some\"), SoftBreak, Str(\"blockquote\")])])]";
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(result, expected);
}

#[test]
fn code_block() {
        let doc = "
normal paragraph

```
code block
```

```rust
rust code block
```
";
    let expected = r#"[Paragraph([Str("normal paragraph")]), CodeBlock("", [Str("code block\n")]), CodeBlock("rust", [Str("rust code block\n")])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(result, expected);
}

#[test]
fn strong_emphasis() {
    let doc = "
*normal emphasis*

**strong emphasis**
";
    let expected = r#"[Paragraph([Emphasis([Str("normal emphasis")])]), Paragraph([Strong([Str("strong emphasis")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(result, expected);
}

#[test]
fn code() {
    let doc = "some `code` inlined";
    let expected = r#"[Paragraph([Str("some "), Code([Str("code")]), Str(" inlined")])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(result, expected);
}

#[test]
fn image_reference() {
    let doc = "
![alt text][logo]

[logo]: http://foo.bar/baz.png \"Title\"
";
    let expected = r#"[Paragraph([Image("http://foo.bar/baz.png", "Title", [Str("alt text")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(result, expected);
}

#[test]
fn image_inline() {
    let doc = "![alt text](http://foo.bar/baz.png \"Title\")";
    let expected = r#"[Paragraph([Image("http://foo.bar/baz.png", "Title", [Str("alt text")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    assert_eq!(result, expected);
}                           
