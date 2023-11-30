use super::{SimpleTokenizer, multi_token::MultiToken, Token};


pub struct MultiTokenTokenizer<'s, 'x, const MAX_NON_EMTY_TOKENS: usize> {
    tokenizer: SimpleTokenizer<'s>,
    multi_token: MultiToken<'s>,
    empty_token_fn: dyn Fn(&'x Token<'s>) -> bool,
}

// impl<'s, const MAX_NON_EMTY_TOKENS: usize> MultiTokenTokenizer<'s> {

// }