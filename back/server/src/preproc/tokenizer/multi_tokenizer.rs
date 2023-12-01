use super::{SimpleTokenizer, MultiToken, SimpleToken};


pub struct MultiTokenTokenizer<'s, IsEmpty: Fn(&SimpleToken<'s>) -> bool, const MAX_NON_EMTY_TOKENS: usize> {
    tokenizer: SimpleTokenizer<'s>,
    multi_token: MultiToken<'s>,
    is_empty_token_predicate: IsEmpty,
    cur_non_empty: usize,
    is_ended: bool,
}

impl<'s, const MAX_NON_EMTY_TOKENS: usize> MultiTokenTokenizer<'s, fn(&SimpleToken<'s>) -> bool, MAX_NON_EMTY_TOKENS> {
    pub fn new_std(token_stream: &'s str) -> Self {
        Self::new(token_stream, SimpleToken::is_empty_std_predicate)
    }
}

impl<'s, IsEmpty: Fn(&SimpleToken<'s>) -> bool, const MAX_NON_EMTY_TOKENS: usize> MultiTokenTokenizer<'s, IsEmpty, MAX_NON_EMTY_TOKENS> {
    pub fn new(token_stream: &'s str, is_empty_token_predicate: IsEmpty) -> Self {
        Self {
            tokenizer: SimpleTokenizer::new(token_stream),
            multi_token: MultiToken::new_empty(),
            is_empty_token_predicate,
            cur_non_empty: 0,
            is_ended: false,
        }
    }

    pub fn next_token(&mut self) -> &MultiToken<'s> {
        if !self.is_ended {
            loop {
                if self.cur_non_empty == MAX_NON_EMTY_TOKENS { break }
                let token = self.tokenizer.next_token();
                if token.is_eof() {
                    self.is_ended = true;
                    break;
                }
                if !(self.is_empty_token_predicate)(&token) {
                    self.cur_non_empty += 1;
                }
                self.multi_token.add_token(token)
            }
        }
        &self.multi_token
    }

    pub fn use_n(&mut self, n: usize) {
        // self.multi_token.use_n(n);
        for _ in 0..n {
            if let Some(token) = self.multi_token.remove_first() {
                if !(self.is_empty_token_predicate)(&token) {
                    self.cur_non_empty -= 1;
                }
            } else {
                break
            }
        }
    }
    
    pub fn is_ended(&self) -> bool {
        self.tokenizer.is_ended() && self.multi_token.is_empty()
    }
}