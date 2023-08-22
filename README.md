# Crowbook

Crowbook's aim is to allow you to write a book in Markdown without worrying about formatting or typography, and let the program generate HTML, PDF and EPUB output for you.
Its focus is novels and fiction, and the default settings should (hopefully) generate readable books with correct typography without requiring you to worry about it.

## Example

To see what Crowbook's output looks like, you can read the Crowbook guide rendered in
[HTML](http://lise-henry.github.io/crowbook/book/book.html),
[PDF](http://lise-henry.github.io/crowbook/book/book.pdf)
or
[EPUB](http://lise-henry.github.io/crowbook/book/book.epub).

## Installing

There are two ways to install Crowbook:
either using precompiled binaries, or compiling it using `cargo`.

### Binaries

See
[the releases page](https://github.com/lise-henry/crowbook/releases)
to download a precompiled binary for your architecture.
Just extract the archive and run `crowbook`
(or `crowbook.exe` on Windows).
You might also want to copy the binary somewhere in your `PATH` for later usage.


### Using Cargo

[Cargo](https://crates.io/)
is
the package manager for
[Rust](https://www.rust-lang.org/).
You can
[install it here](https://www.rust-lang.org/downloads.html).
Once that is done:

```bash
$ cargo install crowbook
```

will automatically download the latest `crowbook` release on
[crates.io](https://crates.io/crates/crowbook),
compile it, and install it on your system.

> Some dependencies also require building C libraries;
> you might thus also need to install a C compiler and `make`/`cmake` build tools.

## Dependencies

While there should be, strictly speaking, no real dependencies to be able to run Crowbook (it is published as a statically compiled binary), 
PDF rendering requires a working installation of LaTeX (preferably `xelatex`).

## Quick tour

The simplest command is:

```bash
$ crowbook <BOOK>
```

where `BOOK` is a configuration file.
Crowbook will parse this file and generate HTML, EPUB, and/or PDF output formats, according to the settings in the configuration file.

To create a new book, assuming you have a list of Markdown files, you can generate a template configuration file with the `--create` argument:

```bash
$ crowbook my.book --create chapter_*.md
```

This will generate a default `my.book` file, which you'll need to complete.
This configuration file contains some metadata, options, and lists the Markdown files.

For short books containing only a single Markdown file, it is possible to embed some metadata at the beginning of the file and use the `--single` or `-s` option to run `crowbook` directly on this Markdown file and avoid creating a separate book configuration file:

```bash
$ crowbook -s text.md
```

For more information, see the chapters on
[the arguments supported by `crowbook`](guide/01_arguments.md)
and on
[the configuration file](guide/02_config.md).

## Current features

### Output formats

Crowbook supports HTML, PDF and EPUB (either version 2 or 3) as output formats.
See the Crowbook User Guide  rendered in
[HTML](http://lise-henry.github.io/crowbook/book/book.html),
[EPUB](http://lise-henry.github.io/crowbook/book/book.epub)
and
[PDF](http://lise-henry.github.io/crowbook/book.pdf).

### Input format

Crowbook uses
[pulldown-cmark](https://crates.io/crates/pulldown-cmark)
and thus should support most of
[CommonMark Markdown](http://commonmark.org/).
Inline HTML, however, is not implemented, and probably won't be, as the goal is to have books that can also be generated in PDF (and maybe ODT).

### Typographic "cleaning"

Maybe the most specific "feature" of Crowbook is that it does its best to "clean" the input text before rendering it.
By default, it removes superfluous spaces and tries to use curly quotes.
If the  book's language is set to french, it also tries to respect french typography by replacing spaces with non-breaking ones when it is appropriate (e.g. before '?', '!', ';' or ':').

> Please
> [open an issue](https://github.com/lise-henry/crowbook/issues/new)
> describing typographic rules if you want them to be implemented for other languages.

### Links handling

Crowbook tries to correctly translate local links in the input Markdown files:
e.g. if you have a link to a Markdown file that is part of your book, it will be transformed into a link inside the document.

### Inline YAML blocks

Crowbook supports inline YAML blocks:

```yaml
---
author: Me
title: My title
---
```

This is mostly useful when Crowbook is run with the `--single` argument (receiving a single Markdown file instead of a book configuration file), for short texts that only contain one "chapter".

### Interactive fiction

Crowbook has experimental support for writing interactive fiction (only for HTML).
For more information, read the
[interactive fiction chapter](guide/06_interactive_fiction.md).

### Customization

While the default settings will hopefully generate something that should look "good enough", it is possible to customize the output, essentially by providing different
[templates](guide/04_templates.md).

### Bugs

See the
[issue tracker on GitHub](https://github.com/lise-henry/crowbook/issues).

## Contributors

<!-- readme: contributors -start -->
<table>
<tr>
    <td align="center">
        <a href="https://github.com/lise-henry">
            <img src="https://avatars.githubusercontent.com/u/1961791?v=4" width="100;" alt="lise-henry"/>
            <br />
            <sub><b>Élisabeth Henry</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/stefan0xC">
            <img src="https://avatars.githubusercontent.com/u/509385?v=4" width="100;" alt="stefan0xC"/>
            <br />
            <sub><b>Stefan Melmuk</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/hirschenberger">
            <img src="https://avatars.githubusercontent.com/u/1053180?v=4" width="100;" alt="hirschenberger"/>
            <br />
            <sub><b>Falco Hirschenberger</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/Geobert">
            <img src="https://avatars.githubusercontent.com/u/72570?v=4" width="100;" alt="Geobert"/>
            <br />
            <sub><b>Geobert Quach</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/jrappen">
            <img src="https://avatars.githubusercontent.com/u/8577450?v=4" width="100;" alt="jrappen"/>
            <br />
            <sub><b>Johannes Rappen</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/taophp">
            <img src="https://avatars.githubusercontent.com/u/5610065?v=4" width="100;" alt="taophp"/>
            <br />
            <sub><b>Stéphane Mourey</b></sub>
        </a>
    </td></tr>
<tr>
    <td align="center">
        <a href="https://github.com/dkotrada">
            <img src="https://avatars.githubusercontent.com/u/698296?v=4" width="100;" alt="dkotrada"/>
            <br />
            <sub><b>Alfa</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/hfiguiere">
            <img src="https://avatars.githubusercontent.com/u/114441?v=4" width="100;" alt="hfiguiere"/>
            <br />
            <sub><b>Hubert Figuière</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/dvalter">
            <img src="https://avatars.githubusercontent.com/u/38795282?v=4" width="100;" alt="dvalter"/>
            <br />
            <sub><b>Dmitry Valter</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/Dylan-DPC">
            <img src="https://avatars.githubusercontent.com/u/99973273?v=4" width="100;" alt="Dylan-DPC"/>
            <br />
            <sub><b>Dylan DPC</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/steffahn">
            <img src="https://avatars.githubusercontent.com/u/3986214?v=4" width="100;" alt="steffahn"/>
            <br />
            <sub><b>Frank Steffahn</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/cuviper">
            <img src="https://avatars.githubusercontent.com/u/36186?v=4" width="100;" alt="cuviper"/>
            <br />
            <sub><b>Josh Stone</b></sub>
        </a>
    </td></tr>
<tr>
    <td align="center">
        <a href="https://github.com/mgeisler">
            <img src="https://avatars.githubusercontent.com/u/89623?v=4" width="100;" alt="mgeisler"/>
            <br />
            <sub><b>Martin Geisler</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/sigurdsvela">
            <img src="https://avatars.githubusercontent.com/u/5571884?v=4" width="100;" alt="sigurdsvela"/>
            <br />
            <sub><b>Sigurd Svela</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/twirrim">
            <img src="https://avatars.githubusercontent.com/u/59949?v=4" width="100;" alt="twirrim"/>
            <br />
            <sub><b>Twirrim</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/ar1ocker">
            <img src="https://avatars.githubusercontent.com/u/109543340?v=4" width="100;" alt="ar1ocker"/>
            <br />
            <sub><b>Ar1oc</b></sub>
        </a>
    </td></tr>
</table>
<!-- readme: contributors -end -->

## Acknowledgements

Besides the
[Rust](https://www.rust-lang.org/)
compiler and standard library, Crowbook uses the following libraries:
[pulldown-cmark](https://crates.io/crates/pulldown-cmark),
[yaml-rust](https://crates.io/crates/yaml-rust),
[mustache](https://crates.io/crates/mustache),
[clap](https://github.com/kbknapp/clap-rs),
[chrono](https://crates.io/crates/chrono),
[uuid](https://crates.io/crates/uuid),
[mime_guess](https://crates.io/crates/mime_guess),
[crossbeam](https://crates.io/crates/crossbeam),
[walkdir](https://crates.io/crates/walkdir),
[rustc-serialize](https://crates.io/crates/rustc-serialize),
[caribon](https://crates.io/crates/caribon),
[hyper](https://crates.io/crates/hyper),
[url](https://crates.io/crates/url),
[lazy_static](https://crates.io/crates/lazy_static),
[regex](https://crates.io/crates/regex),
[term](https://crates.io/crates/term),
[numerals](https://crates.io/crates/numerals),
[syntect](https://crates.io/crates/syntect).

It can also embed
[Highlight.js](https://highlightjs.org/)
in HTML output to enable syntax highlighting for code blocks.

It also uses configuration files from
[rust-everywhere](https://github.com/japaric/rust-everywhere)
to use
[Travis](https://travis-ci.org/)
and
[Appveyor](http://www.appveyor.com/)
to generate binaries for various platforms on each release.

While Crowbook directly doesn't use them, there was also inspiration from
[Pandoc](http://pandoc.org/)
and
[mdBook](https://github.com/azerupi/mdBook).

Also, the
[W3C HTML validator](https://validator.w3.org/)
and the
[IDPF EPUB validator](http://validator.idpf.org/)
proved to be very useful during development and testing.

## ChangeLog

See [ChangeLog](ChangeLog.md).

## Contributing

See [how you can contribute to Crowbook](guide/08_contributing.md).

If you find this project useful, you can also support its author by
[making a Paypal donation](https://www.paypal.me/crowdagger).

## Library

While the main purpose of Crowbook is to be run as a standalone program, the code is written as a library, so if you want to build on it you can use it as such.
You can look at the generated documentation on
[docs.rs](https://docs.rs/releases/search?query=crowbook).

Note that, in order to facilitate code reuse, some features have been split to separate libraries:

* [epub-builder](https://github.com/lise-henry/epub-builder)
  makes it easier to generate EPUB files.
* [crowbook-text-processing](https://github.com/lise-henry/crowbook-text-processing/)
  contains all the "typographic" functions (smart quotes, handling of non-breaking spaces in french, ...).
* [crowbook-intl](https://github.com/lise-henry/crowbook-intl/)
  is used for the internationalization (translation) process.

## License

Crowbook is free software:
you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License (LGPL), version 2.1 or (at your option) any later version.
See
[LICENSE](LICENSE.md)
for more information.

Crowbook's logo is licensed under the
[Creative Commons Attribution 4.0 International license](https://creativecommons.org/licenses/by/4.0/deed.en),
based on the
[Rust logo](https://commons.wikimedia.org/wiki/File:Rust_programming_language_black_logo.svg)
by Mozilla Corporation.

Crowbook includes binary (minified) CSS and Javascript files from
[Highlight.js](https://highlightjs.org/),
written by Ivan Sagalaev, see
[license](https://raw.githubusercontent.com/lise-henry/crowbook/master/templates/highlight/LICENSE)
