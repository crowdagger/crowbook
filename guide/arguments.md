Arguments 
=========

Crowbook can take a number of arguments:

```text
Render a Markdown book in EPUB, PDF or HTML.

USAGE:
    crowbook [OPTIONS] [--] [BOOK]

FLAGS:
    -h, --help            Print help information
    -l, --list-options    List all possible options
    -p, --proofread       Enable proofreading
    -q, --quiet           Don't print info/error messages
    -s, --single          Use a single Markdown file instead of a book configuration file
    -v, --verbose         Print warnings in parsing/rendering
    -V, --version         Print version information

OPTIONS:
    -c, --create <FILES>...            Create a new book with existing Markdown files
    -o, --output <FILE>                Specify output file
        --print-template <TEMPLATE>    Prints the default content of a template
        --set <KEY_VALUES>             Set a list of book options
    -t, --to <FORMAT>                  Generate specific format

ARGS:
    <BOOK>    File containing the book configuration file, or a Markdown file when called with --single
```

The most important option is obviously <BOOK>, i.e. the book
configuration file. It is mandatory in most cases: if you don't 
pass it, Crowbook will simply display this help message. In a normal use case this is
the only argument you'll need to pass, and Crowbook will generate
the book in all formats specified in the configuration file.

It is, however, possible to pass more arguments to `crowbook`:

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

When `crowbook` is run with `--create`, it can also use the
keys/values set by `--set` (see below):

```bash
$ crowbook foo.book --create file1.md file2.md --set author "Pierre Dupont" title "Mon œuvre" lang fr
```

will generate a `foo.book` file containing:

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
YAML blocks must start and end with a line containing only `---` (three dashes). E.g:

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
short story or an article. `crowbook -s foo.md` is rougly equivalent to having a book
configuration file containing:

```yaml
! foo.md
```

That is, the chapter heading (if any) won't be displayed in the output
documents (though they still appear in the TOC).

> Note that by default, using `--single` or `-s` sets the default LaTeX class
> of the book to `article` instead of `book`.


`--set` 
-------

**usage**: `crowbook <BOOK> --set [KEY] [VALUE]...`

This argument takes a list of  `KEY` `VALUE` pairs and allows setting or
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
$ crowbook foo.book --set author Foo title Bar
```

will override the book title to `Bar` and its author to `Foo`.

`--proofread`
-------------

**usage**: `crowbook --proofread <BOOK>`

(or `crowbook -p <BOOK>`)

Equivalent to `--set proofread true`. Enable proofreading. See [Proofreading](proofreading.md).

`--list-options`
----------------

**usage**: `crowbook --list-options`

(or `crowbook -l`)

Displays all the valid options to use, whether in a book configuration
file, with `--set`, or in an inline YAML block.

`--print-template`
------------------

**usage**: `crowbook --print-template template`

Prints the built-in template to stdout. Useful if you want to
customize the appearance of your document. E.g., if you want to modify
the CSS used for HTML rendering:

```bash
$ crowbook --print-template html.css > my_style.css
# edit my_style.css in your favourite editor
$ crowbook my.book --set html.css my_style.css
# or add "html.css: my_style.css" in my.book
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
`pdf`, `html`, `html.dir`, `odt` or `tex`.

If an output file for the format is not specified in the book
configuration file, `crowbook` will fail to render PDF, ODT and EPUB,
whereas it will print HTML and TeX files on stdout. It is, however, 
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

will generate a `foo.pdf` file.

`--output`
---------

**usage**: `crowbook <BOOK> --to <FORMAT> --output <FILE> `

(or `crowbook -t <FORMAT> -o <FILE> <BOOK>`)

Specifies an output file. Only valid when `--to` is used.


`--lang`
----------

**usage**: `crowbook --lang <LANG>`

(or `crowbook -L <LANG>`)

Set the runtime language used by Crowbook. Currently, only a french
translation is available. By default, Crowbook uses the `LANG`
environment variable to determine which language to use, but this
option allows to override it (e.g. for operating systems that don't
use such an option, such as Windows).

### Example 

`$ crowbook --lang fr --help`

will display Crowbook's help messages in french.

> Note that this argument has nothing to do with the `lang` option
> that you can set in the book configuration file, which specifies the
> language *of the book*. This argument specifies the language of the text messages
> that Crowbook will display while running.


