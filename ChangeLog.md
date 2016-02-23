ChangeLog
=========

0.2.0 (unreleased) 
------------------
* `--create` can now be used without specifying a `BOOK`, printing its
  result on `stdout`.
* Added support for numbering all headers, not just level-1 (e.g.,
  having a subsection numbered `2.3.1`).
* Added a `--list-options` to the binary that prints out all valid
  options in a config file (or in `set`), their type and default
  value.
* Refactored `Book` to use an HashMap of `BookOption`s instead of
  having like 42 fields.
* Tables and Footnotes are now implemented for the parser, and
  Rendered via \footnote{} for latex, and via side notes for HTML.
* `crowbook` now have a `--set` option, allowing to define or override
  whatever option set in a book configuration.

0.1.0 (2016-02-21)
------------------
* initial release
