use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlLoader};
use rust_i18n::t;

static EN: &str = include_str!("../../lang/document/en.yaml");
static ES: &str = include_str!("../../lang/document/es.yaml");
static FR: &str = include_str!("../../lang/document/fr.yaml");
static RU: &str = include_str!("../../lang/document/ru.yaml");
static DE: &str = include_str!("../../lang/document/de.yaml");

/// Get the hashmap for a given language
pub fn get_hash(lang: &str) -> Hash {
    let lang = lang.to_lowercase();
    let docs = if lang.starts_with("fr") {
        YamlLoader::load_from_str(FR).unwrap()
    } else if lang.starts_with("es") {
        YamlLoader::load_from_str(ES).unwrap()
    } else if lang.starts_with("de") {
        YamlLoader::load_from_str(DE).unwrap()
    } else if lang.starts_with("ru") {
        YamlLoader::load_from_str(RU).unwrap()
    } else {
        YamlLoader::load_from_str(EN).unwrap()
    };
    let elem = docs.into_iter().next().unwrap();
    if let Yaml::Hash(hash) = elem {
        hash
    } else {
        panic!(
            "{}",
            t!("error.yaml_lang",
               lang = lang
            )
        );
    }
}

/// Get a string for a given language
pub fn get_str(lang: &str, s: &str) -> String {
    let hash = get_hash(lang);
    let yaml = hash.get(&Yaml::String(s.to_owned())).expect(&t!(
        "error.yaml_translation",
        key = s,
        lang = lang
    ));
    if let Yaml::String(result) = yaml {
        result.clone()
    } else {
        panic!(
            "{}",
            t!("error.yaml_translation_string",
                key = s,
                lang = lang
            )
        );
    }
}
