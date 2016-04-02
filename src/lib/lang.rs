use yaml_rust::{YamlLoader, Yaml};
use yaml_rust::yaml::Hash;

static EN:&'static str = include_str!("../../lang/en.yaml");
static FR:&'static str = include_str!("../../lang/fr.yaml");

/// Get the hashmap for a given language
pub fn get_hash(lang: &str) -> Hash {
    let docs = match lang {
        "fr" => YamlLoader::load_from_str(FR).unwrap(),
        _ => YamlLoader::load_from_str(EN).unwrap(),
    };
    let elem = docs.into_iter().next().unwrap();
    if let Yaml::Hash(hash) = elem {
        hash
    } else {
        panic!("Yaml file for language {} didn't contain a hash", lang);
    }
}
