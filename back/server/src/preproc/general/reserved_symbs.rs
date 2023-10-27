use crate::preproc::{Preproc, PreprocVerdict};

#[derive(Default)]
pub struct ReservedSymbsPreproc {
    // TODO:
    // it's good to pass full match into `action`... but for it we need to change code
    // we can do it by use unwrited_span U span
    token: char,
}

impl Preproc for ReservedSymbsPreproc {
    fn close(&mut self, _: &mut String, _: ()) {}
    fn reset(&mut self) {
        self.token = ' ';
    }

    fn action(&mut self, output: &mut String, _: ()) {
        let token = self.token;
        match token {
            '"' => output.push_str("&#34;"), 
            '\'' => output.push_str("&#39;"), 
            '&' => output.push_str("&#38;"), 
            '<' => output.push_str("&#60;"), 
            '>' => output.push_str("&#62;"), 
            _ => println!("[ALGO WARN] unexpected token"),
        }
    }

    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        if token.len() != 1 {
            return PreprocVerdict::No;
        }

        self.token = token.as_bytes()[0] as char;
        match self.token {
            '\'' | '"' | '>' | '<' | '&' => PreprocVerdict::Matched,
            _ => PreprocVerdict::No,
        }
    }
}
