use rustc_serialize::json;
use hyper;
use hyper::{Client};
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::USERINFO_ENCODE_SET;

use std::io::Read;
use std::borrow::Cow;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct GrammarError {
    pub message: String,
    pub offset: usize,
    pub length: usize,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Check {
    pub matches: Vec<GrammarError>,
}

impl Check {
    fn error_at(&self, n: usize) -> Option<&GrammarError> {
        for e in &self.matches {
            if e.offset == n {
                return Some(e)
            }
        }
        None
    }
}

fn escape_query<'a>(s: &str) -> Cow<'a, str> {
    let mut res = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => res.push_str("%26"),
            _ => res.push(c),
        }
    }
    Cow::Owned(res)
}

/// Send a query to LanguageTools server
pub fn get_errors(text: &str) -> Check {
    let query = format!("language=fr&text={}", escape_query(text));
    println!("{}", query);
    
    let client = Client::new();

    let mut res = client.post("http://localhost:8081/v2/check").body(&query).send().unwrap();

    let mut s = String::new();
    assert_eq!(res.status, hyper::Ok);
    res.read_to_string(&mut s).unwrap();
    let reponse: Check = json::decode(&s).unwrap();
    println!("{:?}", reponse);
    return reponse;
}


/// Check grammar errors
pub fn grammar_check<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    let check = get_errors(input.as_ref());
    if check.matches.is_empty() {
        return input;
    }

    let chars = input.chars().collect::<Vec<_>>();
    let mut res = String::with_capacity(chars.len());
    let mut i = 0;
    while i < chars.len() {
        if let Some(error) = check.error_at(i) {
            let end = i + error.length;
            let mut s = String::with_capacity(error.length);
            for x in 0.. error.length {
                s.push(chars[i+x])
            }
            res.push_str(&format!("<span title = \"{}\" style = \"background: red\">{}</span>",
                         error.message,
                         s));
            i += error.length;
            continue;
        } else {
            res.push(chars[i]);
        }
        i += 1;
    }
    Cow::Owned(res)
}
