use yaml_rust::{YamlLoader, Yaml};

static EN:&'static str = include_str!("../../lang/en.yaml");
static FR:&'static str = include_str!("../../lang/fr.yaml");

/// Localize a string in a given language
pub fn localize(lang: &str, s: &str) -> String {
    let docs = match lang {
        "fr" => YamlLoader::load_from_str(FR).unwrap(),
        _ => YamlLoader::load_from_str(EN).unwrap(),
    };
    let hash = docs[0].as_hash().unwrap();

    String::from(hash.get(&Yaml::from_str(s))
                 .unwrap()
                 .as_str()
                 .unwrap())
}
