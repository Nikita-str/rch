use crate::preproc::Span;

#[derive(Debug, Clone)]
pub struct Token<'s> {
    pub token: &'s str,
    pub span: Span,
}

impl<'s, 'x> PartialEq<((usize, usize), &'x str)> for Token<'s> {
    fn eq(&self, other: &((usize, usize), &'x str)) -> bool {
        self.token == other.1 && self.span == other.0
    }
}
