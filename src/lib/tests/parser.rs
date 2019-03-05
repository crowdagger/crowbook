use crate::parser::Parser;
use crate::token::Token;
use crate::book::Book;
use super::test_eq;

fn parse_from_str(doc: &str) -> Vec<Token> {
    let mut book = Book::new();
    book.set_options(&[("crowbook.markdown.superscript", "true")]);
    let mut parser = Parser::from(&book);
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
    let expected = vec![Token::Header(1, vec![Token::Str(String::from("Test"))]),
                        Token::Paragraph(vec![Token::Str(String::from("some ")),
                              Token::Emphasis(vec![Token::Str(String::from("emphasis"))]),
                              Token::Str(String::from(" required"))])];
    assert_eq!(res, expected);
}

#[test]
fn link_inline() {
    let doc = "[a link](http://foo.bar)";
    let mut parser = Parser::new();
    let res = parser.parse(doc).unwrap();

    assert_eq!(res,
               vec![Token::Paragraph(vec![Token::Link(String::from("http://foo.bar"),
                                                      String::from(""),
                                                      vec![Token::Str(String::from("a link"))])])]);
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
    test_eq(&result, expected);
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
    let expected = "[List([Item([Str(\"banana\"), OrderedList(3, [Item([Str(\"3\")])]), \
                    List([Item([Str(\"4\")])])]), Item([Str(\"apple\")]), \
                    Item([Str(\"orange\")])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn blockquote() {
    let doc = "
normal paragraph

> some
> blockquote
";
    let expected = "[Paragraph([Str(\"normal paragraph\")]), \
                    BlockQuote([Paragraph([Str(\"some blockquote\")])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
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
    test_eq(&result, expected);
}

#[test]
fn strong_emphasis() {
    let doc = "
*normal emphasis*

**strong emphasis**
";
    let expected = r#"[Paragraph([Emphasis([Str("normal emphasis")])]), Paragraph([Strong([Str("strong emphasis")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn code() {
    let doc = "some `code` inlined";
    let expected = r#"[Paragraph([Str("some "), Code([Str("code")]), Str(" inlined")])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn image_reference_inline() {
    let doc = "
Test: ![alt text][logo]

[logo]: http://foo.bar/baz.png \"Title\"
";
    let expected = r#"[Paragraph([Str("Test: "), Image("http://foo.bar/baz.png", "Title", [Str("alt text")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn image_reference_standalone() {
    let doc = "
![alt text][logo]

[logo]: http://foo.bar/baz.png \"Title\"
";
    let expected = r#"[StandaloneImage("http://foo.bar/baz.png", "Title", [Str("alt text")])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}


#[test]
fn image_standalone() {
    let doc = "![alt text](http://foo.bar/baz.png \"Title\")";
    let expected = r#"[StandaloneImage("http://foo.bar/baz.png", "Title", [Str("alt text")])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn image_link_standalone() {
    let doc = "[![alt text](http://foo.bar/baz.png \"Title\")](http://foo.bar)";
    let expected = r#"[Link("http://foo.bar", "", [StandaloneImage("http://foo.bar/baz.png", "Title", [Str("alt text")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn table_simple() {
    let doc = "
| A             | Simple        | Table |
| ------------- |---------------| ------|
| bla           | bla           |  bla  |
| bla           | bla           |  bla  |
";
    let expected = "[Table(3, [TableHead([TableCell([Str(\" A             \")]), \
                    TableCell([Str(\" Simple        \")]), TableCell([Str(\" Table \")])]), \
                    TableRow([TableCell([Str(\" bla           \")]), TableCell([Str(\" bla           \
                    \")]), TableCell([Str(\"  bla  \")])]), TableRow([TableCell([Str(\" bla           \
                    \")]), TableCell([Str(\" bla           \")]), TableCell([Str(\"  bla  \
                    \")])])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn superscript() {
    let doc = "Some text^up^";
    let expected = "[Paragraph([Str(\"Some text\"), Superscript([Str(\"up\")])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    let doc = "*Some text^up^*";
    let expected = "[Paragraph([Emphasis([Str(\"Some text\"), Superscript([Str(\"up\")])])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    let doc = "1^2^3";
    let expected = "[Paragraph([Str(\"1\"), Superscript([Str(\"2\")]), Str(\"3\")])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    let doc = "`1^2^3`";
    let expected = r#"[Paragraph([Code([Str("1^2^3")])])]"#;
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    let doc = "*Some text^up^*";
    let expected = "[Paragraph([Emphasis([Str(\"Some text\"), Superscript([Str(\"up\")])])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    let doc = r"Some text^up\ and\ more^";
    let expected = "[Paragraph([Str(\"Some text\"), Superscript([Str(\"up and more\")])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    // let doc = r"Some text\^notup^";
    // let expected = "[Paragraph([Str(\"Some text^notup^\")])]";
    // let result = format!("{:?}", parse_from_str(doc));
    // test_eq(&result, expected);

    let doc = "Some text^not up^";
    let expected = "[Paragraph([Str(\"Some text^not up^\")])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    let doc = "Some text^^notup^^";
    let expected = "[Paragraph([Str(\"Some text^^notup^^\")])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn subscript() {
    let doc = "Some text~down~";
    let expected = "[Paragraph([Str(\"Some text\"), Subscript([Str(\"down\")])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);

    let doc = "*Some text~down~*";
    let expected = "[Paragraph([Emphasis([Str(\"Some text\"), Subscript([Str(\"down\")])])])]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn foonote_correct() {
    let doc = "A foonote[^1]...

[^1]: with a valid definition";
    let expected = "[Paragraph([Str(\"A foonote\"), Footnote([Paragraph([Str(\"with a valid \
                    definition\")])]), Str(\"...\")]), SoftBreak]";
    let result = format!("{:?}", parse_from_str(doc));
    test_eq(&result, expected);
}

#[test]
fn footnote_incorrect() {
    let doc = "A foonote[^1]...

[^2]: without a valid definition";

    let mut parser = Parser::new();
    let result = parser.parse(doc);
    assert!(result.is_err());
}
