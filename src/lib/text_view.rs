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

use std::default::Default;

use crate::token::Token;

pub fn traverse_token<F1, F2, R>(token: &Token, f: &F1, add: &F2) -> R
where
    F1: Fn(&str) -> R,
    R: Default,
    F2: Fn(R, R) -> R,
{
    match *token {
        Token::Str(ref s) | Token::Code(ref s) | Token::CodeBlock(_, ref s) => f(s),

        Token::SoftBreak => f(" "),

        Token::Rule | Token::HardBreak => f("\n"),

        Token::Image(..)
        | Token::StandaloneImage(..)
        | Token::FootnoteDefinition(..)
        | Token::FootnoteReference(..)
        | Token::Table(..)
        | Token::TableHead(..)
        | Token::TableRow(..)
        | Token::TableCell(..) => f(""),

        _ => traverse_vec(token.inner().unwrap(), f, add),
    }
}

/// Traverse a vector of tokens
#[doc(hidden)]
pub fn traverse_vec<F1, F2, R>(tokens: &[Token], f: &F1, add: &F2) -> R
where
    F1: Fn(&str) -> R,
    F2: Fn(R, R) -> R,
    R: Default,
{
    tokens
        .iter()
        .map(|t| traverse_token(t, f, add))
        .fold(R::default(), add)
}

/// Returns the content of an AST as raw text, without any formatting
/// Useful for tools like grammar checks
pub fn view_as_text(tokens: &[Token]) -> String {
    traverse_vec(tokens, &|s| s.to_owned(), &|s1, s2| s1 + &s2)
}

// Should it be removed?
// pub fn count_length(tokens: &[Token]) -> usize {
//     traverse_vec(tokens, &|s| s.chars().count(), &|s1, s2| s1 + s2)
// }

/// Insert an annotation at begin and end pos begin+len in the text_view
// Should it be removed ?
// #[doc(hidden)]
// pub fn insert_annotation(
//     tokens: &mut Vec<Token>,
//     annotation: &Data,
//     pos: usize,
//     length: usize,
// ) -> Option<usize> {
//     let mut pos = pos;
//     let mut found_left = None;
//     let mut found_right = None;
//     for (i, item) in tokens.iter_mut().enumerate() {
//         let recurse = match item {
//             Token::Str(ref s) => {
//                 let len = s.chars().count();
//                 if pos < len || (pos == len && found_left.is_some()) {
//                     // We found the first element already, so now it's the right
//                     if found_left.is_some() {
//                         found_right = Some((i, pos));
//                         break;
//                     }
//                     found_left = Some((i, pos));
//                     pos += length;
//                     if pos <= len {
//                         found_right = Some((i, pos));
//                         break;
//                     }
//                 }
//                 pos -= len;
//                 false
//             }

//             Token::Rule | Token::SoftBreak | Token::HardBreak => {
//                 if pos < 1 {
//                     if found_left.is_some() {
//                         found_right = Some((i, pos));
//                         break;
//                     }
//                     found_left = Some((i, pos));
//                     pos += length;
//                     if pos <= 1 {
//                         found_right = Some((i, pos));
//                         break;
//                     }
//                 }
//                 pos -= 1;
//                 false
//             }

//             _ => {
//                 if let Some(inner) = item.inner() {
//                     let len = count_length(inner);
//                     // Only recurse if the two is in this subtree
//                     if pos < len {
//                         if found_left.is_none() {
//                             true
//                         } else {
//                             warn!(
//                                 "{}",
//                                 lformat!(
//                                     "ignored annotation {:?} as it \
//                                                   wasn't compatible with the \
//                                                   Markdown structure",
//                                     annotation
//                                 )
//                             );
//                             return None;
//                         }
//                     } else {
//                         pos -= len;
//                         false
//                     }
//                 } else {
//                     false
//                 }
//             }
//         };

//         // Moved out of the match 'thanks' to borrowcheck
//         if recurse {
//             if let Some(ref mut inner) = item.inner_mut() {
//                 if let Some(new_pos) = insert_annotation(inner, annotation, pos, length) {
//                     pos = new_pos;
//                 } else {
//                     return None;
//                 }
//             }
//         }
//     }

//     if let (Some((i, pos_left)), Some((j, pos_right))) = (found_left, found_right) {
//         let pos_right = pos_right;
//         let mut vec = vec![];

//         // Beginning token: keep the left part in the str, put the right one in our vec
//         if !tokens[i].is_str() || pos_left == 0 {
//             // do nothing
//         } else {
//             let old_token = mem::replace(&mut tokens[i], Token::Str(String::new()));
//             if let Token::Str(old_str) = old_token {
//                 let mut chars_left: Vec<char> = old_str.chars().collect();
//                 let mut chars_right = chars_left.split_off(pos_left);

//                 let str_left: String = chars_left.into_iter().collect();
//                 tokens[i] = Token::Str(str_left);

//                 if i == j {
//                     // i and j are in same str, so split again
//                     if length != chars_right.len() {
//                         let inline_token = chars_right.split_off(length);
//                         let inline_token = Token::Str(inline_token.into_iter().collect());
//                         if pos_left == 0 {
//                             tokens.insert(i, inline_token)
//                         } else if i == tokens.len() {
//                             tokens.push(inline_token);
//                         } else {
//                             tokens.insert(i + 1, inline_token);
//                         }
//                     }
//                     let annot = Token::Annotation(
//                         annotation.clone(),
//                         vec![Token::Str(chars_right.into_iter().collect())],
//                     );
//                     if pos_left == 0 {
//                         tokens.insert(i, annot)
//                     } else if i == tokens.len() {
//                         tokens.push(annot);
//                     } else {
//                         tokens.insert(i + 1, annot);
//                     }
//                     return None;
//                 }

//                 let str_right: String = chars_right.into_iter().collect();
//                 vec.push(Token::Str(str_right));
//             } else {
//                 unreachable!();
//             }
//         }

//         // Middle tokens: remove them entirely and put them in our vec
//         for _ in i + 1..j {
//             vec.push(tokens.remove(i + 1));
//         }

//         // End token: keep the right part in the str, put the left one in our vec
//         // j is now i + 1 because all tokens in between have been removed
//         // unless j was equal to i to begin with
//         let j = if i == j || i >= tokens.len() - 1 {
//             i
//         } else {
//             i + 1
//         };

//         if !tokens[j].is_str() {
//             // do nothing
//         } else {
//             let count = if let Token::Str(ref s) = tokens[j] {
//                 s.chars().count()
//             } else {
//                 unreachable!()
//             };
//             if count != pos_right {
//                 let old_token = mem::replace(&mut tokens[j], Token::Rule);
//                 if let Token::Str(old_str) = old_token {
//                     let mut chars_left: Vec<char> = old_str.chars().collect();
//                     let chars_right = chars_left.split_off(pos_right);
//                     let str_left: String = chars_left.into_iter().collect();
//                     let str_right: String = chars_right.into_iter().collect();
//                     tokens[j] = Token::Str(str_right);
//                     // todo: if only one token, maybe concat the strings
//                     vec.push(Token::Str(str_left));
//                 } else {
//                     unreachable!();
//                 }
//             }
//         }
//         let new_token = Token::Annotation(annotation.clone(), vec);
//         if pos_left == 0 {
//             tokens.insert(i, new_token);
//         } else if i >= tokens.len() - 1 {
//             tokens.push(new_token);
//         } else {
//             tokens.insert(i + 1, new_token);
//         }
//         None
//     } else if found_left.is_none() && found_right.is_none() {
//         Some(pos)
//     } else {
//         warn!(
//             "{}",
//             lformat!(
//                 "ignored annotation {:?} as it wasn't compatible \
//                                           with the Markdown structure",
//                 annotation
//             )
//         );
//         None
//     }
// }

#[test]
fn test_text_view() {
    let ast = vec![
        Token::Str("123".to_owned()),
        Token::Emphasis(vec![Token::Str("456".to_owned())]),
        Token::Str("789".to_owned()),
    ];
    assert_eq!(view_as_text(&ast), "123456789");
}

// #[test]
// fn test_text_insert() {
//     let mut ast = vec!(Token::Str("123".to_owned()),
//                        Token::Emphasis(vec!(Token::Str("456".to_owned()))),
//                        Token::Str("789".to_owned()));
//     insert_at(&mut ast, "!!!", 5);
//     let expected = vec!(Token::Str("123".to_owned()),
//                         Token::Emphasis(vec!(Token::Str("45".to_owned()),
//                                              Token::Comment("!!!".to_owned()),
//                                              Token::Str("6".to_owned()))),
//                         Token::Str("789".to_owned()));
//     assert_eq!(expected, ast);
// }
