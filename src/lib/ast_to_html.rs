use std::borrow::Cow;
use escape::escape_html;
use token::Token;

fn parse_token(token: Token) -> String {
    match token {
        Token::Str(text) => escape_html(&*text),
        Token::Paragraph(vec) => format!("<p>{}</p>\n", ast_to_html(vec)),
        Token::Header(n, vec) => format!("<h{}>{}</h{}>\n", n, ast_to_html(vec), n),
        Token::Emphasis(vec) => format!("<em>{}</em>", ast_to_html(vec)),
        Token::Strong(vec) => format!("<b>{}</b>", ast_to_html(vec)),
        Token::Code(vec) => format!("<code>{}</code>", ast_to_html(vec)),
        Token::BlockQuote(vec) => format!("<blockquote>{}</blockquote>\n", ast_to_html(vec)),
        Token::CodeBlock(language, vec) => {
            let s = ast_to_html(vec);
            if language.is_empty() {
                format!("<pre><code>\n{}</code></pre>\n", s)
            } else {
                format!("<pre><code class = \"language-{}\">{}</code></pre>\n", language, s)
            }
        },
        Token::Rule => String::from("<p class = \"rule\">***</p>\n"),
        Token::SoftBreak => String::from(" "),
        Token::HardBreak => String::from("<br />\n"),
        Token::List(vec) => format!("<ul>\n{}</ul>\n", ast_to_html(vec)),
        Token::OrderedList(n, vec) => format!("<ol start = \"{}\">\n{}</ol>\n", n, ast_to_html(vec)),
        Token::Item(vec) => format!("<li>{}</li>\n", ast_to_html(vec)),
        Token::Link(url, title, vec) => format!("<a href = \"{}\"{}>{}</a>",
                                                url,
                                                if title.is_empty() {
                                                    String::new()
                                                } else {
                                                    format!(" title = \"{}\"", title)
                                                },
                                                ast_to_html(vec)),
        Token::Image(url, title, alt) => format!("<img src = \"{}\" title = \"{}\" alt = \"{}\" />",
                                                  url,
                                                  title,
                                                  ast_to_html(alt))
            
    }
}

/// Transform a vector of `Token`s to HTML format.
pub fn ast_to_html(tokens: Vec<Token>) -> String {
    let mut res = String::new();

    for token in tokens {
        res.push_str(&parse_token(token));
    }
    res
}
