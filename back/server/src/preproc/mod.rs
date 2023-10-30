
mod board_specific;

mod general;
mod generic;

pub use head_preproc::HeadPreproc;
mod head_preproc;
mod tokenizer;
mod span;

use span::Span;
use tokenizer::Tokenizer;
use inner::{Preproc, PreprocVerdict};

mod all_available_preproc;
pub use all_available_preproc::*;


mod inner {
    pub enum PreprocVerdict {
        No,
        Maybe,
        Matched,
    }

    pub trait Preproc<State = ()> {
        /// called on end of stream: we need close all open tag or smth like this
        fn close(&mut self, output: &mut String, state: State);
        /// called after unsuitable subseq for this preprocesor
        /// (when current unwrited seq is not suitable) 
        fn reset(&mut self);
        /// called after successful match
        fn action(&mut self, output: &mut String, matched_tokens: &str, state: State);
        fn state_upd(&mut self, token: &str) -> PreprocVerdict;
    }

    // if it needed we can use ref for State:
    // ```
    // impl Preproc<&StateType> for PreprocType {
    //     fn close(&mut self, output: &mut String, state: &StateType) { ... }
    //     fn action(&mut self, output: &mut String, state: &StateType) { ... }
    //     ...
    // }
    // ``` 
}
