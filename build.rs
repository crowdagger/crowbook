extern crate crowbook_intl;
use crowbook_intl::{Localizer, Extractor};

use std::path::Path;
use std::env;


fn main() {
//    println!("cargo:rerun-if-changed=build.rs");
//    println!("cargo:rerun-if-changed=lang/fr.po");
    let mut extractor = Extractor::new();
    extractor.add_messages_from_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap();
    extractor.write_pot_file(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/crowbook.pot")).unwrap();
    
    let mut localizer = Localizer::new(&extractor);
    localizer.add_lang("fr", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/fr.po"))).unwrap();
    let out_dir_path = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_path);
    let dest_path = out_dir.join("localize_macros.rs");
    localizer.write_macro_file(dest_path.to_str().unwrap()).unwrap();
}
