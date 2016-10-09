extern crate crowbook_localize;
use crowbook_localize::{Localizer, Extractor};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lang/fr.po");
    let mut localizer = Localizer::new();
    localizer.add_lang("fr", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/fr.po"))).unwrap();
    localizer.write_macro_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib/localize_macros.rs")).unwrap();

    let mut extractor = Extractor::new();
    extractor.add_messages_from_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap();
    extractor.write_pot_file(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/crowbook.pot")).unwrap();
}
