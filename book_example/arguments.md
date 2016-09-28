Arguments 
=========

Crowbook can takes a list of arguments:

```text
Render a markdown book in Epub, PDF or HTML.

USAGE:
    crowbook [FLAGS] [OPTIONS] [--] [BOOK]

FLAGS:
    -h, --help            Prints help information
    -l, --list-options    Lists all possible option
    -s, --single          Use a single Markdown file instead of a book configuration file
    -V, --version         Prints version information
    -v, --verbose         Print warnings in parsing/rendering

OPTIONS:
    -c, --create <FILES>...            Creates a new book with existing markdown files
    -o, --output <FILE>                Specifies output file
        --print-template <TEMPLATE>    Displays the default value of a template
        --set <KEY_VALUES>             Sets a list of book options
    -t, --to <FORMAT>                  Generate specific forma [values: epub, pdf, html, tex, odt]

ARGS:
    <BOOK>    File containing the book configuration, or a Markdown file when called with --single
```

Note that Crowbook generates output files relatively to the directory
where <BOOK> is[^1]:

[^1]: Unless the option `output.base_path` is set, see
[the configuration file](config.md).



```bash
$ crowbook foo/bar.book --to pdf --output baz.pdf
```
will thus generate baz.pdf in directory foo and not in current directory.

The most important option is obviously <BOOK>, i.e. the file
configuration book. It is mandatory for most options: if you don't
pass it, `crowbook` 
will simply display this help message. In a normal use case this is
the only argument you'll need to pass, and `crowbook` will generate
the book in all formats specified in the configuration file.

It is, however, possible to pass more arguments to `crowbook`.

`--create`
---------

**Usage**: `crowbook [BOOK] --create file_1.md file_2.md ...`

Creates a new book from a list of Markdown files. It will generate a
book configuration file with all file names specified as
chapters. It either prints the result to stdout (if `BOOK` is not
specified) or generate the file `BOOK` (or abort if it already
exists). 

### Examples ###

```bash
crowbook foo.book --create  README.md ChangeLog.md LICENSE.md
```

will generate a file `foo.book` containing:

```yaml
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

while

```bash
crowbook --create README.md ChangeLog.md LICENSE.md
```

will print the same result, but to stdout (without creating a file).

When `crowbook` is runned with `--create`, it can also use the
keys/values set by `--set` (see below):

```bash
$ crowbook foo.book --create file1.md file2.md --set author "Pierre Dupont" title "Mon œuvre" lang fr
```

will generate a `foo.book` file containing

```yaml
author: Pierre Dupont
title: Mon œuvre
lang: fr

# List of chapters
+ file1.md
+ file2.md
```

`--single`
----------

**usage**: `crowbook --single <FILE>`

(or `crowbook -s <FILE>`)

This argument allows to give `crowbook` a single Markdown file. This
file can contain an inline YAML block to set some book options. Inline
YAML blocks must start and end with a line with `---` (three dashes). E.g:

```yaml
---
author: Joan Doe
title: A short story
output.html: short.html
---
```

If this YAML block is not at the beginning of a file, it must also be
preceded by a blank line.

This allows to not have to write a `.book` configuration file for a
short story or an article. `crowbook --single foo.md` is rougly equivalent to having a book
configuration file containing:

```yaml
! foo.md
```

That is, the chapter heading (if any) won't be displayed in the output
documents (though they still appear in the TOC).

> Note that by default, using `--single` sets the default LaTeX class
> of the book to `article` instead of `book`.


`--set` 
-------

**usage**: `crowbook <BOOK> --set [KEY] [VALUE]...`

This argument takes a list `KEY` `VALUE` pairs and allows setting or
overriding a book configuration option. All valid options in the
configuration files are valid as keys. For more information, see
[the configuration file](config.md).

### Examples ###

```bash
$ crowbook foo.book --set html.css style.css
```

will override the CSS for HTML generation (the `html.css` key) to the
file `style.css`.

```bash
$ crowbook foo.book --set author Foo --title Bar
```

will override the book title to `Bar` and its author to `Foo`.

`--list-options`
----------------

**usage**: `crowbook --list-options`

(or `crowbook -l`)

Displays all the valid options to use, whether in a book configuration
file, with `--set`, or in an inline YAML block.

`--print-template`
------------------

**usage**: `crowbook --print-template template`

Prints to stdout the built-in template. Useful if you want to
customize the appearance of your document. E.g., if you want to modify
the CSS used for HTML rendering:

```bash
$ crowbook --print-template html.css > my_style.css
# edit my_style.css in your favourite editor
$ crowbook my.book --set html.css my_style.css
# or add "html.css: my_style.css" in my.book
```

Note that it is possible to use this option in conjonction with
`--set`, though it is currently only useful for EPUB template:

```bash
$ crowbook --print-template epub.template --set epub.version 2
# Returns the template for Epub 2 (currently it is the default one)
$ crowbook --print-template epub.template --set epub.version 3
# Returns the template for Epub 3
```


`--verbose`
-----------

**usage**: `crowbook <BOOK> --verbose`

If this flag is set, Crowbook will print the warnings it detects while
parsing and rendering. These warnings are typically related to the
inclusion of non-local images, linking to Markdown files that are not
part of the book, and so on.

`--to`
------

**usage**: `crowbook <BOOK>--to [FORMAT]`

(or `crowbook <BOOK> -t [FORMAT]`)

Generate only the specified format. `FORMAT` must be either `epub`,
`pdf`, `html`, `odt` or `tex`.

If an output file for the format is not specified in the book
configuration file, `crowbook` will fail to render PDF, ODT and EPUB,
whereas it will print HTML and Tex files on stdout. It is, however, 
possible to specify a file with the `--output` option.

### Examples ###

```bash
crowbook --to html foo.book
```

will generate some HTML, and prints it either to the file specified by
`output.html` in `foo.book`, or to stdout if it is not specified.

```bash
crowbook --to pdf --output foo.pdf foo.book
```

will generate a `foo.pdf` file,.

`--output`
---------

**usage**: `crowbook <BOOK> --to <FORMAT> --output <FILE> `

(or `crowbook -t <FORMAT> -o <FILE> <BOOK>`)

Specifies an output file. Only valid when `--to` is used.

Note that Crowbook generates output files relatively to the directory
where `BOOK` is (unless the option `output.base_path` is set):

```bash
$ crowbook foo/bar.book --to pdf --output baz.pdf
```
will thus generate `baz.pdf` in directory `foo` and not in current
directory.
