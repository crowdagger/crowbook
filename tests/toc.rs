extern crate crowbook;
use crowbook::Toc;

mod test_helper;
use test_helper::test_eq;

#[test]
fn toc_simple() {
    let mut toc = Toc::new();
    toc.add(3, String::new(), "0.0.1".to_owned());
    toc.add(1, String::new(), "1".to_owned());
    toc.add(3, String::new(), "1.0.1".to_owned());
    toc.add(2, String::new(), "1.1".to_owned());
    toc.add(1, String::new(), "2".to_owned());
    let actual = toc.render();
    let expected = "<ul>
 <ul>
  <ul>
   <li>0.0.1
   </li>
  </ul>
 </ul>
 <li>1
 <ul>
  <ul>
   <li>1.0.1
   </li>
  </ul>
  <li>1.1
  </li>
 </ul>
 </li>
 <li>2
 </li>
</ul>";
    test_eq(&actual, expected);
}
