use token::Token;
use std::borrow::Cow;

fn parse_token<'a>(token: Token<'a>) -> Cow<'a, str> {
    match token {
        Token::Str(text) => text,
        Token::Paragraph(vec) => {
            let mut s = ast_to_md(vec);
            s.push_str("\n\n");
            Cow::Owned(s)
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
            Cow::Owned(format!("{} {} {}\n", hashes, s, hashes))
        },
        Token::Emphasis(vec) => Cow::Owned(format!("*{}*", ast_to_md(vec))),
        Token::Strong(vec) => Cow::Owned(format!("**{}**", ast_to_md(vec))),
        Token::Code(vec) => Cow::Owned(format!("`{}`", ast_to_md(vec))),
        Token::BlockQuote(vec) => Cow::Owned(format!("> {}", ast_to_md(vec))),
        Token::CodeBlock(language, vec) => Cow::Owned(format!("```{}\n{}\n```\n", language, ast_to_md(vec))),
        Token::Rule => Cow::Borrowed("***"),
        Token::SoftBreak => Cow::Borrowed(" "),
        Token::HardBreak => Cow::Borrowed("\n"),
        _ => Cow::Borrowed("???")
    }
}


pub fn ast_to_md(tokens: Vec<Token>) -> String {
    let mut res = String::new();

    for token in tokens {
        res.push_str(&parse_token(token));
    }
    res
}
