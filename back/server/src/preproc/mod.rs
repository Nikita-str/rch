
mod board_specific;

mod general;
mod generic;

pub use head_preproc::HeadPreproc;
mod head_preproc;
mod tokenizer;
mod span;

use span::Span;
use tokenizer::SimpleTokenizer;
use inner::{Preproc, PreprocVerdict};

mod all_available_preproc;
pub use all_available_preproc::*;


mod inner {
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

    pub trait Preproc<State = ()> {
        /// called on end of stream: we need close all open tag or smth like this
        fn close(&mut self, output: &mut String, state: State);
        /// called after unsuitable subseq for this preprocesor
        /// (when current unwrited seq is not suitable) 
        fn reset(&mut self);
        /// called after successful match
        fn action(&mut self, output: &mut String, matched_tokens: &str, state: State);
        fn state_upd_str(&mut self, token: &str) -> PreprocVerdict;
        fn state_upd_simple_token(&mut self, token: &super::tokenizer::SimpleToken) -> PreprocVerdict {
            self.state_upd_str(token.token.token)
        }
        fn state_upd_multi_token(&mut self, token: &super::tokenizer::MultiToken) -> PreprocVerdict {
            self.state_upd_simple_token(token.first_token_ref())
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
