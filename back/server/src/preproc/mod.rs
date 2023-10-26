
mod general;
mod generic;

mod tokenizer;
mod span;
use span::Span;

enum PreprocVerdict {
    No,
    Maybe,
    Matched,
}

