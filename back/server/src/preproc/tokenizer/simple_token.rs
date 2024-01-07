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
}

impl SimpleTokenType {
    pub const fn is_single_char_seq(self) -> bool {
        matches!(self, Self::SpecialChar | Self::NewLine)
    }

    pub const fn is_number(self) -> bool {
        matches!(self, Self::Number)
    }

    pub const fn is_spaces(self) -> bool {
        matches!(self, Self::Spaces)
    }

    pub const fn is_eof(self) -> bool {
        matches!(self, Self::Empty)
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
        self.ty.is_eof() || self.ty.is_spaces()
    }
    pub const fn is_empty_never_predicate(&self) -> bool {
        false
    }
}
