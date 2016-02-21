Crowbook
========

Creates a book in HTML, Epub or PDF format from markdown files.

[![Build Status](https://travis-ci.org/lise-henry/crowbook.svg?branch=master)](https://travis-ci.org/lise-henry/crowbook)

Usage
-----

The simplest command is:

```bash
$ crowbook <BOOK>
$ # or runninng from cargo:
$ cargo run <BOOK>
```

Where `BOOK` is a configuration file. Crowbook will then parse the
config file and generate book in HTML, Epub, LaTeX, and/or PDF,
according to the setting in the configuration file.

This configuration file contains some metadata, options, and list the
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
[the configuration file](book_example/config.md), or the whole
[book_example](book_example) directory.

It is also possible to give additional parameters to `crowbook`;
arguments set from the command line will override the ones set in the
`BOOK` configuration file.

```
USAGE:
        crowbook [FLAGS] [OPTIONS] <BOOK>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Activate verbose mode

OPTIONS:
        --autoclean <BOOL>    Try to clean input markdown [values: true, false]
        --numbering <BOOL>    Number chapters or not [values: true, false]
    -o, --output <FILE>       Specify output file
    -t, --to <FORMAT>         Generate specific format [values: epub, epub, pdf, html, tex]

ARGS:
    <BOOK>    A file containing the book configuration

Command line options allow to override options defined in <BOOK> configuration file. 
E.g., even if this file specifies 'verbose: false', calling 'crowbook --verbose <BOOK>' 
will activate verbose mode.

Note that Crowbook generates output files relatively to the directory where <BOOK> is:
$ crowbook foo/bar.book --to pdf --output baz.pdf
will thus generate baz.pdf in directory foo and not in current
directory.
```

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

ChangeLog
---------

See [ChangeLog](ChangeLog.md).

License 
-------

Crowbook is free software: you can redistribute it and/or modify it
under the terms of the GNU Lesser General Public License (LGPL),
version 2.1 or (at your option) any ulterior version. See 
[LICENSE](LICENSE.md) file for more information.

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


