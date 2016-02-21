The configuration file
======================

If you want to use Crowbook for your book, this configuration file
is all you'll have to add (assuming you'll already have the book in
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

* a list of options, under the form `key: value`;
* a list of Markdown files.

Files starting with the `#` characters are comments and are discarded
by Crowbook when parsing the files. Note that `#` must be at the
*beginning* of the line, so e.g.:

```
author: John Smith # aka John Doe
```

will set the `author` key to `John Smith # aka John Doe`.

The list of files
-----------------

There are various options to include a markdown file.

* `+ file_name.md` includes a numbered chapter.
* `- file_name.md` includes an unnumbered chapter.
* `! file_name.md` includes a chapter whose title won't be displayed
  (except in the toc for epub); this is useful for e.g. including a
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
+ chapter_3.md
...
```

There are two important things to note:

1. you must *not* use quotes around the file names
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

If you have more than one, Crowbook won't get too angry at you and
will just print a warning and treat it as another chapter (numbered
according to the scheme specified for including the file). It will
however mess the table of contents if Crowbook tries to generate one
(e.g. for Epub). 

It's also a problem if you do *not* have a level-1 header in a
markdown file. If it is a numbered chapter Crowbook will still be
able to infer a chapter name, but if it is not numbered Crowbook
will fail to generate an Epub file.

****

So, to sum it up. *please*: one file = one chapter, a chapter starts with a
title, and this way this will work nice.


Crowbook options 
----------------

The first part of the configuration file is dedicated to pass options
to Crowbook. Each one is of the form `option: value`. Note that you
don't have to put string in quotes, e.g.:

```
title: My title
```

If you *do* use quotes, Crowbook will actually put those quotes in the
string, so basically don't do that.

### Metadata ###

#### author ####

Quite obviously, the author of the book. Note that it's currently just
a single string, so if you want to have multiple authors, you'll have
to do something like:

```
author: Jane Doe, John Smith
```

**default**: `Anonymous`

#### title ####

The title of the book.

**default**: `Untitled`

#### lang ####

The language of the book, in a standard format. "en", "fr", and so on.

**default**: `en`

#### cover ####

The file name of a cover image for the book. Note that, here again,
you must not use quotes:

```
cover: cover.png
```

**default**: `None`

#### subject ####

What your book is about: e.g. Programming, Science-Fiction...

**default**: `None`

#### description ####

A description of your book. Note that Crowbook does *not* support
multi-line strings in configuration field, and it is a field where it
might be a problem if you don't like very long lines.

**default**: `None`

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
format, and only this one, by using the `--to` argument on the command line).

#### output.epub ####

The name of the epub file you want to generate.

**default**: `None`

Crowbook use the `zip` command to generate the epub, so it won't work
if this command is not installed on your system.

#### output.html ####

The name of the HTML file you want to generate. Note that this HTML
file is self-contained, it doesn't require e.g. CSS from other files.

**default**: `None`

#### output.tex ####

The name of the LaTeX file you want to generate. 

**default**: `None`

#### output.pdf ####

The name of the PDF file you want to generate. Crowbook uses LaTeX to
generate it, so it won't work if it isn't installed on your
computer. (See `tex_command` to specify which flavour of LaTeX you
want to use.)

**default**: `None`

### output.odt ###

The name of the ODT (OpenOffice) file you want to generate. Beware: 
ODT support is still experimental.

**default**: `None`

As for Epub generation, Crowbook depends on the presence of the `zip`
command to generate this file.

### Generic options for rendering  ###

#### numbering ####

A boolean that sets whether or not you want numbering. Setting it to
`false` is equivalent to including all your chapters with `-
my_chapter.md`. Note that even if it is set to `true`, numbering will
be desactivated for chapters that are included with `- my_chapter.md`.

**default**:: `true`


#### numbering_template ####

A string will be used as chapter title. You can use `{{number}}` and
`{{title}}` in this string, e.g.:

```
numbering_template: Chapter {{number}} {{title}}
```

Note that:
* this string isn't used for unnumbered chapters;
* this string isn't used for LaTeX, either.

**default**: `{{number}}. {{title}}`

#### autoclean ####

This option cleans a bit the input markdown. With the default
implementation, it only removes consecutive spaces, which has not real
impact (they are ignored anyway both by HTML viewers and by LaTeX).

However, if `lang` is set to `fr`, it also tries to add non-breaking
spaces in front (or after) characters like '?', '!', ';' to respect
french typography.

**default**: `true`

#### nb_char ####

This option allows you to specify the non breaking character used by
the french cleaning method (see above). Probably not really something
you need to modify. 

**default**: `' '` (i.e. narrrow non-breaking space)

### Options for HTML rendering ###

#### html.template ####

A file containing a (mustache) HTML template.

**default**: `None` (built-in template)

#### html.css ####

A file containing a stylesheet for the HTML file.

**default**: `None` (built-in)

### Options for Epub rendering ###

#### epub.template ####

A file containing a (mustache) xhtml template for the files generated
for each chapter.

**default**: `None` (built-in template)

#### epub.css ####

A file containing a stylesheet for the Epub file.

**default**: `None` (built-in)

### epub_version ###

Sets the version for generated Epub, either 2 or 3.

**default**: `2`

### Options for LaTeX / PDF rendering ###

#### tex.template ####

Sets the LaTeX template.

**default**: `None` (builtin)

#### tex.command ####

The command used to generate a PDF file.

**default**: `pdflatex`

#### tex.links_as_footnotes ####

Display the link in the document, using the `\footnote{}`
command. This is useful if you want your readers to be able to see the
URLs in a printed document.

**default**: `true`

### Additional options ###

#### temp_dir ####

When it is generating epub or pdf files, Crowbook creates a temporary
directory (which is then removed), named from a random uuid (so we can
be pretty certain it's not gonna exist). This option specify where to
create this directory. E.g., if you set:

```
temp_dir: /tmp
```

crowbook might create a temporary directory
`/tmp/7fcbe41e-1676-46ba-b1a7-40c2fa37a3a7`.

By default, this temporary directory is created where the config file
is.

**default**: `.`




