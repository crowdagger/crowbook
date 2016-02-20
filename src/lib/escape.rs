/// Escape characters <, >, and &
pub fn escape_html(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c),
        }
    }

    output
}

/// Escape characters for tex file
pub fn escape_tex(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            '&' => output.push_str(r"\&"),
            '%' => output.push_str(r"\%"),
            '$' => output.push_str(r"\$"),
            '#' => output.push_str(r"\#"),
            '_' => output.push_str(r"\_"),
            '{' => output.push_str(r"\{"),
            '}' => output.push_str(r"\}"),
            '~' => output.push_str(r"\textasciitilde"),
            '^' => output.push_str(r"\textasciicircum"),
            '\\' => output.push_str(r"\textbackslash"),
            _  => output.push(c)
        }
    }

    output
}
