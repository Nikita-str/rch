use crate::preproc::{Preproc, PreprocVerdict};
use rand::Rng;

#[derive(Default)]
pub struct KawaiiPreproc;

impl Preproc for KawaiiPreproc {
    fn close(&mut self, _: &mut String, _: ()) { }
    fn reset(&mut self) { }

    fn action(&mut self, output: &mut String, matched_tokens: &str, _: ()) {
        let color = match rand::thread_rng().gen_range(0..=2) {
            0 => "#18c5e1",
            1 => "#ea018f",
            _ => "#b823d7",
        };

        output.push_str("<span style=\"color: ");
        output.push_str(color);
        output.push_str(";>");
        output.push_str(matched_tokens);
        output.push_str("</span>");
    }

    fn state_upd(&mut self, token: &str) -> PreprocVerdict {
        match token {
            | "UwU" | "^w^" | ":3" 
            | ">~<" | ">0<" | "kawaii"
            | "cute" | "миленько" 
            | "кавай" | "кавайно" | "кавайненько"
            => PreprocVerdict::Matched,
            
            _ => PreprocVerdict::No,
        }
    }
}
