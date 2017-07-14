Proofreading with Crowbook 
==========================

Crowbook includes some proofreading features,
that can be enabled if you set one of the

* `output.proofread.html`
* `output.proofread.html_dir`
* `output.proofread.pdf`

output files (or include `proofread.pdf` in the list of formats to
render to `output`). This allows you to generate different files for
publishing and proofreading (you probably don't want to publish a
version that highlights your grammar errors or your repetitions).

Current proofreading features are:

* repetition detection;
* grammar check.

Enabling proofreading
---------------------

Since proofreading can take quite a lot of time, particularly for a long
book, it is disabled by default. You'll have to run

```bash
$ crowbook --proofread my.book
```

or

```bash
$ crowbook -p my.book
```

to generate proofreading copies. Alternatively, if you want it to be
activated each time you run `crowbook` on this book (which is *not*
recommanded for long books, particularly if you want to perform a
grammar check), you can set 

```yaml
proofread: true
```

in the book configuration file.



Repetition detection 
--------------------

Repetition detection is enabled with:

```yaml
proofread.repetitions: true
```

It uses [Caribon](https://github.com/lise-henry/caribon) library to
detect the repetition in your text. Since the notion of a repetition
is relatively arbitrary, it is possible to adapt the settings. Default
are:

```yaml
# The maximum distance between two identical words to
# consider them a repetition 
proofread.repetitions.max_distance: 25
# The minimal number of occurences to consider it a repetition
proofread.repetitions.threshold: 2.0
# Ignore proper nouns (words starting by a capital,
# not at a beginning of a sentence)
proofread.repetitions.ignore_proper: true

# Activate fuzzy string matching
proofread.repetitions.fuzzy: true
# The maximal ratio of difference to consider
# that two words are identical
# (E.g., with 0.2, "Rust" and "Lust" won't be
# considered as the same word, but they will be with 0.5)
proofread.repetitions.fuzzy.threshold: 0.2
```

For more information, see
[Caribon](https://github.com/lise-henry/caribon)'s documentation.


> Currently, repetitions are not displayed in PDF proofreading
> output.

Grammar checking
----------------

### With Languagetool

Crowbook can use [LanguageTool](https://languagetool.org/) to
detect grammar errors in your text. It is, however, a bit more
complex to activate. 

First, you'll have to activate this feature in your book configuration
file:

```yaml
# Activate language tool support
proofread.languagetool: true
# (Optional) Sets the port number to connect to (default below)
proofread.languagetool.port: 8081
```

You'll then have to download the stand-alone version of
[LanguageTool](https://languagetool.org/). It includes a server mode,
which you'll have to launch:

```bash
$ java -cp languagetool-server.jar org.languagetool.server.HTTPServer --port 8081
```

You can also use the LanguageTool GUI (`languagetool.jar`) and start
the server from the menu "Text Checking -> Options". This also allows
you to configure LanguageTool more precisely by activating or
deactivating rules.

You can then run Crowbook, and it will highlight grammar errors in
HTML or PDF proofreading output files.

> Note: running a grammar check on a long book (like a novel) can take
> up to a few minutes.

### With Grammalecte

[Grammalecte](http://grammalecte.net/) is a grammar checker
specialized for the french language. If the language of your book is
french, you can use it in a similar fashion to languagetool: 

```yaml
# Activate grammalecte support
proofread.grammalecte: true
# (Optional) Sets the port number to connect to (default below)
proofread.grammalecte.port: 8080
```

You'll also need to run the Grammalecte server. First [download the CLI
and server version](https://www.dicollecte.org/#download_div), then:

```bash
$ python3 server.py
```

You can then run Crowbook with `--proofread` to check the grammar of
your book. It is possible to run both LanguageTool and Grammalecte on
the same book (though might take a while for a long book...).
