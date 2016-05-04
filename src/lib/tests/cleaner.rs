use cleaner::{French, Cleaner, Default};
use super::test_eq;

#[test]
fn cleaner_default() {
    let mut res = String::from("   Remove    supplementary   spaces    but    don't    trim     either   ");
    Default.clean(&mut res, false);
    test_eq(&res, " Remove supplementary spaces but don't trim either ");
}


#[test]
fn cleaner_french() {
    let mut res = String::from("  «  Comment allez-vous ? » demanda-t-elle à son   interlocutrice  qui lui répondit  : « Mais très bien ma chère  !  »");
    let french = French;
    french.clean(&mut res, false);
    println!("{}", &res);
    test_eq(&res, " « Comment allez-vous ? » demanda-t-elle à son interlocutrice qui lui répondit : « Mais très bien ma chère ! »");
}
