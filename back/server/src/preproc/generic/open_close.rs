use crate::preproc::{Preproc, PreprocVerdict};

#[derive(Clone, Copy, )]
enum PreprocType {
    Unkn,
    /// after `[` `{INNER:start}`
    Open,
    /// after `[/`
    Close,
}

impl PreprocType {
    /// # panic
    /// if `self` is `Self::Unkn` 
    fn is_open(self) -> bool {
        match self {
            PreprocType::Open => true,
            PreprocType::Close => false,
            PreprocType::Unkn => panic!("[ALGO ERR]: preproc type must be known at this point"),
        }
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
enum State {
    NotStarted,

    /// `[`
    Open,
    /// `/`
    CloseMark,
    Inner,
    /// `]`
    Close,

    Err,
}

impl State {
    fn state_upd(self, token: &str) -> Self {
        // if we need &mut self:
        //    | let mut state = Self::Tmp;
        //    | std::mem::swap(self, &mut state);
        
        match (self, token) {
            (Self::NotStarted, "[") => Self::Open,
            (Self::Open, "/") => Self::CloseMark,
            (Self::Inner, "]") => Self::Close,

            _ => Self::Err,
        }
    }

    fn transfer_ctrl_to_inner(self, token: &str) -> bool {
        match self {
            Self::Open => { token != "/" },
            Self::CloseMark => true,
            _ => false,
        }
    }
    
    /// # !!!
    /// call only after successful `transfer_ctrl_to_inner`
    /// # panic
    /// * if self isn't `::Open` / `::CloseMark`
    fn get_type(&self) -> PreprocType {
        match self {
            Self::Open => PreprocType::Open,
            Self::CloseMark => PreprocType::Close,
            _ => panic!("[ALGO ERR]: incorrect `get_type` call"),
        }
    }

    #[inline]
    fn to_inner(&mut self) {
        *self = Self::Inner
    }

    #[inline]
    fn is_err(self) -> bool {
        return matches!(self, Self::Err)
    }

    #[inline]
    fn is_inner(self) -> bool {
        return matches!(self, Self::Inner)
    }
    
    #[inline]
    fn is_ended(self) -> bool {
        return matches!(self, Self::Close)
    }
}


pub(in crate::preproc)
struct InnerState {
    pub open_times: usize,
    pub is_open: bool,
}

pub(in crate::preproc)
struct OpclPreproc<Inner: Preproc<InnerState> + Default> {
    inner: Inner,
    // TODO:MAYBE: successful open inner queue ?
    
    state: State,
    open_times: usize,
    // catch_unopen: bool, // setting of `Self::new()` 

    cur_match_type: PreprocType,
    cur_inner: bool,

    changed: bool,
}

impl<Inner: Preproc<InnerState> + Default> Default for OpclPreproc<Inner> {
    fn default() -> Self {
        Self {
            inner: Inner::default(),
            state: State::NotStarted,
            open_times: 0,
            cur_match_type: PreprocType::Unkn,
            cur_inner: false,
            changed: false,
        }
    }
}

impl<Inner: Preproc<InnerState> + Default> Preproc for OpclPreproc<Inner> {
    fn close(&mut self, output: &mut String, _: ()) {
        self.reset();
        let is_open = false;
        for open_times in (1..=self.open_times).rev() {
            let preproc_state = InnerState {
                open_times,
                is_open,
            };
            self.inner.close(output, preproc_state);
        }
    }

    fn reset(&mut self) {
        if !self.changed { return }

        self.inner.reset();
        self.state = State::NotStarted;
        self.cur_match_type = PreprocType::Unkn;
        self.cur_inner = false;
        self.changed = false;
    }

    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        if !self.cur_inner {
            if self.state.transfer_ctrl_to_inner(token) {
                self.cur_inner = true;
                self.cur_match_type = self.state.get_type();
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
            match self.inner.state_upd(token) {
                PreprocVerdict::No => return PreprocVerdict::No,
                PreprocVerdict::Maybe => { }
                PreprocVerdict::Matched => {
                    self.cur_inner = false;
                }
            }
        }
        
        return PreprocVerdict::Maybe
    }

    fn action(&mut self, output: &mut String, _: ()) {
        let is_open = self.cur_match_type.is_open();
        let open_times = self.open_times + if is_open { 1 } else { 0 }; 
        let preproc_state = InnerState {
            open_times,
            is_open,
        };

        self.inner.action(output, preproc_state);

        if is_open {
            self.open_times += 1;
        } else {
            if self.open_times > 0 {
                self.open_times -= 1;
            }
        }
    }
}