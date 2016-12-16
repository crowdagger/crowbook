ChangeLog
=========

unreleased
------------
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
  * Crowbook uses `term` library in order to display colours correctly
    on e.g. Windows.
  * The new argument `--lang` (or `-L`) allows to set the runtime
    language used by Crowbook, overriding `LANG` environment variable.
  * `--list-options` no longer uses colours as it caused problems
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
  * `html.css.colours` allows to provide a CSS file that only redefine
    the colour scheme. Such a file can be built from `crowbook
    --print-template html.css.colours`.
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
  * `crowbook --list-options` uses colours. This might hurt your eyes.
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
      previously try to load `tests/tests/test.md).
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
      different colours.
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
