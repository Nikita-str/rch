
mod general;
mod generic;

mod head_preproc;
mod tokenizer;
mod span;

use span::Span;
use tokenizer::Tokenizer;

enum PreprocVerdict {
    No,
    Maybe,
    Matched,
}

trait Preproc {
    /// called on end of stream: we need close all open tag or smth like this
    fn close(&mut self, output: &mut String);
    /// called after unsuitable subseq for this preprocesor
    /// (when current unwrited seq is not suitable) 
    fn reset(&mut self);
    /// called after successful match
    fn action(&mut self, output: &mut String);
    fn state_upd(&mut self, token: &str) -> PreprocVerdict;
}

