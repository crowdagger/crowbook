use rustc_serialize::json;
use hyper;
use hyper::{Client};
use url::form_urlencoded;
use crossbeam;

use std::io::Read;

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
    pub short_message: Option<String>,
    pub issue_type: Option<String>,
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

/// GrammarChecker
pub struct GrammarChecker {
    lang: String,
    port: usize,
}

impl GrammarChecker {
    /// Initialize the grammarchecker
    pub fn new<S:Into<String>>(port: usize, lang: S) -> Result<GrammarChecker> {
        let checker = GrammarChecker {
            lang: lang.into(),
            port: port
        };

        let res = try!(Client::new()
                           .get(&format!("http://localhost:{}/v2/languages", port))
                           .send()
                           .map_err(|e| Error::grammar_check(Source::empty(),
                                                             lformat!("could not connect to language tool server: {error}",
                                                                      error = e))));
        if res.status != hyper::Ok {
            return Err(Error::grammar_check(Source::empty(),
                                            lformat!("server didn't respond with a OK status code")));
        }
        Ok(checker)
    }
    
    /// Send a query to LanguageTools server and get back a list of errors
    pub fn check(&self, text: &str) -> Result<GrammarCheck> {
        let query: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("language", &self.lang)
            .append_pair("text", text)
            .finish();
        
        let client = Client::new();
        
        let mut res = try!(client.post(&format!("http://localhost:{}/v2/check", self.port))
                           .body(&query)
                           .send()
                           .map_err(|e| Error::grammar_check(Source::empty(),
                                                             lformat!("could not send request to server: {error}",
                                                                      error = e))));

        if res.status != hyper::Ok {
            return Err(Error::grammar_check(Source::empty(),
                                      lformat!("server didn't respond with a OK status code")));
        }
        
        let mut s = String::new();
        try!(res.read_to_string(&mut s)
             .map_err(|e| Error::grammar_check(Source::empty(),
                                               lformat!("could not read response: {error}",
                                                        error = e))));
        let reponse: GrammarCheck = try!(json::decode(&s).map_err(|e| Error::default(Source::empty(),
                                                                                     lformat!("could not decode JSON: {error}",
                                                                                              error = e))));
        Ok(reponse)
    }
 
}


impl GrammarChecker {
    /// Check the grammar in a vector of tokens. 
    ///
    /// This modifies the AST
    pub fn check_chapter(&self, tokens: &mut Vec<Token>) -> Result<()> {
        let n_threads = 4; // seems to give best results but quite random
        let len = tokens.len();
        let subvecs = tokens.chunks_mut(len / n_threads);

        let mut result = Ok(());
        crossbeam::scope(|scope| {
            let mut handles = vec!();
            for subvec in subvecs {
                for mut token in subvec.iter_mut() {
                    match *token {
                        Token::Paragraph(ref mut v)
                            | Token::Header(_, ref mut v)
                            | Token::BlockQuote(ref mut v) 
                            | Token::List(ref mut v)
                            | Token::OrderedList(_, ref mut v)
                            => handles.push(scope.spawn(move || {
                                let check = try!(self.check(&view_as_text(v)));
                                for error in check.matches {
                                    insert_annotation(v, &Data::GrammarError(error.message.clone()), error.offset, error.length);
                                }
                                Ok(())
                            })),
                        _ => (),
                    }
                }
            }
            for handle in handles.into_iter() {
                if let Err(err) = handle.join() {
                    result = Err(err);
                    break;
                }
            }
        });
        result
        
        // crossbeam::scope(|scope| {
        //     for subvec in subvecs {
        //         scope.spawn(|| Self::check_grammar(lang, subvec)
        //     }
        // });
        
//        Ok(())
    }
}


