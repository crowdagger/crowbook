Crowbook
========

Yet another converter from Markdown to (HTML, LaTeX, Epub).

[![Build Status](https://travis-ci.org/lise-henry/crowbook.svg?branch=master)](https://travis-ci.org/lise-henry/crowbook)

Usage
-----

```
$ crowbook some_file.book # or
$ cargo run some_file.book
```

Parse the config file and generate book in HTML, Epub and/or Latex/PDF
(according to config file).

For more information see the book_example directory.

Features
--------

### Done ###
* Support for some simple formatconfiguration files to
  list chapters and metadatas, so you just run `crowbook
  some_file` and you don't have to pass more options, it generates the
  rest.
* Support for basic Markdown features useful in writing novels.
* Support for Epub2 format as output.
* Basic support for LaTeX format as output, and PDF through it.
* Support for HTML format as output.
* Support for basic french typography in HTML/Epub format, and by
that I mostly mean non-breaking spaces.
* Decent default templates and CSS.
* Some configuration for HTML/Epub templates and CSS.

### ToDo ###
* Support Epub3.
* Allow more customization.
* Provide a binary which accepts some option and not just an input
file.
* Support for easily embedding custom fonts (and other files) in Epub/HTML.
* Support for ODT as output format.
* Correct support for technical books.

See also [Bugs](Bugs.md).

License 
-------

Currently, MIT but this might change.

Acknowledgements
----------------

Besides the Rust compiler and standard library, Crowbook uses the
following libraries:

* [pulldown-cmark](https://crates.io/crates/pulldown-cmark) (for
parsing markdown)
* [mustache](https://crates.io/crates/pulldown-cmark) (for templating)
* [chrono](https://crates.io/crates/chrono) (date and time library)
* [uuid](https://crates.io/crates/uuid) (to generate uuid)

While Crowbook directly doesn't use them, there was also some
inspiration from [Pandoc](http://pandoc.org/) and [mdBook](https://github.com/azerupi/mdBook).


