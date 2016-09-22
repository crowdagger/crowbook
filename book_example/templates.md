Templates 
=========

List of templates 
-----------------

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


### html_single.html ###

The main template for standalone HTML renderer.

### html_dir.chapter.html ###

The main template for multiple files HTML renderer. It is the template
for rendering each chapter. 

### html_dir.index.html ###

The template used by multiple files HTML renderer to render the
`index.html` file.

### tex.template ###

The main (and currently only) template used by the LaTeX renderer.


### epub.chapter.xhtml ###

This template is the main template used by the Epub renderer. It
contains the XHTML template that will be used for each chapter.

### epub.css ###

This template is used by the Epub renderer and contains the style
sheet.


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

|    Mustache tag     |    Value    |   Available in...   |
|---------------------|-------------|---------------------|
| `content` | A rendered version of the book or chapter's content | `html_single.html`, `html_dir.index.html`, `html_dir.chapter.html`, `tex.temlplate`, `epub.chapter.xhtml` |
| `toc` | A rendered vesion of the table of contents | `html_single.html`, `html_dir.chapter.html`, `html_dir.index.html` |
| `footer` | The content of `html.footer` | `html_single.html`, `html_dir.index.html`, `html_dir.chapter.html` |
| `header` | The content of `html.header` | `html_single.html`, `html_dir.index.html`, `html_dir.chapter.html` |
| `script` | The javascript file for this HTML document | `html_single.html`, `html_dir.index.html`, `html_dir.chapter.html` |
| `style` | The CSS file for this HTML document, that is, a rendered version of `html.css` | `html_single.html` |
| A variable whose whose name corresponds to `lang` in book options | `true`  | `html.css`, `epub.css` |
| `chapter_title` | The title of current chapter | `html_dir.chapter.html`, `epub.chapter.xhtml` |
| `highlight_code` | True if `html.highlight_code` is true | `html_single.html`, `html_dir.chapter.html` |
| `highlight_css` | The content of `html.highlight.css` | `html_single.html` |
| `highlight_js` | The base64-encoded content of `html.highlight.js` | `html_single.html` |
| `common_script` | The content of `html.js` | `html_single.js` |
| `one_chapter`   | True if `html_single.one_chapter` is true, else not present | `html_single.html`, `html_single.js` |
| `book.svg` | The base64-encoded image of the button to display all chapters | `html_single.js`, `html_single.html` |
| `pages.svg` | The base64-encoded image of the button to display one chapter at a time | `html_single.js`, `html_single.html` |
| `menu_svg` | The base64-encoded image of the hamburger menu image | `hml_single.html` |
| `prev_chapter` | Title and a link of previous chapter | `html_dir.chapter.html` |
| `next_chapter` | Title and a link of nexts chapter | `html_dir.chapter.html` |
| `class` | The content of `tex.class` | `tex.template` |
| `book`  | True if `tex.class` is `book`, not set else | `tex.template` |
| `tex_lang` | The babel equivalent of `lang` | `tex.template` |
| `initials` | True if `rendering.initials` is true, not set else | `tex.template` |



