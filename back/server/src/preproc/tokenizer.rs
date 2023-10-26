use crate::preproc::Span;

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

#[derive(Debug)]
pub struct Token<'s> {
    pub token: &'s str,
    pub span: Span,
}

impl<'s, 'x> PartialEq<((usize, usize), &'x str)> for Token<'s> {
    fn eq(&self, other: &((usize, usize), &'x str)) -> bool {
        self.token == other.1 && self.span == other.0
    }
}

pub struct Tokenizer<'s> {
    token_stream: &'s str,
    byte_pos: usize,
}

impl<'s> Tokenizer<'s> {
    fn new(token_stream: &'s str) -> Self {
        Self {
            token_stream,
            byte_pos: 0,
        }
    }

    fn next_token(&mut self) -> Token<'s> {
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

    fn is_ended(&self) -> bool {
        self.token_stream.len() == self.byte_pos
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper(s: &str, expected_tokens: &[(usize, &str)]) {
        let mut tokenizer = Tokenizer::new(s);

        let mut prev_pos = 0;
        for x in expected_tokens {
            let token = tokenizer.next_token();
            assert_eq!(token, ((prev_pos, x.0), x.1));
            if prev_pos == x.0 {
                assert!(tokenizer.is_ended());
            }
            prev_pos = x.0;
        }
        assert!(tokenizer.is_ended());
    }

    #[test]
    fn test_tokenizer_01() {
        let s = "today  I'll finally find [xY]xYxo--tag[/xY]";

        let expected_tokens = [
            (5_, "today"),
            (7_, "  "),
            (8_, "I"),
            (9_, "'"),
            (11, "ll"),
            (12, " "),
            (19, "finally"),
            (20, " "),
            (24, "find"),
            (25, " "),
            (26, "["),
            (28, "xY"),
            (29, "]"),
            (33, "xYxo"),
            (34, "-"),
            (35, "-"),
            (38, "tag"),
            (39, "["),
            (40, "/"),
            (42, "xY"),
            (43, "]"),
            (43, ""),
            (43, ""),
        ];

        test_helper(s, &expected_tokens);
    }

    #[test]
    fn test_tokenizer_02() {
        let s = "aaa\n000bbb111c+c\n\ncc";

        let expected_tokens = [
            (3_, "aaa"),
            (4_, "\n"),
            (7_, "000"),
            (10, "bbb"),
            (13, "111"),
            (14, "c"),
            (15, "+"),
            (16, "c"),
            (17, "\n"),
            (18, "\n"),
            (20, "cc"),
        ];

        test_helper(s, &expected_tokens);
    }

    #[test]
    fn test_tokenizer_03() {
        let s = "а сегодня я воздушных шариков купил";
        let expected_words = s.split(' ');
        let mut tokenizer = Tokenizer::new(s);
        for word in expected_words {
            let mut token = tokenizer.next_token();
            if token.token == " " {
                token = tokenizer.next_token()
            }
            assert_eq!(token.token, word)
        }
        assert!(tokenizer.is_ended());
    }
}