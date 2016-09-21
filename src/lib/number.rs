/// Numbering for a given chapter or part
///
/// This Enum is only public so it can be passed to `Book` methods, but
/// library users should **not** do exhaustive matchs on the variants,
/// since it is possible new variants will be added without being
/// considered a breaking change
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

    /// Chapter is actually an unnumbered part
    #[doc(hidden)]
    UnnumberedPart,

    /// Chapter is actually a part following book numbering
    #[doc(hidden)]
    DefaultPart,
    
    /// Chapter is actually a part whose number is specified
    #[doc(hidden)]
    SpecifiedPart(i32)
}


impl Number {
    /// Returns true if self is a part
    pub fn is_part(&self) -> bool {
        match *self {
            Number::UnnumberedPart | Number::DefaultPart | Number::SpecifiedPart(..) => true,
            _ => false
        }
    }
    
    /// Returns true if self is hidden
    pub fn is_hidden(&self) -> bool {
        *self == Number::Hidden
    }

    /// Returns true if self is numbered
    pub fn is_numbered(&self) -> bool {
        match *self {
            Number::Hidden | Number::Unnumbered | Number::UnnumberedPart => false,
            _ => true,
        }
    }
}
