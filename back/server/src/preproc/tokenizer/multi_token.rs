use std::collections::VecDeque;
use crate::preproc::span::Span;
use super::SimpleToken;

#[derive(Debug)]
pub struct MultiToken<'s> {
    tokens: VecDeque<SimpleToken<'s>>,
}

const DFAULT_CAPACITY: usize = 16;

impl<'s> MultiToken<'s> {
    pub fn new_empty() -> Self {
        Self { tokens: VecDeque::with_capacity(DFAULT_CAPACITY) }
    }

    pub fn new_single(token: SimpleToken<'s>) -> Self {
        let mut ret = Self::new_empty();
        ret.add_token(token);
        ret
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

    pub fn span_n_tokens(&self, n_tokens: usize) -> Span {
        Span::new_union(
            self.tokens[0].span(),
            self.tokens[n_tokens - 1].span(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn first_token_ref(&self) -> &SimpleToken {
        self.tokens.front().unwrap()
    }

    pub fn use_n(&mut self, n: usize) {
        self.tokens.drain(0..n);
    }
    
    pub fn remove_first(&mut self) -> Option<SimpleToken<'s>> {
        self.tokens.pop_front()
    }
    
    pub fn get_simple_token(&self, index: usize) -> Option<&SimpleToken> {
        self.tokens.get(index)
    }
    
    pub fn test_token_seq_never_empty(&self, predicates: &[&dyn Fn(&SimpleToken<'s>) -> bool]) -> bool {
        self.test_token_seq(predicates, SimpleToken::is_empty_never_predicate)
    }

    pub fn test_token_seq<IsEmpty: Fn(&SimpleToken<'s>) -> bool>(&self, predicates: &[&dyn Fn(&SimpleToken<'s>) -> bool], is_empty: IsEmpty) -> bool {
        if self.tokens.len() < predicates.len() { return false }

        let mut index = 0;
        for predicate in predicates {
            loop {
                let Some(token) = self.tokens.get(index) else { return false };
                index += 1;
                if (is_empty)(token) { continue }
                if !(predicate)(token) { return false }
                break
            }
        }
        return  true
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