use crate::preproc::{Preproc, PreprocVerdict};

#[derive(Clone, Copy)]
enum CatCase {
    KittyRu,
    KittyEn,
    CatRu,
    CatEn,
    Nyash,
}

impl CatCase {
    const fn word(self) -> &'static str {
        match self {
            CatCase::KittyRu => "котик",
            CatCase::KittyEn => "kitty",
            CatCase::CatRu => "кот",
            CatCase::CatEn => "cat",
            CatCase::Nyash => "няш",
        }
    }
    const fn max_len_diff(self) -> usize {
        match self {
            CatCase::Nyash | CatCase::KittyRu | CatCase::CatRu => "ик".len(),
            CatCase::CatEn => 1,
            CatCase::KittyEn => 0,
        }
    }
    fn test(self, token: &str) -> bool {
        let word = self.word();
        let Some(word_c0) = word.chars().next() else { return false };
        let Some(token_c0) = token.chars().next() else { return false };        
        if word_c0.to_lowercase().next() != token_c0.to_lowercase().next() { return false }

        let word = &word[word_c0.len_utf8()..];
        let token = &token[token_c0.len_utf8()..];

        token.starts_with(word) && (token.len() - token_c0.len_utf8() <= word.len() + self.max_len_diff())
    }

    fn from_token(token: &str) -> Option<Self> {
        if Self::KittyRu.test(token) { Some(Self::KittyRu) }
        else if Self::CatRu.test(token) { Some(Self::CatRu) }
        else if Self::Nyash.test(token) { Some(Self::Nyash) }
        else if Self::KittyEn.test(token) { Some(Self::KittyEn) }
        else if Self::CatEn.test(token) { Some(Self::CatEn) }
        else { None }
    }
}

#[derive(Default)]
pub struct CatPreproc {
    case: Option<CatCase>,
}

impl Preproc for CatPreproc {
    fn close(&mut self, _: &mut String, _: ()) { }
    fn reset(&mut self) { }

    fn action(&mut self, output: &mut String, matched_tokens: &str, _: ()) {
        let Some(case) = self.case else {
            println!("[ALGO ERROR] cat-case was None");
            return
        };

        output.push_str("<span class=\"P-a-cat\">:3</span>");
        output.push_str(&matched_tokens[case.word().len()..]);
    }

    fn state_upd_str(&mut self, token: &str) -> PreprocVerdict {
        self.case = CatCase::from_token(token);
        if self.case.is_some() {
            PreprocVerdict::Matched
        } else {
            PreprocVerdict::No
        }
    }
}
