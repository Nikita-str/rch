use crate::preproc::Span;
use crate::preproc::tokenizer::Token;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CharSeqType {
    Number,
    Text,
    Spaces,
    NewLine,
    SpecialChar,
}

impl CharSeqType {
    fn is_single_char_seq(self) -> bool {
        match self {
            | Self::SpecialChar 
            | Self::NewLine
            => true,

            _ => false,
        }
    }
}

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

    pub fn next_token(&mut self) -> Token<'s> {
        let token_stream = &self.token_stream[self.byte_pos..];
        let mut seq_type: Option<CharSeqType> = None;
        let mut span = Span::new_empty(self.byte_pos);
        for c in token_stream.chars() {
            let c_type = match c {
                '\n' => CharSeqType::NewLine,
                '0'..='9' => CharSeqType::Number,
                c if c.is_ascii_punctuation() => CharSeqType::SpecialChar,
                c if c.is_whitespace() => CharSeqType::Spaces,
                _ => CharSeqType::Text,
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

        return Token {
            token: span.extract_str(self.token_stream),
            span,
        }
    }

    pub fn is_ended(&self) -> bool {
        self.token_stream.len() == self.byte_pos
    }
}
