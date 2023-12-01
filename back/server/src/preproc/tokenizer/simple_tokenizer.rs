use crate::preproc::Span;
use crate::preproc::tokenizer::Token;
use crate::preproc::tokenizer::simple_token::{SimpleToken, SimpleTokenType};


pub struct SimpleTokenizer<'s> {
    token_stream: &'s str,
    byte_pos: usize,
}

impl<'s> SimpleTokenizer<'s> {
   pub fn new(token_stream: &'s str) -> Self {
        Self {
            token_stream,
            byte_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> SimpleToken<'s> {
        let token_stream = &self.token_stream[self.byte_pos..];
        let mut seq_type: Option<SimpleTokenType> = None;
        let mut span = Span::new_empty(self.byte_pos);
        for c in token_stream.chars() {
            let c_type = match c {
                '\n' => SimpleTokenType::NewLine,
                '0'..='9' => SimpleTokenType::Number,
                c if c.is_ascii_punctuation() => SimpleTokenType::SpecialChar,
                c if c.is_whitespace() => SimpleTokenType::Spaces,
                _ => SimpleTokenType::Text,
            };
            
            let finish;
            let upd;
            if let Some(seq_type) = seq_type {
                finish = seq_type != c_type;
                upd = !finish; 
            } else {
                seq_type = Some(c_type);
                finish = c_type.is_single_char_seq();
                upd = true;
            }

            if upd {
                let byte_len = c.len_utf8();
                span.expand_right(byte_len);
                self.byte_pos += byte_len;
            }
            if finish { break; }
        }

        let token = Token {
            token: span.extract_str(self.token_stream),
            span,
        };
        
        return SimpleToken {
            token,
            ty: seq_type.unwrap_or(SimpleTokenType::Empty),
        }
    }

    pub fn is_ended(&self) -> bool {
        self.token_stream.len() == self.byte_pos
    }
}
