use crate::preproc::{Preproc, PreprocVerdict};


#[derive(Debug)]
#[derive(Clone, Copy)]
enum State {
    NotStarted,

    /// `[`
    Open,
    /// CMD
    Inner,
    /// `]`
    Close,

    Err,
}

impl State {
    fn state_upd(self, token: &str) -> Self {
        match (self, token) {
            (Self::NotStarted, "[") => Self::Open,
            (Self::Inner, "]") => Self::Close,

            _ => Self::Err,
        }
    }

    fn transfer_ctrl_to_inner(self) -> bool {
        matches!(self, Self::Open)
    }

    #[inline]
    fn to_inner(&mut self) {
        *self = Self::Inner
    }

    #[inline]
    fn is_err(self) -> bool {
        return matches!(self, Self::Err)
    }

    #[allow(unused)]
    #[inline]
    fn is_inner(self) -> bool {
        return matches!(self, Self::Inner)
    }
    
    #[inline]
    fn is_ended(self) -> bool {
        return matches!(self, Self::Close)
    }
}


pub struct SingleCmd<Inner: Preproc + Default> {
    inner: Inner,
    state: State,

    cur_inner: bool,
    changed: bool,
}
impl<Inner: Preproc + Default> SingleCmd<Inner> {
    pub(in crate::preproc)
    fn inner_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

impl<Inner: Preproc + Default> Default for SingleCmd<Inner> {
    fn default() -> Self {
        Self {
            inner: Inner::default(),
            state: State::NotStarted,
            cur_inner: false,
            changed: false,
        }
    }
}

impl<Inner: Preproc + Default> Preproc for SingleCmd<Inner> {
    fn close(&mut self, _: &mut String, _: ()) {
        self.reset()
    }

    fn reset(&mut self) {
        if self.changed {
            self.inner.reset();
            self.state = State::NotStarted;
            self.cur_inner = false;
            self.changed = false;
            // *self = Self::default()
        }
    }

    fn action(&mut self, output: &mut String, matched_tokens: &str, state: ()) {
        self.inner.action(output, matched_tokens, state)
    }

    fn state_upd_str(&mut self, token: &str) -> PreprocVerdict {
        if !self.cur_inner {
            if self.state.transfer_ctrl_to_inner() {
                self.cur_inner = true;
                self.state.to_inner();
            }
        }

        if !self.cur_inner {
            let state = self.state.state_upd(token);
            if state.is_err() {
                return PreprocVerdict::No
            }

            self.state = state;
            self.changed = true;
            
            if self.state.is_ended() {
                return PreprocVerdict::Matched
            }
        } else {
            match self.inner.state_upd_str(token) {
                PreprocVerdict::No => return PreprocVerdict::No,
                PreprocVerdict::Maybe => { }
                PreprocVerdict::Matched => {
                    self.cur_inner = false;
                }
            }
        }
        
        return PreprocVerdict::Maybe
    } 
}
