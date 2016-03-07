The configuration file
======================

If you want to use Crowbook for your book, this configuration file
is all you'll have to add (assuming you already have the book in
Markdown files; if you don't, you'll also have to write a book first,
but that's besides the scope of this document).

The format is not very complicated. This is an example of it:

```
# metadata
author: Joan Doe
title: Some book
lang: en

output.html: some_book.html

# list of chapters
- preface.md
+ chapter_1.md
+ chapter_2.md
+ chapter_3.md
+ chapter_4.md
- epilogue.md
```
Basically, it is divided in two parts:

* a list of options, under the form `key: value`, following YAML syntax.
* a list of Markdown files.

Lines starting with the `#` characters are comments and are discarded.

The list of files
-----------------

There are various options to include a markdown file.

* `+ file_name.md` includes a numbered chapter.
* `- file_name.md` includes an unnumbered chapter.
* `! file_name.md` includes a chapter whose title won't be displayed
  (except in the table of contents); this is useful for e.g. including a
  copyright at the beginning or the book, or for short stories where
  there is only one chapter.
* `42. file_name.md` specifies the number for a chapter.

So a typical usage might look like this:

```
! copyright.md
- preface.md
# We want first chapter to be Chapter 0 because we are programmers!
0. chapter_0.md
# Next chapters can be numbered automatically
+ chapter_1.md
+ chapter_2.md
...
```

There are two important things to note:

1. you must *not* use quotes around the file names.
2. the path of these files are relative to the directory where your
   config file is, *not* to the directory where you are when running
   `crowbook`. E.g. you can run `crowbook
   books/my_trilogy/first_book/config.book` without being in the
   book's directory.

Also note that you don't have to specify a title. This is because the title
of the chapter is inferred from the Markdown document. To go back to
our previous example:

```
+ chapter_1.md
```

does not specify a chapter title, because it will read it directly in
`chapter_1.md`, e.g.:

```markdown
The day I was born
==================

...
```

You should have one and only one level-one header (i.e. chapter title)
in each markdown file.

If you have more than one, Crowbook will print a warning and treat it
as another chapter (numbered according to the scheme specified for
including the file). It might however mess the table of contents in
some cases (e.g. for Epub). 

If you do *not* have a level-1 header in a
markdown file:
* if it is a numbered chapter, Crowbook will infer a chapter name from
the numbering scheme;
* if it is not numbered, chapter's title will default to the empty
string and won't be displayed in the TOC.


Crowbook options 
----------------

The first part of the configuration file is dedicated to pass options
to Crowbook. This is
[YAML syntax](https://en.wikipedia.org/wiki/YAML), so each line should
be of the form `key: value`. Note that in most cases you don't have to
put string in quotes, e.g.:

```
title: My title
```

It is however possible (and sometimes necessary) to escape some
characters to use quotes around strings:

```
title: "My: title!"
```


It is possible to use multiline strings with `>-` and
then indenting the lines that are part of the string:

```
title: >-
 A
 long
 title
author: Joan Doe
```

will set `title` to `"A long title"`. See
[block literals in YAML](https://en.wikipedia.org/wiki/YAML#Block_literals)
for more information on the various way to insert multiline strings
(which mostly change the way newlines will or won't be inserted).

A final note on the syntax: all options must be set *before* the first
chapter inclusion (that is, a line beginning with '+', '-', 'x.'
(where `x` is a number) or '!'). 

Here is the complete list of options, with a short description. The
usage of some of them is detailed later on.

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
    - **type**: path
    - **default value**: `not set`
    -  File name of the cover of the book

###  Output options ###
- **`output.epub`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for EPUB rendering
- **`output.html`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for HTML rendering
- **`output.html_dir`**
    - **type**: path
    - **default value**: `not set`
    -  Output directory name for HTML rendering
- **`output.tex`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for LaTeX rendering
- **`output.pdf`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for PDF rendering
- **`output.odt`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for ODT rendering

###  Resources option ###
- **`resources.base_path`**
    - **type**: path
    - **default value**: `not set`
    -  Path where to find resources (in the source tree). By default, links and images are relative to the Markdown file. If this is set, it will be to this path.
- **`resources.base_path.links`**
    - **type**: path
    - **default value**: `not set`
    -  Set base path but only for links. Useless if resources.base_path is set.
- **`resources.base_path.images`**
    - **type**: path
    - **default value**: `.`
    -  Set base path but only for images. Useless if resources.base_path is set.
- **`resources.base_path.files`**
    - **type**: path
    - **default value**: `.`
    -  Set base path but only for additional files. Useless if resources.base_path is set.
- **`resources.out_path`**
    - **type**: path
    - **default value**: `data`
    -  Paths where additional resources should be copied in the EPUB file or HTML directory
- **`resources.files`**
    - **type**: string
    - **default value**: `not set`
    -  Whitespace-separated list of files to embed in e.g. EPUB file

###  Misc options ###
- **`enable_yaml_blocks`**
    - **type**: boolean
    - **default value**: `false`
    -  Enable inline YAML blocks to override options set in config file
- **`zip.command`**
    - **type**: string
    - **default value**: `zip`
    -  Command to use to zip files (for EPUB/ODT)
- **`numbering`**
    - **type**: integer
    - **default value**: `1`
    -  The  maximum heading levels to number (0: no numbering, 1: only chapters, ..., 6: all)
- **`display_toc`**
    - **type**: boolean
    - **default value**: `false`
    -  If true, display a table of content in the document
- **`toc_name`**
    - **type**: string
    - **default value**: `Table of contents`
    -  Name of the table of contents if toc is displayed in line
- **`autoclean`**
    - **type**: boolean
    - **default value**: `true`
    -  Toggles cleaning of input markdown (not used for LaTeX)
- **`verbose`**
    - **type**: boolean
    - **default value**: `false`
    -  If set to true, print warnings in Markdown processing
- **`side_notes`**
    - **type**: boolean
    - **default value**: `false`
    -  Display footnotes as side notes in HTML/Epub
- **`temp_dir`**
    - **type**: path
    - **default value**: ``
    -  Path where to create a temporary directory (default: uses result from Rust's std::env::temp_dir())
- **`numbering_template`**
    - **type**: string
    - **default value**: `"{{number}}. {{title}}"`
    -  Format of numbered titles
- **`nb_char`**
    - **type**: char
    - **default value**: `' '`
    -  The non-breaking character to use for autoclean when lang is set to fr

###  HTML options ###
- **`html.template`**
    - **type**: path
    - **default value**: `not set`
    -  Path of an HTML template
- **`html.css`**
    - **type**: path
    - **default value**: `not set`
    -  Path of a stylesheet to use with HTML rendering
- **`html.script`**
    - **type**: path
    - **default value**: `not set`
    -  Path of a javascript file

###  Multifile HTML options ###
- **`html_dir.css`**
    - **type**: path
    - **default value**: `not set`
    -  Path of a CSS template
- **`html_dir.index.html`**
    - **type**: path
    - **default value**: `not set`
    -  Path of index.html template
- **`html_dir.chapter.html`**
    - **type**: path
    - **default value**: `not set`
    -  Path of a chapter.html template

###  EPUB options ###
- **`epub.version`**
    - **type**: integer
    - **default value**: `2`
    -  The EPUB version to generate
- **`epub.css`**
    - **type**: path
    - **default value**: `not set`
    -  Path of a stylesheet to use with EPUB rendering
- **`epub.template`**
    - **type**: path
    - **default value**: `not set`
    -  Path of an epub template for chapter

###  LaTeX options ###
- **`tex.short`**
    - **type**: boolean
    - **default value**: `false`
    -  If set to true, use article class instead of book and a the default `\maketitle` command
- **`tex.links_as_footnotes`**
    - **type**: boolean
    - **default value**: `true`
    -  If set to true, will add foontotes to URL of links in LaTeX/PDF output
- **`tex.command`**
    - **type**: string
    - **default value**: `pdflatex`
    -  LaTeX flavour to use for generating PDF
- **`tex.template`**
    - **type**: path
    - **default value**: `not set`
    -  Path of a LaTeX template file

###  Deprecated options ###
- **`base_path`**
    - **type**: DEPRECATED
    - **default value**: `resources.base_path`
    -  Renamed
- **`base_path.links`**
    - **type**: DEPRECATED
    - **default value**: `resources.base_path.links`
    -  Renamed
- **`base_path.images`**
    - **type**: DEPRECATED
    - **default value**: `resources.base_path.images`
    -  Renamed

Note that these options have a type, which in most case should be
pretty straightforward (a boolean can be `true` or `false`, an integer
must be composed a number, a string is, well, any string). The `path`
type might puzzle you a 
bit, but it's equivalent a string, except Crowbook will consider it
relatively to the book file.


### Output options ###

These options specify which files to generate. You must at least set
one of this option, or Crowbook won't do anything.

Recall that all file paths are relative to the directory where the
config file is, not to the one where you run `crowbook`. So if you set

```
output.epub = foo.epub
```

and runs

```bash
$ crowbook some/dir/config.book
```

`foo.epub` will be generated in `some/dir`, not in your current
directory.

Crowbook will try to generate each of the `output.xxx` files that are
specified. That means that you'll have to set at least one of those if you want a call to

```
$ crowbook my.book
```

to generate anything. (It's still possible to generate a specific
format, and only this one, by using the `--to` argument on the command
line).

Note that some formats depend on some commands being installed on your
system. Most notably, Crowbook depends on LaTeX (`pdflatex` by
default, though you can specify the command to use with `tex.command`) to generate a PDF file,
so PDF rendering won't work if it is not installed on your
system. Crowbook also uses the `zip` command to generate the EPUB and
ODT, files.

Current output options are:

* `output.html`: renders a standalone HTML file;
* `output.html_dir`: render a HTML directory with one page by chapter;
* `output.epub`: renders an EPUB file;
* `output.tex`: renders a LaTeX file;
* `output.pdf`: renders a PDF file (using `tex.command`);

### Resources options ###

These two options allow to embed additional files for some formats
(currently, only EPUB). This can be useful for embedding fonts.

#### resources.files ####

A list of files or directories that should be added. It's a
whitespace-separated list, so it can be, e.g.:

```
resources.files: font1.otf font2.otf
```

It is also possible to specify a directory (or multiple
directories). So if you have a `fonts` directories containing
`font1.otf` and `font2.otf`,

```
resources.files: fonts
```

will be equivalent to:

```
resources.files fonts/font1.otf fonts/font2.otf
```


**default**: not set

#### resources.path ####

This option determine where (in which directory), *in the resulting
document*, will those files be copied. The default is `data`, so by
default the `resources.files` in the first example above will search
`font1.otf` and `font2.otf` *in the same directory than the `.book`
file, and will copy them to `data/font1.otf` and `data/font2.otf` *in
the EPUB file*. This is therefore this last path that you should use
if you want to access those files e.g. in a custom CSS stylesheet.

Note that if you pass directories to `resources.files`, the whole
directory would be copied. So assuming `font/` contains `font1.otf`
and `font2.otf`

```
resources.files: fonts
resources.path: data
```

will copy these two files to `data/fonts/font1.otf` and
`data/fonts/font2.otf` (and not `data/font1.otf` and `data/font2.otf`).

Similarly, the whole path of `resources.files` is copied, so

```
resources.files: fonts/font1.otf fonts/font2.otf
```

will yield the same result.

**default**: `data`

### Generic options for rendering  ###

#### numbering ####

An integer that represents the maximum level of numbering for your
book. E.g., `1` will only number chapters, while `2` will number
chapters, sections, but not anything below that. `6` is the maximum  level
and turns numbering on for all headers.

**default**: `1`


#### numbering_template ####

A string that will be used for chapter titles. You can use `{{number}}` and
`{{title}}` in this string, e.g.:

```
numbering_template: "Chapter {{number}} {{title}}"
```

Note that:
* in this case, quoting is necessary because `{` and `}` have special
  meaning in YAML;
* this string won't be used for unnumbered chapters;
* this string isn't currently used by LaTeX, either.

#### autoclean ####

This option cleans a bit the input markdown. With the default
implementation, it only removes consecutive spaces, which has not real
impact (they are ignored anyway both by HTML viewers and by LaTeX).

However, if `lang` is set to `fr`, it also tries to add non-breaking
spaces in front (or after) characters like '?', '!', ';' to respect
french typography.




