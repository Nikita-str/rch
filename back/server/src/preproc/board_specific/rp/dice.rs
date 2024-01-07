use crate::preproc::{Preproc, PreprocVerdict};
use std::fmt::Write;
use rand::Rng;

#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// match `d : number` / `d$number`
enum State {
    #[default]
    NotStarted,
    Err,

    /// `d` / `dice` / `D`
    WordDice,
    Colon,
    /// number of sides 
    N,
}

impl State {
    fn state_upd(self, token: &str) -> Self {
        if token.chars().all(|c| c.is_whitespace()) {
            return self
        }

        let is_num = token.chars().all(|c| c.is_ascii_digit());
        
        const VALID_DICE_IDENT: &[&str] = &["d", "D", "dice", "Dice"];
        match self {
            Self::NotStarted if VALID_DICE_IDENT.contains(&token) => Self::WordDice,
            Self::WordDice if token == ":" => Self::Colon,
            Self::WordDice | Self::Colon if is_num => Self::N,

            _ => Self::Err,
        }
    }

    #[inline]
    fn is_err(self) -> bool {
        matches!(self, Self::Err)
    }

    #[inline]
    fn is_ended(self) -> bool {
        matches!(self, Self::N)
    }
}

#[derive(Default)]
pub struct __InnerPreproc<D: Dices> {
    state: State,
    dice: usize,
    phantom: std::marker::PhantomData<(D,)>
}

impl<D: Dices> Preproc for __InnerPreproc<D> {
    fn close(&mut self, _: &mut String, _: ()) {
        self.reset()
    }

    fn reset(&mut self) {
        self.state = State::NotStarted
    }

    fn action(&mut self, output: &mut String, _: &str, _: ()) {
        let mut rng = rand::thread_rng();
        let dice = self.dice;    
        let rand_num = rng.gen_range(1..=dice);
        
        let _ = write!(output, "<span class=\"P-d{dice}\" title=\"d{dice}\">{rand_num}</span>");
    }

    fn state_upd_str(&mut self, token: &str) -> PreprocVerdict {
        let state = self.state.state_upd(token);
        if state.is_err() {
            return PreprocVerdict::No
        }
        
        if self.state != state {
            if state == State::N {
                let Ok(dice) = token.parse::<usize>() else { return PreprocVerdict::No };
                if !D::valid_dice(&dice) { return PreprocVerdict::No };
                self.dice = dice;
            }
            self.state = state;
        }
        
        PreprocVerdict::new(self.state.is_ended())
    }
}

pub type DiceStdPreproc = crate::preproc::generic::SingleCmdPreproc<__InnerPreproc<StdDices>>;


pub trait Dices {
    const DICES: &'static [usize]; // yeah, usize, yeah Dice:999_999
    fn valid_dice(dice: &usize) -> bool {
        Self::DICES.contains(dice)
    }
}

#[derive(Default)]
pub struct StdDices;
const STD_DICES: [usize; 7] = [4, 6, 8, 10, 12, 20, 100];
impl Dices for StdDices {
    const DICES: &'static [usize] = &STD_DICES;
}