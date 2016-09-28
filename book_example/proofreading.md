Proofreading with Crowbook 
==========================

Since version 0.9.1, Crowbook includes some proofreading features,
that can be enabled if you set one of the

* `output.proofread.html`
* `output.proofread.html_dir`
* `output.proofread.pdf`

output files. This allows you to generate different files for
publishing and proofreading (you probably don't want to publish a
version that highlights your grammar errors or your repetitions).

Current proofreading features are:

* repetition detection;
* grammar check;
* highlighting non-breaking spaces.

> Note that, by default, `cargo build` *won't* compile Crowbook with
> proofreading features. In order to enable them (at the cost of a
> bigger binary and a longer compilation time), you'll have to run:

```
$ cargo install --features "proofread" crowbook
```

> or:

```bash
$ cargo build --release --features "proofread"
```

> in the source directory.


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

Crowbook can also use [LanguageTool](https://languagetool.org/) to
detect grammar errors in your text. It is, however, a bit more
difficult to activate. 

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


Highlighting non-breaking spaces 
--------------------------------

The last proofreading feature is a bit less important, but it can be
useful in some cases. It is is dis/activated by setting
`proofread.nb_spaces` to "true" or false, and it will highlight
different sort of non-breaking spaces in HTML proofreading output
files. This can be useful in some cases, but it is mostly a debugging
feature to check that the french cleaner of Crowbook correctly
replaces spaces with correct non-breaking spaces in the relevant places.


