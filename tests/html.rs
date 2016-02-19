extern crate crowbook;

use self::crowbook::{Parser, HtmlRenderer, Book, Token};

fn ast_to_html(v: &[Token]) -> String {
    let mut book = Book::new();
    book.numbering = false;
    let mut html = HtmlRenderer::new(&book);
    html.render_vec(v)
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
    let expected = "<h1>Foo</h1>
<pre><code class = \"language-rust\">fn min(x: &amp;u32, y: u32) -&gt; &amp;u32 {
    if x &lt; y { x } else { y }
}
</code></pre>
<h2>Bar</h2>
<p>Some paragraph</p>
<ul>
<li><p>a list</p>
<ul>
<li>inside a list</li>
</ul>
</li>
<li><p>another item</p>
</li>
</ul>
<ol start = \"3\">
<li>three</li>
<li>four</li>
<li>five</li>
</ol>
<p><a href = \"http://foo/bar?baz=42&coin=plop\">&amp; some link</a></p>
";
    let actual = ast_to_html(&Parser::new().parse(doc).unwrap());
    assert_eq!(actual, expected);
}
