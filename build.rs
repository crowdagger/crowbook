extern crate crowbook_localize;
use crowbook_localize::Localizer;

fn main() {
    let mut localizer = Localizer::new();
    localizer.add_lang("fr", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/fr.mo"))).unwrap();
    localizer.write_macro_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib/localize_macros.rs")).unwrap();
}
