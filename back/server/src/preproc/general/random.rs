use crate::preproc::{Preproc, PreprocVerdict};
use std::fmt::Write;
use rand::Rng;

#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// match `random(from, to)`
enum State {
    #[default]
    NotStarted,

    /// `random`
    WordRandom,
    /// `(`
    Open,

    /// `-`
    FromMinus,
    FromNumber,
    /// `,` or `:`
    Comma,
    /// `-`
    ToMinus,
    ToNumber,

    /// `)`
    Close,

    Err,
}

impl State {
    fn state_upd(self, token: &str) -> Self {
        if token.chars().into_iter().all(|c| c.is_whitespace()) {
            return match self {
                Self::FromMinus | Self::ToMinus => Self::Err,
                _ => self,
            }
        }

        let is_num = token.chars().into_iter().all(|c| c.is_ascii_digit());

        match self {
            Self::NotStarted if token == "random" => Self::WordRandom,
            Self::WordRandom if token == "(" => Self::Open,
            
            Self::Open if token == "-" => Self::FromMinus,
            Self::Open | Self::FromMinus if is_num => Self::FromNumber,

            Self::FromNumber if token == "," || token == ":" => Self::Comma,
            
            Self::Comma if token == "-" => Self::ToMinus,
            Self::Comma | Self::ToMinus if is_num => Self::ToNumber,

            Self::ToNumber if token == ")" => Self::Close,

            _ => Self::Err,
        }
    }

    #[inline]
    fn is_err(self) -> bool {
        return matches!(self, Self::Err)
    }

    #[inline]
    fn is_ended(self) -> bool {
        return matches!(self, Self::Close)
    }
}

#[derive(Default)]
pub struct __InnerPreproc {
    state: State,
    from: i32,
    to: i32,
}

impl Preproc for __InnerPreproc {
    fn close(&mut self, _: &mut String, _: ()) {
        self.reset()
    }

    fn reset(&mut self) {
        self.state = State::NotStarted
    }

    fn action(&mut self, output: &mut String, _: ()) {
        let mut rng = rand::thread_rng();
        let from = self.from.min(self.to);
        let to = self.from.max(self.to);
        let rand_num = rng.gen_range(from..=to);
        let _ = write!(output, "{rand_num}");
    }

    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        let state = self.state.state_upd(token);
        if state.is_err() {
            return PreprocVerdict::No
        }
        
        if self.state != state {
            if state == State::FromNumber {
                let minus = self.state == State::FromMinus;
                let Ok(x) = token.parse::<i32>() else { return PreprocVerdict::No };
                self.from = if minus { -x } else { x };
            }
            if state == State::ToNumber {
                let minus = self.state == State::ToMinus;
                let Ok(x) = token.parse::<i32>() else { return PreprocVerdict::No };
                self.to = if minus { -x } else { x };
            }
            self.state = state;
        }
        
        if self.state.is_ended() {
            return PreprocVerdict::Matched
        }
        return PreprocVerdict::Maybe
    }
}

pub type RandomPreproc = crate::preproc::generic::SingleCmdPreproc<__InnerPreproc>;
