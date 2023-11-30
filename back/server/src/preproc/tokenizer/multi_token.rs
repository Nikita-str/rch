use std::collections::VecDeque;
use crate::preproc::span::Span;
use super::SimpleToken;

#[derive(Debug)]
pub struct MultiToken<'s> {
    tokens: VecDeque<SimpleToken<'s>>,
}

impl<'s> MultiToken<'s> {
    pub fn new_single(token: SimpleToken<'s>) -> Self {
        let mut tokens = VecDeque::new();
        tokens.push_back(token);
        Self { tokens }
    }

    pub fn add_token(&mut self, token: SimpleToken<'s>) {
        self.tokens.push_back(token)
    }

    pub fn span(&self) -> Span {
        Span::new_union(
            self.tokens[0].span(),
            self.tokens[self.tokens.len() - 1].span(),
        )
    }

    pub fn first_token_ref(&self) -> &SimpleToken {
        self.tokens.front().unwrap()
    }
}


pub struct MultiTokenRef<'s, 'mt> {
    multi_token: &'mt MultiToken<'s>,
    to: usize,
}

impl<'s, 'mt> MultiTokenRef<'s, 'mt> {
    pub fn span(&self) -> Span {
        Span::new_union(
            self.multi_token.tokens[0].span(),
            self.multi_token.tokens[self.to].span(),
        )
    }
}