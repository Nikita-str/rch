mod token;
pub use token::Token;

mod simple_token;
mod simple_tokenizer;
pub use simple_token::SimpleToken;
pub use simple_tokenizer::SimpleTokenizer;

mod multi_token;
mod multi_tokenizer;
pub use multi_token::MultiToken;
pub use multi_tokenizer::MultiTokenTokenizer;

// mod generic_token;
// pub use generic_token::GenericToken;

#[cfg(test)]
mod tests;