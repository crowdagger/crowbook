use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Str(Cow<'a, str>), 
    Paragraph(Vec<Token<'a>>), 
    Header(i32, Vec<Token<'a>>), //title level, list of tokens
    Emphasis(Vec<Token<'a>>),
    Strong(Vec<Token<'a>>),
    Code(Vec<Token<'a>>),
    BlockQuote(Vec<Token<'a>>),
    CodeBlock(Cow<'a, str>, Vec<Token<'a>>), //language, content of the block

    List(Vec<Token<'a>>),
    OrderedList(usize, Vec<Token<'a>>), //starting number, list
    Item(Vec<Token<'a>>),
    
    Rule,
    SoftBreak,
    HardBreak,
    
    Link(Cow<'a, str>, Cow<'a, str>, Vec<Token<'a>>), //url, title, list
    Image(Cow<'a, str>, Cow<'a, str>, Vec<Token<'a>>), //url, title, alt text
}
