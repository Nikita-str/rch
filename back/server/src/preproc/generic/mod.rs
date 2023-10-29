
mod open_close;
pub(in crate::preproc) use open_close::InnerState as OpclInnerState;
pub(in crate::preproc) use open_close::OpclPreproc;

mod single_cmd;
pub(in crate::preproc) use single_cmd::SingleCmd as SingleCmdPreproc;


#[macro_export(local_inner_macros)]
macro_rules! define_opcl {
    [$prep_type_name:ident [$($expect_inner:literal);*] $open:literal / $close:literal] => {
        use crate::preproc::{Preproc, PreprocVerdict};
        use crate::preproc::generic::OpclInnerState as State;

        #[derive(Default)]
        pub struct __InnerPreproc;

        impl Preproc<State> for __InnerPreproc {
            fn reset(&mut self) { }
            fn state_upd(&mut self, token: &str) -> PreprocVerdict {
                match token {
                    $( | $expect_inner )* => PreprocVerdict::Matched,
                    _ => PreprocVerdict::No,
                }
            }

            fn action(&mut self, output: &mut String, _: &str, state: State) {
                if state.open_times == 1 {
                    if state.is_open {
                        output.push_str($open)
                    } else {
                        output.push_str($close)
                    }
                }
            }
            fn close(&mut self, output: &mut String, state: State) {
                if state.is_open {
                    std::println!("[ALGO WARN]: close with `is_open == true`");
                    return
                }
                if state.open_times == 1 {
                    output.push_str($close)
                }
            }
        }

        pub type $prep_type_name = crate::preproc::generic::OpclPreproc<__InnerPreproc>;
    };

    [MULTI $prep_type_name:ident [$($expect_inner:literal);*] $open:literal / $close:literal] => {
        use crate::preproc::{Preproc, PreprocVerdict};
        use crate::preproc::generic::OpclInnerState as State;

        #[derive(Default)]
        pub struct __InnerPreproc;

        impl Preproc<State> for __InnerPreproc {
            fn reset(&mut self) { }
            fn state_upd(&mut self, token: &str) -> PreprocVerdict {
                match token {
                    $( | $expect_inner )* => PreprocVerdict::Matched,
                    _ => PreprocVerdict::No,
                }
            }

            fn action(&mut self, output: &mut String, _: &str, state: State) {
                if state.open_times != 0 {
                    if state.is_open {
                        output.push_str($open)
                    } else {
                        output.push_str($close)
                    }
                }
            }
            fn close(&mut self, output: &mut String, state: State) {
                if state.is_open {
                    std::println!("[ALGO WARN]: close with `is_open == true`");
                    return
                }
                if state.open_times != 0 {
                    output.push_str($close)
                }
            }
        }

        pub type $prep_type_name = crate::preproc::generic::OpclPreproc<__InnerPreproc>;
    };
}

pub (in crate::preproc) use define_opcl;