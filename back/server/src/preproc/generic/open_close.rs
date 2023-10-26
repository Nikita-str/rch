use crate::preproc::PreprocVerdict;

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
trait InnerState {
    // const CAN_BE_MULTI_OPEN: bool;
    // fn is_err(&self) -> bool;
    // fn is_ended(&self) -> bool;
    

    /// # return
    /// * if err must return `PreprocVerdict::No`
    /// * if ended must return `PreprocVerdict::Matched`
    fn state_upd(&mut self, token: &str) -> PreprocVerdict;


    /// # params
    /// ## `open_times`
    /// ```
    /// [/A] [A][A][/A][A][/A][/A]  [A] [/A] [/A] [/A]
    ///   0   1  2   2  2   2   1    1    1    0    0
    /// ````
    fn action(&mut self, output: &mut String, open_times: usize, is_open: bool);
    fn reset(&mut self);


    /*
    // forced close call when there incorrect parrentheses order
    // for example in case of `000 [a] 111 [b] 222 [/a] 333 [/b]`
    // `forced_close_action` will be called for `[b]` before `[/a]`
    // and if [b]::CAN_BE_FORCED_OPEN => after `[/a]` will be called `forced_open_action`
    // so it will be like `000 <a> 111 <b> 222 </b></a><b> 333 </b>`

    const CAN_BE_FORCED_OPEN: bool;
    fn forced_close_action(self, output: &mut String);
    fn forced_open_action(self, output: &mut String);
    */
}

pub(in crate::preproc)
struct PreprocState<Inner: InnerState + Default> {
    inner: Inner,
    // TODO:MAYBE: successful open inner queue ?
    
    state: State,
    open_times: usize,
    // catch_unopen: bool, // setting of `Self::new()` 

    cur_match_type: PreprocType,
    cur_inner: bool,
}

impl<Inner: InnerState + Default> Default for PreprocState<Inner> {
    fn default() -> Self {
        Self {
            inner: Inner::default(),
            state: State::NotStarted,
            open_times: 0,
            cur_match_type: PreprocType::Unkn,
            cur_inner: false,
        }
    }
}

impl<Inner: InnerState + Default> PreprocState<Inner> {
    pub fn reset(&mut self) {
        self.inner.reset();
        self.state = State::NotStarted;
        self.cur_match_type = PreprocType::Unkn;
        self.cur_inner = false;
    }

    pub fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        if !self.cur_inner {
            if self.state.transfer_ctrl_to_inner(token) {
                self.cur_inner = true;
                self.cur_match_type = self.state.get_type();
                self.state.to_inner();
            }
        }

        if self.cur_inner {
            self.state = self.state.state_upd(token);
            if self.state.is_err() {
                return PreprocVerdict::No
            }
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

    pub fn action(&mut self, output: &mut String) {
        let is_open = self.cur_match_type.is_open();
        let open_times = self.open_times + if is_open { 1 } else { 0 }; 
        
        self.inner.action(output, open_times, is_open);

        if is_open {
            self.open_times += 1;
        } else {
            if self.open_times > 0 {
                self.open_times -= 1;
            }
        }
    }
}