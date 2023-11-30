use crate::preproc::{Preproc, PreprocVerdict, PreprocVerdictInfo};
use crate::preproc::tokenizer::MultiToken;
use std::fmt::Write;

#[derive(Default)]
pub struct ReplyPreproc {
    n: u64,
}

impl Preproc for ReplyPreproc {
    fn close(&mut self, _: &mut String, _: ()) {}
    fn reset(&mut self) {}

    fn action(&mut self, output: &mut String, _: &str, _: ()) {
        let n = self.n;
        // let _ = write!(output, "<a class=\"P-rep\">{n}</a>");
        // let _ = write!(output, "<pkg reply {n}/>");
        let _ = write!(output, "<pkg reply {n}></pkg>");
    }
    fn state_upd_str(&mut self, _: &str) -> PreprocVerdict {
        unimplemented!("use `multi_token` fn")
    }

    fn state_upd_multi_token(&mut self, token: &MultiToken) -> PreprocVerdictInfo {
        let ok = token.test_token_seq_never_empty(&mut[
            &mut|t|t.token() == ">",
            &mut|t|t.token() == ">",
            &mut|t|{
                if t.ty.is_number() {
                    let n: Result<u64, _> = t.token().parse();
                    if let Ok(n) = n {
                        self.n = n;
                        return true
                    }
                }
                false
            }
        ]);

        match ok {
            true => return PreprocVerdictInfo {
                verdict: PreprocVerdict::Matched,
                n_tokens: 3,
                propagate: false,
            },
            false => PreprocVerdictInfo::new_no(),
        }
    }
}
