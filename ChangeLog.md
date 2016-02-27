ChangeLog
=========

0.3.0 (2016-02-27)
------------------
* Crowbook now tries to convert local links. That is, if you link to a
  Markdown file that is used in the book
  (e.g. [README.md](README.md)), it *should* link to an appropriate
  inner reference inside the book.
* Latex renderer now supports (local) images.
* Epub renderer now embed (local) images in the EPUB file.
* Some changes to the HTML/Epub stylesheets.
* Internal (or usage as a library):
    * Crowbook no longer changes current directory, which worked in
      the binary but could cause problem if library was used in
      multithreaded environment (e.g. in `cargo test`).
    * More modules and methods are now private.
    * Improved documentation.
    * Added more unit tests.
* Bugfixes: 
    * Epub renderer now correctly renders unnumbered chapter without a
      number in its toc.ncx file



0.2.2 (2016-02-25)
------------------
* Bugfixes:
    * French cleaner now correctly replaces space after — (in
      e.g. dialogs) with "em space".


0.2.1 (2016-02-25)
------------------
* Bugfixes:
    * HTML/Epub rendering no longer incorrectly increment chapter
      count for unnumbered chapters.
    * Latex: makes what is possible to avoid orverflowing the page.
* Minor changes:
    * Latex: improvement of the default way URLs are displayed.
      

0.2.0 (2016-02-25) 
------------------
* Command line arguments:
    * New argument `--print-template` now allows to print a built-in
      template to stdout. 
    * New argument `--list-options` prints out all valid
      options in a config file (or in `set`), their type and default
      value.
    * New argument `--set` allows to define or override whatever
      option set in a book configuration.
    * `--create` can now be used without specifying a `BOOK`, printing
      its result on `stdout`.
* Configuration file:
    * Added support for multiline strings in `.book` files, with
      either '|' (preserving line returns) or '>' (transforming line
      returns in spaces) 
    * New option `display_toc` allows to display the table of contents
      (whose name, at least for HTML, is specified by `toc_name`) in
      HTML and PDF documents.
    * Option `numbering` now takes an int instead of a boolean,
      allowing to specify the maximum level to number (e.g. `1`:
      chapters only, `2`: chapters and sectino, ..., `6`: everything).
* Rendering: 
    * Added support for numbering all headers, not just level-1 (e.g.,
      having a subsection numbered `2.3.1`).
    * Tables and Footnotes are now implemented for HTML/Epub and LaTeX
    output.
* Internal:
    * Refactored `Book` to use an HashMap of `BookOption`s instead of
      having like 42 fields.


0.1.0 (2016-02-21)
------------------
* initial release
