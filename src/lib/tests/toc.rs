use toc::Toc;
use super::test_eq;

#[test]
fn toc_simple() {
    let mut toc = Toc::new();
    toc.add(3, String::from("#1"), "0.0.1".to_owned());
    toc.add(1, String::from("#2"), "1".to_owned());
    toc.add(3, String::from("#3"), "1.0.1".to_owned());
    toc.add(2, String::from("#4"), "1.1".to_owned());
    toc.add(1, String::from("#5"), "2".to_owned());
    let actual = toc.render();
    let expected = "<ul>
 <ul>
  <ul>
   <li><a href = \"#1\">0.0.1</a>
   </li>
  </ul>
 </ul>
 <li><a href = \"#2\">1</a>
 <ul>
  <ul>
   <li><a href = \"#3\">1.0.1</a>
   </li>
  </ul>
  <li><a href = \"#4\">1.1</a>
  </li>
 </ul>
 </li>
 <li><a href = \"#5\">2</a>
 </li>
</ul>";
    test_eq(&actual, expected);
}
