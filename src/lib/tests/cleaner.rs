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
