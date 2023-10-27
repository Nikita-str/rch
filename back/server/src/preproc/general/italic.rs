use crate::preproc::{Preproc, PreprocVerdict};
use crate::preproc::generic::OpclPreproc;
use crate::preproc::generic::OpclInnerState as State;

#[derive(Default)]
pub(in crate::preproc)
struct ItalicInnerPreproc;

impl Preproc<State> for ItalicInnerPreproc {
    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        match token {
            "i" | "italic" => PreprocVerdict::Matched,
            _ => PreprocVerdict::No,
        }
    }

    fn action(&mut self, output: &mut String, state: State) {
        if state.open_times == 1 {
            if state.is_open {
                output.push_str("<i>")
            } else {
                output.push_str("</i>")
            }
        }
    }

    fn reset(&mut self) { }
    fn close(&mut self, output: &mut String, state: State) {
        if state.is_open {
            println!("[ALGO WARN]: close with `is_open == true`");
            return
        }
        if state.open_times == 1 {
            output.push_str("</i>")
        }
    }
}

pub(in crate::preproc)
type ItalicPreproc = OpclPreproc<ItalicInnerPreproc>;