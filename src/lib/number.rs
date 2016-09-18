/// Numbering for a given chapter
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    /// Chapter's title is hidden
    Hidden,
    /// Chapter is not numbered
    Unnumbered,
    /// Chapter follows books numbering, number is given automatically
    Default,
    /// Chapter number set to specified number
    Specified(i32), 
}


impl Number {
    /// Returns true if self is hidden
    pub fn is_hidden(&self) -> bool {
        *self == Number::Hidden
    }

    /// Returns true if self is numbered
    pub fn is_numbered(&self) -> bool {
        match *self {
            Number::Hidden | Number::Unnumbered => false,
            _ => true,
        }
    }
}
