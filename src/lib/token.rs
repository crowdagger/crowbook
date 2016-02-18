#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Str(String), 
    Paragraph(Vec<Token>), 
    Header(i32, Vec<Token>), //title level, list of tokens
    Emphasis(Vec<Token>),
    Strong(Vec<Token>),
    Code(Vec<Token>),
    BlockQuote(Vec<Token>),
    CodeBlock(String, Vec<Token>), //language, content of the block

    List(Vec<Token>),
    OrderedList(usize, Vec<Token>), //starting number, list
    Item(Vec<Token>),
    
    Rule,
    SoftBreak,
    HardBreak,
    
    Link(String, String, Vec<Token>), //url, title, list
    Image(String, String, Vec<Token>), //url, title, alt text
}
