use crate::preproc::{Preproc, PreprocVerdict, PreprocVerdictInfo};
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
    /// `>%smth%<` case
    is_xd: bool,
}

impl Preproc for KawaiiPreproc {
    fn close(&mut self, _: &mut String, _: ()) {
        self.reset();
    }

    fn reset(&mut self) {
        self.state = State::NotStarted;
        self.is_xd = false;
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
        if self.is_xd {
            output.push_str("&#62;"); // '>'
            output.push_str(&matched_tokens[1..=matched_tokens.len() - 2]);
            output.push_str("&#60;"); // '<'
        } else {
            output.push_str(matched_tokens);
        }
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
    
    fn state_upd_multi_token(&mut self, token: &crate::preproc::tokenizer::MultiToken) -> PreprocVerdictInfo {
        let kawaii_center = |s:&str|s == "~" || s == "w" || s == ".";
        let center = token.test_token_seq_never_empty(&mut[
            &mut|t|t.token() == ">",
            &mut|t|kawaii_center(t.token()),
            &mut|t|t.token() == "<",
        ]);
        if center {
            self.is_xd = true;
            return PreprocVerdictInfo {
                verdict: PreprocVerdict::Matched,
                n_tokens: 3,
                propagate: false,
            } 
        }

        PreprocVerdictInfo::new_by_verdict(self.state_upd_str(token.first_token_ref().token()))
    }
}
