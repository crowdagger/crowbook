The configuration file
======================

If you want to use Crowbook for your book, this configuration file
is all you'll have to add (assuming you already have the book in
Markdown files; if you don't, you'll also have to write a book first,
but that's besides the scope of this document).

The format is not very complicated. This is an example of it:

```yaml
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


Configuration in an inline YAML block
---------------------------------------------

Sometimes, you only have one Markdown file and might not want to have
a separate configuration file. In this case, you can specify options
at the beginning of your Markdown file, using an inline YAML block,
separated by two lines containing only `---`:

```markdown
---
author: Joan Doe
title: Some (short) book
lang: en

output.html: some_book.html
---

# Some (short) book

The book content, formatted in Markdown.
```

This method only allows to set up options: you can't include a list of
chapters in this way, since the only "chapter" that will be included
is this Markdown file itself.

You can then use 

```bash
crowbook -s some_book.md
```

to generate output formats from this Markdown file.

> By default (unless `input.yaml_blocks` is set to true), Crowboook
> will only read those inline blocks when it is runned with `crowbook
> --single` (or `crowbook -s`). 


The list of files
-----------------

There are various options to include a Markdown file.

* `+ file_name.md` includes a numbered chapter.
* `- file_name.md` includes an unnumbered chapter.
* `! file_name.md` includes a chapter whose title won't be displayed
  (except in the table of contents); this is useful for e.g. including a
  copyright at the beginning or the book, or for short stories where
  there is only one chapter.
* `42. file_name.md` specifies the number for a chapter.
* `@` includes a part instead of a chapter.

So a typical usage might look like this:

```yaml
! copyright.md
- preface.md
0. chapter_0.md # We want to start at chapter 0 instead of 1
# Next chapters can be numbered automatically
+ chapter_1.md
+ chapter_2.md
...
```

There are two important things to note:

1. you must *not* use quotes around the file names.
2. the path of these files are relative to the directory where your
   configuration file is. This means you can run `crowbook
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
# The day I was born #
...
```

You should have one and only one level-one header (i.e. chapter title)
in each Markdown file.

If you have more than one, Crowbook will print a warning and treat it
as another chapter (numbered according to the scheme specified for
including the file). It might however mess the table of contents in
some cases (e.g. for EPUB). 

If you do *not* have a level-1 header in a
markdown file:
* if it is a numbered chapter, Crowbook will infer a chapter name from
the numbering scheme;
* if it is not numbered, chapter's title will default to the empty
string.


### Parts ###

Parts are included using the `@` character, followed by the same characters than for chapters:

```
@+ numbered_part.md
+ chapter_01.md
+ chapter_02.md
@- unnumbered_part.md
+ chapter_03.md
+ chapter_04.md
@42. part_with_number_42.md
+ chapter_05.md
```

However, you usually don't really want to have a content directly
below the part, only chapters (though it can be useful to add an
introduction before the first chapter of this part), so there is also
a more straighforward way to use parts, using only the `@` character
followed by the (markdown-formatted) title of this part:

```
@ Beginning
+ chapter_01.md
+ chapter_02.md
@ Middle
+ chapter_03.md
+ chapter_04.md
@ Appendix
- notes.md
```

With this shortcut, parts are always numbered. So 

```
@ Beginning
```

is equivalent to 

```
@+ some_file.md
```

with `some_file.md` containing only a title:

```markdown
# Beginning
```



Crowbook options 
----------------

The first part of the configuration file is dedicated to pass options
to Crowbook. This is
[YAML syntax](https://en.wikipedia.org/wiki/YAML), so each line should
be of the form `key: value`. Note that in most cases you don't have to
put string in quotes, e.g.:

```yaml
title: My title
```

It is however possible (and sometimes necessary) to escape some
characters to use quotes around strings:

```yaml
title: "My: title!"
```


It is possible to use multiline strings with `>-` and
then indenting the lines that are part of the string:

```yaml
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




### Metadata ###

Metadata are data about the book. Except for `cover`, which points to
an image file, all its fields are strings. The main metadata are:

* `author`: the author(s) of the book.
* `title`: the title of the book.
* `lang`: the language of the book. The unicode language code should
be used, e.g. `en_GB` or `en`, `fr_FR`, ...
* `cover`: path to an image file for the cover of the book (not displayed in all output formats).

There are also additional metadata:

* `subject`
* `description`
* `license`
* `version`
* `date`

You can define your own metadata by starting an option name with
`metadata.foo`.

All metadata are accessible from templates, see
[Templates](templates.md).

### The `import` special option ###

The special `import` option allows you to include the options
of another book configuration file. E.g., assuming that
you want some common options to be applied to both `foo.book`
and `bar.book`, you can create a `common.book` file:

```yaml
author: Joan Doe
lang: en
license: "Copyright (C) Joan Doe. All rights reserved."

html.header: "[Joan Doe's website](http://joan-doe.com)"
tex.template: my_template.tex
```

You can then include this file in `foo.book`:

```yaml
import: common.book
title: Foo

+ foo_01.md
+ foo_02.md
```

Or include it in `bar.book`, but override some of its features:

```yaml
import: common.book
title: Bar
license: CC-BY-SA  # Override the license from common.book

+ bar_01.md
```

### Output options ###

These options specify which files to generate.

Note that all file paths are relative to the directory where the
config file is, not to the one where you run `crowbook`. So if you set

```yaml
output.epub: foo.epub
```

and runs

```bash
$ crowbook some/dir/config.book
```

`foo.epub` will be generated in `some/dir`, not in your current
directory.

Crowbook will try to generate each of the `output.xxx` files that are
specified. That means that you'll have to set at least one of those if you want a call to

```bash
$ crowbook my.book
```

to generate anything. (It's still possible to generate a specific
format, and only this one, by using the `--to` and `--output` argument on the command
line).

Note that some formats depend on some commands being installed on your
system. Most notably, Crowbook depends on LaTeX (`xelatex` by
default, though you can specify the command to use with `tex.command`) to generate a PDF file,
so PDF rendering won't work if it is not installed on your
system. Crowbook also uses the `zip` command to generate the EPUB and
ODT files.

Current output options are:

* `output.html`: renders a standalone HTML file.
* `output.html.dir`: render a HTML directory with one page by chapter.
* `output.epub`: renders an EPUB file.
* `output.tex`: renders a LaTeX file.
* `output.pdf`: renders a PDF file (using `tex.command`).

(There are other output options for generating proofreading files, see
[Proofreading](proofreading.md).)

#### `output.base_path` ####

Additionally, the `output.base_path` option allows you to set where
the output files will be written (relatively to the book configuration
file). E.g.,

```yaml
output.base_path: docs/book
output.epub: book.epub
```

will render the EPUB file in `docs/book/book.epub`.

### Input options ###

Crowbook does its best to improve the typography of your text. Default
settings should be good enough for most usages, but you 
can enable/disable specific options: 

* `input.clean`: if set to `false`, will disable all typographic
  "cleaning" (default: `true`). The `clean` algorithm is
  dependent on the language, though currently there is only a variant
  implemented for `fr` (french), dealing with the specific
  non-breaking spaces rules for this language.
* `input.clean.smart_quotes`: if set to `false`, disable the "smart
  quote" feature, that (tries to) replace straight quotes with curly
  ones. As it is an heuristics and can't be perfect, you might want to
  disable it in some circumstances (default: `true`).
* `input.clean.ligature_dashes`: if set to `true`, will convert `--`
  to en dash (`–`) and `---` to em dash (`—`). This can be useful if
  you want to use these characters but can't access them easily on
  your keymap; however, as it can also cause problems if you *do* want
  to have two successive dashes, it is disabled by default.
* `input.clean.ligature_guillemets` is a similar feature for french 'guilemets', replacing
  `<<` and `>>` to `«` and `»`. For the same reason, it is also
  disabled by default.

### Generic options for rendering  ###

These options allow to configure the rendering; they are used (or at
least should be) for all formats.

* `rendering.highlight`: specify if and how to perform syntax
  highlighting for code blocks. Default value is `highlight.js`; this
  will use [`highlight.js`](https://highlightjs.org/) for HTML
  rendering, and will not perform any syntax highlighting for other
  output formats. Other valid values are:
  * `syntect`: uses the [syntect](https://crates.io/crates/syntect)
    library to perform syntax highlighting. This has the advantage of
    also enabling syntax highlighting for LaTeX/PDF and EPUB formats;
    however syntect support is still experimental in Crowbook.
  * `none`: disable syntax highlighting.
* `rendering.highlight.theme`: only used if `rendering.highlight` is
  set to `syntect`, selects the theme to use for syntax
  highlighting. Default is "InspiredGitHub". Valid theme names are
  "InspiredGitHub", "Solarized (dark)", "Solarized (light)",
  "base16-eighties.dark", "base16-mocha.dark", "base16-ocean.dark" and
  "base16-ocean.light".
* `rendering.num_depth`: an integer that represents the maximum level of numbering for your
book. E.g., `1` will only number chapters, while `2` will number
chapters, sections, but not anything below that. `6` is the maximum  level
and turns numbering on for all headers. (Default is `1`.)
* `rendering.chapter.template` and `rendering.part.template`: the
  strings that will be used for chapter and part titles. It is
  possible to include Markdown formatting in this template, but it
  isn't advised, because it might cause problems for some formats
  (e.g. your EPUB file might not be correct anymore). 
* `rendering.part.roman_numerals` and
  `rendering.chapter.roman_numerals`: these two booleans allow you to
  specify if you want roman numerals for part or chapter numbers
  (default is `true` for part numbers, and `false` for chapter
  numbers).
* `rendering.inline_toc`: if set to true, Crowbook will include a table of contents at the
beginning of the document.
* `rendering.inline_toc.name`: the name of this table of contents as
  it should be displayed in the document.
* `rendering.initials`: if set to true, Crowbook will use initials, or "lettrines", displaying
the first letter of each chapter bigger than the others.
* `rendering.part.reset_counter`: set it to `false` if you don't want
your chapter numbers to start again at 1 at each part.

### HTML Options ###

These options allow you to customize the HTML rendering (used both by
the default HTML standalone renderer and the HTML multifile renderer):

* `html.icon`: allows to set a `favicon` for the page.
* `html.header` and `html.footer` allow to set a custom (Markdown)
  string at the top and at the bottom of the HTML page. This is
  actually a template, so you can access
  metadata, such as `{{{author}}}`, `{{{title}}}`, or `{{{version}}}`
  in it. See the [template](templates.md) chapter for more information
  on the fields you can use.
* `html.css` allow to set up a custom CSS file. You can also redefine
  the colours in a file and set it using `html.css.colours`. 
* `html.highlight.theme`: similar to `rendering.highlight.theme` but
  only sets the theme for HTML output. 

#### Options for standalone HTML  ####

There are a few options specific to the standalone HTML renderer
(default, set with `output.html`):

* `html.standalone.one_chapter`, if set to true, will only display one
  chapter at a time (using Javascript), making it look similarly to
  the multifile HTML.
* `html.standalone.template` allows you to change or modify the HTML
  template for standalone HTML.

### Options for LaTeX/PDF rendering ###

These options allow you to customize the LaTeX renderer (and, thus,
the generated PDF documents):

* `tex.template` specifies a different LaTeX template.
* `tex.class` changes the LaTeX class used.
* `tex.paper_size` and `tex.font.size` (default `a5paper` and `10pt`)
  allows to modify the page and font size .
* `tex.links_as_footnotes` can be set to `false` if you don't want
  links to also appear as footnotes (which means losing them
  if it is actually printed).
* `tex.highlight.theme`: similar to `rendering.highlight.theme`, but
  only sets the theme for LaTeX/PDF rendering.
  
### Options for EPUB rendering ###

There are also options specific to the EPUB format:

* `epub.version` can be set to 2 or 3 (default 2).
* `epub.css` can be useful if you want to specify a customized stylesheet.
* `epub.highlight.theme`: similar to `rendering.highlight.theme` but
  only sets a theme for EPUB output.

### Resources options ###

These options allow to embed additional files for some formats
(currently, only EPUB). This can be useful for embedding fonts.

#### resources.files ####

A list of files or directories that should be added. It's a
whitespace-separated list, so it can be, e.g.:

```yaml
resources.files: font1.otf font2.otf
```

It is also possible to specify a directory (or multiple
directories). So if you have a `fonts` directories containing
`font1.otf` and `font2.otf`,

```yaml
resources.files: fonts
```

will be equivalent to:

```yaml
resources.files: fonts/font1.otf fonts/font2.otf
```


**default**: not set

#### resources.out_path ####

This option determine where (in which directory), *in the resulting
document*, those files will be copied. The default is `data`, so by
default the `resources.files` in the first example above will search
`font1.otf` and `font2.otf` *in the same directory than the `.book`
file*, and will copy them to `data/font1.otf` and `data/font2.otf` *in
the EPUB file*. This is therefore this last path that you should use
if you want to access those files e.g. in a custom CSS stylesheet.

Note that if you pass directories to `resources.files`, the whole
directory would be copied. So assuming `fonts/` contains `font1.otf`
and `font2.otf`

```yaml
resources.files: fonts
resources.path: data
```

will copy these two files to `data/fonts/font1.otf` and
`data/fonts/font2.otf` (and not `data/font1.otf` and `data/font2.otf`).

Similarly, the whole path of `resources.files` is copied, so

```yaml
resources.files: fonts/font1.otf fonts/font2.otf
```

will yield the same result.

**default**: `data`

## Full list of options ##

Here is the complete list of options. You can always look at it by running `crowbook --list-options` or `crowbook -l`.

### Metadata ###
- **`author`**
    - **type**: metadata
    - **default value**: `""`
    -  Author of the book
- **`title`**
    - **type**: metadata
    - **default value**: `""`
    -  Title of the book
- **`lang`**
    - **type**: metadata
    - **default value**: `en`
    -  Language of the book
- **`subject`**
    - **type**: metadata
    - **default value**: `not set`
    -  Subject of the book (used for EPUB metadata)
- **`description`**
    - **type**: metadata
    - **default value**: `not set`
    -  Description of the book (used for EPUB metadata)
- **`cover`**
    - **type**: path
    - **default value**: `not set`
    -  Path to the cover of the book

### Additional metadata ###
- **`license`**
    - **type**: metadata
    - **default value**: `not set`
    -  License of the book
- **`version`**
    - **type**: metadata
    - **default value**: `not set`
    -  Version of the book
- **`date`**
    - **type**: metadata
    - **default value**: `not set`
    -  Date the book was revised

### Output options ###
- **`output.epub`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for EPUB rendering
- **`output.html`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for HTML rendering
- **`output.html.dir`**
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
- **`output.base_path`**
    - **type**: path
    - **default value**: `""`
    -  Directory where those output files will we written

### Rendering options ###
- **`rendering.highlight`**
    - **type**: string
    - **default value**: `syntect`
    -  If/how highligh code blocks. Possible values: "syntect" (default, performed at runtime), "highlight.js" (HTML-only, uses Javascript), "none"
- **`rendering.highlight.theme`**
    - **type**: string
    - **default value**: `InspiredGitHub`
    -  Theme for syntax highlighting (if rendering.highlight is set to 'syntect')
- **`rendering.initials`**
    - **type**: boolean
    - **default value**: `false`
    -  Use initials ('lettrines') for first letter of a chapter (experimental)
- **`rendering.inline_toc`**
    - **type**: boolean
    - **default value**: `false`
    -  Display a table of content in the document
- **`rendering.inline_toc.name`**
    - **type**: string
    - **default value**: `"{{{loc_toc}}}"`
    -  Name of the table of contents if it is displayed in document
- **`rendering.num_depth`**
    - **type**: integer
    - **default value**: `1`
    -  The  maximum heading levels that should be numbered (0: no numbering, 1: only chapters, ..., 6: all)
- **`rendering.chapter.template`**
    - **type**: string
    - **default value**: `"{{{number}}}\\. {{{chapter_title}}}"`
    -  Naming scheme of chapters
- **`rendering.chapter.roman_numerals`**
    - **type**: boolean
    - **default value**: `false`
    -  If set to true, display chapter number with roman numerals
- **`rendering.part.template`**
    - **type**: string
    - **default value**: `"{{{loc_part}}} {{{number}}}"`
    -  Naming scheme of parts
- **`rendering.part.roman_numerals`**
    - **type**: boolean
    - **default value**: `true`
    -  If set to true, display part number with roman numerals
- **`rendering.part.reset_counter`**
    - **type**: boolean
    - **default value**: `true`
    -  If set to true, reset chapter number at each part

### Special option ###
- **`import`**
    - **type**: path
    - **default value**: `not set`
    -  Import another book configuration file

### HTML options ###
- **`html.icon`**
    - **type**: path
    - **default value**: `not set`
    -  Path to an icon to be used for the HTML files(s)
- **`html.highlight.theme`**
    - **type**: string
    - **default value**: `not set`
    -  If set, set theme for syntax highlighting for HTML output (syntect only)
- **`html.header`**
    - **type**: string
    - **default value**: `not set`
    -  Custom header to display at the beginning of html file(s)
- **`html.footer`**
    - **type**: string
    - **default value**: `not set`
    -  Custom footer to display at the end of HTML file(s)
- **`html.css`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a stylesheet for HTML rendering
- **`html.css.add`**
    - **type**: string
    - **default value**: `not set`
    -  Some inline CSS added to the stylesheet template
- **`html.css.colours`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a stylesheet for the colours for HTML
- **`html.js`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a javascript file
- **`html.css.print`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a media print stylesheet for HTML rendering
- **`html.highlight.js`**
    - **type**: template path
    - **default value**: `not set`
    -  Set another highlight.js version than the bundled one
- **`html.highlight.css`**
    - **type**: template path
    - **default value**: `not set`
    -  Set another highlight.js CSS theme than the default one
- **`html.side_notes`**
    - **type**: boolean
    - **default value**: `false`
    -  Display footnotes as side notes in HTML/Epub (experimental)
- **`html.escape_nb_spaces`**
    - **type**: boolean
    - **default value**: `true`
    -  Replace unicode non breaking spaces with HTML entities and CSS

### Standalone HTML options ###
- **`html.standalone.template`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of an HTML template for standalone HTML
- **`html.standalone.one_chapter`**
    - **type**: boolean
    - **default value**: `false`
    -  Display only one chapter at a time (with a button to display all)
- **`html.standalone.js`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a javascript file

### Multifile HTML options ###
- **`html.dir.template`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a HTML template for multifile HTML

### EPUB options ###
- **`epub.version`**
    - **type**: integer
    - **default value**: `2`
    -  EPUB version to generate (2 or 3)
- **`epub.highlight.theme`**
    - **type**: string
    - **default value**: `not set`
    -  If set, set theme for syntax highlighting for EPUB output (syntect only)
- **`epub.css`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a stylesheet for EPUB
- **`epub.css.add`**
    - **type**: string
    - **default value**: `not set`
    -  Inline CSS added to the EPUB stylesheet template
- **`epub.chapter.xhtml`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of an xhtml template for each chapter
- **`epub.toc.extras`**
    - **type**: boolean
    - **default value**: `true`
    -  Add 'Title' and (if set) 'Cover' in the EPUB table of contents
- **`epub.escape_nb_spaces`**
    - **type**: boolean
    - **default value**: `true`
    -  Replace unicode non breaking spaces with HTML entities and CSS

### LaTeX options ###
- **`tex.highlight.theme`**
    - **type**: string
    - **default value**: `not set`
    -  If set, set theme for syntax highlighting for LaTeX/PDF output (syntect only)
- **`tex.links_as_footnotes`**
    - **type**: boolean
    - **default value**: `true`
    -  Add foontotes to URL of links so they are readable when printed
- **`tex.command`**
    - **type**: string
    - **default value**: `xelatex`
    -  LaTeX command to use for generating PDF
- **`tex.template`**
    - **type**: template path
    - **default value**: `not set`
    -  Path of a LaTeX template file
- **`tex.template.add`**
    - **type**: string
    - **default value**: `not set`
    -  Inline code added in the LaTeX template
- **`tex.class`**
    - **type**: string
    - **default value**: `book`
    -  LaTeX class to use
- **`tex.paper_size`**
    - **type**: string
    - **default value**: `a5paper`
    -  Specifies the size of the page.
- **`tex.title`**
    - **type**: boolean
    - **default value**: `true`
    -  If true, generate a title with \maketitle
- **`tex.font.size`**
    - **type**: integer
    - **default value**: `not set`
    -  Specify latex font size (in pt, 10 (default), 11, or 12 are accepted)
- **`tex.stdpage`**
    - **type**: boolean
    - **default value**: `false`
    -  If set to true, use 'stdpage' package to format a manuscript according to standards

### Resources option ###
- **`resources.files`**
    - **type**: string
    - **default value**: `not set`
    -  Whitespace-separated list of files to embed in e.g. EPUB file; useful for including e.g. fonts
- **`resources.out_path`**
    - **type**: path
    - **default value**: `data`
    -  Paths where additional resources should be copied in the EPUB file or HTML directory
- **`resources.base_path`**
    - **type**: path
    - **default value**: `not set`
    -  Path where to find resources (in the source tree). By default, links and images are relative to the Markdown file. If this is set, it will be to this path.
- **`resources.base_path.links`**
    - **type**: path
    - **default value**: `not set`
    -  Set base path but only for links. Useless if resources.base_path is set
- **`resources.base_path.images`**
    - **type**: path
    - **default value**: `.`
    -  Set base path but only for images. Useless if resources.base_path is set
- **`resources.base_path.files`**
    - **type**: path
    - **default value**: `.`
    -  Set base path but only for additional files. Useless if resources.base_path is set.
- **`resources.base_path.templates`**
    - **type**: path
    - **default value**: `.`
    -  Set base path but only for templates files. Useless if resources.base_path is set

### Input options ###
- **`input.clean`**
    - **type**: boolean
    - **default value**: `true`
    -  Toggle typographic cleaning of input markdown according to lang
- **`input.clean.smart_quotes`**
    - **type**: boolean
    - **default value**: `true`
    -  If enabled, tries to replace vertical quotations marks to curly ones
- **`input.clean.ligature.dashes`**
    - **type**: boolean
    - **default value**: `false`
    -  If enabled, replaces '--' to en dash ('–') and '---' to em dash ('—')
- **`input.clean.ligature.guillemets`**
    - **type**: boolean
    - **default value**: `false`
    -  If enabled, replaces '<<' and '>>' to french "guillemets" ('«' and '»')
- **`input.yaml_blocks`**
    - **type**: boolean
    - **default value**: `false`
    -  Enable inline YAML blocks to override options set in config file

### Crowbook options ###
- **`crowbook.temp_dir`**
    - **type**: path
    - **default value**: ``
    -  Path where to create a temporary directory (default: uses result from Rust's std::env::temp_dir())
- **`crowbook.zip.command`**
    - **type**: string
    - **default value**: `zip`
    -  Command to use to zip files (for EPUB/ODT)

### Output options (for proofreading) ###
- **`output.proofread.html`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for HTML rendering with proofread features
- **`output.proofread.html.dir`**
    - **type**: path
    - **default value**: `not set`
    -  Output directory name for HTML rendering with proofread features
- **`output.proofread.pdf`**
    - **type**: path
    - **default value**: `not set`
    -  Output file name for PDF rendering with proofread features

### Proofreading options (only for output.proofread.* targets) ###
- **`proofread`**
    - **type**: boolean
    - **default value**: `false`
    -  If set to false, will disactivate proofreading even if one of output.proofread.x is present
- **`proofread.languagetool`**
    - **type**: boolean
    - **default value**: `false`
    -  If true, try to use language tool server to grammar check the book
- **`proofread.languagetool.port`**
    - **type**: integer
    - **default value**: `8081`
    -  Port to connect to languagetool-server
- **`proofread.repetitions`**
    - **type**: boolean
    - **default value**: `false`
    -  If set to true, use Caribon to detect repetitions
- **`proofread.repetitions.max_distance`**
    - **type**: integer
    - **default value**: `25`
    -  Max distance between two occurences so it is considered a repetition
- **`proofread.repetitions.fuzzy`**
    - **type**: boolean
    - **default value**: `true`
    -  Enable fuzzy string matching
- **`proofread.repetitions.fuzzy.threshold`**
    - **type**: float
    - **default value**: `0.2`
    -  Max threshold of differences to consider two strings a repetition
- **`proofread.repetitions.ignore_proper`**
    - **type**: boolean
    - **default value**: `true`
    -  Ignore proper nouns for repetitions
- **`proofread.repetitions.threshold`**
    - **type**: float
    - **default value**: `2.0`
    -  Threshold to detect a repetition

Note that these options have a type, which in most case should be
pretty straightforward (a boolean can be `true` or `false`, an integer
must be composed by a number, a string is, well, any string). The `path`
type might puzzle you a 
bit, but it's equivalent to a string, except Crowbook will consider it
relatively to the book file. The `template path` type is just the
`path` of a template. Metadata are just strings.
