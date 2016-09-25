use rustc_serialize::json;
use hyper;
use hyper::{Client};
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::USERINFO_ENCODE_SET;

use std::io::Read;
use std::borrow::Cow;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Match {
    pub message: String,
    pub offset: u32,
    pub length: u32,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Reponse {
    pub matches: Vec<Match>,
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
pub fn send_query(text: &str) -> Reponse {
    let query = format!("language=fr&text={}", escape_query(text));
    println!("{}", query);
    
    let client = Client::new();

    let mut res = client.post("http://localhost:8081/v2/check").body(&query).send().unwrap();

    let mut s = String::new();
    assert_eq!(res.status, hyper::Ok);
    res.read_to_string(&mut s).unwrap();
    let reponse: Reponse = json::decode(&s).unwrap();
    println!("{:?}", reponse);
    return reponse;
}
