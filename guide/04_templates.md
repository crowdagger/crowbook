# Templates

Crowbook allows the user to specify a number of templates.[^1]

Each of this template can be overriden by a custom one, by setting e.g.:

```yaml
html.css: my_template.css
```

in the book configuration file.
The templates that you are most susceptible to modify are the following:

* `html.css`: stylesheet for HTML output;
* `epub.css`: stylesheet for EPUB output;
* `tex.template`: template of a LaTeX file.

[^1]: Some of them, though, are not "real" templates, they are just files that are inserted, but can't contain mustache tags.
This will probably evolve in future versions.


## Create and edit template

Except for inline templates, which are set directly in the book configuration file:

```yaml
# Template that modify how a chapter title is displayed
rendering.chapter.template: "{{{loc_chapter}}} {{{number}}}: {{{chapter_title}}}"

# CSS code added to default CSS templates (but don't override it)
html.css.add: "h1 { background-color: red; }"
epub.css.add: "h1 { background-color: gray; }"

# LaTeX code added to default LaTeX template (but doesn't override it)
tex.template.add: "\\usepackage{libertineotf}"
```

most templates must be in a separate file:

```yaml
tex.template: my_template.tex
```

### `--print-template`

The easiest way to create a new template is to start with the default one.
In order to do so, you can use the `--print-template` argument:

```bash
$ crowbook --print-template tex.template > my_template.tex
```

In order to get the `chapter.xhtml` template for EPUB3, you'll also have to use `--set epub.version 3`:

```bash
$ crowbook --print-template epub.chapter.xhtml --set epub.version 3 > my_epub3_template.xhtml
```

### Mustache syntax

Crowbook uses
[rust-mustache](https://crates.io/crates/mustache) as its templating engine, which allows to use
[Mustache](http://mustache.github.io/)
syntax in the templates.

It mainly boils down to using `{{{foo}}}`[^2] to insert the value of variable `foo` in the document:

```html
<h1 class = "title" >{{{title}}}<h1>
<h2 class = "author">{{{author}}}</h2>
```

Mustache also provides the possibility of checking whether a variable is set:

```
{{#foo}}
Foo exists
{{/foo}}
{{^foo}}
Foo does not exist
{{^foo}}
```

Crowbook uses this and sets some variables to `true` to allow templates to conditionally include some portions.
E.g., in `html.css`:

```css
{{#lang_fr}}
/* Make list displays '–' instead of bullets */
ul li {
    list-style-type: '–';
    padding-left: .5em;
}
{{/lang_fr}}
```

In this case, Crowbook sets a variable whose name is equal to `lang_foo` to `true`, allowing to have different styles for some elements according to the language.

For more information about Mustache syntax, see the
[Mustache manual](http://mustache.github.io/mustache.5.html).

#### Syntax in LaTeX ####

Since LaTeX already uses a lot of curly brackets, the default template sets an altenative syntax to access variables, with `<<&foo>>`[^3]:

```latex
\title{<<&title>>}
\author{<<&author>>}
<<#has_date>>\date{<<&date>>}<</has_date>
```


[^2]: Mustache also provides the `{{foo}}` variant, which HTML-escapes the content of the variable.
You should not use this, as Crowbook a ready renders and correctly escapes the variables it sets for use in templates.


[^3]: `<<foo>>` might also work, but the ampersand is required to prevent mustache HTML-escaping the value.
This is not good because:
1) escaping is already done by Crowbook before setting variable content;
2) escaping HTML in a LaTeX document won't probably look good.

## List of templates

### html.js

The javascript file used by both the standalone HTML renderer and the multiple files HTML renderer.

This is not currently an actual template, just a plain javascript file which cannot contain `mustache` tags.

### html.css

The main CSS file used by both the standalone HTML renderer and the multiple files HTML renderer.

### html.css.colors

A CSS file containing only colour settings.
Used by `html.css`.

This is not currently an actual template, just a plain CSS file which cannot contain `mustache` tags.

### html.css.print

An additional CSS file used by both the standalone HTML renderer and the multiple files HTML renderer.
Its purpose is to provide CSS instructions for printing (i.e., when the user clicks the `print` button in her browser).

This is not currently an actual template, just a plain CSS file which cannot contain `mustache` tags.

### html.highlight.js

A javascript file used by both HTML renderers to highlight codes in code blocks.
It should be a variant of
[highlight.js](https://highlightjs.org/).

This is not an actual template, just a plain javascript file.

### html.highlight.css

A CSS file used by both HTML renderers to set the theme of
[highlight.js](https://highlightjs.org/).
It should, though, be an `highlight.js` theme.

This is not an actual template, just a plain CSS file.

### html.standalone.js

A javascript file used only by the standalone HTML renderer.
Its main purpose is to handle the displaying of a single chapter at a time when `one_chapter` is set to true.

### html.standalone.template

The main HTML template for standalone HTML renderer.

### html.dir.template

The main HTML template for multiple files HTML renderer.

### tex.template

The main (and currently only) template used by the LaTeX renderer.

### epub.chapter.xhtml

This template is the main template used by the Epub renderer.
It contains the XHTML template that will be used for each chapter.

### epub.titlepage.xhtml

This template is the template used by the Epub renderer for the title page.

### epub.css

This template is used by the Epub renderer and contains the style sheet.

### Inline templates

Crowbook also has some inline templates, that are set in the book configuration file:

* `tex.template.add`, `html.css.add` and `epub.css.add` allow to specify some LaTeX or CSS code directly in the book configuration file.
  This code will be added respectively to `tex.template`, `html.css` or `epub.css` template.
  For CSS templates, this code is inserted at the end of the template (allowing to redefine rules that are set by the template);
  for the LaTeX template, the code is inserted at the end of the preambule, just before the `\begin{document}` tag, allowing to redefine commands.
* `rendering.inline_toc.name` sets the name of the inline table of content, if it is displayed.
  By default, is is set to `{{{loc_toc}}}`, that is, a localised version of "Table of Contents".
* `rendering.chapter.template` sets the naming scheme for chapters, while `rendering.part.template` does the same for part.
  These are used only for text-only output, such as in the TOC.
  `html.chapter.template` and `html.part.template` allow to change the HTML formatting for parts and chapters.
  *These options should probably only be used if you know what you're doing, as they can break the document.*
  If you only need to change the name of chapters or parts, use `rendering.part` and `rendering.chapter` instead.

## List of accessible variables

### Metadata

For every template, Crowbook exports all of the metadata:

* `author`;
* `title`;
* `subtitle`;
* `lang`;
* `subject`;
* `description`;
* `license`;
* `version`;
* `date`;
* any option `metadata.foo` defined in the book configuration file will also be exported as `metadata_foo`.

These metadata can contain Markdown, which will be rendered.
E.g., setting `date: "20th of **september**"` will render `september` in bold, using `<b>` tag for HTML or `\textbf` for LaTeX.
If you need to use these data in places that don't support formatted text (e.g. in meta tags), you can use the raw content by accessing `xxx_raw` instead (e.g., `author_raw`, `title_raw`, ...).
(Note that the content of the raw metadata is *not* HTML-escaped, so in this case you might want to use `{{xxx_raw}}` instead of `{{{xxx_raw}}}`.)

For each metadata `foo` that is set, Crowbook also inserts a `has_foo` bool set to true.
This allows to use Mustache's section for some logic, e.g.:

```
{{{title}}}
{{#has_version}}, version {{{version}}}{{/has_version}}
```

will avoid rendering ", version" when `version` is not set.

### Localisation strings

For all templates, Crowbook also exports some localisation strings `loc_foo`.
They currently include:

| Localisation key            | Value in english             |
|-----------------------------|------------------------------|
| `loc_toc`                   | Table of contents            |
| `loc_cover`                 | Cover                        |
| `loc_title`                 | Title                        |
| `loc_chapter`               | Chapter                      |
| `loc_part`                  | Part                         |
| `loc_notes`                 | Notes                        |
| `loc_display_all`           | Display all chapters         |
| `loc_display_one`           | Display one chapter          |

### Template-dependent values

Crowbook also exports some additional fields for some templates, see below.

|    Mustache tag     |    Value    |   Available in...   |
|---------------------|-------------|---------------------|
| `content` | A rendered version of the book or chapter's content | `html.standalone.template`, `html.dir.template`, `tex.template`, `epub.chapter.xhtml` |
| `toc` | A rendered version of the table of contents | `html.standalone.template`, `html.dir.template` |
| `has_toc`| Set to `true` if the table of contents is not empty | `html.standalone.template` |
| `colors`| The content of `html.css.colors` | `html.css` |
| `footer` | The content of `html.footer` | `html.standalone.template`, `html.dir.template` |
| `header` | The content of `html.header` | `html.standalone.template`, `html.dirtemplate` |
| `script` | The javascript file for this HTML document | `html.standalone.template`, `html.dir.template` |
| `style` | The CSS file for this HTML document, that is, a rendered version of `html.css` | `html.standalone.template` |
| A variable whose name corresponds to `lang` in book options (e.g. `lang_en` if lang is set to "en", `lang_fr` if it is set to "fr", ...) | `true`  | `html.css`, `epub.css` |
| `chapter_title` | The title of current chapter | `html.dir.template`, `epub.chapter.xhtml`, `rendering.chapter.template` |
| `chapter_title_raw` | The title of current chapter (raw text without HTML formatting) | `html.dir.template`, `epub.chapter.xhtml`, `rendering.chapter.template` |
| `json_data`  | Contains structured data with book's metadata in JSON-LD format | `html.standalone.template`, `html.dir.template` |
| `highlight_code` | True if `html.highlight_code` is true | `html.standalone.template`, `html.dir.template` |
| `highlight_css` | The content of `html.highlight.css` | `html.standalone.template` |
| `highlight_js` | The base64-encoded content of `html.highlight.js` | `html.standalone.tempate` |
| `common_script` | The content of `html.js` | `html.single.js` |
| `one_chapter`   | True if `html.standalone.one_chapter` is true, else not present | `html.standalone.template`, `html.standalone.js` |
| `book.svg` | The base64-encoded image of the button to display all chapters | `html.standalone.js`, `html.standalone.template` |
| `pages.svg` | The base64-encoded image of the button to display one chapter at a time | `html.standalone.js`, `html.standalone.template` |
| `favicon` | The `<link rel = "icon" ...>` tag if `html.icon` is set | `html.standalone.template`, `html.dir.template` |
| `menu_svg` | The base64-encoded image of the hamburger menu image | `html.standalone.template` |
| `prev_chapter` | Title and a link of previous chapter | `html.dir.template` |
| `next_chapter` | Title and a link of nexts chapter | `html.dir.template` |
| `class` | The content of `tex.class` | `tex.template` |
| `book`  | True if `tex.class` is `book`, not set else | `tex.template` |
| `tex_lang` | The babel equivalent of `lang` | `tex.template` |
| `tex_title` | Set to true to run `\maketitle` | `tex.template` |
| `tex_size` | The font size to pass to the LaTeX class | `tex.template` |
| `has_tex_size` | Set to true if `tex_size` is set | `tex.template` |
| `margin_left`, `margin_right`, `margin_top`, `margin_bottom` | The margins of the document | `tex.template` |
| `initials` | True if `rendering.initials` is true, not set else | `tex.template` | 
| `additional_code` | Set to the content of `tex.template.add`, `html.css.add` or `epub.css.add` | `tex.template`, `html.css`, `epub.css` |
