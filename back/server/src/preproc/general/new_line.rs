use crate::preproc::{Preproc, PreprocVerdict};

#[derive(Default)]
pub struct NewLinePreproc {
    space_mode: bool, // if we need trait then just rename into `ignore_mdoe`
}

impl NewLinePreproc {
    pub fn set_space_mode(&mut self, space_mode: bool) {
        self.space_mode = space_mode
    } 
}

impl Preproc for NewLinePreproc {
    fn close(&mut self, _: &mut String, _: ()) {}
    fn reset(&mut self) {}

    fn action(&mut self, output: &mut String, _: &str, _: ()) {
        if self.space_mode {
            output.push(' ');
        } else {
            output.push_str("<br/>");
        }
    }

    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        if token == "\n" {
            PreprocVerdict::Matched
        } else {
            PreprocVerdict::No
        }
    }
}
