use crowbook_intl::{Extractor, Localizer};

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lang/fr.po");
    // Extract and localize src/lib
    let mut extractor = Extractor::new();
    extractor
        .add_messages_from_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib"))
        .unwrap();
    // Uncomment to update crowbook.pot
    //extractor.write_pot_file(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/lib/crowbook.pot")).unwrap();

    let mut localizer = Localizer::new(&extractor);
    localizer
        .add_lang(
            "fr",
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/lib/fr.po")),
        )
        .unwrap();
    let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("localize_macros.rs");
    localizer.write_macro_file(dest_path).unwrap();

    // Extract and localize src/bin
    let mut extractor = Extractor::new();
    extractor
        .add_messages_from_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin"))
        .unwrap();
    // Uncomment to update crowbook.pot
    //extractor.write_pot_file(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/bin/crowbook.pot")).unwrap();

    let mut localizer = Localizer::new(&extractor);
    localizer
        .add_lang(
            "fr",
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/bin/fr.po")),
        )
        .unwrap();
    let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("localize_macros_bin.rs");
    localizer.write_macro_file(dest_path).unwrap();
}
