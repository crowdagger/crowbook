use html::HtmlRenderer;
use book::Book;
use parser::Parser;
use token::Token;
use super::test_eq;
use renderer::Renderer;

fn ast_to_html(v: &[Token]) -> String {
    let mut book = Book::new(&[]);
    book.options.set("numbering", "0").unwrap();
    let mut html = HtmlRenderer::new(&book);
    html.render_vec(v).unwrap()
}


#[test]
fn html_combination() {
    let doc = "
Foo
===

```rust
fn min(x: &u32, y: u32) -> &u32 {
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
    let expected = r#"<h1 id = "link-1">Foo</h1>
<pre><code class = "language-rust">fn min(x: &amp;u32, y: u32) -&gt; &amp;u32 {
    if x &lt; y { x } else { y }
}
</code></pre>
<h2 id = "link-2">Bar</h2>
<p id = "para-1">Some paragraph</p>
<ul>
<li><p id = "para-2">a list</p>
<ul>
<li>inside a list</li>
</ul>
</li>
<li><p id = "para-3">another item</p>
</li>
</ul>
<ol start = "3">
<li>three</li>
<li>four</li>
<li>five</li>
</ol>
<p id = "para-4"><a href = "http://foo/bar?baz=42&amp;coin=plop">&amp; some link</a></p>
"#;
    let actual = ast_to_html(&Parser::new().parse(doc).unwrap());
    println!("ecpected:\n {}", expected);
    println!("actual:\n {}", actual);
    test_eq(&actual, &expected);
}
