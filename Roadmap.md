Roadmap 
=======

Here are a list of features I wish to have, more or less classified by
order of importance:

Main features
-------------

* Support for some simple format (maybe TOML?) configuration files to
  list chapters and metadatas, so ideally you just run `crowbook
  some_file` and you don't have to pass more options, it generates the
  rest. (Status: thinking about how to do it).
* Support for basic Markdown features useful in writing novels (Status: more or less done,
  thanks to pulldown-cmark)
* Support for Epub format as output. (Status: HTML underway, but more
  work needed to generate tocs and stuff)
* Support for LaTeX format as output. (Status: shouldn't be that
difficult, at least for basic stuff).
* Support for good french typography including in HTML format, and by
that I mostly mean non-breaking spaces. (Status: basic is there.)
* "Good" default templates and CSS. And by "good" I mean, well, for
me. (Status: shouldn't be that hard, mostly copy/pasting what I
already have)
* Some configuration if other people want to use it too. (Status:
thinking about how to do it).

Additional, low priority features wishlist
------------------------------------------

* Support for HTML as output format. 
* Support for ODT as output format.
* Support for easily embedding custom fonts in Epub/HTML 
* Correct support for technical books
* Maybe some kind of GUI because not everyone is a geek (though
  non-geeks might not write books in markdown)
