use escape::escape_html;
use escape::escape_tex;
use super::test_eq;

#[test]
fn escape_html_simple() {
    let actual = escape_html("<foo> & <bar>");
    let expected = "&lt;foo&gt; &amp; &lt;bar&gt;";
    test_eq(&actual, expected);
}

#[test]
fn escape_tex_braces() {
    let actual = escape_tex(r"\foo{bar}");
    let expected = r"\textbackslash{}foo\{bar\}";
    test_eq(&actual, expected);
}

#[test]
fn escape_tex_dashes() {
    let actual = escape_tex("--foo, ---bar");
    let expected = r"-{}-foo, -{}-{}-bar";
    test_eq(&actual, expected);
}

#[test]
fn escape_tex_numbers() {
    let actual = escape_tex(r"30000$ is 10% of number #1 income");
    let expected = r"30000\$ is 10\% of number \#1 income";
    test_eq(&actual, expected);
}
