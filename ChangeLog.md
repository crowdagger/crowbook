ChangeLog
=========

0.2.0 (unreleased) 
------------------
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
