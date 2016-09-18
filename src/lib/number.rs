/// Numbering for a given chapter
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    /// chapter's title is hidden
    Hidden,
    /// chapter is not numbered
    Unnumbered,
    /// chapter follows books numbering, number is given automatically
    Default,
    /// chapter number set to specified number
    Specified(i32), 
}


impl Number {
    /// Returns true if number is hidden
    pub fn is_hidden(&self) -> bool {
        *self == Number::Hidden
    }

    /// Returns true if number is numbered
    pub fn is_numbered(&self) -> bool {
        match *self {
            Hidden | Unnumbered => false,
            _ => true,
        }
    }
}
