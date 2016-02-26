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
