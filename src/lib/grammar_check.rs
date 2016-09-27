use rustc_serialize::json;
use hyper;
use hyper::{Client};

use std::io::Read;
use std::borrow::Cow;

use text_view::view_as_text;
use text_view::insert_annotation;
use token::Token;
use token::Data;
use error::{Error, Result, Source};

/// Represents a grammar error from language tool
///
/// Note: lots of fields are missing 
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct GrammarError {
    pub message: String,
    pub offset: usize,
    pub length: usize,
}

/// Contains a list of matches to errors
///
/// Corresponds to the JSON that LanguageTool-server sends back
///
/// Note: lots of fields are missing
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct GrammarCheck {
    pub matches: Vec<GrammarError>,
}

impl GrammarCheck {
    /// Send a query to LanguageTools server and get back a list of errors
    pub fn new(text: &str, port: usize) -> Result<GrammarCheck> {
        let query = format!("language=en&text={}", escape_query(text));
        
        let client = Client::new();
        
        let mut res = try!(client.post(&format!("http://localhost:{}/v2/check", port))
                           .body(&query)
                           .send()
                           .map_err(|e| Error::default(Source::empty(),
                                                       format!("could not send request to server: {}", e))));

        if res.status != hyper::Ok {
            return Err(Error::default(Source::empty(),
                                      format!("server didn't respond with a OK status code")));
        }
        
        let mut s = String::new();
        try!(res.read_to_string(&mut s)
             .map_err(|e| Error::default(Source::empty(),
                                         format!("could not read response: {}", e))));
        let reponse: GrammarCheck = try!(json::decode(&s).map_err(|e| Error::default(Source::empty(),
                                                                                     format!("could not decode JSON: {}", e))));
        Ok(reponse)
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

/// Check the grammar in a chapter
///
/// This modifies the AST
pub fn check_grammar(tokens: &mut Vec<Token>) -> Result<()> {
    let input = view_as_text(tokens);
    let check = try!(GrammarCheck::new(&input, 8081));
    
    for error in check.matches {
        insert_annotation(tokens, &Data::GrammarError(error.message.clone()), error.offset, error.length);
    }
    Ok(())
}

