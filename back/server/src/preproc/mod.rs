
mod board_specific;

mod general;
mod generic;

pub use head_preproc::HeadPreproc;
mod head_preproc;
mod tokenizer;
mod span;

use span::Span;
#[allow(unused)]
use tokenizer::{SimpleTokenizer, MultiTokenTokenizer};
use inner::{Preproc, PreprocVerdict, PreprocVerdictInfo, FullActInfo};

mod all_available_preproc;
pub use all_available_preproc::*;


mod inner {
    use std::collections::HashSet;

    pub enum PreprocVerdict {
        No,
        Maybe,
        Matched,
    }
    impl PreprocVerdict {
        /// # return 
        /// * `Self::Matched` if `matched`
        /// * `Self::Maybe` otherwise
        #[inline]
        pub fn new(matched: bool) -> Self {
            match matched {
                true => Self::Matched,
                _ => Self::Maybe,
            }
        }
    }

    // pub trait Preproc<T = str, State = ()>
    // where T: ?Sized {
    //     /// called on end of stream: we need close all open tag or smth like this
    //     fn close(&mut self, output: &mut String, state: State);
    //     /// called after unsuitable subseq for this preprocesor
    //     /// (when current unwrited seq is not suitable) 
    //     fn reset(&mut self);
    //     /// called after successful match
    //     fn action(&mut self, output: &mut String, matched_tokens: &str, state: State);
    //     fn state_upd(&mut self, token: &T) -> PreprocVerdict;
    // }
    //
    // impl<'s, State, X> Preproc<super::tokenizer::Token<'s>, State> for X
    // where X: for<'x> Preproc<str, State> {
    //     fn close(&mut self, output: &mut String, state: State) {
    //         Preproc::<str, State>::close(self, output, state)
    //     }

    //     fn reset(&mut self) {
    //         Preproc::<str, State>::reset(self)
    //     }

    //     fn action(&mut self, output: &mut String, matched_tokens: &str, state: State) {
    //         Preproc::<str, State>::action(self, output, matched_tokens, state)
    //     }

    //     fn state_upd(&mut self, token: &super::tokenizer::Token<'s>) -> PreprocVerdict {
    //         Preproc::<str, State>::state_upd(self, token.token)
    //     }
    // }

    pub struct PreprocVerdictInfo {
        pub verdict: PreprocVerdict,
        pub n_tokens: usize,
        pub propagate: bool,
    }
    impl PreprocVerdictInfo {
        pub fn new_by_verdict(verdict: PreprocVerdict) -> Self {
            Self {
                verdict,
                n_tokens: 1,
                propagate: true,
            }
        }
        pub fn new_single_matched_no_propagate() -> Self {
            Self {
                verdict: PreprocVerdict::Matched,
                n_tokens: 1,
                propagate: false,
            }
        }
        pub fn new_no() -> Self {
            Self {
                verdict: PreprocVerdict::No,
                n_tokens: 1,
                propagate: true,
            }
        }
    }

    pub struct FullActInfo<'x> {
        pub output: &'x mut String,
        pub reply_to: &'x mut HashSet<u64>,
    }

    pub trait Preproc<State = ()> {
        // const MAX_NON_EMTY_TOKENS: usize = 1;

        /// called on end of stream: we need close all open tag or smth like this
        fn close(&mut self, output: &mut String, state: State);
        /// called after unsuitable subseq for this preprocesor
        /// (when current unwrited seq is not suitable) 
        fn reset(&mut self);
        fn reset_by_no_propagate(&mut self, _token: &super::tokenizer::MultiToken, _n_tokens: usize) {
            self.reset()
        }
        /// called after successful match
        fn action(&mut self, output: &mut String, matched_tokens: &str, state: State);
        
        /// called after successful match but with more info than `action`
        /// # !
        /// instead if `act_info: FullActInfo` we can change default State... `state: &mut FullActInfo`(by default)
        fn action_full(&mut self, act_info: FullActInfo, matched_tokens: &str, state: State) {
            self.action(act_info.output, matched_tokens, state)
        }

        fn state_upd_str(&mut self, token: &str) -> PreprocVerdict;
        fn state_upd_simple_token(&mut self, token: &super::tokenizer::SimpleToken) -> PreprocVerdict {
            self.state_upd_str(token.token.token)
        }
        fn state_upd_multi_token(&mut self, token: &super::tokenizer::MultiToken) -> PreprocVerdictInfo {
            let verdict = self.state_upd_simple_token(token.first_token_ref());
            PreprocVerdictInfo::new_by_verdict(verdict)
        }
    }

    // impl<State, X> X
    // where X: for<'x> Preproc<str, State> {
    //     fn state_upd_by_str(&mut self, token: super::tokenizer::Token, matched_tokens: &str, state: State) -> PreprocVerdict {
    //         Preproc::<str, State>::state_upd(self, token.token)
    //     }
    // }


    // if it needed we can use ref for State:
    // ```
    // impl Preproc<&StateType> for PreprocType {
    //     fn close(&mut self, output: &mut String, state: &StateType) { ... }
    //     fn action(&mut self, output: &mut String, state: &StateType) { ... }
    //     ...
    // }
    // ``` 
}

pub fn pkg_del(board_url: &str) -> String {
    if board_url == "a" {
        "<pkg del a />".into()
    } else {
        let n = crate::utility::general::rand(0, 2);
        format!("<pkg del {n} />")
    }
}
