use super::{Token, SimpleToken, MultiToken};


#[derive(Debug, Clone)]
pub enum GenericToken<'s> {
    Token(Token<'s>),
    SimpleToken(SimpleToken<'s>),
    MultiToken(MultiToken<'s>),
}