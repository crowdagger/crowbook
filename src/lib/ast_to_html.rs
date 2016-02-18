use std::borrow::Cow;
use escape::escape_html;
use token::Token;

fn parse_token<'a>(token: Token<'a>) -> Cow<'a, str> {
    match token {
        Token::Str(text) => Cow::Owned(escape_html(&*text)),
        Token::Paragraph(vec) => Cow::Owned(format!("<p>{}</p>\n", ast_to_html(vec))),
        Token::Header(n, vec) => Cow::Owned(format!("<h{}>{}</h{}>\n", n, ast_to_html(vec), n)),
        Token::Emphasis(vec) => Cow::Owned(format!("<em>{}</em>", ast_to_html(vec))),
        Token::Strong(vec) => Cow::Owned(format!("<b>{}</b>", ast_to_html(vec))),
        Token::Code(vec) => Cow::Owned(format!("<code>{}</code>", ast_to_html(vec))),
        Token::BlockQuote(vec) => Cow::Owned(format!("<blockquote>{}</blockquote>\n", ast_to_html(vec))),
        Token::CodeBlock(language, vec) => {
            let s = ast_to_html(vec);
            if language.is_empty() {
                Cow::Owned(format!("<pre><code>\n{}</code></pre>\n", s))
            } else {
                Cow::Owned(format!("<pre><code class = \"language-{}\">{}</code></pre>\n", language, s))
            }
        },
        Token::Rule => Cow::Borrowed("<p class = \"rule\">***</p>\n"),
        Token::SoftBreak => Cow::Borrowed(" "),
        Token::HardBreak => Cow::Borrowed("<br />\n"),
        Token::List(vec) => Cow::Owned(format!("<ul>\n{}</ul>\n", ast_to_html(vec))),
        Token::OrderedList(n, vec) => Cow::Owned(format!("<ol start = \"{}\">\n{}</ol>\n", n, ast_to_html(vec))),
        Token::Item(vec) => Cow::Owned(format!("<li>{}</li>\n", ast_to_html(vec))),
        Token::Link(url, title, vec) => Cow::Owned(format!("<a href = \"{}\"{}>{}</a>",
                                                           url,
                                                           if title.is_empty() {
                                                               String::new()
                                                           } else {
                                                               format!(" title = \"{}\"", title)
                                                           },
                                                           ast_to_html(vec))),
                                            Token::Image(url, title, alt) => Cow::Owned(format!("<img src = \"{}\" title = \"{}\" alt = \"{}\" />",
                                                                                                url,
                                                                                                title,
                                                                                                ast_to_html(alt)))
                                            
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
