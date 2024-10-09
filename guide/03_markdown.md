# Markdown format

`crowbook` uses
[pulldown-cmark](https://github.com/google/pulldown-cmark),
which is an implementation of
[CommonMark](http://commonmark.org/),
so for more information on Markdown syntax, you can refer to those websites.

However, `pulldown-cmark` also implements a handful of unofficial extensions, and `crowbook` also adds its own variants, so there are a few syntax elements that are not covered by the `CommonMark` reference.

## Tables

Tables can be included in your Markdown file.

E.g.:

```markdown
|        Author      |   Book                     |
|--------------------|----------------------------|
| Anne Rice          | Interview With the Vampire |
| Terry Pratchett    | Hogfather                  |
| George Martin      | A Dance with Dragons       |
```

will render as

|        Author      |   Book                     |
|--------------------|----------------------------|
| Anne Rice          | Interview With the Vampire |
| Terry Pratchett    | Hogfather                  |
| George Martin      | A Dance with Dragons       |

> Crowbook doesn't currently support specifying column alignment.

## Footnotes

Footnotes can be specified the following way:

```markdown
Footnotes can be useful[^1] and make you look clever.

[^1]: But you shouldn't use them too much.
```

Will be rendered as:

> Footnotes can be useful[^1] and make you look clever.
>
> [^1]: But you shouldn't use them too much.

You can use multiple paragraphs in a footnote definition.
This can sometimes be useful, but it can also be tricky, as if you only let an empty line before the next paragraph, it will also be included in the footnote.
And probably the next one and the following one too:

```markdown
This is a footnote usage[^1].

[^1]: This is obviously part of the footnote definition.

This is less obviously ALSO part of the footnote definition.


This is NOT part of the foonote.
```

Due to its own quirks, `crowbook` will duplicate footnotes if you reference them multiple times:

```markdown
This footnote is unique[^2] but referenced twice[^2].

[^2]: Or is it?
```

> This footnote is unique[^2] but referenced twice[^2].
>
> [^2]: Or is it?


## Superscript and subscript

Crowbook
[`v0.12.0`](https://github.com/lise-henry/crowbook/tree/v0.12.0)
added experimental support for superscript and subscript, using respectively `foo^up^` and `bar~down~` syntax, which will render as "foo^up^" and "bar~down~";
this feature is quite a hack above the Markdown parsing library, and as such might cause issue if you mix it with other Markdown syntax elements (or, in the previous example, for smart quote detection).
This is why you'll need to enable it with `crowbook.mardown.superscript`.

## "Standalone" images

This is not *per se* a new syntactic element, but Crowbook distinguish two kind of images, according to their position in the document:

* standalone images, which are the only elements of a paragraph;
* inline images, which are placed in a container containing other
  elements.

Standalone images will typically be resized to fill the width of the page, while inline images are not resized.

This image is on its own paragraph, and thus considered "standalone" and resized to fit width:

![Logo](../img/crowbook-small.png)

While this one ![Logo](../img/crowbook-small.png) is embedded in a paragraph and its size is unchanged.
