Arguments 
=========

Crowbook can takes a list of arguments:

```bash
Render a markdown book in Epub, PDF or HTML.

USAGE:
        crowbook [OPTIONS] [--] [ARGS]

OPTIONS:
        --create              Creates a new book with existing markdown files.
    -h, --help                Prints help information
    -l, --list-options        Lists all possible option
        --list-options-md     List all options, formatted in Markdow
    -o, --output <FILE>       Specifies output file.
    -s, --set <KEY_VALUES>    Sets a list of book options
    -t, --to <FORMAT>         Generate specific format [values: epub, pdf, html, tex, odt]
    -V, --version             Prints version information
    -v, --verbose             Activate verbose mode

ARGS:
    <BOOK>        File containing the book configuration.
    <FILES>...    Files to list in book when using --create
```

Command line options allow to override options defined in <BOOK> configuration file. 
E.g., even if this file specifies 'verbose: false', calling 'crowbook --verbose <BOOK>' 
will activate verbose mode.

Note that Crowbook generates output files relatively to the directory
where <BOOK> is:
```
$ crowbook foo/bar.book --to pdf --output baz.pdf
```
will thus generate baz.pdf in directory foo and not in current directory.

The most important option obviously <BOOK>, i.e. the file
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
book configuration file with alle file names specified as
chapter. It either prints the result to stdout (if `BOOK` is not
specified) or generate the file `BOOK` (or abort if it already
exists). 

### Examples ###

```
crowbook foo.book --create  README.md ChangeLog.md LICENSE.md
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

while

```
crowbook --create README.md ChangeLog.md LICENSE.md
```

will prints the same result, but to stdout (without creating a file).

When `crowbook` is runned with `--create`, it can also uses the
keys/values set by `--set` (see below):

```
$ crowbook foo.book --create file1.md file2.md --set author "Pierre
Dupont" title "Mon œeuvre" lang fr
```

will generate a `foo.book` file containing

```
author: Pierre Dupont
title: Mon œeuvre
lang: fr

# List of chapters
+ file1.md
+ file2.md
```

`--set` 
-------

**usage**: `crowbook <BOOK> --set [KEY] [VALUE]...

(or `crowbook <BOOK> -s [KEY] [VALUE]...`

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

`--list-options`
----------------

**usage**: `crowbook --list-options`

(or `crowbook -l`)

Displays all the valid options to use either in a book configuration
file or with `--set`, with a short description. There is also
`--list-options-md`, which outputs markdown, giving the following
result:

###  Metadata ###
- **`author`**
    - **type**: string
    - **default value**: `Anonymous`
    -  The author of the book
- **`title`**
    - **type**: string
    - **default value**: `Untitled`
    -  The title of the book
- **`lang`**
    - **type**: string
    - **default value**: `en`
    -  The language of the book
- **`subject`**
    - **type**: string
    - **default value**: `not set`
    -  Subject of the book (used for EPUB metadata)
- **`description`**
    - **type**: string
    - **default value**: `not set`
    -  Description of the book (used for EPUB metadata)
- **`cover`**
    - **type**: string
    - **default value**: `not set`
    -  File name of the cover of the book

###  Output options ###
- **`output.epub`**
    - **type**: string
    - **default value**: `not set`
    -  Output file name for EPUB rendering
- **`output.html`**
    - **type**: string
    - **default value**: `not set`
    -  Output file name for HTML rendering
- **`output.tex`**
    - **type**: string
    - **default value**: `not set`
    -  Output file name for LaTeX rendering
- **`output.pdf`**
    - **type**: string
    - **default value**: `not set`
    -  Output file name for PDF rendering
- **`output.odt`**
    - **type**: string
    - **default value**: `not set`
    -  Output file name for ODT rendering

###  Misc options ###
- **`numbering`**
    - **type**: boolean
    - **default value**: `false`
    -  Toggles chapter numbering for whole book
- **`autoclean`**
    - **type**: boolean
    - **default value**: `true`
    -  Toggles cleaning of input markdown (not used for LaTeX)
- **`verbose`**
    - **type**: boolean
    - **default value**: `false`
    -  Toggle verbose mode
- **`side_notes`**
    - **type**: boolean
    - **default value**: `false`
    -  Display footnotes as side notes in HTML/Epub
- **`nb_char`**
    - **type**: char
    - **default value**: `' '`
    -  The non-breaking character to use for autoclean when lang is set to fr
- **`temp_dir`**
    - **type**: string
    - **default value**: `.`
    -  Path where to create a temporary directory
- **`numbering_template`**
    - **type**: string
    - **default value**: `{{number}}. {{title}}`
    -  Format of numbered titles

###  HTML options ###
- **`html.template`**
    - **type**: string
    - **default value**: `not set`
    -  Path of an HTML template
- **`html.css`**
    - **type**: string
    - **default value**: `not set`
    -  Path of a stylesheet to use with HTML rendering

###  EPUB options ###
- **`epub.version`**
    - **type**: integer
    - **default value**: `2`
    -  The EPUB version to generate
- **`epub.css`**
    - **type**: string
    - **default value**: `not set`
    -  Path of a stylesheet to use with EPUB rendering
- **`epub.template`**
    - **type**: string
    - **default value**: `not set`
    -  Path of an epub template for chapter

###  LaTeX options ###
- **`tex.links_as_footnotes`**
    - **type**: boolean
    - **default value**: `true`
    -  If set to true, will add foontotes to URL of links in LaTeX/PDF output
- **`tex.command`**
    - **type**: string
    - **default value**: `pdflatex`
    -  LaTeX flavour to use for generating PDF
- **`tex.template`**
    - **type**: string
    - **default value**: `not set`
    -  Path of a LaTeX template file



`--verbose`
-----------

**usage**: `crowbook <BOOK> --verbose`

If this flag is set, Crowbook will print some more messages.

`--to`
------

**usage**: `crowbook <BOOK>--to [FORMAT]`

(or `crowbook <BOOK> -t [FORMAT]`)

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

**usage**: `crowbook <BOOK> --to <FORMAT> --output <FILE> `

(or `crowbook -t <FORMAT> -o <FILE> <BOOK>`)

Specifies an output file. Only valid when `--to` is used.

Note that Crowbook generates output files relatively to the directory
where `BOOK` is:
```
$ crowbook foo/bar.book --to pdf --output baz.pdf
```
will thus generate `baz.pdf` in directory `foo` and not in current
directory.
