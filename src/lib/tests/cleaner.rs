use cleaner::{French, Cleaner, Default};
use super::test_eq;
use std::borrow::Cow;

#[test]
fn cleaner_default() {
    let s = Cow::Borrowed("   Remove    supplementary   spaces    but    don't    trim     either   ");
    let res = Default.clean(s, false);
    test_eq(&res, " Remove supplementary spaces but don't trim either ");
}


#[test]
fn cleaner_french() {
    let s = Cow::Borrowed("  «  Comment allez-vous ? » demanda-t-elle à son   interlocutrice  qui lui répondit  : « Mais très bien ma chère  !  »");
    let res = French.clean(s, false);
    test_eq(&res, " « Comment allez-vous ? » demanda-t-elle à son interlocutrice qui lui répondit : « Mais très bien ma chère ! »");
}

#[test]
fn cleaner_french_dashes_1() {
    let s = Cow::Borrowed("Il faudrait gérer ces tirets – sans ça certains textes rendent mal – un jour ou l'autre");
    let res = French.clean(s, true);
    test_eq(&res, "Il faudrait gérer ces tirets –~sans ça certains textes rendent mal~– un jour ou l'autre");
}

#[test]
fn cleaner_french_dashes_2() {
    let s = Cow::Borrowed("Il faudrait gérer ces tirets – sans ça certains textes rendent mal. Mais ce n'est pas si simple – si ?");
    let res = French.clean(s, true);
    test_eq(&res, "Il faudrait gérer ces tirets –~sans ça certains textes rendent mal. Mais ce n'est pas si simple –~si~?");
}

#[test]
fn cleaner_french_numbers() {
    let cleaner = French;
    
    let s = Cow::Borrowed("10 000");
    let res = cleaner.clean(s, true);
    test_eq(&res, "10~000");

    let s = Cow::Borrowed("10 000 €");
    let res = cleaner.clean(s, true);
    test_eq(&res, "10~000~€");
    
    let s = Cow::Borrowed("10 000 euros");
    let res = cleaner.clean(s, true);
    test_eq(&res, "10~000 euros");

    let s = Cow::Borrowed("10 000 EUR");
    let res = cleaner.clean(s, true);
    test_eq(&res, "10~000~EUR");

    let s = Cow::Borrowed("50 %");
    let res = cleaner.clean(s, true);
    test_eq(&res, "50~%");

    let s = Cow::Borrowed("20 °C");
    let res = cleaner.clean(s, true);
    test_eq(&res, "20~°C");

    let s = Cow::Borrowed("20 F");
    let res = cleaner.clean(s, true);
    test_eq(&res, "20~F");

    let s = Cow::Borrowed("20 BALLES");
    let res = cleaner.clean(s, true);
    test_eq(&res, "20 BALLES");
}
