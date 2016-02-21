Crowbook
========

Render a markdown book in HTML, Epub or PDF.

[![Build Status](https://travis-ci.org/lise-henry/crowbook.svg?branch=master)](https://travis-ci.org/lise-henry/crowbook)

Building and installing
-----------------------

Youl'll need to have the [Rust](https://www.rust-lang.org/) compiler
on your machine first; you can
[download and install it here](https://www.rust-lang.org/downloads.html). Once
it is down:

```
$ cargo install crowbook
```

will download `crowbook` and install it. 


Usage
-----

The simplest command is:

```bash
$ crowbook <BOOK>
```

Where `BOOK` is a configuration file. Crowbook will then parse the
config file and generate book in HTML, Epub, LaTeX, and/or PDF,
according to the setting in the configuration file. So if you clone
this repository and run

```bash
$ crowbook book_example/config.book
```

(or `cargo run -- book_example/config.book` if you don't want to
install it), you'll generate the example book in various format. The
HTML version should look
[like that](http://lise-henry.github.io/crowbook/book.html).

Now, let's say you want to make your own book. Assuming you have a
list of Markdown files, you can generate a template configuration file
with the `--create` argument:

```bash
$ crowbook --create my.book chapter_*.md
```

This will generate a default `my.book` file, which you'll need to complete.

This configuration file contains some metadata, options, and lists the
Markdown files. Here is a basic example:

```
author: Joan Doe
title: Some book
lang: en

output_html: some_book.html

+ chapter_1.md
+ chapter_2.md
+ chapter_3.md
+ ...
```

For more information see
[the configuration file page](book_example/config.md), or the whole
[book_example](book_example) directory. (A (not necessarily
up-to-date) [rendered version is available in HTML here](http://lise-henry.github.io/crowbook/book.html)).

It is also possible to give additional parameters to `crowbook`;
we have already seen `--create`, but if you want the full list, see
[the arguments page](book_example/arguments.md).

Features
--------

### Done ###
* Support for some simple format configuration files to
  list chapters and metadatas, so you just run `crowbook
  some_file` and you don't have to pass more options, it generates the
  rest.
* Support for Epub2 and Epub3 format as output.
* Support for HTML format as output.
* Partial support for LaTeX format as output, and PDF through it.
* Experimental support for Odt format as output.
* Support for basic french typography (i.e. non-breaking spaces) in HTML/Epub format.
* Some configuration for HTML/Epub templates and CSS.

### ToDo ###
* Allow more customization.
* Support for easily embedding custom fonts (and other files) in
Epub/HTML.
* Improve LaTeX and Odt generation.
* Correct support for technical books.

See also [Bugs](Bugs.md).


Acknowledgements
----------------

Besides the Rust compiler and standard library, Crowbook uses the
following libraries:

* [pulldown-cmark](https://crates.io/crates/pulldown-cmark) (for
parsing markdown)
* [mustache](https://crates.io/crates/mustache) (for templating)
* [clap](https://github.com/kbknapp/clap-rs) (for parsing command line arguments)
* [chrono](https://crates.io/crates/chrono) (date and time library)
* [uuid](https://crates.io/crates/uuid) (to generate uuid)

While Crowbook directly doesn't use them, there was also inspiration from [Pandoc](http://pandoc.org/) and [mdBook](https://github.com/azerupi/mdBook).

ChangeLog
---------

See [ChangeLog](ChangeLog.md).

Library
-------

While the main purpose of Crowbook is to be runned as a command line,
the code is written as a library so if you want to build on it you can
use it as such. The code is currently badly documented (and badly in a
general manner), but you can look at the generated documentation [here](http://lise-henry.github.io/rust/crowbook/).

License 
-------

Crowbook is free software: you can redistribute it and/or modify it
under the terms of the GNU Lesser General Public License (LGPL),
version 2.1 or (at your option) any ulterior version. See 
[LICENSE](LICENSE.md) file for more information.
