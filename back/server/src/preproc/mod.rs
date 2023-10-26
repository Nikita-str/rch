
mod general;
mod generic;


enum PreprocVerdict {
    No,
    Maybe,
    Matched,
}


#[derive(Clone, Copy)]
pub struct Span {
    from: usize,
    to: usize,
}
impl Span {
    pub fn union(a: Self, b: Self) -> Self {
        Self {
            from: a.from.min(b.from),
            to: a.to.max(b.to),
        }
    }
}

