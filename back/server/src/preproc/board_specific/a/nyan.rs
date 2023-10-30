use crate::preproc::{Preproc, PreprocVerdict};


#[derive(Debug)]
#[derive(Default)]
#[derive(Clone, Copy)]
enum State {
    #[default]
    NotStarted,

    /// `~`
    Open1,
    /// `~~`
    Open2,
    Inner,
    /// `~`
    Close1,
    /// `~~`
    Close2,

    Err,
}

impl State {
    fn state_upd(self, token: &str) -> Self {
        match (self, token) {
            (Self::NotStarted, "~") => Self::Open1,
            (Self::Open1, "~") => Self::Open2,

            (Self::Open2, "~")  => Self::Err,
            (Self::Open2, token) if token.chars().all(|c|c.is_whitespace()) => Self::Open2,
            (Self::Open2, _)  => Self::Inner,

            (Self::Inner, "~") => Self::Close1,
            (Self::Close1, "~") => Self::Close2,
            
            (Self::Inner, _) => Self::Inner,

            _ => Self::Err,
        }
    }

    #[inline]
    fn is_err(self) -> bool {
        return matches!(self, Self::Err)
    }

    #[inline]
    fn is_ended(self) -> bool {
        return matches!(self, Self::Close2)
    }
}


#[derive(Default)]
pub struct NyanPreproc {
    state: State,
}

impl Preproc for NyanPreproc {
    fn close(&mut self, _: &mut String, _: ()) {
        self.reset()
    }

    fn reset(&mut self) {
        *self = Self::default()
    }

    fn action(&mut self, output: &mut String, matched_tokens: &str, _: ()) {
        // TODO: or as different action (open and close) for multiline and another tags inner posibility?!
        output.push_str("<span class=\"P-a-nyan\">");
        output.push_str(matched_tokens);
        output.push_str("</span>");
    }

    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        self.state = self.state.state_upd(token);
        if self.state.is_err() {
            return PreprocVerdict::No
        }

        if self.state.is_ended() {
            return PreprocVerdict::Matched
        }

        return PreprocVerdict::Maybe
    } 
}
