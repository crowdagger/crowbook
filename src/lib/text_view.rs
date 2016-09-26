// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use std::mem;

use token::Token;

/// Returns the content of an AST as raw text, without any formatting
/// Useful for tools like grammar checks
pub fn view_as_text(tokens: &[Token]) -> String {
    let mut output = String::new();

    for token in tokens {
        match token {
            &Token::Str(ref s) => output.push_str(s),
            &Token::Rule
                | &Token::SoftBreak
                | &Token::HardBreak
                => output.push('\n'),

            _ => output.push_str(&view_as_text(token.inner().unwrap()))
        }
    }
    output
}

/// Insert a comment at the given pos in the text_view
pub fn insert_at(tokens: &mut Vec<Token>, comment: &str, pos: usize) -> Option<usize> {
    let mut pos = pos;
    let mut found = None;
    for i in 0..tokens.len() {
        let recurse  = match tokens[i] {
            Token::Str(ref s) => {
                let len = s.chars().count();
                if pos < len {
                    found = Some(i);
                    break;
                } else {
                    pos = pos - len;
                    false
                }
            },
            
            Token::Rule
                | Token::SoftBreak
                | Token::HardBreak
                => {
                    if pos < 1 {
                        found = Some(i);
                        break;
                    } else {
                        pos -= 1;
                        false
                    }
                }
            Token::Comment(_) => {
                false
            },
                
            _ => true
        };

        // Moved out of the match 'thanks' to borrowcheck
        if recurse {
            if let Some(ref mut inner) = tokens[i].inner_mut() {
                if let Some(new_pos) = insert_at(inner, comment, pos) {
                    pos = new_pos;
                } else {
                    return None;
                }
            }
        }
    }

    let new_token = Token::Comment(comment.to_owned());
    if let Some(i) = found {
        if !tokens[i].is_str() {
            if i >= tokens.len() - 1 {
                tokens.push(new_token);
            } else {
                tokens.insert(i+1, new_token);
            }
        } else {
            let old_token = mem::replace(&mut tokens[i], Token::Str(String::new()));
            if let Token::Str(old_str) = old_token {
                let mut chars_left:Vec<char> = old_str.chars().collect();
                let chars_right = chars_left.split_off(pos);
                let str_left:String = chars_left.into_iter().collect();
                let str_right:String = chars_right.into_iter().collect();
                tokens[i] = Token::Str(str_left);
                if i >= tokens.len() - 1 {
                    tokens.push(new_token);
                    tokens.push(Token::Str(str_right));
                } else {
                    tokens.insert(i+1, new_token);
                    tokens.insert(i+2, Token::Str(str_right));
                }   
            }
        }
        return None;
    } else {
        return Some(pos);
    }

}


#[test]
fn test_text_view() {
    let ast = vec!(Token::Str("123".to_owned()),
                   Token::Emphasis(vec!(Token::Str("456".to_owned()))),
                   Token::Str("789".to_owned()));
    assert_eq!(view_as_text(&ast), "123456789");
}

#[test]
fn test_text_insert() {
    let mut ast = vec!(Token::Str("123".to_owned()),
                       Token::Emphasis(vec!(Token::Str("456".to_owned()))),
                       Token::Str("789".to_owned()));
    insert_at(&mut ast, "!!!", 5);
    let expected = vec!(Token::Str("123".to_owned()),
                        Token::Emphasis(vec!(Token::Str("45".to_owned()),
                                             Token::Comment("!!!".to_owned()),
                                             Token::Str("6".to_owned()))),
                       Token::Str("789".to_owned()));
    assert_eq!(expected, ast);
}
