use crate::book::Book;
use crate::bookoption::BookOption;
use crate::error::{Error, Result, Source};
use crate::style;

use std::collections::HashMap;
use std::env;
use std::mem;
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader};

lazy_static! {
    static ref OPTIONS: String = format!("\
# {metadata}
author:meta:\"\"                    # {author}
title:meta:\"\"                     # {title}
lang:meta:en                        # {lang}
subject:meta                        # {subject}
description:meta                    # {description}
cover:path                          # {cover}

# {metadata2}
subtitle:meta                       # {subtitle}
license:meta                        # {license}
version:meta                        # {version}
date:meta                           # {date}
autograph:meta                      # {autograph}

# {output_opt}
output:strvec                       # {output}
output.epub:path                    # {output_epub}
output.html:path                    # {output_html}
output.html.dir:path                # {output_html_dir}
output.tex:path                     # {output_tex}
output.pdf:path                     # {output_pdf}
output.odt:path                     # {output_odt}
output.html.if:path                 # {output_if}
output.base_path:path:\"\"            # {output_base_path}

# {render_opt}
rendering.highlight:str:syntect                                      # {rendering_highlight}
rendering.highlight.theme:str:InspiredGitHub                         # {rendering_highlight_theme}
rendering.initials:bool:false                                        # {rendering_initials}
rendering.inline_toc:bool:false                                      # {inline_toc}
rendering.inline_toc.name:str:\"{{{{{{loc_toc}}}}}}\"                        # {toc_name}
rendering.num_depth:int:1                                            # {num_depth}
rendering.chapter:str                                                # {chapter}
rendering.part:str                                                   # {part}
rendering.chapter.roman_numerals:bool:false                                  # {roman_numerals_chapters}
rendering.part.roman_numerals:bool:true                                      # {roman_numerals_parts}
rendering.part.reset_counter:bool:true                                      # {reset_counter}
rendering.chapter.template:str:\"{{{{{{number}}}}}}. {{{{{{chapter_title}}}}}}\" # {chapter_template}

rendering.part.template:str:\"{{{{{{number}}}}}}. {{{{{{part_title}}}}}}\" # {part_template}




# {special_ops}
import:path                  # {import_config}

# {html_opt}
html.icon:path                      # {html_icon}
html.highlight.theme:str            # {html_theme}
html.header:str                     # {html_header}
html.footer:str                     # {html_footer}
html.css:tpl                        # {html_css}
html.css.add:str                    # {html_css_add}
html.css.colors:tpl                # {css_colors}
html.js:tpl                         # {html_js}
html.css.print:tpl                  # {css_print}
html.highlight.js:tpl               # {highlight_js}
html.highlight.css:tpl              # {highlight_css}
html.side_notes:bool:false          # {side_notes}
html.escape_nb_spaces:bool:true     # {nb_spaces}
html.chapter.template:str:\"<h1 id = 'link-{{{{{{link}}}}}}'>{{{{#has_number}}}}<span class = 'chapter-header'>{{{{{{header}}}}}} {{{{{{number}}}}}}</span>{{{{#has_title}}}}<br />{{{{/has_title}}}}{{{{/has_number}}}}{{{{{{title}}}}}}</h1>\" # {html_chapter_template}
html.part.template:str:\"<h2 class = 'part'>{{{{{{header}}}}}} {{{{{{number}}}}}}</h2> <h1 id = 'link-{{{{{{link}}}}}}' class = 'part'>{{{{{{title}}}}}}</h1>\" # {html_part_template}

# {html_single_opt}
html.standalone.template:tpl                # {single_html}
html.standalone.one_chapter:bool:false  # {one_chapter}
html.standalone.js:tpl                  # {single_js}


# {html_dir_opt}
html.dir.template:tpl               # {html_dir_template}

# {html_if_opt}
html.if.js:tpl                      # {if_js}
html.if.new_turn:str               # {if_new_turn}
html.if.end_turn:str                # {if_end_turn}
html.if.new_game:tpl                # {if_new_game}

# {epub_opt}
epub.version:int:2                  # {epub_ver}
epub.highlight.theme:str            # {epub_theme}
epub.css:tpl                        # {epub_css}
epub.css.add:str                    # {epub_css_add}
epub.chapter.xhtml:tpl              # {chapter_xhtml}
epub.titlepage.xhtml:tpl            # {titlepage_xhtml}
epub.toc.extras:bool:true           # {epub_toc}
epub.escape_nb_spaces:bool:true     # {nb_spaces}

# {tex_opt}
tex.highlight.theme:str             # {tex_theme}
tex.links_as_footnotes:bool:true    # {tex_links}
tex.command:str:xelatex             # {tex_command}
tex.escape_nb_spaces:bool:true      # {nb_spaces_tex}
tex.template:tpl                    # {tex_tmpl}
tex.template.add:str                # {tex_tmpl_add}
tex.class:str:book                  # {tex_class}
tex.paper.size:str:a5paper          # {tex_paper_size}
tex.margin.left:str                 # {tex_margin_left}
tex.margin.right:str                # {tex_margin_right}
tex.margin.top:str:\"2cm\"          # {tex_margin_top}
tex.margin.bottom:str:\"1.5cm\"     # {tex_margin_bottom}
tex.title:bool:true                 # {tex_title}
tex.font.size:int                   # {tex_font_size}
tex.hyperref:bool:true              # {tex_hyperref}
tex.stdpage:bool:false              # {tex_stdpage}


# {rs_opt}
resources.files:strvec               # {rs_files}
resources.out_path:path:data         # {rs_out}
resources.base_path:path             # {rs_base}
resources.base_path.links:path       # {rs_links}
resources.base_path.images:path:.    # {rs_img}
resources.base_path.files:path:.     # {rs_base_files}
resources.base_path.templates:path:. # {rs_tmpl}

# {input_opt}
input.clean:bool:true               # {autoclean}
input.clean.smart_quotes:bool:true  # {smart_quotes}
input.clean.ligature.dashes:bool:false # {ligature_dashes}
input.clean.ligature.guillemets:bool:false # {ligature_guillemets}
input.yaml_blocks:bool:false        # {yaml}


# {crowbook_opt}
crowbook.html_as_text:bool:true     # {html_as_text}
crowbook.files_mean_chapters:bool   # {files_mean_chapters}
crowbook.markdown.superscript:bool:false  # {superscript}
crowbook.temp_dir:path:             # {tmp_dir}
crowbook.zip.command:str:zip        # {zip}

# {prf_opt}
output.proofread.html:path          # {prf_html}
output.proofread.html.dir:path      # {prf_html_dir}
output.proofread.pdf:path           # {prf_pdf}

# {prf_opt2}
proofread:bool:false                              # {prf}
proofread.languagetool:bool:false                 # {prf_lng}
proofread.languagetool.port:int:8081              # {prf_lng_port}
proofread.grammalecte:bool:false                  # {prf_grammalecte}
proofread.grammalecte.port:int:8080               # {prf_grammalecte_port}
proofread.repetitions:bool:false                  # {prf_repet}
proofread.repetitions.max_distance:int:25         # {prf_max_dist}
proofread.repetitions.fuzzy:bool:true             # {prf_fuzzy}
proofread.repetitions.fuzzy.threshold:float:0.2   # {prf_fuzzy_t}
proofread.repetitions.ignore_proper:bool:true     # {prf_ignore}
proofread.repetitions.threshold:float:2.0         # {prf_threshold}



# {deprecated_opt}
html.css.colours:alias:html.css.colors              # {renamed}
input.smart_quotes:alias:input.clean.smart_quotes   # {renamed}
input.autoclean:alias:input.clean                   # {renamed}
base_path:alias:resources.base_path                 # {renamed}
base_path.links:alias:resources.base_path.links     # {renamed}
base_path.images:alias:resources.base_path.images   # {renamed}
side_notes:alias:html.side_notes                    # {renamed}
html.top:alias:html.header                          # {renamed}
autoclean:alias:input.autoclean                     # {renamed}
enable_yaml_blocks:alias:input.yaml_blocks          # {renamed}
use_initials:alias:rendering.initials               # {renamed}
toc_name:alias:rendering.inline_toc.name            # {renamed}
display_toc:alias:rendering.inline_toc              # {renamed}
numbering:alias:rendering.num_depth                 # {renamed}
numbering_template:alias:rendering.chapter_tempalte # {renamed}
html.display_chapter:alias:html_single.one_chapter  # {renamed}
temp_dir:alias:crowbook.temp_dir                    # {renamed}
zip.command:alias:crowbook.zip.command              # {renamed}
verbose:alias:crowbook.verbose                      # {renamed}
html.script:alias:html_singe.js                     # {renamed}
html.print_css:alias:html.css.print                 # {renamed}
html.template:alias:html_single.html                # {renamed}
html_dir.script:alias:html_dir.js                   # {renamed}
epub.template:alias:epub.chapter.xhtml              # {renamed}
html_dir.css:alias:html.css                         # {renamed}
rendering.chapter_template:alias:rendering.chapter.template # {renamed}
import_config:alias:import                          # {renamed}
html_single.one_chapter:alias:html.standalone.one_chapter #{renamed}
html_single.html:alias:html.standalone.template         # {renamed}
html_single.js:alias:html.standalone.js             # {renamed}
output.html_dir:alias:output.html.dir               # {renamed}
html_dir.index.html:alias:html.dir.template         # {renamed}
html_dir.chapter.html:alias:html.dir.template       # {renamed}
output.proofread.html_dir:alias:output.proofread.html.dir # {renamed}
tex.paper_size:alias:tex.paper.size                 # {renamed}
tex.font_size:alias:tex.font.size                   # {renamed}
html.highlight_code:alias:rendering.highlight       # {renamed}
proofread.nb_spaces:alias                           # {removed}
nb_char:alias                                       # {removed}
tex.short:alias                                     # {removed}
html.crowbook_link:alias                            # {removed}
crowbook.verbose:alias                              # {removed}
",
                                         metadata = lformat!("Metadata"),
                                         metadata2 = lformat!("Additional metadata"),
                                         output_opt = lformat!("Output options"),
                                         output = lformat!("Specify a list of output formats to render"),
                                         render_opt = lformat!("Rendering options"),
                                         special_ops = lformat!("Special option"),
                                         html_opt = lformat!("HTML options"),
                                         html_single_opt = lformat!("Standalone HTML options"),
                                         html_dir_opt = lformat!("Multifile HTML options"),
                                         html_if_opt = lformat!("Interactive fiction HTML options"),
                                         epub_opt = lformat!("EPUB options"),
                                         tex_opt = lformat!("LaTeX options"),
                                         rs_opt = lformat!("Resources option"),
                                         input_opt = lformat!("Input options"),
                                         crowbook_opt = lformat!("Crowbook options"),
                                         prf_opt = lformat!("Output options (for proofreading)"),
                                         prf_opt2 = lformat!("Proofreading options (only for output.proofread.* targets)"),
                                         deprecated_opt = lformat!("Deprecated options"),

                                         author = lformat!("Author of the book"),
                                         title = lformat!("Title of the book"),
                                         lang = lformat!("Language of the book"),
                                         subject = lformat!("Subject of the book (used for EPUB metadata)"),
                                         description = lformat!("Description of the book (used for EPUB metadata)"),
                                         cover = lformat!("Path to the cover of the book"),

                                         subtitle = lformat!("Subtitle of the book"),
                                         license = lformat!("License of the book. This information will be displayed on PDF documents"),
                                         version = lformat!("Version of the book"),
                                         date = lformat!("Date the book was revised"),
                                         autograph = lformat!("An autograph"),

                                         output_epub = lformat!("Output file name for EPUB rendering"),
                                         output_html = lformat!("Output file name for HTML rendering"),
                                         output_tex = lformat!("Output file name for LaTeX rendering"),
                                         output_pdf = lformat!("Output file name for PDF rendering"),
                                         output_odt = lformat!("Output file name for ODT rendering"),
                                         output_if = lformat!("Output file name for HTML (interactive fiction) rendering"),
                                         output_html_dir = lformat!("Output directory name for HTML rendering"),
                                         output_base_path = lformat!("Directory where those output files will we written"),

                                         rendering_highlight = lformat!("If/how highligh code blocks. Possible values: \"syntect\" (default, performed at runtime), \"highlight.js\" (HTML-only, uses Javascript), \"none\""),
                                         rendering_highlight_theme = lformat!("Theme for syntax highlighting (if rendering.highlight is set to 'syntect')"),
                                         rendering_initials = lformat!("Use initials ('lettrines') for first letter of a chapter"),
                                         inline_toc = lformat!("Display a table of content in the document"),
                                         toc_name = lformat!("Name of the table of contents if it is displayed in document"),
                                         num_depth = lformat!("The  maximum heading levels that should be numbered (0: no numbering, 1: only chapters, ..., 6: all)"),
                                         part = lformat!("How to call parts (or 'books', 'episodes', ...)"),
                                         chapter = lformat!("How to call chapters"),
                                         chapter_template = lformat!("Naming scheme of chapters, for TOC"),
                                         part_template = lformat!("Naming scheme of parts, for TOC"),
                                         roman_numerals_parts = lformat!("If set to true, display part number with roman numerals"),
                                         roman_numerals_chapters = lformat!("If set to true, display chapter number with roman numerals"),
                                         reset_counter = lformat!("If set to true, reset chapter number at each part"),

                                         import_config = lformat!("Import another book configuration file"),

                                         html_icon = lformat!("Path to an icon to be used for the HTML files(s)"),
                                         html_header = lformat!("Custom header to display at the beginning of html file(s)"),
                                         html_footer = lformat!("Custom footer to display at the end of HTML file(s)"),
                                         html_css = lformat!("Path of a stylesheet for HTML rendering"),
                                         html_css_add = lformat!("Some inline CSS added to the stylesheet template"),
                                         css_colors = lformat!("Path of a stylesheet for the colors for HTML"),
                                         html_js = lformat!("Path of a javascript file"),
                                         css_print = lformat!("Path of a media print stylesheet for HTML rendering"),
                                         highlight_js = lformat!("Set another highlight.js version than the bundled one"),
                                         highlight_css = lformat!("Set another highlight.js CSS theme than the default one"),
                                         side_notes = lformat!("Display footnotes as side notes in HTML/Epub (experimental)"),
                                         nb_spaces = lformat!("Replace unicode non breaking spaces with HTML entities and CSS"),
                                         nb_spaces_tex = lformat!("Replace unicode non breaking spaces with TeX code"),

                                         one_chapter = lformat!("Display only one chapter at a time (with a button to display all)"),
                                         single_html = lformat!("Path of an HTML template for standalone HTML"),
                                         single_js = lformat!("Path of a javascript file"),
                                         if_js = lformat!("Path of a javascript file"),
                                         if_new_turn = lformat!("Javascript code that will be run at the beginning of each segment"),
                                         if_end_turn = lformat!("Javascript code that will be run at the end of each segment"),
                                         if_new_game = lformat!("Javascript code that will be run at the beginning of a 'game'"),

                                         html_chapter_template = lformat!("Inline template for HTML chapter formatting"),
                                         html_part_template = lformat!("Inline template for HTML part formatting"),
                                         html_dir_template = lformat!("Path of a HTML template for multifile HTML"),

                                         epub_ver = lformat!("EPUB version to generate (2 or 3)"),
                                         epub_css = lformat!("Path of a stylesheet for EPUB"),
                                         epub_css_add = lformat!("Inline CSS added to the EPUB stylesheet template"),
                                         chapter_xhtml = lformat!("Path of an xhtml template for each chapter"),
                                         titlepage_xhtml = lformat!("Path of an xhtml template for the title page"),
                                         epub_toc = lformat!("Add 'Title' and (if set) 'Cover' in the EPUB table of contents"),

                                         tex_links = lformat!("Add foontotes to URL of links so they are readable when printed"),
                                         tex_command = lformat!("LaTeX command to use for generating PDF"),
                                         tex_tmpl = lformat!("Path of a LaTeX template file"),
                                         tex_tmpl_add = lformat!("Inline code added in the LaTeX template"),
                                         tex_class = lformat!("LaTeX class to use"),
                                         tex_title = lformat!("If true, generate a title with \\maketitle"),
                                         tex_paper_size = lformat!("Specifies the size of the page."),
                                         tex_margin_left = lformat!("Specifies left margin (note that with book class left and right margins are reversed for odd pages, thus the default value is 1.5cm for book class and 2cm else)"),
                                         tex_margin_right = lformat!("Specifies right margin(note that with book class left and right margins are reversed for odd pages, thus the default value is 2.5cm for book class and 2cm else)"),
                                         tex_margin_top = lformat!("Specifies top margin"),
                                         tex_margin_bottom = lformat!("Specifies left margin"),
                                         tex_font_size = lformat!("Specify latex font size (in pt, 10 (default), 11, or 12 are accepted)"),
                                         tex_hyperref = lformat!("If disabled, don't try to find references inside the document"),
                                         tex_stdpage = lformat!("If set to true, use 'stdpage' package to format a manuscript according to standards"),

                                         rs_files = lformat!("Whitespace-separated list of files to embed in e.g. EPUB file; useful for including e.g. fonts"),
                                         rs_out = lformat!("Paths where additional resources should be copied in the EPUB file or HTML directory"),
                                         rs_base = lformat!("Path where to find resources (in the source tree). By default, links and images are relative to the Markdown file. If this is set, it will be to this path."),
                                         rs_links = lformat!("Set base path but only for links. Useless if resources.base_path is set"),
                                         rs_img = lformat!("Set base path but only for images. Useless if resources.base_path is set"),
                                         rs_base_files = lformat!("Set base path but only for additional files. Useless if resources.base_path is set."),
                                         rs_tmpl = lformat!("Set base path but only for templates files. Useless if resources.base_path is set"),

                                         autoclean = lformat!("Toggle typographic cleaning of input markdown according to lang"),
                                         smart_quotes = lformat!("If enabled, tries to replace vertical quotations marks to curly ones"),
                                         ligature_dashes = lformat!("If enabled, replaces '--' to en dash ('–') and '---' to em dash ('—')"),
                                         ligature_guillemets = lformat!("If enabled, replaces '<<' and '>>' to french \"guillemets\" ('«' and '»')"),
                                         superscript = lformat!("If enabled, allow support for superscript and subscript using respectively foo^up^  and bar~down~ syntax."),
                                         yaml = lformat!("Enable inline YAML blocks to override options set in config file"),
                                         html_as_text = lformat!("Consider HTML blocks as text. This avoids having <foo> being considered as HTML and thus ignored."),
                                         files_mean_chapters = lformat!("Consider that a new file is always a new chapter, even if it does not include heading (default: only for numbered chapters)"),
                                         tmp_dir = lformat!("Path where to create a temporary directory (default: uses result from Rust's std::env::temp_dir())"),
                                         zip = lformat!("Command to use to zip files (for EPUB/ODT)"),

                                         prf_html = lformat!("Output file name for HTML rendering with proofread features"),
                                         prf_html_dir = lformat!("Output directory name for HTML rendering with proofread features"),
                                         prf_pdf = lformat!("Output file name for PDF rendering with proofread features"),
                                         prf = lformat!("If set to false, will disactivate proofreading even if one of output.proofread.x is present"),
                                         prf_lng = lformat!("If true, try to use language tool server to grammar check the book"),
                                         prf_lng_port = lformat!("Port to connect to languagetool-server"),
                                         prf_grammalecte = lformat!("If true, try to use grammalecte server to grammar check the book"),
                                         prf_grammalecte_port = lformat!("Port to connect to grammalecte server"),
                                         prf_repet = lformat!("If set to true, use Caribon to detect repetitions"),
                                         prf_max_dist = lformat!("Max distance between two occurences so it is considered a repetition"),
                                         prf_fuzzy = lformat!("Enable fuzzy string matching"),
                                         prf_fuzzy_t = lformat!("Max threshold of differences to consider two strings a repetition"),
                                         prf_ignore = lformat!("Ignore proper nouns for repetitions"),
                                         prf_threshold = lformat!("Threshold to detect a repetition"),

                                         tex_theme = lformat!("If set, set theme for syntax highlighting for LaTeX/PDF output (syntect only)"),
                                         html_theme = lformat!("If set, set theme for syntax highlighting for HTML output (syntect only)"),
                                         epub_theme = lformat!("If set, set theme for syntax highlighting for EPUB output (syntect only)"),

                                         renamed = lformat!("Renamed"),
                                         removed = lformat!("Removed"),
    );
}

/// Contains the options of a book.
///
/// This structure offers some facilities to check the content of an option.
/// It also verifies, when setting an option, that it corresponds to certain
/// values (e.g. if you expect an int, you can't set this option to "foo").
///
/// # Example
///
/// ```
/// use crowbook::BookOptions;
/// let mut options = BookOptions::new();
///
/// // By default, `lang` is set to "en"
/// assert_eq!(options.get_str("lang").unwrap(), "en");
///
/// // We can change it to "fr"
/// options.set("lang", "fr").unwrap();
/// assert_eq!(options.get_str("lang").unwrap(), "fr");
///
/// // `epub.version` must be an int, we can't set it to a string
/// let res = options.set("epub.version", "foo");
/// assert!(res.is_err());
/// ```
#[derive(Debug)]
pub struct BookOptions {
    options: HashMap<String, BookOption>,
    defaults: HashMap<String, BookOption>,
    deprecated: HashMap<String, Option<String>>,
    valid_tpls: Vec<&'static str>,
    valid_bools: Vec<&'static str>,
    valid_chars: Vec<&'static str>,
    valid_strings: Vec<&'static str>,
    valid_paths: Vec<&'static str>,
    valid_ints: Vec<&'static str>,
    valid_floats: Vec<&'static str>,
    valid_str_vecs: Vec<&'static str>,
    metadata: Vec<String>,

    /// Source for errors (unnecessary copy :/)
    #[doc(hidden)]
    pub source: Source,

    /// Root path of the book (unnecessary copy :/)
    #[doc(hidden)]
    pub root: PathBuf,
}

impl BookOptions {
    /// Creates a new BookOptions struct from the default compiled string
    pub fn new() -> BookOptions {
        let mut options = BookOptions {
            options: HashMap::new(),
            deprecated: HashMap::new(),
            defaults: HashMap::new(),
            valid_bools: vec![],
            valid_chars: vec![],
            valid_ints: vec![],
            valid_floats: vec![],
            valid_strings: vec![],
            valid_paths: vec![],
            valid_tpls: vec![],
            valid_str_vecs: vec![],
            metadata: vec![],
            root: PathBuf::new(),
            source: Source::empty(),
        };

        // Load default options and types from OPTIONS
        for (_, key, option_type, default_value) in Self::options_to_vec() {
            if key.is_none() {
                continue;
            }
            let key = key.unwrap();
            match option_type.unwrap() {
                "meta" => {
                    options.metadata.push(key.to_owned());
                    options.valid_strings.push(key);
                }
                "str" => options.valid_strings.push(key),
                "strvec" => options.valid_str_vecs.push(key),
                "bool" => options.valid_bools.push(key),
                "int" => options.valid_ints.push(key),
                "float" => options.valid_floats.push(key),
                "char" => options.valid_chars.push(key),
                "path" => options.valid_paths.push(key),
                "tpl" => {
                    options.valid_tpls.push(key);
                    options.valid_paths.push(key);
                }
                "alias" => {
                    options
                        .deprecated
                        .insert(key.to_owned(), default_value.map(|s| s.to_owned()));
                    continue;
                }
                _ => {
                    panic!(
                        "{}",
                        lformat!(
                            "Ill-formatted OPTIONS string: unrecognized type \
                                     '{option_type}'",
                            option_type = option_type.unwrap()
                        )
                    )
                }
            }
            if key == "crowbook.temp_dir" {
                // "temp_dir" has a special default value that depends on the environment
                options
                    .set(key, &env::temp_dir().to_string_lossy())
                    .unwrap();
                continue;
            }
            if let Some(value) = default_value {
                options.set(key, value).unwrap();
                // hack to get the BookOption without changing the API
                let option = options.set(key, value).unwrap();
                options.defaults.insert(key.to_owned(), option.unwrap());
            }
        }
        options
    }

    /// Sets an option from a Yaml tuple
    ///
    /// # Arguments
    /// * `key`: the identifier of the option, must be Yaml::String(_)
    /// * `value`: the value of the option
    ///
    /// # Returns
    ///
    /// * an error either if `key` is not a valid option or if the value is not of the right type.
    /// * an option containing None if key was not set, and Some(previous_value) if key was
    ///   already present.
    #[doc(hidden)]
    pub fn set_yaml(&mut self, key: Yaml, value: Yaml) -> Result<Option<BookOption>> {
        let key: String = if let Yaml::String(key) = key {
            key
        } else {
            return Err(Error::book_option(
                &self.source,
                lformat!("Expected a String as a key, found {:?}", key),
            ));
        };

        if self.valid_str_vecs.contains(&key.as_ref()) {
            // Value is a list of string
            if let Yaml::Array(array) = value {
                let mut inner: Vec<String> = vec![];
                for value in array.into_iter() {
                    if let Yaml::String(value) = value {
                        inner.push(value);
                    } else {
                        return Err(Error::book_option(
                            &self.source,
                            lformat!(
                                "Expected only string in the list for key {}, found {:?}",
                                &key,
                                &value
                            ),
                        ));
                    }
                }
                // special case
                if &key == "output" {
                    for format in &inner {
                        self.set_yaml(
                            Yaml::String(format!("output.{format}")),
                            Yaml::String(String::from("auto")),
                        )
                        .map_err(|_| {
                            Error::book_option(
                                &self.source,
                                lformat!(
                                    "The output format {format} for key {key} is not recognized",
                                    key = key,
                                    format = format
                                ),
                            )
                        })?;
                    }
                }
                Ok(self.options.insert(key, BookOption::StringVec(inner)))
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "Expected a list as value for key {}, found {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else if self.valid_strings.contains(&key.as_ref()) {
            // value is a string
            if let Yaml::String(value) = value {
                Ok(self.options.insert(key, BookOption::String(value)))
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "Expected a string as value for key {}, found \
                                                 {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else if self.valid_paths.contains(&key.as_ref()) {
            // value is a path
            if let Yaml::String(value) = value {
                if &key == "import" {
                    // special case, not a real option
                    let tmp = self.root.join(&value);
                    let file = tmp.to_str().ok_or_else(|| {
                        Error::book_option(
                            &self.source,
                            lformat!(
                                "'{value}''s path contains invalid \
                                                                    UTF-8 code",
                                value = &value
                            ),
                        )
                    })?;
                    let mut book = Book::new();
                    book.load_file(file)?;
                    let options = mem::replace(&mut book.options, BookOptions::new());
                    self.merge(options)?;
                    Ok(None)
                } else {
                    Ok(self.options.insert(key, BookOption::Path(value)))
                }
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "expected a string as value for key '{}', found \
                                                 {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else if self.valid_chars.contains(&key.as_ref()) {
            // value is a char
            if let Yaml::String(value) = value {
                let chars: Vec<_> = value.chars().collect();
                if chars.len() != 1 {
                    return Err(Error::book_option(
                        &self.source,
                        lformat!(
                            "could not parse '{value}' as a \
                                                            char: does not contain exactly one \
                                                            char",
                            value = &value
                        ),
                    ));
                }
                Ok(self.options.insert(key, BookOption::Char(chars[0])))
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "expected a string as value containing a char \
                                                 for key '{}', found {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else if self.valid_bools.contains(&key.as_ref()) {
            // value is a bool
            if let Yaml::Boolean(value) = value {
                Ok(self.options.insert(key, BookOption::Bool(value)))
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "expected a boolean as value for key '{}', \
                                                 found {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else if self.valid_ints.contains(&key.as_ref()) {
            // value is an int
            if let Yaml::Integer(value) = value {
                Ok(self.options.insert(key, BookOption::Int(value as i32)))
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "expected an integer as value for key '{}', \
                                                 found {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else if self.valid_floats.contains(&key.as_ref()) {
            // value is a float
            if let Yaml::Real(value) = value {
                match value.parse::<f32>() {
                    Ok(value) => Ok(self.options.insert(key, BookOption::Float(value))),
                    Err(_) => Err(Error::book_option(
                        &self.source,
                        lformat!(
                            "could not parse '{value}' as a float \
                                                         for key '{key}'",
                            value = &value,
                            key = &key
                        ),
                    )),
                }
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "expected a float as value for key '{}', found \
                                                 {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else if self.deprecated.contains_key(&key) {
            let opt = self.deprecated[&key].clone();
            if let Some(new_key) = opt {
                warn!(
                    "{}",
                    lformat!(
                        "'{old_key}' has been deprecated, you should \
                                                  now use '{new_key}'",
                        old_key = &key,
                        new_key = &new_key
                    )
                );
                self.set_yaml(Yaml::String(new_key), value)
            } else {
                Err(Error::book_option(
                    self.source.clone(),
                    lformat!("key '{key}' has been deprecated.", key = &key),
                ))
            }
        } else if key.starts_with("metadata.") {
            // key is a custom metadata
            // value must be a string
            if let Yaml::String(value) = value {
                self.metadata.push(key.clone());
                Ok(self.options.insert(key, BookOption::String(value)))
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "expected a string as value for key '{}', found \
                                                 {:?}",
                        &key,
                        &value
                    ),
                ))
            }
        } else {
            // key not recognized
            Err(Error::book_option(
                self.source.clone(),
                lformat!("unrecognized key '{key}'", key = &key),
            ))
        }
    }

    /// Sets an option
    ///
    /// # Arguments
    /// * `key`: the identifier of the option, e.g.: "author"
    /// * `value`: the value of the option as a string
    ///
    /// # Returns
    /// * an error either if `key` is not a valid option or if the
    ///   value is not of the right type.
    /// * an option containing None if key was
    ///   not set, and Some(previous_value) if key was already present.
    ///
    /// # Examples
    /// ```
    /// use crowbook::Book;
    /// let mut book = Book::new();
    /// // Set author
    /// book.options.set("author", "Joan Doe").unwrap();
    /// // Set numbering to chapters and subsections
    /// book.options.set("rendering.num_depth", "2").unwrap();
    /// // Try to set invalid key "autor"
    /// let result = book.options.set("autor", "John Smith");
    /// assert!(result.is_err()); // error: "author" was mispelled "autor"
    ///
    /// let result = book.options.set("rendering.num_depth", "foo");
    /// assert!(result.is_err()); // error: numbering must be an int
    /// ```
    pub fn set(&mut self, key: &str, value: &str) -> Result<Option<BookOption>> {
        let result = YamlLoader::load_from_str(value);
        if let Ok(yaml_docs) = result {
            if yaml_docs.len() == 1 {
                let yaml_value = yaml_docs.into_iter().next().unwrap();
                self.set_yaml(Yaml::String(key.to_owned()), yaml_value)
            } else {
                Err(Error::book_option(
                    &self.source,
                    lformat!(
                        "value '{value}' for key '{key}' does not \
                                                 contain one and only one YAML value",
                        value = value,
                        key = key
                    ),
                ))
            }
        } else {
            Err(Error::book_option(
                &self.source,
                lformat!(
                    "could not parse '{value}' as a valid YAML value",
                    value = value
                ),
            ))
        }
    }

    /// Return the list of keys that are metadata
    #[doc(hidden)]
    pub fn get_metadata(&self) -> &[String] {
        &self.metadata
    }

    /// Gets an option
    #[doc(hidden)]
    pub fn get(&self, key: &str) -> Result<&BookOption> {
        self.options.get(key).ok_or_else(|| {
            Error::invalid_option(
                &self.source,
                lformat!("option '{key}' is not present", key = key),
            )
        })
    }

    /// Gets a string option.
    ///
    /// # Returns
    ///
    /// * A string if `key` is valid and corresponds to a string
    /// * An error either if `key` is not valid or is not a string.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook::BookOptions;
    /// let options = BookOptions::new();
    /// assert!(options.get_str("author").is_ok());
    /// assert!(options.get_str("rendering.inline_toc").is_err());
    /// ```
    pub fn get_str(&self, key: &str) -> Result<&str> {
        self.get(key)?.as_str()
    }

    /// Get a stringvec option
    pub fn get_str_vec(&self, key: &str) -> Result<&[String]> {
        self.get(key)?.as_str_vec()
    }

    /// Get a path option.
    ///
    /// Adds the correct path correction before it.
    pub fn get_path(&self, key: &str) -> Result<String> {
        let path: &str = self.get(key)?.as_path()?;

        if Path::new(path).is_absolute() {
            // path is absolute, do nothing
            return Ok(path.to_owned());
        }

        let new_path: PathBuf = match key {
            "resources.base_path.links"
            | "resources.base_path.images"
            | "resources.base_path.files"
            | "resources.pase_path.templates" => {
                // If resources.base_path is set, return it, else return itself
                let base_path = self.get_path("resources.base_path");
                if base_path.is_ok() {
                    return base_path;
                }
                self.root.join(path)
            }

            "cover" | "html.icon" => {
                // Translate according to resources.base_path.images
                let base = self.get_path("resources.base_path.images").unwrap();
                Path::new(&base).join(path)
            }

            "output.epub"
            | "output.html"
            | "output.html.dir"
            | "output.pdf"
            | "output.tex"
            | "output.odt"
            | "output.proofread.html"
            | "output.proofread.html.dir"
            | "output.proofread.pdf"
            | "output.html.if" => {
                // Translate according to output.base_path
                let base = self.get_path("output.base_path").unwrap();
                Path::new(&base).join(path)
            }

            key if self.valid_tpls.contains(&key) => {
                // Translate according to resources.base_path.template
                let base = self.get_path("resources.base_path.templates").unwrap();
                Path::new(&base).join(path)
            }

            _ => self.root.join(path),
        };
        if let Some(path) = new_path.to_str() {
            Ok(path.to_owned())
        } else {
            Err(Error::book_option(
                &self.source,
                lformat!("'{key}''s path contains invalid UTF-8 code", key = key),
            ))
        }
    }

    /// Get a path option
    ///
    /// Don't add book's root path before it.
    pub fn get_relative_path(&self, key: &str) -> Result<&str> {
        self.get(key)?.as_path()
    }

    /// Gets a bool option
    ///
    /// # Example
    ///
    /// ```
    /// # use crowbook::BookOptions;
    /// let options = BookOptions::new();
    /// assert!(options.get_bool("epub.toc.extras").is_ok());
    /// ```
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.get(key)?.as_bool()
    }

    /// Gets a char option
    pub fn get_char(&self, key: &str) -> Result<char> {
        self.get(key)?.as_char()
    }

    /// Gets an int  option
    ///
    /// # Example
    ///
    /// ```
    /// # use crowbook::BookOptions;
    /// let options = BookOptions::new();
    /// assert!(options.get_i32("rendering.num_depth").is_ok());
    /// ```
    pub fn get_i32(&self, key: &str) -> Result<i32> {
        self.get(key)?.as_i32()
    }

    /// Gets a float option
    pub fn get_f32(&self, key: &str) -> Result<f32> {
        self.get(key)?.as_f32()
    }

    /// Merges the other list of options to the first one
    ///
    /// If option is already set in self, don't add it, unless it was the default.
    /// Option is not inserted either if new value is equal to default.
    #[doc(hidden)]
    pub fn merge(&mut self, other: BookOptions) -> Result<()> {
        for (key, value) in &other.options {
            // Check if option was already set, and if it was to default or to something else
            if self.defaults.contains_key(key) {
                let previous_opt = self.options.get(key);
                let default = &self.defaults[key];
                // If new value is equal to default, don't insert it
                if value == default {
                    continue;
                }
                if let Some(previous_opt) = previous_opt {
                    if previous_opt != default {
                        // Previous value is other than default, don't merge
                        continue;
                    }
                }
            }
            // If it's a path, get the corrected path
            if let &BookOption::Path(_) = value {
                // Sets key with an absolute path so it
                // won't be messed up if resources.base_path is
                // redefined later on
                let path = other.get_path(key).unwrap();
                let new_path = ::std::env::current_dir()
                    .map_err(|_| {
                        Error::default(
                            Source::empty(),
                            lformat!("could not get current directory!"),
                        )
                    })?
                    .join(&path);
                let new_path = if let Some(path) = new_path.to_str() {
                    path.to_owned()
                } else {
                    return Err(Error::book_option(
                        Source::new(other.root.to_str().unwrap()),
                        lformat!(
                            "'{key}''s path contains invalid \
                                                            UTF-8 code",
                            key = key
                        ),
                    ));
                };
                self.options.insert(key.clone(), BookOption::Path(new_path));
            } else {
                self.options.insert(key.clone(), value.clone());
            }
        }
        Ok(())
    }

    /// Returns a description of all options valid to pass to a book.
    ///
    /// # Arguments
    /// * `md`: whether the output should be formatted in Markdown
    ///
    /// # Example
    /// ```
    /// use crowbook::BookOptions;
    /// println!("{}", BookOptions::description(false));
    /// ```
    pub fn description(md: bool) -> String {
        let mut out = String::new();
        let mut previous_is_comment = true;
        for (comment, key, o_type, default) in Self::options_to_vec() {
            // Don't display deprecated options if md is not set
            if !md && comment.trim() == lformat!("Deprecated options") {
                return out;
            }
            if key.is_none() {
                if !previous_is_comment {
                    out.push('\n');
                    previous_is_comment = true;
                }
                let header = if md {
                    format!("### {} ###\n", comment.trim())
                } else {
                    format!("{}\n", style::header(&comment.trim().to_uppercase()))
                };
                out.push_str(&header);
                continue;
            }
            previous_is_comment = false;
            let o_type = match o_type.unwrap() {
                "bool" => lformat!("boolean"),
                "float" => lformat!("float"),
                "int" => lformat!("integer"),
                "char" => lformat!("char"),
                "str" => lformat!("string"),
                "path" => lformat!("path"),
                "tpl" => lformat!("template path"),
                "meta" => lformat!("metadata"),
                "strvec" => lformat!("list of strings"),
                "alias" => lformat!("DEPRECATED"),
                _ => unreachable!(),
            };
            let def = if let Some(value) = default {
                value.to_owned()
            } else {
                lformat!("not set")
            };
            if md {
                out.push_str(&lformat!(
                    "- **`{key}`**
    - **type**: {option_type}
    - **default value**: `{default}`
    - {comment}\n",
                    key = key.unwrap(),
                    option_type = o_type,
                    default = def,
                    comment = comment
                ));
            } else {
                out.push_str(&format!("{key}
  {type} {option_type} ({msg} {default})
{comment}\n",
                                      type = style::field(&lformat!("type:")),
                                      key = style::element(key.unwrap()),
                                      option_type = style::tipe(&o_type),
                                      msg = lformat!("default:"),
                                      default = style::value(&def),
                                      comment = style::fill(comment.trim(), "  ")));
            }
        }
        out
    }

    /// OPTIONS to a vec of tuples (comment, key, type, default value)
    #[allow(clippy::type_complexity)]
    fn options_to_vec() -> Vec<(
        &'static str,
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
    )> {
        let mut out = vec![];
        for line in OPTIONS.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(stripped) = line.strip_prefix('#') {
                out.push((stripped, None, None, None));
                continue;
            }
            let v: Vec<_> = line.split(" #").collect();
            let content = v[0];
            let comment = v[1];
            let v: Vec<_> = content.split(':').collect();
            let key = Some(v[0].trim());
            let option_type = Some(v[1].trim());
            let default_value = if v.len() > 2 { Some(v[2].trim()) } else { None };
            out.push((comment, key, option_type, default_value));
        }
        out
    }
}

impl Default for BookOptions {
    fn default() -> Self {
        Self::new()
    }
}
