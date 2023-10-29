
#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// include
    from: usize,
    /// exclude 
    to: usize,
}

#[allow(unused)]
impl Span {
    #[inline]
    pub fn start(&self) -> usize {
        self.from
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.to
    }


    pub fn new_empty(byte_pos: usize) -> Self {
        Self {
            from: byte_pos,
            to: byte_pos,
        }
    }

    pub fn new_union(mut a: Self, b: Self) -> Self {
        a.union(b);
        a
    }


    pub fn union(&mut self, span: Self) {
        self.from = self.from.min(span.from);
        self.to = self.to.max(span.to);
    }
    
    /// `[A; B)` --> `[A; B + expand)`
    pub fn expand_right(&mut self, expand: usize) {
        self.to += expand
    }

    pub fn extract_str<'s>(&self, s: &'s str) -> &'s str {
        &s[self.from..self.to]
    }
}

impl PartialEq<(usize, usize)> for Span {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.from == other.0 && self.to == other.1
    }
}
