use crate::preproc::{Preproc, PreprocVerdict, PreprocVerdictInfo, FullActInfo};
use crate::preproc::tokenizer::MultiToken;
use std::fmt::Write;

#[derive(Default)]
pub struct ReplyPreproc {
    n: u64,
}

impl Preproc for ReplyPreproc {
    fn close(&mut self, _: &mut String, _: ()) {}
    fn reset(&mut self) {}

    fn action_full(&mut self, act_info: FullActInfo, _: &str, _: ()) {
        let n = self.n;
        let _ = write!(act_info.output, "<pkg reply {n}></pkg>");
        act_info.reply_to.push(n);
    }
    fn action(&mut self, _: &mut String, _: &str, _: ()) {
        unimplemented!("use `action_full` fn");
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
    fn state_upd_str(&mut self, _: &str) -> PreprocVerdict {
        unimplemented!("use `multi_token` fn")
    }
}
