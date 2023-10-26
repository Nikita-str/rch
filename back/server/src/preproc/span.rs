
#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// include
    from: usize,
    /// exclude 
    to: usize,
}

impl Span {
    pub fn union(a: Self, b: Self) -> Self {
        Self {
            from: a.from.min(b.from),
            to: a.to.max(b.to),
        }
    }

    pub fn new_empty(byte_pos: usize) -> Self {
        Self {
            from: byte_pos,
            to: byte_pos,
        }
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
