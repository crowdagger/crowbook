Crowbook
========

[![Build Status](https://travis-ci.org/lise-henry/crowbook.svg?branch=master)](https://travis-ci.org/lise-henry/crowbook)

Render a book written in markdown to HTML, EPUB and/or PDF.

Crowbook's purpose is to allow you to automatically generate multiple
outputs formats from a book written in Markdown. Its focus is
novels, and the default settings should (hopefully) generate readable
books with correct typography without requiring you to worry about it.
    

Example
-------

To see what Crowbook's output looks like, you can read the Crowbook
guide rendered in
[HTML](http://lise-henry.github.io/crowbook/book/book.html), 
[PDF](http://lise-henry.github.io/crowbook/book/book.pdf) or [EPUB](http://lise-henry.github.io/crowbook/book/book.epub). 

You can also play with the [online demo version here](http://vps.crowdagger.fr/crowbook/).


 

Installing
----------

There are three ways to install Crowbook:

### Packages ###

If you are on Debian GNU/Linux or Ubuntu (on a PC architecture), you can
download `.deb` packages on
[the releases page](https://github.com/lise-henry/crowbook/releases). 

### Binaries ###

See [the releases page](https://github.com/lise-henry/crowbook/releases)
to download a precompiled binary for your architecture (currently:
Linux, Windows and MacOSX). Just extract the archive and run
`crowbook` (or `crowbook.exe` on Windows). You might also want to copy
the binary somewhere in your `PATH` for later usage.

> Note: only the Linux binaries are really tested, please contact me
> if you have any trouble executing the Windows or Mac binaries.

### Using Cargo ###

[Cargo](https://crates.io/) is the
[Rust](https://www.rust-lang.org/)'s package manager. You can
[install it here](https://www.rust-lang.org/downloads.html). Once
it is done: 

```bash
$ cargo install crowbook
```

will automatically download the latest `crowbook` release on
[crates.io](https://crates.io/crates/crowbook), compile it, and
install it on your system.

Dependencies
------------

While there should be, strictly speaking, no real dependencies to be able to
run Crowbook (it is published a a statically compiled binary), some
features require additional commands to work correctly:

* EPUB rendering requires that the `zip` command be present on your system;
* PDF rendering requires a working installation of LaTeX (preferably
`xelatex`).

Quick tour
----------

The simplest command is:

```bash
$ crowbook <BOOK>
```

where `BOOK` is a configuration file. Crowbook will parse this
file and generate a book in HTML, EPUB, and/or PDF,
according to the settings in the configuration file. 

To create a new book, assuming you have a
list of Markdown files, you can generate a template configuration file
with the `--create` argument:

```bash
$ crowbook --create my.book chapter_*.md
```

This will generate a default `my.book` file, which you'll need to
complete. This configuration file contains some metadata, options, and lists the
Markdown files. 

For more information see
[the configuration file](guide/config.md).

It is also possible to give additional parameters to `crowbook`;
we have already seen `--create`, but if you want the full list, see
[the arguments](guide/arguments.md).

Current features
----------------

### Output formats ###

Crowbook supports HTML, PDF and EPUB (either
version 2 or 3) as output formats. See the Crowbook User Guide 
rendered in
[HTML](http://lise-henry.github.io/crowbook/book/book.html),
[EPUB](http://lise-henry.github.io/crowbook/book/book.epub) and
[PDF](http://lise-henry.github.io/crowbook/book.pdf).

### Input format ###

Crowbook uses
[pulldown-cmark](https://crates.io/crates/pulldown-cmark) and thus
should support most of CommonMark Markdown. Inline HTML, however, is
not implemented, and probably won't be, as the goal is to have books
that can also be generated in PDF (and maybe ODT).

### Typographic "cleaning" ###

Maybe the most specific "feature" of Crowbook is that (by default, it
can be deactivated) it tries to "clean" the input files. By default,
it removes superfluous spaces and tries to use curly quotes. If the 
book's language is set to french, it also tries its best to respect french
typography by replacing spaces with non-breaking ones when it is
appropriate (e.g. before '?', '!', ';' or ':').

> This feature is currently limited to french language, but please
> [open an issue](https://github.com/lise-henry/crowbook/issues/new) describing typographic rules if you want it to be
> implemented for another language.

### Links handling ###

Crowbook tries to correctly translate local links in the input
Markdown files: e.g. if you have a link to a markdown file that is
part of your book, it will be transformed into a link inside the
document. 

### Inline YAML blocks ###

Crowbook supports inline YAML blocks: 

```yaml
---
author: Me
title: My title
---
```

This is mostly useful when Crowbook is runned with the `--single`
argument (receiving a single Markdown file instead of a book
configuration file). E.g., the following Markdown file:

```yaml
---
author: John Doe
title: A book

output.html: book.html
---

This is a very tiny book!
```

can be processed with `crowbook --single foo.md` or `crowbook -s
foo.md` to produce the `book.html` file. This is useful for short
texts that only contain one "chapter".

### Proofreading ###

Crowbook can also generate "proofreading" copies in HTML or PDF,
highlighting grammar errors and repetitions.

> This feature has been introduced in version `0.9.1` and is still
> experimental. For more information, see
> [the proofreading chapter of the guide](guide/proofreading.md). 

### Bugs ###

See the [github's issue tracker](https://github.com/lise-henry/crowbook/issues).

Contributors
------------

* [Stéphane Mourey](http://stephanemourey.fr/) `<s+crowbook AT stephanemourey DOT fr>`

Acknowledgements
----------------

Besides the [Rust](https://www.rust-lang.org/) compiler and standard
library, Crowbook uses the following libraries:

* [pulldown-cmark](https://crates.io/crates/pulldown-cmark) 
* [yaml-rust](https://crates.io/crates/yaml-rust) 
* [mustache](https://crates.io/crates/mustache) 
* [clap](https://github.com/kbknapp/clap-rs)
* [chrono](https://crates.io/crates/chrono) 
* [uuid](https://crates.io/crates/uuid) 
* [mime_guess](https://crates.io/crates/mime_guess)
* [crossbeam](https://crates.io/crates/crossbeam)
* [walkdir](https://crates.io/crates/walkdir)
* [rustc-serialize](https://crates.io/crates/rustc-serialize)
* [caribon](https://crates.io/crates/caribon)
* [hyper](https://crates.io/crates/hyper)
* [url](https://crates.io/crates/url)
* [lazy_static](https://crates.io/crates/lazy_static)
* [regex](https://crates.io/crates/regex)

It also embeds [Highlight.js](https://highlightjs.org/) in HTML output
to enable syntax highlighting for code blocks.

It also uses configuration files from
[rust-everywhere](https://github.com/japaric/rust-everywhere) to use
[Travis](https://travis-ci.org/) and
[Appveyor](http://www.appveyor.com/) to generate binaries for
various platforms on each release.

While Crowbook directly doesn't use them, there was also inspiration
from [Pandoc](http://pandoc.org/) and
[mdBook](https://github.com/azerupi/mdBook).

Also, the [W3C HTML validator](https://validator.w3.org/) and the
[IDPF EPUB validator](http://validator.idpf.org/) proved very useful
during development.

ChangeLog
-----------

See [ChangeLog](ChangeLog.md).

Library
-------

While the main purpose of Crowbook is to be runned as a standalone
program, 
the code is written as a library, so if you want to build on it you can
use it as such. You can look at the generated documentation on
[docs.rs](https://docs.rs/releases/search?query=crowbook).

Additionally, [crowbook-text-processing](https://github.com/lise-henry/crowbook-text-processing/) is
a separate library containing all the "typographic" functions (smart
quotes, handling of non-breaking spaces in french, ...).

License 
-------



Crowbook is free software: you can redistribute it and/or modify it
under the terms of the GNU Lesser General Public License (LGPL),
version 2.1 or (at your option) any ulterior version. See 
[LICENSE](LICENSE.md) for more information.

Crowbook's logo is licensed under the [Creative Commons Attribution 4.0
International license](https://creativecommons.org/licenses/by/4.0/deed.en),
based on the
[Rust logo](https://commons.wikimedia.org/wiki/File:Rust_programming_language_black_logo.svg)
by Mozilla Corporation.

Crowbook includes binary (minified) CSS and Javascript files from
[Highlight.js](https://highlightjs.org/), written by Ivan
Sagalaev, licensed under the following terms:

> Copyright (c) 2006, Ivan Sagalaev
>
> All rights reserved.
>
> Redistribution and use in source and binary forms, with or without
> modification, are permitted provided that the following conditions are met:
>  * Redistributions of source code must retain the above copyright
>    notice, this list of conditions and the following disclaimer.
>  * Redistributions in binary form must reproduce the above copyright
>    notice, this list of conditions and the following disclaimer in the
>    documentation and/or other materials provided with the
>    distribution.
>  * Neither the name of highlight.js nor the names of its contributors 
>    may be used to endorse or promote products derived from this software 
>    without specific prior written permission.
> 
> THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND ANY
> EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
> WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
> DISCLAIMED. IN NO EVENT SHALL THE REGENTS AND CONTRIBUTORS BE LIABLE FOR ANY
> DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
> (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
> LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
> ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
> (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
> SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
