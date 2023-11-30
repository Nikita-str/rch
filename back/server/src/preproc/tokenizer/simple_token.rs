use crate::preproc::span::Span;
use super::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleTokenType {
    Number,
    Text,
    Spaces,
    NewLine,
    SpecialChar,

    /// aka EOF (EOS acrually // end of stream)
    Empty,
    Unkn,
    // Err,
}

impl SimpleTokenType {
    pub fn is_single_char_seq(self) -> bool {
        match self {
            | Self::SpecialChar 
            | Self::NewLine
            => true,

            _ => false,
        }
    }

    pub const fn is_eof(self) -> bool {
        matches!(self, Self::Empty)
    }
    
    pub const fn is_unkn(self) -> bool {
        matches!(self, Self::Unkn)
    }
}

#[derive(Debug, Clone)]
pub struct SimpleToken<'s> {
    pub token: Token<'s>,
    pub ty: SimpleTokenType,
}
impl<'s> SimpleToken<'s> {
    pub fn token(&self) -> &str {
        self.token.token
    }

    pub fn span(&self) -> Span {
        self.token.span
    }

    pub fn is_eof(&self) -> bool {
        self.ty.is_eof()
    }

    pub const fn is_empty_std_predicate(&self) -> bool {
        self.ty.is_eof() || (self.ty.is_unkn() && self.token.token.is_empty())
    }
    pub const fn is_empty_never_predicate(&self) -> bool {
        false
    }
}
