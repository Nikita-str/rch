use crate::preproc::{Preproc, PreprocVerdict};
use rand::Rng;

#[derive(Clone, Copy, Default)]
enum State {
    #[default]
    NotStarted,

    /// `^`
    Upper,
    /// `^X`
    UpperX,

    // /// `>`
    // Left,
    // /// `>X`
    // LeftX,

    /// `:`
    Colon,
}

#[derive(Default)]
pub struct KawaiiPreproc {
    state: State,
}

impl Preproc for KawaiiPreproc {
    fn close(&mut self, _: &mut String, _: ()) {
        self.reset();
    }

    fn reset(&mut self) {
        self.state = State::NotStarted;
    }

    fn action(&mut self, output: &mut String, matched_tokens: &str, _: ()) {
        let color = match rand::thread_rng().gen_range(0..=2) {
            0 => "#18c5e1",
            1 => "#ea018f",
            _ => "#b823d7",
        };

        output.push_str("<span style=\"color: ");
        output.push_str(color);
        output.push_str("\";>");
        output.push_str(matched_tokens);
        output.push_str("</span>");
    }

    fn state_upd_str(&mut self, token: &str) -> PreprocVerdict {
        match (self.state, token) {
            (
                State::NotStarted,
                | "UwU"
                | "kawaii"
                | "cute"
                | "миленько"
                | "мило"
                | "кавай"
                | "кавая"
                | "кавайно"
                | "кавайненько",
            ) => return PreprocVerdict::Matched,

            // "^w^" | ":3" | ">~<" | ">0<"
            // (State::NotStarted, ">") => self.state = State::Left,
            // (State::Left, "~" | "w" | "0") => self.state = State::LeftX,
            // (State::LeftX, "<") => return PreprocVerdict::Matched,

            // "^w^" | ":3"
            (State::NotStarted, "^") => self.state = State::Upper,
            (State::Upper, "w" | "~") => self.state = State::UpperX,
            (State::UpperX, "^") => return PreprocVerdict::Matched,

            (State::NotStarted, ":") => self.state = State::Colon,
            (State::Colon, "3") => return PreprocVerdict::Matched,
            
            _ => return PreprocVerdict::No,
        }

        PreprocVerdict::Maybe
    }
}
