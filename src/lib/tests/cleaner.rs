use cleaner::{French, Cleaner};
use super::test_eq;

#[test]
fn cleaner_default() {
    let mut res = String::from("   Remove    supplementary   spaces    but    don't    trim     either   ");
    ().clean(&mut res);
    test_eq(&res, " Remove supplementary spaces but don't trim either ");
}


#[test]
fn cleaner_french() {
    let mut res = String::from("  «  Comment allez-vous ? » demanda-t-elle à son   interlocutrice  qui lui répondit  : « Mais très bien ma chère  !  »");
    let french = French::new('~');
    french.clean(&mut res);
    println!("{}", &res);
    test_eq(&res, " «~Comment allez-vous~?~» demanda-t-elle à son interlocutrice qui lui répondit~: «~Mais très bien ma chère~!~»");
}
