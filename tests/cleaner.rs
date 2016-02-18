extern crate crowbook;

use self::crowbook::cleaner::{French, Cleaner};
use std::borrow::Cow;

#[test]
fn default() {
    let mut res = Cow::Borrowed("   Remove    supplementary   spaces    but    don't    trim     either   ");
    ().clean(&mut res);
    assert_eq!(&res, " Remove supplementary spaces but don't trim either ");
}


#[test]
fn french() {
    let mut res = Cow::Borrowed("  «  Comment allez-vous ? » demanda-t-elle à son   interlocutrice  qui lui répondit  : « Mais très bien ma chère  !  »");
    let french = French::new('~');
    french.clean(&mut res);
    println!("{}", &res);
    assert_eq!(&res, " «~Comment allez-vous~?~» demanda-t-elle à son interlocutrice qui lui répondit~: «~Mais très bien ma chère~!~»");
}
