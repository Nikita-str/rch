use crate::preproc::{Preproc, PreprocVerdict, PreprocVerdictInfo};
use crate::preproc::tokenizer::MultiToken;

pub struct QuotePreproc {
    is_new_line: bool,
    open: bool,
}

impl Default for QuotePreproc {
    fn default() -> Self {
        Self {
            is_new_line: true, 
            open: false,
        }
    }
}

impl Preproc for QuotePreproc {
    fn close(&mut self, _: &mut String, _: ()) {}
    fn reset(&mut self) {}
    fn reset_by_no_propagate(&mut self, token: &MultiToken, n_tokens: usize) {
        if !self.is_new_line { return }
        for index in 0..n_tokens {
            let Some(token) = token.get_simple_token(index) else { return };
            if !token.ty.is_spaces() {
                self.is_new_line = false;
                return
            }
        }
    }

    fn action(&mut self, output: &mut String, _: &str, _: ()) {
        if self.open {
            output.push_str("</div>"); // close
            self.is_new_line = true;
            self.open = false;
        } else if self.is_new_line {
            output.push_str("<div class=\"P-quote\">");
            self.is_new_line = false;
            self.open = true;
        } else {
            self.is_new_line = true;
            self.open = false;
        }
    }
    fn state_upd_str(&mut self, _: &str) -> PreprocVerdict {
        unimplemented!("use `multi_token` fn")
    }

    fn state_upd_multi_token(&mut self, token: &MultiToken) -> PreprocVerdictInfo {
        //TODO:try move `is_new_line` in MultiToken (because there exist case when you cant catch because of `reset`)... hmm...
        let t0 = token.first_token_ref();
    
        match t0.ty {
            crate::preproc::tokenizer::SimpleTokenType::NewLine if self.open => {
                PreprocVerdictInfo::new_by_verdict(PreprocVerdict::Matched)
            }
            crate::preproc::tokenizer::SimpleTokenType::NewLine => {
                self.is_new_line = true;
                self.open = false;
                PreprocVerdictInfo::new_no()
            }
            crate::preproc::tokenizer::SimpleTokenType::Spaces if self.is_new_line => {
                PreprocVerdictInfo::new_by_verdict(PreprocVerdict::Maybe)
            }
            crate::preproc::tokenizer::SimpleTokenType::SpecialChar if self.is_new_line && t0.token() == ">" => {
                PreprocVerdictInfo::new_single_matched_no_propagate()
            }
            // crate::preproc::tokenizer::SimpleTokenType::Unkn => todo!(),
            _ => {
                self.is_new_line = false;
                PreprocVerdictInfo::new_no()
            }
        }
    }
}
