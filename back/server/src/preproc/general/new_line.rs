use crate::preproc::{Preproc, PreprocVerdict};

#[derive(Default)]
pub struct NewLinePreproc;

impl Preproc for NewLinePreproc {
    fn close(&mut self, _: &mut String, _: ()) {}
    fn reset(&mut self) {}

    fn action(&mut self, output: &mut String, _: ()) {
        output.push_str("<br/>");
    }

    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        if token == "\n" {
            PreprocVerdict::Matched
        } else {
            PreprocVerdict::No
        }
    }
}
