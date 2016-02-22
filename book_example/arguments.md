Arguments 
=========

Crowbook can takes a list of arguments:

```bash
Render a markdown book in Epub, PDF or HTML.

USAGE:
        crowbook [OPTIONS] <BOOK> [--] [ARGS]

OPTIONS:
        --create               Creates a new book with existing markdown files.
    -h, --help                 Prints help information
    -o, --output <FILE>        Specifies output file.
    -s, --set <KEY> <VALUE>    Sets a book option.
    -t, --to <FORMAT>          Generate specific format [values: epub, pdf, html, tex, odt]
    -V, --version              Prints version information
    -v, --verbose              Activate verbose mode

ARGS:
    <BOOK>        File containing the book configuration.
    <FILES>...    Files to list in book when using --create

Command line options allow to override options defined in <BOOK> configuration file. 
E.g., even if this file specifies 'verbose: false', calling 'crowbook --verbose <BOOK>' 
will activate verbose mode.

Note that Crowbook generates output files relatively to the directory where <BOOK> is:
$ crowbook foo/bar.book --to pdf --output baz.pdf
will thus generate baz.pdf in directory foo and not in current directory.
```

The most important ones is obviously <BOOK>, i.e. the file
configuration book. It is mandatory: if you don't pass it, `crowbook`
will simply display this help message. In a normal use case this is
the only argument you'll need to pass, and `crowbook` will generate
the book in all formats specified in the configuration file.

It is, however, possible to pass more arguments to `crowbook`.

`--create`
---------

**Usage**: `crowbook --create <BOOK> file_1.md file_2.md ...`

Creates a new book from a list of Markdown files. It will generate the
file `BOOK` (or abort if it already exists) with all file names
specified added as chapters.

### Example ###

```
crowbook --create foo.book README.md ChangeLog.md LICENSE.md
```

will generate a file `foo.book` containing:

```
author: Your name
title: Your title
lang: en

# Uncomment and fill to generate files
# output.html: some_file.html
# output.epub: some_file.epub
# output.pdf: some_file.pdf

# Uncomment and fill to set cover image (for Epub)
# cover: some_cover.png

# List of chapters
+ README.md
+ ChangeLog.md
+ LICENSE.md
```

`--set` 
-------

**usage**: `crowbook <BOOK> --set [KEY] [VALUE]...

This options takes a list `KEY` `VALUE` pairs and allows to set or
override a book configuration option. All valid options in the
configuration files are valid as keys. For more information, see
[the configuration file page](config.md).

### Examples ###

```
$ crowbook foo.book --set html.css style.css
```

will override the CSS for HTML generation (the `html.css` key) to `style.css`.

```
$ crowbook foo.book --set author Foo --title Bar
```

will override the book title to `Bar` and its author to `Foo`.

`--verbose`
-----------

**usage**: `crowbook --verbose <BOOK>`

If this flag is set, Crowbook will print some more messages.

`--to`
------

**usage**: `crowbook --to <FORMAT> <BOOK>`

(or `crowbook -t <FORMAT> <BOOK>`)

Generate only the specified format. `FORMAT` must be either `epub`,
`pdf`, `html`, `odt` or `tex`.

If an output file for the format is not specified in the book
configuration file, `crowbook` will fail to render PDF, ODT and Epub
(whereas it will print HTML and Tex files on stdout). It is however
possible to specify a file with the `--output` option.

### Examples ###

```
crowbook --to html foo.book
```

will generate some HTML, and prints it either to the file specified by
`output.html` in `foo.book`, or to stdout.

```
crowbook --to pdf --output foo.pdf foo.book
```

will (try to) generate a `foo.pdf` file,.

`--output`
---------

**usage**: `crowbook --to <FORMAT> --output <FILE> <BOOK>`

(or `crowbook -t <FORMAT> -o <FILE> <BOOK>`)

Specifies an output file. Only valid when `--to` is used.

Note that Crowbook generates output files relatively to the directory
where `BOOK` is:
```
$ crowbook foo/bar.book --to pdf --output baz.pdf
```
will thus generate `baz.pdf` in directory `foo` and not in current
directory.
