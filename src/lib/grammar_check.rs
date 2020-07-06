// Copyright (C) 2016, 2017, 2018, 2019 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Crowbook is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use serde_json;
use reqwest;
use url::form_urlencoded;
use rayon::prelude::*;

use std::io::Read;

use crate::text_view::view_as_text;
use crate::text_view::insert_annotation;
use crate::token::Token;
use crate::token::Data;
use crate::error::{Error, Result, Source};

/// Represents a grammar error from language tool
///
/// Note: lots of fields are missing
#[derive(Deserialize, Debug)]
struct GrammarError {
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
#[derive(Deserialize, Debug)]
struct GrammarCheck {
    pub matches: Vec<GrammarError>,
}

/// GrammarChecker
pub struct GrammarChecker {
    lang: String,
    port: usize,
    client: reqwest::blocking::Client,
}

impl GrammarChecker {
    /// Initialize the grammarchecker
    pub fn new<S: Into<String>>(port: usize, lang: S) -> Result<GrammarChecker> {
        let checker = GrammarChecker {
            lang: lang.into(),
            port: port,
            client: reqwest::blocking::Client::new(),
        };

        let res = checker.client
            .get(&format!("http://localhost:{}/v2/languages", port))
            .send()
            .map_err(|e| {
                Error::grammar_check(Source::empty(),
                                     lformat!("could not connect to language tool server: {error}",
                                              error = e))
            })?;
        if !res.status().is_success() {
            return Err(Error::grammar_check(Source::empty(),
                                            lformat!("server didn't respond with a OK status \
                                                      code")));
        }
        Ok(checker)
    }

    /// Send a query to LanguageTools server and get back a list of errors
    fn check(&self, text: &str) -> Result<GrammarCheck> {
        let query: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("language", &self.lang)
            .append_pair("text", text)
            .finish();

        let mut res = self.client.post(&format!("http://localhost:{}/v2/check", self.port))
            .body(query)
            .send()
            .map_err(|e| {
                Error::grammar_check(Source::empty(),
                                     lformat!("could not send request to server: {error}",
                                              error = e))
            })?;

        if !res.status().is_success() {
            return Err(Error::grammar_check(Source::empty(),
                                            lformat!("server didn't respond with a OK status \
                                                      code")));
        }

        let mut s = String::new();
        res.read_to_string(&mut s)
            .map_err(|e| {
                Error::grammar_check(Source::empty(),
                                     lformat!("could not read response: {error}", error = e))
            })?;
        let reponse: GrammarCheck = serde_json::from_str(&s)
            .map_err(|e| {
                Error::default(Source::empty(),
                               lformat!("could not decode JSON: {error}", error = e))
            })?;
        Ok(reponse)
    }
}


impl GrammarChecker {
    /// Check the grammar in a vector of tokens.
    ///
    /// This modifies the AST
    pub fn check_chapter(&self, tokens: &mut Vec<Token>) -> Result<()> {
        let res = tokens.par_iter_mut()
            .map(|token| {
                match *token {
                    Token::Paragraph(ref mut v) |
                    Token::Header(_, ref mut v) |
                    Token::BlockQuote(ref mut v) |
                    Token::List(ref mut v) |
                    Token::OrderedList(_, ref mut v) => {
                        let check = self.check(&view_as_text(v))?;
                        for error in check.matches {
                            insert_annotation(v,
                                              &Data::GrammarError(error.message.clone()),
                                              error.offset,
                                              error.length);
                        }
                        Ok(())
                    },

                    _ => Ok(()),
                }
            })
            .find_any(|r| r.is_err());
        if let Some(err) = res {
            err
        } else {
            Ok(())
        }
    }
}
