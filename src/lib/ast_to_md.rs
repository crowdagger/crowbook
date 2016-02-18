use token::Token;

fn parse_token(token: &Token) -> String {
    match *token {
        Token::Str(ref text) => text.clone(),
        Token::Paragraph(ref vec) => {
            let mut s = ast_to_md(vec);
            s.push_str("\n\n");
            s
        },
        Token::Header(n, ref vec) => {
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
        Token::Emphasis(ref vec) => format!("*{}*", ast_to_md(vec)),
        Token::Strong(ref vec) => format!("**{}**", ast_to_md(vec)),
        Token::Code(ref vec) => format!("`{}`", ast_to_md(vec)),
        Token::BlockQuote(ref vec) => format!("> {}", ast_to_md(vec)),
        Token::CodeBlock(ref language, ref vec) => format!("```{}\n{}\n```\n", language, ast_to_md(vec)),
        Token::Rule => String::from("***"),
        Token::SoftBreak => String::from(" "),
        Token::HardBreak => String::from("\n"),
        _ => String::from("???")
    }
}


pub fn ast_to_md(tokens: &[Token]) -> String {
    let mut res = String::new();

    for token in tokens {
        res.push_str(&parse_token(token));
    }
    res
}
