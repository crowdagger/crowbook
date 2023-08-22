ChangeLog
=========

0.17.0 (???)
---------------
* Try to get rid of technical debt, including removing features that were half baked and not really useful.
  * Remove proofread options.
  * Remove ODT renderer.
  * Replace mustache for templates by [upon](https://crates.io/crates/upon)
  * Use rust-i18n for internationalization instead of hackish (and unmaintained) crowbook-intl


0.16.1 (2023-08-04)
-----------------------
* Allow HTML in titles's toc where it's legal to do so in EPUB.
* Remove some invalid characters in EPUB's XML 1.0

0.16.0 (2023-07-27)
-------------
* `epub.titlepage.xhtml` can now be overriden (geobert)
* Fix an issue where horizontal rules could be interpreted as additional front matter 
* Generated PDF now include some metadata
* Fix internal links under Windows

0.15.2 (2020-07-07)
-----------------------
* Fixed endless progress bar on renderer failure

0.15.1 (2020-07-07)
------------
* `html.css.colours` has been renamed `html.css.colors`
* Fixed issue with table of contents page numbers in PDF output
* Fixed issue with invalid XHTML in EPUB when using description terms
* Fixed left/right margins in LaTeX (which were reversed)
* LaTeX outputs of french documents now use enspaces and narrow non-breaking spaces
* New option: 
     * `tex:escape_nb_spaces` defaults to `true` and will uses TeX codes to display non-breaking spaces.

0.15.0 (2019-07-18)
-----------------------
* Moved from `pulldown-cmark` to `comrak` for parsing Markdown. This may have some
  performances drawbacks but allows for a few more features:
  * Description lists
  * Strikethrough
  * Task items
* New option:
  * `crowbook.files_mean_chapters` allow to enforce that each files
    means a chapter break or to make sure that it doesn't (by default,
    only true for numbered chapters).
* Fallback on Rust zip library when there is no `zip` command.
* By default, don't add an empty chapter title for non numbered
  "chapters" that don't contain a title.
* Now uses `reqwest` instead of `hyper` to connect to languagetool/grammalecte.
* `hyphenation` dependency is now optional.
* Dependencies update.
* Fix type deduction issues for new rustc compliler


0.14.1 (2018-06-01)
-------------------
* `--stats` can now display more statistics when used with the
  `--verbose` option (if support for advanced statistics is compiled)
* LaTeX outputs now make uses of user-defined `rendering.chapter` and
  `rendering.part`
* Dependencies update

0.14.0 (2017-11-26)
-----------------------
* New option:
  * `autograph` is an autograph added after title.
* User interface:
  * new argument `--autograph` prompts for an autograph.
  * `--list-options` and `--stats` now use colors if available.
  * options description with `--list-options` are now wrapped.
* Bugfixes:
  * Preserve errors/warnings order with fancy UI.
  * Clean secondary bar when there is an error instead of hanging the UI.
  * LaTeX: fancy headers only applies to fancy pages (not chapter pages).

0.14.0-beta (2017-10-08)
------------------------------
* Bugfixes:
  * EPUB: escape quotes in content.opf.
  * LaTeX/PDF: allow hyphenations in typewriter font.
* User interface:
   * User interface is quite fancier, with progress bars and all
  * Debug/warning/info levels should be displayed in a more coherent manner
  * New `--no-fancy` option if you don't like the fancy UI (or if it doesn't work
    in your terminal)
  * New `--force-emoji` option to force emoji usage.
* Library interface:
  * Removed `Book::set_verbosity` method (uses a logger library instead).
* Now requires rustc >= 1.20.0

0.13.0 (2017-07-14)
-----------------------
* Breaking changes:
	* The `template.tex` template was quite modified. Crowbook now
    uses custom command for most markdown elements, defined in the
    template. This allow an user to redefine the way the book is
    rendered without having to modify Crowbook itself. Unfortunately,
	as tex templates for previous Crowbook versions won't work anymore.
   * the `resources.files` option is now a YAML list of strings, instead of a
     comma-seprated string.
* Add support for grammalecte grammar checker.
* `crowbook` command takes a new argument, `-S` or `--stats` which
  displays stats on the book (currently, words and characters count).
* Interactive fiction:
  * Added conditional blocks.
* Options:
  * `output.xxx` options can now take the "auto" value, which will infer the
    output file name based on the book file name.
   * `output` is a new option that can specify a series of format to
     render, with default output file name.
   * `proofread.grammalecte` and `proofread.grammalecte.port` allow
     respectively to enable grammar checking with Grammalecte and
     (optionnally) to specify the port to connect.
   * `tex.margin.left`, `tex.margin.right`, `tex.margin.bottom` and
     `tex.margin.top` are new options that allow to specify margins
     for LaTeX/PDF outputs.
   * `tex.paper_size` was renamed `tex.paper_size`.
* HTML:
  * Add JSON-LD structured data to the book's HTML files.
* Bugfixes:
  * LaTeX: fix rendering of part/chapter (part previously displayed as
    chapter and its first chapter as part)
  * EPUB:
    * Fix `.rule` so it is centered despite KOBO CSS injection.
  * Fix resources/images inclusion when they are symlinks to the
    actual file.

0.12.0 (2017-06-05)
-----------------------
This release includes a few new features, such as the possiblity to
include Markdown files as section/subsections and not only as chapter,
experimental support for superscript and subscript, and yet more
experimental support for writing interactive fiction.

* Book configuration file:
  * It is now possible to include subchapters using the `--` command
    (with one dash per sublevel: `--- foo.md` will include `foo.md` as
    a subsection).
* Markdown:
  * Added support for superscript and subscript features, using
  respectively `foo^up^` or `bar~down~` syntax.
* New options:
  * `rendering.chapter`: change what is displayed in place of
    "chapter".
  * `rendering.part`: change what is displayed in place of "part".
  * `html.chapter.template` and `html.part.template` allow to tune a
    little how the chapters and parts are displayed in HTML.
  * `tex.hyperref`, if set to `false`, will disable hyperrefs for
    local links. Can be useful for some files.
  * `crowbook.html_as_text`, if set to false, will not treat HTML as
    text but ignore it.
  * `subtitle`, as its name suggest, set the subtitle of a book.
  * `crowbook.markdown.superscript` can enable or disable superscript/subscript "extension".
* Rendering:
  * Change the way chapters are displayed by default.
  * PDF output now has a better-looking (hopefully) title page.
  * Internal links are a bit more flexible, e.g. if you link to
    `Readme.html` it will now try to link to the chapter corresponding
    to `Readme.md`.
* Bugfixes:
  * LaTeX:
    * Fix bug in syntax highlighting.
	* Fix label placements (and thus navigation inside PDF document).
  * EPUB:
    * Add unnamed but numbered chapters to the TOC.
	* Fix HTML escaping issue for chapter titles.
	* Fix the way parts were handled in the TOC.
  * Book configuration file:
	* Fix issue when setting custom number for parts.
* Crowbook now requires rustc >= 1.17.0

0.11.4 (2017-03-21)
-----------------------
* An image can now be considered standalone even if it is inside a
  link.
* Bugfixes:
  * HTML/EPUB: use raw (not HTML rendered) metadata in the places
    where HTML code is not appropriate. Templates can use this
    metadata with the `foo_raw` value.
  * HTML/EPUB: fix double-escaping/rendering issues in titles.
  * EPUB:
      * Escape title and author before feeding them to epub-builder.
	  * Fix content.opf issue by not rendering first
        chapter's title (marked as beginning of document) in `<guide>`.
* Rendering:
  * HTML/EPUB: standalone images are now displayed centered.

0.11.3 (2017-03-19)
-----------------------
* When crowbook parses the book's contents, it now detects which
  features are used. This is useful in various ways:
  * The ODT renderer only displays a global warning showing the lists of
    used features that are not implemented, instead of a warning each
    time such a feature is encountered.
  * The LaTeX and HTML/EPUB renderers only initialize `syntect` (which
    can take some time) if code blocks are used in the document.
  * The LaTeX renderer only requires LaTeX packages that are actually
    used in the document.
* Command-line interface:
  * Warnings are now displayed by default.
  * The (undocumented) `--debug` argument has been removed.
  * The status of some messages have been modified ("warning" to
  "debug" or "error" to "warning").
* Deprecated option:
  * `crowbook.verbose` has been deprecated, at it should be set by the CLI.

0.11.2 (2017-03-05)
-----------------------
* General:
  * When there is an error setting an option from the book configuration
	file (e.g. because it is an invalid key), print an error but do not
    abort, only ignore this specific option.
* New options:
  * `tex.stdpage`: if set to `true`, will use the `stdpage` package to
    render the book according to standards for submitting manuscripts.
  * `rendering.highlight.theme` allows to specifies a theme for syntax
    highlighting (only used if `rendering.highlight` is set to
    "syntect").
  * `html.highlight.theme`, `epub.highlight.theme` and
    `tex.highlight.theme` allow to specify a theme for
    HTML/EPUB/LaTeX renderers (only used with syntect).
* Deprecated option:
  * `proofread.nb_spaces`.
* Rendering:
  * `[syntect](https://crates.io/crates/syntect)` is now the default
    for `rendering.highlight`. Concretely, this means that by default
    syntax highlighting is now done when `crowbook` is run instead of
    using `[highlight.js](https://highlightjs.org/)`.
  * EPUB:
    * Now sets the "cover-image" property and meta so readers should
      display cover correctly.
	* Narrow non-breaking spaces should display more correctly on KOBO
      ereaders (hoping this won't break the way they are displayed
      everywhere else).
* Proofreading:
  * Repetition detection is now a bit less of an hack, and should
    cause less problems when used in conjunction with grammar
    checking. It now also works on PDF output (so the way it is
    highlighted could be improved).
* Bugfixes:
  * Fix `mimetype` of EPUB files (make sure it is always "stored" and
    not "deflated" by the `zip` command).
  * Avoid initializing `syntect` (at the cost of performances) if it
    is not used.
   * Avoid creating an empty file if some book renderer fails
     (e.g. EPUB or ODT because `zip` command is not present).


0.11.1 (2017-01-05)
-----------------------
* Rendering:
	* Avoid page break before or after a separating rule.
	* Add support for [syntect](https://crates.io/crates/syntect) for
      syntax highlighting. This is activated by setting
      `rendering.highlight` to `syntect` (see below).
	* EPUB:
	    * Set back HTML escape of narrow non-breaking spaces to `true`
          by default (it caused problems on some readers, but cause
          much more serious one if `false`).
		* Add more information to guide/nav landmarks.
	* LaTeX/PDF:
	    * Improve the way code blocks are displayed, using the
          `mdframed` package.
		* Try to reduce the issues of too long lines when using code
          and code blocks, by inserting `\allowbreak{}` directive
          after some characters (`.`, `/`, `_`, ...).
		* Block quotes are now displayed in italics.
		* Tables now use `tabularx`, which allows to break too long
          lines (it still doesn't break pages, though).
* New options:
  * `rendering.highlight` can be set to `none`, `highlight.js` (by
    default, enables syntax highlighting via Javascript, but only on
    HTML document) or `syntect` (doesn't necessitate javascript, and
    can work in EPUB or LaTeX, but more experimental at this point).
* Deprecated options:
  * `html.highlight_code` (use `rendering.highlight` instead).
* Bugfixes:
  * HTML (standalone): fix the template that contained invalid HTML code.

0.11.0 (2016-12-31)
-----------------------
Substantial changes in this release, the more important one being
support for parts!
* **Breaking changes**: the API has undergone some breaking changes,
  hoping they will be the last ones for a while. API should now be
  more simple and consistent (?). This version contains also
  substantial options renaming (see below).
* Crowbook now supports parts (above the "chapter" level), using the
  '@' character in the book configuration file.
* Command-line interface:
  * Behaviour of `--to` should now be consistent for all output
    formats.
  * If `--output` is set to `-`, prints to stdout.
  * Conversely, if `<BOOK>` is set to `-`, reads from stdin.
  * Path specified by `--output` is now interpreted relatively to
    current directory (and not depending on where `<BOOK>` is or its
    options).
* Rendering:
  * Chapters with no titles now have an empty title added (so it can
    at least display e.g. "Chapter X").
  * EPUB:
    * The `toc.ncx` file now displays links to "title" and (if set)
    "cover" (can be deactivated, see below).
    * The `toc.ncx` file now displays toc levels below chapter.
	* The table of contents is now displayed inline if
      `rendering.inline_toc` is set to `true`.
* New options:
  * `epub.toc.extras`, set to `true` by default, will add links to the
    title and the cover (if it is set) in the table of contents.
  * `epub.escape_nb_spaces`, similar to `html.escape_nb_spaces` and
    set to false by default since at least Kobo reader don't seem to
    be able to understand the CSS to escape those nb spaces...
  * `rendering.chapter.roman_numerals`, if set to `true`, will display chapter
    numbers using roman numerals.
  * `rendering.part.roman_numerals`, if set to `true` (it is by
    default) will display part numbers using roman numerals.
  * `rendering.part.template` specifies the numbering scheme of parts.
  * `rendering.part.reset_counter`, if set to `true` (it is by
    default), resets chapter number to zero after a part.
* Renamed options:
  * `import_config` renamed to `import`.
  * `rendering.chapter_template` renamed to `rendering.chapter.template`.
  * `html_single.html` renamed to `html.standalone.template`.
  * `html_single.js` renamed to `html.standalone.js`.
  * `html_single.one_chapter` renamed to `html.standalone.one_chapter`.
  * `output.html_dir` renamed to `output.html.dir`.
  * `output.proofread.html_dir` renamed to `output.proofread.html.dir`.
  * `html_dir.index.html` and `html.dir.chapter.html` have been merged
    and both renamed to `html.dir.template`.
  * `tex.font_size` renamed to `tex.font.size`.
* Bugfixes:
  * EPUB:
    * Fix duplicate HTML escaping (resulting in e.g. "&amp;" instead
      of "&").
  * HTML directory:
    * Fix panic when trying to generate html directory in "../xxx"
      ([#23](https://github.com/lise-henry/crowbook/issues/23)).
	* Fix "previous chapter" links that were not displayed when
      "html.header" was set.
  * HTML:
    * Fix the way initial letter is displayed if `rendering.initials`
      is true.
* Internationalization:
  * Strings in generated Crowbook documents (such as "Table of
    contents", "Title", "Cover" and such) are now translated in spanish.

0.10.4 (2016-12-16)
-----------------------
* New options:
  * `tex.font_size` specifies an optional font size (in pt) passed to
    the LaTeX class (must be 10, 11 or 12).
  * `tex.title` can be set to `false` to avoid rendering the title
    with `\maketitle`.
  * `tex.paper_size` specifies the paper size for PDF output.
  * `tex.template.add`, `html.css.add` and `epub.css.add`allow to
    specify inline LaTex or CSS code in the book configuration file
    that will be added respectively to `tex.template.add`,
    `html.css.add` and `epub.css.add`.
  * `html.icon` allows to specify the path of an icon for HTML documents.
* Command-line interface:
  * Paths that are displayed should now be normalized,
    e.g. "foo/bar.pdf" instead of "baz/../foo/bar.pdf".
* Rendering:
  * HTML:
    * The default CSS style has been slightly modified.

0.10.3 (2016-11-19)
-----------------------
* Building:
  * Crowbook now requires rustc >= 1.13.0 to build.
  * Pre-built binaries now all include the proofreading feature.
  * Linux binaries are now linked against `musl` library so they
    should really work on any Linux platform.
* Bugfixes:
  * Fixed escaping of `author` and `title` fields.
  * Fixed text cleaning in ODT rendering that causes corrupt files to
    be generated.
* CommandLine Interface:
  * Crowbook displays clearer error messages when unable to launch
    `latex` or `zip` commands.
  * Crowbook uses `term` library in order to display colors correctly
    on e.g. Windows.
  * The new argument `--lang` (or `-L`) allows to set the runtime
    language used by Crowbook, overriding `LANG` environment variable.
  * `--list-options` no longer uses colors as it caused problems
    depending on the terminal or when piping to `less`.

0.10.2 (2016-10-21)
-----------------------
Only minor changes in this version:
* Options:
  * `author` and `title`'s default values are both set to the empty
   string, instead of `Anonymous` and `Untitled`.
  * `input.autoclean` has been renamed `input.clean`.
  * `input.smart_quotes` has been renamed `input.clean.smart_quotes`.
  * new option: `input.clean.ligature.dashes` will (if set to true)
   replace `--` to en dash (`–`) and `---` to em dash (`—`).
  * new option: `input.clean.ligature.guillemets` will (if set to true)
   replace `<<` and `>>` to french guillemets (`«` and `»`).
* Rendering:
  * HTML: if `html_single.one_chapter` and `rendering.inline_toc` are
    both set to true, only render the TOC if currently displayed
    chapter is the first.

0.10.1 (2016-10-18)
-----------------------
Fixed a bug in `fr.po` translation that prevented building from fresh install.

0.10.0 (2016-10-18)
-----------------------
This release contains some breaking changes (mostly for the API, which has been split in separate libraries). It alse features some internationalization support, and
the program should now be tranlated if your `LANG` environment
variable is set to french.
* **Breaking changes**:
  * Templates:
	* Conditional inclusion depending on `lang` must now be done using `lang_LANG` (e.g.
	`lang_fr`, `lang_en`, and so on). This might impact custom `epub.css` and `html.css`
	templates.
  * API:
    * The `escape` module has been moved to a separate crate,
      `crowbook_text_processing`. The `cleaner` module is no longer
      public, but the features it provided are also available in
      `crowbook_text_processing`.
* New options:
  * `html.css.colors` allows to provide a CSS file that only redefine
    the colour scheme. Such a file can be built from `crowbook
    --print-template html.css.colors`.
  * `input.smart_quotes`: if set to `true`, tries to replace `'` and `"` by curly quotes.
* Command line interface:
  * Crowbook is now (imperfectly) localized in french, and can be
    translated to other languages.
  * Added the `--quiet` (or `-q`) argument, that makes crowbook run without
    displaying any messages (except some error messages at this point).
* Rendering:
  * HTML:
    * The table of contents menu is no longer displayed in the HTML single renderer if
      it doesn't contain at least two elements.
	* The default colour theme has been modified a little.
* Bugfixes:
  * Fix the escaping of non-breaking spaces in EPUB, as `&nbsp;` and
    its friends aren't valid entities in XHTML, apparently.

0.9.1 (2016-09-29)
------------------
This release mainly introduces generation of proofreading copies,
allowing, if they are set (and `crowbook` was compiled with the
`proofread` feature) to generate proofreading copies, using tools to
check grammar and detect repetitions. These features are currently
experimental.
* New options:
  * `html.escape_nb_spaces`, if set to true (by default), will replace
    unicode non breaking spaces with HTML entites and CSS so it can
    display correctly even if reader's don't have a browser/font
    supporting these unicode symbols.
  * Output files for proofread documents: `output.proofread.html`,
    `output.proofread.html_dir` and `output.proofread.pdf`.
  * Proofread options `proofread.repetitions` and
  `proofread.nb_spaces` have been added.
     * `proofread.nb_spaces`, if set to true, highlights non-breaking spaces so it is
     easier to check the correct typography of a book. Note that it
     requires that `html.escape_nb_spaces` be set to true (default) to
     work.
     * `proofread.reppetitions`, if set to true, uses
       [Caribon](https://github.com/lise-henry/caribon) to highlight
       repetitions in a document. It also uses the settings `proofread.repetitions.fuzzy`,
       `proofread.repetitions.max_distance`,
       `proofread.repetitions.threshold`,
       `proofread.repetitions.fuzzy.threshold`,
       `proofread.repetitions.ignore_proper`. Note that this feature
       is not built by default, you'll have to build crowbook with
       `cargo build --release --features "repetitions"`.
* New default settings for options:
  * `tex.command` is now `xelatex` by default.
* Rendering:
  * LaTeX:
    * Add support for xelatex in the default template.
  * Improved french cleaner (see [an article (in french)](https://crowdagger.github.io/textes/articles/heuristique.html)
    that talks about what it does).
* Crowbook user guide: documentation has been updated to correctly
  reflect 0.9.x options.
* API:
  * `clap` dependency is now optional, people who want to use Crowbook
    as a library should include it with `crowbook = { version = "0.9",
    default-features = false }`. (`clap` is still required to build a
    working binary).

0.9.0 (2016-09-23)
------------------
The main objective of this release is to clean public interfaces, in
order to limit breaking changes in the future. *Ideally*, all pre-1.0
releases should thus be 0.9.x. Concretely, this meant three things:
* reducing the surface of Crowbook's library API;
* cleaning options names
* cleaning the names exported in templates and document them, in order
  not to break user-defined templates in future (non-breaking)
  releases.
More detailed changes for this release:
* **Breaking change for users**: removed `tex.short` option, replaced
  by a more generic `tex.class` (default being
  `book`). `html.crowbook_link` has also been removed.
* Renamed options. Using the old name will print a deprecation warning
  but will still work for a while.
  * `temp_dir` -> `crowbook.temp_dir`
  * `zip.command` -> `crowbook.zip.command`
  * `verbose` -> `crowbook.verbose`
  * `html.print_css` -> `html.css.print`
  * `html.display_chapter` -> `html_single.one_chapter`
  * `html.script` -> `html_single.js`
  * `numbering` -> `rendering.num_depth`
  * `numbering_template` -> `rendering.chapter_template`
  * `display_toc` -> `rendering.inline_toc`
  * `toc_name` -> `rendering.inline_toc.name`
  * `enable_yaml_blocks` -> `input.yaml_blocks`
  * `use_initials` -> `rendering.initials`
  * `autoclean` -> `input.autoclean`
  * `html_dir.css` -> `html.css` (not really renamed, `html_dir.css`
    isactually removed as there is no point in having different CSS
    for standalone and multifile HTML rendering, is it?)
* New options:
  * More metadata: `license`, `version` and `date`. These metadata are
    not treated by the renderers, but they are exported to the
    templates: `{{{metadata}}}` allows to access the content. If they
    are present, a `has_metadata` is also set to true, allowing to do
    something like `{{{title}}} {{#has_version}}version {{{version}}}
    {{/has_version}}`.
  * Yet more metadata: it is possible to add custom metadata by
    prefixing it with `metadata.`. They will then be accessible in the
    templates, with dots ('.') replaced by underscores ('_'). E.g.,
    with `metadata.foo: bar` you can access it in your templates with
    `{{{metadata_foo}}}`.
  * `output.base_path` specifies a directory where the output files (set
    by `output.FORMAT` will be written.
  * `resources.base_path.templates` specifies where templates can be
    found.
* Rendering:
  * Metadata can now contain Markdown and will be rendered by the
    renderers. This might not be a good idea for common fields
    (e.g. "title"), though. Use with caution.
  * `rendering.inline_toc.name` can use `{{{loc_toc}}}` to specify a
    localized name.
  * HTML:
    * `html.top` and `hstml.footer` are now considered as templates, so
      you can use some `{{{metadata}}}` in it.
    * Improved the way footnotes are displayed.
    * In standalone HTML, footnotes are rendered at the end of the
      document instead of at the end of the chapter, unless
      `html_single.one_chapter` is true.
  * LaTeX:
    * If `tex.class` is set to `article`, chapters will be displayed as
      `\sections` since `article` class doesn't handle chapters.
    * Except if `tex.class` is set to `book`, margins are now
      symmetrical.
    * LaTex template now uses `version` and `date`.
* Bugfixes:
  * `import_config` only import options from another book file that
    are not equal to the default ones and that haven't already been
    set by the caller. E.g., `author: foo` then `import_config:
    bar.book` won't erase the author previously set.
  * `import_config` now correctly translates the imported book's
    paths.
* Crowbook program:
  * Still working to improve error messages.
  * `crowbook --list-options` uses colors. This might hurt your eyes.
  * Display an error message when mustache can't compile a template,
    instead of panicking.
* Internal/API:
  * Added static methods to `Logger` to allows displaying messages
    more easily/prettily.
  * Reduce pubic API's surface so less changes will need to be
    considered breaking in the future.

0.8.0 (2016-09-19)
------------------
This release adds support for syntax higlighting in code blocks,
customized top and footer blocks for HTML rendering, and the special
`import_config` option that allows to import options from another book
file. It also provides (hopefully) better error messages.

* New options:
  * `import_config`is not really an option, but allows to import
    another configuration file, useful if you share a same set of
    options between multiple books.
  * `use_initials` (set to false by default) makes Crowbook use
    initials ("lettrines") at start of each chapter. Support is still
    experimental.
  * `html.highlight_code` (set to true by default) allows syntax
    highlighting for code blocks, using highlight.js.
  * `html.higlight.css` and `html.highlight.js` can be used to provide
    other themes (default is default.css) and an highlight.js build
    that support other languages.
   * `html.footer` allows to specify custom footer. If not set,
     `html.crowbook_link` allows to disable "Generated by Crowbook"
     message.
   * `html.top` allows to specify a custom header that will be
     displayed at the top of HTML file(s).
* Deprecated options:
  * `side_notes` has been renamed `html.side_notes`.
* Crowbook program:
  * All output formats are now rendered concurrently.
  * Better error messages. Crowbook now tries to give more information
  when displaying an error, with the file name where a problem was
  found, and, in some cases, the line. It also tries to detect errors
  (such as files not found) sooner.
  * Some "warning" messages have also been "moved" to error messages, to
  make sure they are displayed even when crowbook isn't runned with
  `--verbose`.
* Rendering:
  * Hidden chapter now produce empty `\chapter*{}` and `<h1>` in LaTeX
    and HTML. This allow to delimit a chapter break even if nothing is
    displayed.
* Bugfixes:
  * Navigation menu of standalone HTML didn't include a call to
    javascript when `html.display_chapter` was set to true, meaning it
    didn't display the chapter correctly.
  * Implementations of `Image` and `StandaloneImage` were reversed in
    LaTeX.
  * `StandaloneImage` urls were not adjusted (meanning that running
    `crowbook` from another directory failed).
  * Image paths are now found correctly in HtmlDir rendering
    even if `crowbook` is called from another directory (same fix
    as 0.6's for Epub and LaTeX, which was forgotten for HtmlDir).
* Internal/API:
  * In order to have better error messages, there was a need to
    refactor the `Error` type, and make more methods return
    `Result<X>` instead of `X`. The API is, therefore, quite modified.
  * Added a `Renderer` trait used by the various renderers.
  * Removed some methods from public API.

0.7.0 (2016-09-11)
------------------
This releases renders images differently when they are on a standalone
paragraph or inside a paragraph.

* Internal/API:
  * `Token` has a new variant, `StandaloneImage`. This is used to
    distinguish an image that is alone in a paragraph of an image that
    is inlined alongside text.
  * `Parser.parse` method now distingues between `Image` and
    `StandaloneImage`. Currently, an image is considered "standalone"
    if it is the sole element of a paragraph, even if it is among a
    link.
  * `Token` has a new `is_image` method.
* Rendering:
  * Standalone images are now rendered differently than inline images
    (80% of width VS original size) in HTML/EPUB and LaTeX.

0.6.0 (2016-09-09)
------------------
* Deprecated options:
  * `nb_char`: since it was only used for french cleaner and for
    typography reasons it's better to use different non breaking
    spaces according to context, this option was not really useful
    anymore.
* Rendering:
  * Images are now displayed at 80% width of the page.
* Bugfixes:
  * Image paths are now found correctly in LaTeX and EPUB rendering
    even if `crowbook` is called from another directory.
  * Fixed a bug in `French` cleaner when a string to clean ended by a
    non-breaking space (space was doubled with a breaking one).
  * LaTeX/PDF:
    * "Autocleaning" is now also activated (for french at least) for
      LaTeX rendering, since it doesn't correctly insert non-breaking
      spaces for e.g. '«' or '»'.
    * Fixed escaping of `--` to `-{}-` to avoid tex ligatures.
  * HTML/EPUB:
    * `html.display_chapter` now defaults to `false` (e.g., by default
      the HTML displays the entirety of a book).
    * Fixed rendering of lists when `lang` is set to `fr`.
    * Links are now HTML-escaped, fixing errors in XHTML (for EPUB
      rendering) when links contained '&' character.


0.5.1 (2016-04-14)
------------------
Mostly rendering fixes:
* Epub:
  * Fix a validation problem when book contained hidden chapters.
* French cleaner:
  * Use semi-cadratine space instead of cadratine
  space for dialogs.
  * Use non-narrow non-breaking spapce instead of
  narrow one for ':', '«' and '»' (following
  https://fr.wikipedia.org/wiki/Espace_ins%C3%A9cable#En_France).
* HTML:
  * Add viewport meta tags.
  * Standalone HTML:
    * Don't display the button to display chapter and
    the previous/next chapter link if `html.display_chapter` is set to
    `false`.
    * Fix chapter displaying when some chapters are not
    numbered.
  * Multi-files HTML:
    * Fix previous/next chapter display to make it consistent with
      standalone HTML.


0.5.0 (2016-04-02)
------------------
* Crowbook now requires Rustc 1.7.0.
* It is now possible to render HTML in multiple files:
     * `output.html_dir` will activate this renderer, and specify in
       which directory to render these files;
     * `html_dir.css` allows to override the CSS for this rendering;
     * `html_dir.index.html` allows to specify a template for the
       `index.html` page;
     * `html_dir.chapter.html` allows to specify a template for the
       chapters pages.
* New book options:
    * `tex.short`: if set to true, the LaTeX renderer will use
      `article` instead of `book` as document class, and will use the
      default `\maketitle` command for article. This option is by
      default set to false, except when Crowbook is called with
      `--single`.
    * `enable_yaml_blocks`: parsing YAML blocks is no longer activated
      by default, except when using `--single`. This is because you
      might want to have e.g. multiple short stories using YAML blocks
      to set their titles and so on, *and* a separate `.book` file to
      render a book as a collection of short stories. In this case,
      you wouldn't want the displayed title or the
      output.pdf/html/epub files be redefined by the short stories .md
      files.
    * `html.print_css`: allows to specify a stylesheet for media print
    * `html.display_chapter`: displays one chapter at a time in
      standalone HTML
    * `html.script`: allows to specify a custom javascript file for
    standalone HTML
    * `html_dir.script`: same thing for multipage HTML
    * `resources.base_path`: by default, Crowbook resolves local links in
      markdown files relatively to the markdown file. This option
      allows to resolve them relatively to a base path. This option
      comes with two variants, `resources.base_path.images` and
      `resources.base_path.links`, which only activate it for respectively
      images tags and links tags. These two options are ignored when
      `base_path` is set. There is also `resources.base_path.files`
      which specify where additional files (see below) should be read,
      but this is one is set to `.` (i.e.,  the directory where the
      `.book` file is) by default.
    * `resources.files`: indicate a (whitespace-separated) list of
      files that should be embedded. Currently only used with the EPUB
      renderer.
    * `resources.out_path`: indicate where `resources.files` should be
      copied in the final document. Default to `data`, meaning that
      files will be placed in a `data` directory in the EPUB.
* Rendering:
    * Templates can now use localized strings according to the `lang`
      option
    * Standalone HTML now includes locale files using base64.
    * Standalone HTML displays one chapter at a time, thouht it can be
      changed via a button in the menu.
    * HTML/EPUB: default CSS now uses the `lang` value do determine
      how to display lists (currently the only difference is it uses
      "–" when `lang` is set to "fr" and standard bullets for other
      languages).
* Bugfixes:
    * Fixed a bug of filename "resolution" when Crowbook was called
      with `--single` (e.g., `crowbook -s tests/test.md` would
      previously try to load `tests/tests/test.md)`.
    * Epub renderer now uses the `mime_guess` library to guess the
      mime type based on extension, which should fix the mime type
      guessed for a wide range of extensions (e.g., svg).
* Internal/API:
    * The `Book::new`, `new_from_file`, and `new_from_markdown_file`
      take an additional `options` parameter. To create a book with
      default options, set it to `&[]`.



0.4.0 (2016-03-01)
------------------
* Crowbook now internally uses a true YAML parser, `yaml_rust`, for its
  options. Since the "old" Crowbooks's config format was similar, but
  had some subtle differences, this is somewhat of a breaking change:
    * strings should now be escaped with "" in some cases (e.g. if it
      contains special characters). On the other hand, it *allows* to
      optionally escape a string with these quotes, which wasn't
      possible until then and might be useful in some cases.
    * multiline strings now follow the YAML format, instead of the
      previous "YAML-ish" format. This can impact the way newlines are
      added at the end of a multiline string. See
      e.g. [this link](http://stackoverflow.com/questions/3790454/in-yaml-how-do-i-break-a-string-over-multiple-lines)
      for the various ways to include mulitiline strings in Yaml.
* Crowbook now parses YAML blocks (delimited by two lines with "---")
  in Markdown files, ignoring keys that it doesn't recognize. This
  allows crowbook to be compatible(-ish) with Markdown that contains
  YAML blocks for Jekyll or Pandoc.
* New option `--single` allows to give Crowbook a single Markdown file
  (which can contain options within an inline YAML block) instead of a
  book configuration file. This is useful for e.g. short stories.
* Enhanced the way debugging/warning/info messages are handled and
displayed:
    * Added a `--debug` option to the binary.
    * Internal: added a `Logger` struct.
    * Different levels of information (debug/warning/info/error) get
      different colors.
* Bugfixes:
    * Crowbook no longer crashes when called with the `--to` argument
      if it can't create a file.


0.3.0 (2016-02-27)
------------------
* Crowbook now tries to convert local links. That is, if you link to a
  Markdown file that is used in the book.
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
