use crate::preproc::span::Span;
use super::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleTokenType {
    Number,
    Text,
    Spaces,
    NewLine,
    SpecialChar,
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
}

#[derive(Debug, Clone)]
pub struct SimpleToken<'s> {
    pub token: Token<'s>,
    pub ty: Option<SimpleTokenType>,
}
impl<'s> SimpleToken<'s> {
    pub fn span(&self) -> Span {
        self.token.span
    }
}
