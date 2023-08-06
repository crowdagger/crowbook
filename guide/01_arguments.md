# Arguments

Crowbook can take a number of arguments, generally in the form:

```bash
crowbook [OPTIONS] [BOOK]
```

The most important argument is obviously the book configuration file.
It is mandatory in most cases:
if you don't pass it, `crowbook` will simply display an error.
In a normal use case this is the only argument you'll need to pass, as most options will be set in this configuration file.

It is, however, possible to pass more arguments to `crowbook`:

## `--create`

**Usage**:

```bash
crowbook [BOOK] --create file_1.md file_2.md ...
```

or:

```bash
crowbook [BOOK] -c file_1.md file_2.md ...
```

Creates a new book from a list of Markdown files.
It will generate a book configuration file with all file names specified as chapters.
It either prints the result to `stdout` (if `BOOK` is not specified) or generates the file `BOOK` (or abort if it already exists).

```bash
crowbook foo.book --create  chapter_1.md chapter_2.md chapter_3.md
```

will thus generate a file `foo.book` containing:

```yaml
author: Your name
title: Your title
lang: en

## Output formats

# Uncomment and fill to generate files
# output.html: some_file.html
# output.epub: some_file.epub
# output.pdf: some_file.pdf

# Or uncomment the following to generate PDF, HTML and EPUB files based on this file's name
# output: [pdf, epub, html]

# Uncomment and fill to set cover image (for EPUB)
# cover: some_cover.png

## List of chapters
+ chapter_1.md
+ chapter_2.md
+ chapter_3.md
```

while

```bash
crowbook --create chapter_1.md chapter_2.md chapter_3.md
```

will print the same result, but to `stdout` (without creating a file).

## `--single`

**Usage**:

```bash
crowbook --single <FILE>
```

or:

```bash
crowbook -s <FILE>
```

This argument allows you to give `crowbook` a single Markdown file.
This file can contain an inline YAML block to set some book options.
Inline YAML blocks must start and end with a line containing only `---` (three dashes).

E.g:

```yaml
---
author: Joan Doe
title: A short story
output: [html, epub, pdf]
---

Content of the story in Markdown.
```

If this YAML block is not at the beginning of a file, it must also be preceded by a blank line.

This allows to not have to write a `.book` configuration file for a short story or an article.
`crowbook -s foo.md` is rougly equivalent to having a book configuration file containing:

```yaml
! foo.md
```

That is, the chapter heading (if any) won't be displayed in the output documents (though they still appear in the TOC).

> Note that by default, using `--single` or `-s` sets the default LaTeX class of the book to `article` instead of `book`.

## `--set`

**Usage**:

```bash
crowbook <BOOK> --set [KEY] [VALUE]...
```

This argument takes a list of  `KEY` `VALUE` pairs and allows setting or overriding a book configuration option.
All valid options in the configuration files are valid as keys.
For more information, see [the configuration file](02_config.md).

```bash
$ crowbook foo.book --set tex.paper.size a4paper
```

will override the paper size for PDF generation.

## `--list-options`

**Usage**:

```bash
crowbook --list-options
```

or:

```bash
crowbook -l
```

Displays all the valid options that can be used, whether in a book configuration file, with `--set`, or in an inline YAML block.

## `--print-template`

**Usage**:

```bash
crowbook --print-template <TEMPLATE>
```

Prints the built-in template to `stdout`.
Useful if you want to customize the appearance of your document.

E.g., if you want to modify the CSS used for HTML rendering:

```bash
$ crowbook --print-template html.css > my_style.css
# edit my_style.css in your favourite editor
$ crowbook my.book --set html.css my_style.css
# or add "html.css: my_style.css" in my.book
```

## `--stats`

**Usage**:

```bash
crowbook --stats <BOOK>
```

or:

```bash
crowbook -S <BOOK>
```

Display some statistics (word and character counts) about the book.

## `--autograph`

**Usage**:

```bash
crowbook --autograph <BOOK>
```

or:

```bash
crowbook -a <BOOK>
```

Prompts for a an autograph execution.
This is a Markdown block that will be inserted at the beginning of the book.

### Example

```text
$ crowbook --autograph my.book
CROWBOOK 0.14.0
Enter autograph:
To my dear friend John,

Cheers, *Joan*
^D
```

will add the block of text that was entered to all output files.

## `--verbose`

**Usage**:

```bash
crowbook <BOOK> --verbose
```

If this flag is set, Crowbook will print more warnings it detects while parsing and rendering.

## `--to`

**Usage**:

```bash
crowbook <BOOK> --to [FORMAT]
```

or:

```bash
crowbook <BOOK> -t [FORMAT]
```

Generate only the specified format.
`FORMAT` must be either `epub`, `pdf`, `html`, `html.dir`, `odt` or `tex`.

If an output file for the format is not specified in the book configuration file, `crowbook` will fail to render PDF, ODT and EPUB, whereas it will print HTML and TeX files on stdout.
It is, however,  possible to specify a file with the `--output` option.

### Examples

```bash
crowbook --to html foo.book
```

will generate some HTML, and prints it either to the file specified by `output.html` in `foo.book`, or to stdout if it is not specified.

```bash
crowbook --to pdf --output foo.pdf foo.book
```

will generate a `foo.pdf` file.

## `--output`

**Usage**:

```bash
crowbook <BOOK> --to <FORMAT> --output <FILE>
```

or:

```bash
crowbook -t <FORMAT> -o <FILE> <BOOK>
```

Specifies an output file.
Only valid when `--to` is used.

## `--lang`

**Usage**:

```bash
crowbook --lang <LANG>
```

or:

```bash
crowbook -L <LANG>
```

Set the runtime language used by Crowbook.
Currently, only a french translation is available.
By default, Crowbook uses the `LANG` environment variable to determine which language to use, but this option allows to override it (e.g. for operating systems that don't use such an option, such as Windows).

### Example

```bash
$ crowbook --lang fr --help
```

will display Crowbook's help message in french.

> Note that this argument has nothing to do with the `lang` option
> that you can set in the book configuration file, which specifies the
> language *of the book*. This argument specifies the language of the text messages
> that Crowbook will display while running, but has no effect on the generated documents.
