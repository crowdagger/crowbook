use crate::book::Book;
use crate::bookoption::BookOption;
use crate::error::{Error, Result, Source};
use crate::style;

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader};
use rust_i18n::t;

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
output.html.if:path                 # {output_if}
output.base_path:path:\"\"            # {output_base_path}

# {render_opt}
rendering.highlight:str:syntect                                      # {rendering_highlight}
rendering.highlight.theme:str:InspiredGitHub                         # {rendering_highlight_theme}
rendering.initials:bool:false                                        # {rendering_initials}
rendering.inline_toc:bool:false                                      # {inline_toc}
rendering.inline_toc.name:str:\"{{{{loc_toc}}}}\"                        # {toc_name}
rendering.num_depth:int:1                                            # {num_depth}
rendering.chapter:str                                                # {chapter}
rendering.part:str                                                   # {part}
rendering.chapter.roman_numerals:bool:false                                  # {roman_numerals_chapters}
rendering.part.roman_numerals:bool:true                                      # {roman_numerals_parts}
rendering.part.reset_counter:bool:true                                      # {reset_counter}
rendering.chapter.template:str:\"{{{{number}}}}. {{{{chapter_title}}}}\" # {chapter_template}

rendering.part.template:str:\"{{{{number}}}}. {{{{part_title}}}}\" # {part_template}




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
html.chapter.template:str:\"<h1 id = 'link-{{{{link}}}}'>{{% if has_number %}}<span class = 'chapter-header'>{{{{header}}}} {{{{number}}}}</span>{{% if has_title %}}<br />{{% endif %}}{{% endif %}}{{{{title}}}}</h1>\" # {html_chapter_template}
html.part.template:str:\"<h2 class = 'part'>{{{{header}}}} {{{{number}}}}</h2> <h1 id = 'link-{{{{link}}}}' class = 'part'>{{{{title}}}}</h1>\" # {html_part_template}

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

# {input_opt}    #[serde(flatten)]

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
tex.paper_size:alias:tex.paper.size                 # {renamed}
tex.font_size:alias:tex.font.size                   # {renamed}
html.highlight_code:alias:rendering.highlight       # {renamed}
output.proofread.html_dir:alias:output.proofread.html.dir # {removed}
proofread.nb_spaces:alias                           # {removed}
nb_char:alias                                       # {removed}
tex.short:alias                                     # {removed}
html.crowbook_link:alias                            # {removed}
crowbook.verbose:alias                              # {removed}
output.proofread.html:path                          # {removed}
output.proofread.html.dir:path                      # {removed}
output.proofread.pdf:path                           # {removed}
proofread:bool:false                                # {removed}
proofread.languagetool:bool:false                   # {removed}
proofread.languagetool.port:int:8081                # {removed}
proofread.grammalecte:bool:false                    # {removed}
proofread.grammalecte.port:int:8080                 # {removed}
proofread.repetitions:bool:false                    # {removed}
proofread.repetitions.max_distance:int:25           # {removed}
proofread.repetitions.fuzzy:bool:true               # {removed}
proofread.repetitions.fuzzy.threshold:float:0.2     # {removed}
proofread.repetitions.ignore_proper:bool:true       # {removed}
proofread.repetitions.threshold:float:2.0           # {removed}
output.odt:path                                     # {removed}

",
                                         metadata = t!("opt.metadata"),
                                         metadata2 = t!("opt.add_metadata"),
                                         output_opt = t!("opt.output_opt"),
                                         output = t!("opt.output"),
                                         render_opt = t!("opt.render"),
                                         special_ops = t!("opt.special"),
                                         html_opt = t!("opt.html"),
                                         html_single_opt = t!("opt.html_single"),
                                         html_dir_opt = t!("opt.html_dir"),
                                         html_if_opt = t!("opt.html_if"),
                                         epub_opt = t!("opt.epub"),
                                         tex_opt = t!("opt.tex"),
                                         rs_opt = t!("opt.resources"),
                                         input_opt = t!("opt.input"),
                                         crowbook_opt = t!("opt.crowbook"),
                                         deprecated_opt = t!("opt.deprecated"),

                                         author = t!("opt.author"),
                                         title = t!("opt.title"),
                                         lang = t!("opt.lang"),
                                         subject = t!("opt.subject"),
                                         description = t!("opt.description"),
                                         cover = t!("opt.cover"),

                                         subtitle = t!("opt.subtitle"),
                                         license = t!("opt.license"),
                                         version = t!("opt.version"),
                                         date = t!("opt.date"),
                                         autograph = t!("opt.autograph"),

                                         output_epub = t!("opt.output_epub"),
                                         output_html = t!("opt.output_html"),
                                         output_tex = t!("opt.output_tex"),
                                         output_pdf = t!("opt.output_pdf"),
                                         output_if = t!("opt.output_if"),
                                         output_html_dir = t!("opt.output_html_dir"),
                                         output_base_path = t!("opt.output_base_path"),

                                         rendering_highlight = t!("opt.rendering_highlight"),
                                         rendering_highlight_theme = t!("opt.rendering_highlight_theme"),
                                         rendering_initials = t!("opt.rendering_initials"),
                                         inline_toc = t!("opt.inline_toc"),
                                         toc_name = t!("opt.toc_name"),
                                         num_depth = t!("opt.num_depth"),
                                         part = t!("opt.part"),
                                         chapter = t!("opt.chapter"),
                                         chapter_template = t!("opt.chapter_template"),
                                         part_template = t!("opt.part_template"),
                                         roman_numerals_parts = t!("opt.roman_numeral_parts"),
                                         roman_numerals_chapters = t!("opt.roman_numerals_chapters"),
                                         reset_counter = t!("opt.reset_counter"),

                                         import_config = t!("opt.import"),

                                         html_icon = t!("opt.html_icon"),
                                         html_header = t!("opt.html_header"),
                                         html_footer = t!("opt.html_footer"),
                                         html_css = t!("opt.html_css"),
                                         html_css_add = t!("opt.html_css_add"),
                                         css_colors = t!("opt.css_colors"),
                                         html_js = t!("opt.html_js"),
                                         css_print = t!("opt.css_print"),
                                         highlight_js = t!("opt.highlight_js"),
                                         highlight_css = t!("opt.highlight_css"),
                                         side_notes = t!("opt.side_notes"),
                                         nb_spaces = t!("opt.nb_spaces"),
                                         nb_spaces_tex = t!("opt.nb_spaces_tex"),

                                         one_chapter = t!("opt.one_chapter"),
                                         single_html = t!("opt.single_html"),
                                         single_js = t!("opt.single_js"),
                                         if_js = t!("opt.if_js"),
                                         if_new_turn = t!("opt.if_new_turn"),
                                         if_end_turn = t!("opt.if_end_turn"),
                                         if_new_game = t!("opt.if_new_game"),

                                         html_chapter_template = t!("opt.html_chapter_template"),
                                         html_part_template = t!("opt.html_part_template"),
                                         html_dir_template = t!("opt.html_dir_template"),

                                         epub_ver = t!("opt.epub_ver"),
                                         epub_css = t!("opt.epub_css"),
                                         epub_css_add = t!("opt.epub_css_add"),
                                         chapter_xhtml = t!("opt.chapter_xhtml"),
                                         titlepage_xhtml = t!("opt.titlepage_xhtml"),
                                         epub_toc = t!("opt.epub_toc"),

                                         tex_links = t!("opt.tex_links"),
                                         tex_command = t!("opt.tex_command"),
                                         tex_tmpl = t!("opt.tex_tmpl"),
                                         tex_tmpl_add = t!("opt.tex_tmpl_add"),
                                         tex_class = t!("opt.tex_class"),
                                         tex_title = t!("opt.tex_title"),
                                         tex_paper_size = t!("opt.tex_paper_size"),
                                         tex_margin_left = t!("opt.tex_margin_left"),
                                         tex_margin_right = t!("opt.tex_margin_right"),
                                         tex_margin_top = t!("opt.tex_margin_top"),
                                         tex_margin_bottom = t!("opt.tex_margin_bottom"),
                                         tex_font_size = t!("opt.tex_font_size"),
                                         tex_hyperref = t!("opt.tex_hyperref"),
                                         tex_stdpage = t!("opt.tex_stdpage"),

                                         rs_files = t!("opt.rs_files"),
                                         rs_out = t!("opt.rs_out"),
                                         rs_base = t!("opt.rs_base"),
                                         rs_links = t!("opt.rs_links"),
                                         rs_img = t!("opt.rs_img"),
                                         rs_base_files = t!("opt.rs_base_files"),
                                         rs_tmpl = t!("opt.rs_tmpl"),

                                         autoclean = t!("opt.autoclean"),
                                         smart_quotes = t!("opt.smart"),
                                         ligature_dashes = t!("opt.dashes"),
                                         ligature_guillemets = t!("opt.guillemets"),
                                         superscript = t!("opt.superscript"),
                                         yaml = t!("opt.yaml"),
                                         html_as_text = t!("opt.html_as_text"),
                                         files_mean_chapters = t!("opt.files_mean_chapters"),
                                         tmp_dir = t!("opt.tmp_dir"),
                                         zip = t!("opt.zip"),

                                         tex_theme = t!("opt.tex_theme"),
                                         html_theme = t!("opt.html_theme"),
                                         epub_theme = t!("opt.epub_theme"),

                                         renamed = t!("opt.renamed"),
                                         removed = t!("opt.removed"),
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
                        t!(
                            "opt.ill_forrmatted",
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
                t!("opt.expected_string", key = format!("{:?}", key)),
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
                            t!(
                                "opt.expected_strings",
                                key = &key,
                                value = format!("{:?}", &value)
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
                                t!(
                                    "opt.format_not_recognized",
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
                    t!(
                        "opt.expected_list",
                        key = &key,
                        value = format!("{:?}", &value)
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
                    t!(
                        "opt.expected_string_value",
                        key = &key,
                        value = format!("{:?}", &value)
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
                            t!(
                                "opt.invalid_utf8",
                                value = &value
                            ),
                        )
                    })?;
                    let mut book = Book::new();
                    book.load_file(file)?;
                    self.merge(&book.options)?;
                    Ok(None)
                } else {
                    Ok(self.options.insert(key, BookOption::Path(value)))
                }
            } else {
                Err(Error::book_option(
                    &self.source,
                    t!(
                        "opt.expected_string_value",
                        key = &key,
                        value = format!("{:?}", &value)
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
                        t!(
                            "opt.expected_char",
                            value = &value
                        ),
                    ));
                }
                Ok(self.options.insert(key, BookOption::Char(chars[0])))
            } else {
                Err(Error::book_option(
                    &self.source,
                    t!(
                        "opt.expected_char_value",
                        key = &key,
                        value = format!("{:?}", &value)
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
                    t!(
                        "opt.expected_bool",
                        key = &key,
                        value = format!("{:?}", &value)
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
                    t!(
                        "opt.expected_int",
                        key = &key,
                        value = format!("{:?}", &value)
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
                        t!(
                            "opt.expected_float",
                            value = &value,
                            key = &key
                        ),
                    )),
                }
            } else {
                Err(Error::book_option(
                    &self.source,
                    t!(
                        "opt.expected_float",
                        key = &key,
                        value = format!("{:?}", &value)
                    ),
                ))
            }
        } else if self.deprecated.contains_key(&key) {
            let opt = self.deprecated[&key].clone();
            if let Some(new_key) = opt {
                warn!(
                    "{}",
                    t!(
                        "opt.warn_deprecated",
                        old_key = &key,
                        new_key = &new_key
                    )
                );
                self.set_yaml(Yaml::String(new_key), value)
            } else {
                Err(Error::book_option(
                    self.source.clone(),
                    t!("opt.err_deprecated", key = &key),
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
                    t!(
                        "opt.expected_string_value",
                        key = &key,
                        value = format!("{:?}", &&value)
                    ),
                ))
            }
        } else {
            // key not recognized
            Err(Error::book_option(
                self.source.clone(),
                t!("opt.unrecognized", key = &key),
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
                    t!(
                        "opt.one_yaml",
                        value = value,
                        key = key
                    ),
                ))
            }
        } else {
            Err(Error::book_option(
                &self.source,
                t!(
                    "opt.yaml_value",
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
                t!("opt.miss_key", key = key),
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
                t!("opt.invalid_utf8", value = key),
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
    pub fn merge(&mut self, other: &BookOptions) -> Result<()> {
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
                            t!("opt.curr_dir"),
                        )
                    })?
                    .join(&path);
                let new_path = if let Some(path) = new_path.to_str() {
                    path.to_owned()
                } else {
                    return Err(Error::book_option(
                        Source::new(other.root.to_str().unwrap()),
                        t!(
                            "opt.invalid_utf8",
                            value = key
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
            if !md && comment.trim() == t!("opt.deprecated") {
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
                "bool" => t!("ty.bool"),
                "float" => t!("ty.float"),
                "int" => t!("ty.int"),
                "char" => t!("ty.char"),
                "str" => t!("ty.str"),
                "path" => t!("ty.path"),
                "tpl" => t!("ty.tpl"),
                "meta" => t!("ty.meta"),
                "strvec" => t!("ty.strvec"),
                "alias" => t!("ty.alias"),
                _ => unreachable!(),
            };
            let def = if let Some(value) = default {
                value.to_owned()
            } else {
                t!("opt.not_set")
            };
            if md {
                out.push_str(&t!(
                    "opt.option_description_md",
                    key = key.unwrap(),
                    option_type = o_type,
                    default = def,
                    comment = comment
                ));
            } else {
                out.push_str(&format!("{key}
  {type} {option_type} ({msg} {default})
{comment}\n",
                                      type = style::field(&t!("ty.type")),
                                      key = style::element(key.unwrap()),
                                      option_type = style::tipe(&o_type),
                                      msg = t!("ty.default"),
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
