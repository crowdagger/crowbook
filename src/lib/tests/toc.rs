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

#[test]
fn toc_epub_simple() {
    let mut toc = Toc::new();
    toc.add(1, String::from("#1"), "1".to_owned());
    toc.add(1, String::from("#2"), "2".to_owned());
    let actual = toc.render_epub(1);
    let expected = "
   <navPoint id = \"navPoint-1\">
     <navLabel>
       <text>1</text>
     </navLabel>
     <content src = \"#1\" />
    </navPoint>

   <navPoint id = \"navPoint-2\">
     <navLabel>
       <text>2</text>
     </navLabel>
     <content src = \"#2\" />
    </navPoint>
";
    test_eq(&actual, expected);
}

#[test]
fn toc_epub_simple_sublevels() {
    let mut toc = Toc::new();
    toc.add(1, String::from("#1"), "1".to_owned());
    toc.add(2, String::from("#1.1"), "1.1".to_owned());
    toc.add(1, String::from("#2"), "2".to_owned());
    toc.add(2, String::from("#2.1"), "2.1".to_owned());
    let actual = toc.render_epub(1);
    let expected = "
   <navPoint id = \"navPoint-1\">
     <navLabel>
       <text>1</text>
     </navLabel>
     <content src = \"#1\" />

   <navPoint id = \"navPoint-2\">
     <navLabel>
       <text>1.1</text>
     </navLabel>
     <content src = \"#1.1\" />
    </navPoint>
    </navPoint>

   <navPoint id = \"navPoint-3\">
     <navLabel>
       <text>2</text>
     </navLabel>
     <content src = \"#2\" />

   <navPoint id = \"navPoint-4\">
     <navLabel>
       <text>2.1</text>
     </navLabel>
     <content src = \"#2.1\" />
    </navPoint>
    </navPoint>
";
    test_eq(&actual, expected);
}


#[test]
fn toc_epub_broken_sublevels() {
    let mut toc = Toc::new();
    toc.add(2, String::from("#1.1"), "1.1".to_owned());
    toc.add(1, String::from("#2"), "2".to_owned());
    toc.add(2, String::from("#2.1"), "2.1".to_owned());
    let actual = toc.render_epub(1);
    let expected = "
   <navPoint id = \"navPoint-1\">
     <navLabel>
       <text>1.1</text>
     </navLabel>
     <content src = \"#1.1\" />
    </navPoint>

   <navPoint id = \"navPoint-2\">
     <navLabel>
       <text>2</text>
     </navLabel>
     <content src = \"#2\" />

   <navPoint id = \"navPoint-3\">
     <navLabel>
       <text>2.1</text>
     </navLabel>
     <content src = \"#2.1\" />
    </navPoint>
    </navPoint>
";
    test_eq(&actual, expected);
}
