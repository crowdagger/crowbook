use token::Token;
use std::borrow::Cow;

fn parse_token(token: Token) -> String {
    match token {
        Token::Str(text) => text,
        Token::Paragraph(vec) => {
            let mut s = ast_to_md(vec);
            s.push_str("\n\n");
            s
        },
        Token::Header(n, vec) => {
            let s = ast_to_md(vec);
            let mut hashes = String::new();
            if n > 0 && n < 6 {
                for _ in 0..n {
                    hashes.push('#');
                }
            } else {
                panic!("Error: wrong title level");
            }
            format!("{} {} {}\n", hashes, s, hashes)
        },
        Token::Emphasis(vec) => format!("*{}*", ast_to_md(vec)),
        Token::Strong(vec) => format!("**{}**", ast_to_md(vec)),
        Token::Code(vec) => format!("`{}`", ast_to_md(vec)),
        Token::BlockQuote(vec) => format!("> {}", ast_to_md(vec)),
        Token::CodeBlock(language, vec) => format!("```{}\n{}\n```\n", language, ast_to_md(vec)),
        Token::Rule => String::from("***"),
        Token::SoftBreak => String::from(" "),
        Token::HardBreak => String::from("\n"),
        _ => String::from("???")
    }
}


pub fn ast_to_md(tokens: Vec<Token>) -> String {
    let mut res = String::new();

    for token in tokens {
        res.push_str(&parse_token(token));
    }
    res
}
