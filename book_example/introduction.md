
Introduction 
============

What is Crowbook
----------------

As its name suggests, Crowbook is a tool to render books and, more
specifically, books written in *Markdown*. Its primary target are
novels, with a focus on generating PDF and Epub files.

This doesn't mean that it is impossible to use Crowbook for
technical documentations (though some features, like footnotes, *are*
actually currently unsupported), but the result won't probably be as
beautiful as tools that are more focused to that, such as
[mdBook](https://github.com/azerupi/mdBook). Let's see an example with
code formatting:

```rust 
fn main() {
    println!("Hello, world!");
}
```

Now, that's not nearly as nice as cool syntax highlighting and even
the possibility to actually run the code in your browser. But that's
OK because, again, Crowbook is focused on novels, and you don't
expect to see a lot of programming in a novel.

*****

At the opposite, having a decent separation ruler to separate lists of
paragraphs, and not some ugly line, is more relevant to our objectives.

Installing Crowbook 
-------------------

Crowbook is written in [Rust](https://www.rust-lang.org/), so you'll
first need to install the Rust compiler, which you can
[download here](https://www.rust-lang.org/downloads.html) if you don't
already have it.

Then you'll need to clone the github repository of Crowbook and run
`cargo build` in it:

```bash
$ git clone https://github.com/lise-henry/crowbook.git
$ cd crowbook
$ cargo build
```

Then you'll have two options: either run `crowbook` directly in this
directory, with `cargo run`, or install it with `cargo install`.

Either way, you'll need to pass it a configuration file. E.g., to
generate this book, you'll do:

```
$ cargo run book_example/config.book
```

