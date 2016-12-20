# Tips and tricks #

## Using Crowbook with Emacs' markdown mode ##

If you use [Emacs](https://www.gnu.org/software/emacs/) as a text
editor, there is a
nice [Markdown mode](http://jblevins.org/projects/markdown-mode/) to
edit Markdown files. 

It is possible to use Crowbook for HTML previewing in this mode, which
[requires only minimal configuration and tweaking](http://xkcd.com/1742/):

```lisp
(custom-set-variables
 '(markdown-command "crowbook - -qs  --to html --output -"))
```

You can then use `markdown-preview` (or `C-c C-c p`) to run
Crowbook on this file and preview it in your browser, or run
`markdown-live-preview-mode` to see a live preview (updated each time
you save you file) in Emacs' integrated browser. 

### Some explanations if it looks a bit cryptic to you 

We set `markdown-command` to `crowbook`, the reason for this is a bit
obvious. The arguments we give to crowbook might be a bit less
obvious:

* the fist argument, `-`, is actually the book file: it tells
  `crowbook` to read it from standard input.
* `-qs` or `--quiet --single` tells Crowbook that is a a standalone
  markdown file, and not a book configuration file, and to be a bit
  quiet on error/info messages;
* `--to html` specifies that HTML must be generated;
* `--output -` tells Crowbook to display the result on the
  stdout, even if you set `output.html` to `some_file.html`.

### Limitations

While it renders correctly, this only works really nicely on standalone
Markdown files where you have specified, e.g.:

```markdown
---
author: Your name
title: Some title
---
```

Else, it will sets `author` and `title` to the default values.

## Embedding fonts in an EPUB file 

In order to embed fonts in an EPUB file, you'll first have to edit the
stylesheet, which you can first obtain with:

```bash
$ crowbook --print-template epub.css > my_epub_stylesheet.css
```

You'll need to use the [`@font-face` attribute](https://developer.mozilla.org/fr/docs/Web/CSS/@font-face):

```css
@font-face {
  font-family: MyFont;
  src: url(data/my_font.ttf);
}
```

Then you can add `my_font.ttf` to the files that need to be added to
the EPUB zip file:

```yaml
title: My Book
author: Me

cover: cover.png
output.epub: book.epub

resources.files: my_font.ttf
```


(Note that you'll have to repeat the process the different
`font-weight` and `font-style` variants of your font if you want it to
display correctly when there is some text in **bold**, *italics*, or **_both_**.) 


