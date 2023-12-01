use crate::preproc::{Preproc, PreprocVerdict};

#[derive(Default)]
pub struct ReservedSymbsPreproc;

impl Preproc for ReservedSymbsPreproc {
    fn close(&mut self, _: &mut String, _: ()) {}
    fn reset(&mut self) { }

    fn action(&mut self, output: &mut String, matched_tokens: &str, _: ()) {
        match matched_tokens {
            "\"" => output.push_str("&#34;"), 
            "\'" => output.push_str("&#39;"), 
            "&" => output.push_str("&#38;"), 
            "<" => output.push_str("&#60;"), 
            ">" => output.push_str("&#62;"), 
            _ => println!("[ALGO WARN] unexpected token"),
        }
    }

    fn state_upd_str(&mut self, token: &str) -> PreprocVerdict {
        if token.len() != 1 {
            return PreprocVerdict::No;
        }

        let token = token.as_bytes()[0] as char;
        match token {
            '\'' | '"' | '>' | '<' | '&' => PreprocVerdict::Matched,
            _ => PreprocVerdict::No,
        }
    }
}
