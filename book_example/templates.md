Templates 
=========

List of templates 
-----------------

Create a new template 
---------------------

### `--print-template` argument ###

### Mustache syntax ###

List of accessible variables 
----------------------------

### Metadata ###

For every template, Crowbook exports all of the metadata:

* `author`;
* `title`;
* `lang`;
* `subject`;
* `description`;
* `license`;
* `version`;
* `date`;
* any option `metadata.foo` defined in the book
  configuration file will also be exported as `metadata_foo`.

These metadata can contain Markdown, which will be rendered. E.g.,
setting `date: "20th of **september**"` will render `september` in
bold, using `<b>` tag for HTML or `\textbf` for LaTeX. (It might be a
bad idea to insert Markdown into `author` or `title` fields, and it
certainly is for `lang`, but it an be useful for custom metadata or
for fields like `description`).

### Localisation strings ###

For all templates, Crowbook also exports some localisation strings.

### Template-dependent values ###

Crowbook also exports some additional fields for some templates, see
below.

Templates
---------

Crowbook allows the user to specify a number of templates. Some of
them, though are not "real" templates, they are just files that are
inserted, but can't contain mustache tags. This will probably evolve
in future versions.

### html.js ###

The javascript file used by both the standalone HTML renderer and the multiple files HTML renderer.

This is not currently an actual template, just a plain
javascript file which cannot contain `mustache` tags.

### html.css ###

The main CSS file used by both the standalone HTML renderer and the
multiple files HTML renderer.

Besides the default elements available in all templates, it
contains the following one:

* A variable whose name corresponds to the `lang` option is set to
  `true`. This means that it is possible to use `{{#foo}}...{{/foo}}`
  to have a CSS block that will only be inserted for language
  `foo`. This is used e.g. to render lists differently if `lang` is
  set to `fr`. 

### html.css.print ###

An additional CSS file used by both the standalone HTML renderer and
the multiple files HTML renderer. Its purpose is to provide CSS
instructions for printing (i.e., when the user clicks the `print`
button in her browser).

This is not currently an actual template, just a plain
CSS file which cannot contain `mustache` tags.

### html.highlight.js ###

A javascript file used by both HTML renderers to highlight codes in
code blocks. It should be a variant of
[highlight.js](https://highlightjs.org/).

This is not an actual template, just a plain javascript file.

### html.highlight.css ###

A CSS file used by both HTML renderers to set the theme of
[highlight.js](https://highlightjs.org/). It should, though, be an
highlight.js theme. 

This is not an actual template, just a plain CSS file.

### html_single.js ###

A javascript file used only by the standalone HTML renderer. Its main
purpose is to handle the displaying of a single chapter at a time when
`one_chapter` is set to true.

Besides the default elements available in all templates, it contains
the following ones:

* `common_script` contains the `html.js` file.
* `one_chapter` is set to true if `html_single.one_chapter` is true,
  else it is not present.
* `book_svg` and `pages_svg` are set to contain the (base64-encoded)
  images that serve as buttons to switch between displaying one
  chapter at a time or displaying all the chapters, when
  `html_single.one_chapter` is set to `true`.

### html_single.html ###

The main template for standalone HTML renderer.

Besides the default elements available in all templates, it contains
the following ones:

* A variable whose name corresponds to the `lang` option is set to
  `true`.
* `one_chapter` is set to true if `html_single.one_chapter` is true,
  else it is not present.
* `content` contains the book content, rendered as HTML.
* `toc` contains the table of content, rendeded as HTML.
* `script` contains the javascript file (that is, a rendered version
  of `html_single.js`).
* `style` contains the CSS file (that is, a rendered version
  of `html.css`).
* `print_style` contains the CSS file for print media (that is, html.css.print).
* `menu_svg` contains the (base64-encoded) hamburger-menu image.
* `book_svg` and `pages_svg` are set to contain the (base64-encoded)
  images that serve as buttons to switch between displaying one
  chapter at a time or displaying all the chapters, when
  `html_single.one_chapter` is set to `true`.
* `footer` and `header` contains the contents of options `html.footer`
  and `html.header`.
* If `html.highlight_code` is set to true in the book file, this
  template also has `highlight_code` set to true, `higlight_css` set
  to the content of `html.highlight.css` and `highlight_js` set to the
  base64-encoded content of `html.highlight.js`.

  
### html_dir.chapter.html ###

The main template for multiple files HTML renderer. It is the template
for rendering each chapter. 

Besides the default elements available in all templates, it contains
the following ones:

* `content` contains the chapter's content, rendered as HTML.
* `script` contains the `html.js` file.
* `chapter_title` contains the chapter's title.
* `toc` contains the table of content, rendeded as HTML.
* `prev_chapter` and `next_chapter` contains the titles of
  previous and next chapters, with a link to them.
* `footer` and `header` contains the contents of options `html.footer`
  and `html.header`.
* If `html.highlight_code` is set to true in the book file, this
  template also has `highlight_code` set to true.

### html_dir.index.html ###

The template used by multiple files HTML renderer to render the
`index.html` file.

Besides the default elements available in all templates, it contains
the following ones:

* `content` contains the title page's content, which will typically
  include the cover.
* `script`, `header`, `footer`, `toc`, `script`, `lang`,
`highlight_code` are similar to `html_dir.chapter.html`.

### tex.template ###

This template is used by the LaTeX renderer.

Besides the default elements available in all templates, it contains
the following ones:

* `content`.
* `class` contains the `tex.class` option.
* If `tex.class` is set to `book`, this template has the variable
  `book` set to true.
* `tex_lang` is set to the latex string corresponding to `lang` that
  must be used with babel.
* If `rendering.initials` is true, the variable `initials` is set to
true.

### epub.chapter.xhtml ###

This template is the main template used by the Epub renderer. It
contains the XHTML template that will be used for each chapter.

* `content`
* `chapter_title`

### epub.css ###

This template is used by the Epub renderer and contains the style
sheet.

* A variable whose name corresponds to the `lang` option is set to
  `true`.
