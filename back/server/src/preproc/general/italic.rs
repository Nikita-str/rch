use crate::preproc::PreprocVerdict;
use crate::preproc::generic::OpclPreprocState;
use crate::preproc::generic::OpclInnerState;

#[derive(Default)]
pub(in crate::preproc)
struct ItalicInnerState;

impl OpclInnerState for ItalicInnerState {
    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        match token {
            "i" | "italic" => PreprocVerdict::Matched,
            _ => PreprocVerdict::No,
        }
    }

    fn action(&mut self, output: &mut String, open_times: usize, is_open: bool) {
        if open_times == 1 {
            if is_open {
                output.push_str("<i>")
            } else {
                output.push_str("</i>")
            }
        }
    }

    fn reset(&mut self) { }
    fn close(&mut self, output: &mut String, open_times: usize, is_open: bool) {
        if is_open {
            println!("[ALGO WARN]: close with `is_open == true`");
            return
        }
        if open_times == 1 {
            output.push_str("</i>")
        }
    }
}

pub(in crate::preproc)
type ItalicState = OpclPreprocState<ItalicInnerState>;