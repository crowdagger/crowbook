# Contributing

Crowbook is a free software, and you can contribute to it.
There are some things that can be accessible even if you don't know anything about programming.

## Internationalization

Crowbook aims to support multiple languages.
However, unfortunately, currently only english, french, and (in a more limited way) spanish are currently supported.
If you want to have better support for the language you write in, there are easy things you can do:

* Provide a translation for the few strings that Crowbook insert into the rendered documents.
  This is really easy, as there are currently less than a dozen of them, and you just need to create a new variant of the
  [`lang/en.yaml`](https://github.com/lise-henry/crowbook/blob/master/lang/en.yaml)
  file.
* Open an
  [issue](https://github.com/lise-henry/crowbook/issues)
  about the typographic rules in your language, if Crowbook doesn't cover them.
* Provide a translation for the Crowbook program.
  It requires creating a variant of the
  [`.po` file](https://github.com/lise-henry/crowbook/blob/master/lang/fr.po),
  which is a bit more work because (at this time) it's around 1,500 lines (and less a priority than the first item of this list, as this translation only affects the the command-line interface and not the rendered documents).
