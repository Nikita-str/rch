use std::collections::HashSet;
use super::{Span, Preproc, PreprocVerdict, Tokenizer};
use crate::preproc::all_available_preproc::*;

pub enum StdHeadPreprocType {
    Std,
    Header,
}

// pub struct HeadPreproc {
//     preprocers: Vec<Box<dyn Preproc>>, // OR `Vec<enum AllPreproc>`,
// }
pub struct HeadPreproc {
    preprocers: Vec<AllPreproc>,
    used: HashSet<AllPreprocType>,
}

impl HeadPreproc {
    pub fn new() -> Self {
        Self {
            preprocers: Vec::new(),
            used: HashSet::new(),
        }
    }

    pub fn new_std(std_ty: StdHeadPreprocType) -> Self {
        use crate::preproc::general::RandomMode as RandomMode;

        let mut head_preproc = Self::new();

        let (ignore, space_mode, random_mode) = match std_ty {
            StdHeadPreprocType::Std => (false, false, RandomMode::Std),
            StdHeadPreprocType::Header => (true, true, RandomMode::HeaderClass),
        };

        head_preproc.add_preproc(AllPreprocCtor::Bold { ignore });
        head_preproc.add_preproc(AllPreprocCtor::Italic { ignore });
        head_preproc.add_preproc(AllPreprocCtor::Strike { ignore });
        head_preproc.add_preproc(AllPreprocCtor::Spoiler { ignore });

        head_preproc.add_preproc(AllPreprocCtor::SupText { ignore });
        head_preproc.add_preproc(AllPreprocCtor::SubText { ignore });

        head_preproc.add_preproc(AllPreprocCtor::NewLine { space_mode });
        head_preproc.add_preproc(AllPreprocCtor::ReservedSymbs);

        head_preproc.add_preproc(AllPreprocCtor::Random { mode: random_mode });

        head_preproc
    }

    pub fn new_by_board(board: &str, header: bool) -> Self {
        match (board, header) {
            ("a", false) => {
                let mut head_preproc = Self::new_std(StdHeadPreprocType::Std);
                head_preproc.add_preproc(AllPreprocCtor::CatFromA);
                head_preproc.add_preproc(AllPreprocCtor::NyanFromA);
                head_preproc.add_preproc(AllPreprocCtor::KawaiiFromA);
                head_preproc
            }
            (_, false) => Self::new_std(StdHeadPreprocType::Std),
            (_, true) => Self::new_std(StdHeadPreprocType::Header),
        }
    }

    pub fn add_preproc_direct(&mut self, preproc: AllPreproc) {
        let ty = preproc.preproc_type();
        if !self.used.insert(ty) {
            println!("[ALGO ERROR]: try add already added preproc type({:?})", ty)
        } else {
            self.preprocers.push(preproc);
        }
    }
    
    pub fn add_preproc(&mut self, preproc_ctor: AllPreprocCtor) {
        self.add_preproc_direct(preproc_ctor.create()) 
    }


    fn init_output(input: &str) -> String {
        const OUT_COEF_99_PROC_CASE: f64 = 1.2;
        const OUT_ADDITIONAL_BYTES: usize = 300;
        let output_exp_capacity = ((input.len() as f64 * OUT_COEF_99_PROC_CASE) as usize) + OUT_ADDITIONAL_BYTES;

        String::with_capacity(output_exp_capacity)
    }

    pub fn preproc(&mut self, input: &str) -> String {
        if self.preprocers.is_empty() {
            println!("[ALGO WARN]: no preprocessors");
            return String::from(input)
        }
        
        let state = ();
        let mut output = Self::init_output(input); 
        let mut tokenizer = Tokenizer::new(input);
        
        let mut unwrited_span = Span::new_empty(0);
        let mut prev_is_maybe = HashSet::new();
        loop {
            let token = tokenizer.next_token();
            let (token, token_span) = (token.token, token.span);
            
            if token.is_empty() {
                assert!(tokenizer.is_ended());
                break;
            }

            let mut maybe_once = false;
            let mut matched = false;
            let mut cur_token_in_use = false;
            for (iprep, preproc) in self.preprocers.iter_mut().enumerate() {
                macro_rules! on_matched {
                    () => {{
                        matched = true;
                        // bad bug fix ... but for ours cases +- k
                        // (in good case we must save pos of start for each preproc...)
                        // ((and allow them work +- independent))
                        if !prev_is_maybe.contains(&iprep) {
                            output.push_str(unwrited_span.extract_str(input));
                            unwrited_span = Span::new_empty(unwrited_span.end());
                        }
                        let matched_tokens = Span::new_union(unwrited_span, token_span).extract_str(input);
                        preproc.action(&mut output, matched_tokens, state);
                        preproc.reset();    
                    }};
                }

                match preproc.state_upd(token) {
                    PreprocVerdict::No => {
                        preproc.reset();
                        if prev_is_maybe.remove(&iprep) {
                            match preproc.state_upd(token) {
                                PreprocVerdict::No => {}
                                PreprocVerdict::Maybe => {
                                    cur_token_in_use = true;
                                    prev_is_maybe.insert(iprep);
                                }
                                PreprocVerdict::Matched => {
                                    // write prev unwrited
                                    on_matched!()
                                }
                            }
                        }
                    }
                    PreprocVerdict::Maybe => {
                        maybe_once = true;
                        prev_is_maybe.insert(iprep);
                    }
                    PreprocVerdict::Matched => on_matched!(),
                }
            }

            if matched {
                unwrited_span = Span::new_empty(token_span.end());
                prev_is_maybe.clear();
            } else if maybe_once /* !prev_is_maybe.is_empty() */ {
                unwrited_span.union(token_span);
            } else {
                if !cur_token_in_use { unwrited_span.union(token_span); }
                output.push_str(unwrited_span.extract_str(input));
                unwrited_span = Span::new_empty(token_span.end());
                if cur_token_in_use { unwrited_span.union(token_span); }
            }
        }
        
        output.push_str(unwrited_span.extract_str(input));
        self.preprocers.iter_mut().for_each(|preproc|preproc.close(&mut output, state));

        output
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn help_only_italic(input: &str, expected_output: &str) {
        let mut head_preproc = HeadPreproc::new();
        head_preproc.add_preproc(AllPreprocCtor::Italic { ignore: false });
        let output = head_preproc.preproc(input);
        assert_eq!(output, expected_output);
    }

    fn help_all(input: &str, expected_output: &str) {
        let mut head_preproc = HeadPreproc::new_std(StdHeadPreprocType::Std);
        let output = head_preproc.preproc(input);
        assert_eq!(output, expected_output);
    }

    fn help_header(input: &str, expected_output: &str) {
        let mut head_preproc = HeadPreproc::new_std(StdHeadPreprocType::Header);
        let output = head_preproc.preproc(input);
        assert_eq!(output, expected_output);
    }


    #[test]
    fn test_preproc_01_simple_italic_only() {
        let input = "test [i]italic[i] and still[/i] italic[/i]... completed";
        let expected_output = "test <i>italic and still italic</i>... completed";
        help_only_italic(input, expected_output);
    }

    #[test]
    fn test_preproc_02_hard_italic_only() {
        let input = "[[i][[/i][";
        let expected_output = "[<i>[</i>[";
        help_only_italic(input, expected_output);
    }
      
    #[test]
    fn test_preproc_03_hardv2_italic_only() {
        let input = "[[[[[[i]";
        let expected_output = "[[[[[<i></i>";
        help_only_italic(input, expected_output);

        let input = "[[[[i][[[[/i][[[";
        let expected_output = "[[[<i>[[[</i>[[[";
        help_only_italic(input, expected_output);
    }
    
    #[test]
    fn test_preproc_04_unclose_italic_only() {
        let input = "aa[i]bb";
        let expected_output = "aa<i>bb</i>";
        help_only_italic(input, expected_output);
        
        let input = "aa[i]bb[i]cc[i]";
        let expected_output = "aa<i>bbcc</i>";
        help_only_italic(input, expected_output);

        let input = "aa[i]bb[/i]cc[i]";
        let expected_output = "aa<i>bb</i>cc<i></i>";
        help_only_italic(input, expected_output);
        
        let input = "aa[i]bb[i]cc[/i]dd[i]";
        let expected_output = "aa<i>bbccdd</i>";
        help_only_italic(input, expected_output);
        
        let input = "aa[i]bb[/i]cc[i]dd[i]ee[i]ff[/i]gg[/i]XX[/i]11[i]YY[i]ZZ[/i]WW";
        let expected_output = "aa<i>bb</i>cc<i>ddeeffggXX</i>11<i>YYZZWW</i>";
        help_only_italic(input, expected_output);
    }
    
    #[test]
    fn test_preproc_05_unclose_hard_italic_only() {
        let input = "aa[i]bb[/i]cc[i]dd[i]ee[i]ff[/i]gg[/i][[[/i]11[i][[[i]]][/i][[";
        let expected_output = "aa<i>bb</i>cc<i>ddeeffgg[[</i>11<i>[[]][[</i>";
        help_only_italic(input, expected_output);
    }

    #[test]
    fn test_preproc_06_simple_spoiler_itelic() {
        let input = "а не подскажите [spoiler]ли вы[/spoiler] почему на [i]имеджеборде [/i]так интелегентно разговаривают?!";
        let expected_output = "а не подскажите <span class=\"P-sp\">ли вы</span> почему на <i>имеджеборде </i>так интелегентно разговаривают?!";
        help_all(input, expected_output);
    }

    #[test]
    fn test_preproc_07_single_unclose() {
        let input = "Y[/strike] x [i]42 24[/i] 33[i]33 [s]321 [/s] 123";
        let expected_output = "Y x <i>42 24</i> 33<i>33 <s>321 </s> 123</i>";
        help_all(input, expected_output);
    }
    
    #[test]
    fn test_preproc_08_simple_spoiler_itelic() {
        // the orde on the end can be (in general case) another
        // but because we know the orden in which we add -- we can 
        let input = "Y x [i]42 24 33[i]33 [s]32[/i]1 [spoiler] 123";
        let expected_output = "Y x <i>42 24 3333 <s>321 <span class=\"P-sp\"> 123</i></s></span>";
        help_all(input, expected_output);
    }
    
    #[test]
    fn test_preproc_09_one_token_preprocs() {
        let input = "Y <x> & ! #\nhmm...";
        let expected_output = "Y &#60;x&#62; &#38; ! #<br/>hmm...";
        help_all(input, expected_output);
    }
    
    #[test]
    fn test_preproc_09v2() {
        let input = "xx[<a]";
        let expected_output = "xx[&#60;a]";
        help_all(input, expected_output);
    }

    #[test]
    fn test_preproc_10_one_token_preprocs() { 
        let input = "br[s]rr & [i]! #\n\nhm[/strike]m...";
        let expected_output = "br<s>rr &#38; <i>! #<br/><br/>hm</s>m...</i>";
        help_all(input, expected_output);
    }
    
    #[test]
    fn test_preproc_11_sup_sub_and_others() { 
        let input = "br[s]rr &[/sub] [i]![i]![sup]![sup]![i]! #\n\nhm[/strike]m...";
        let expected_output = "br<s>rr &#38; <i>!!<sup>!<sup>!! #<br/><br/>hm</s>m...</i></sup></sup>";
        help_all(input, expected_output);
    }

    #[test]
    fn test_preproc_12_rand() {
        let assert_rand = |output: &str, from: i32, to: i32| {
            let prefix = format!("<span class=\"P-rand\" title=\"{from} ≤ x ≤ {to}\">");
            assert!(output.starts_with(&prefix));
            let postfix = "</span>";
            assert!(output.ends_with(postfix));
            let number = &output[prefix.len()..(output.len() - postfix.len())];
            let x = number.parse().unwrap();
            assert!(from <= x && x <= to);
        };

        let mut head_preproc = HeadPreproc::new();
        
        let mode = crate::preproc::general::RandomMode::Std;
        head_preproc.add_preproc(AllPreprocCtor::Random { mode });

        for i in 0..100 {
            let input = if i % 2 == 0 { 
                "[random( -42,  50  )]" 
            } else {
                "[random(50, -42)]" 
            };
            let output = head_preproc.preproc(input);
            assert_rand(&output, -42, 50);
        }
        
        for _ in 0..10 {
            let input = "[random( -1245,  -1245  )]";
            let output = head_preproc.preproc(input);
            assert_rand(&output, -1245, -1245);
        }
        
        for _ in 0..10 {
            let input = "[random(-103:-101)]";
            let output = head_preproc.preproc(input);
            assert_rand(&output, -103, -101);
        }

        for i in 0..10 {
            let from = 4200 + i;
            let to = 4200 + i * 2;
            let delim = if i % 2 == 0 { ":" } else { ", " };
            let input = format!("[random({from}{delim}{to})]");
            let output = head_preproc.preproc(&input);
            assert_rand(&output, from, to);
        }
    }

    
    #[test]
    fn test_preproc_13_all_header() { 
        let input = "br[s]rr &[/sub] [i]![i]![sup]![random(11:11)][sup]![i]! #\n\nhm[/strike]m...";
        let expected_output = "brrr &#38; !!!<span class=\"H-rand\" title=\"11 ≤ x ≤ 11\">11</span>!! #  hmm...";
        help_header(input, expected_output);
    }
    
    #[test]
    fn test_preproc_14_board_a() { 
        let input = "какие ~~сладкие~~ Котики!!!";
        let expected_output = "какие <span class=\"P-a-nyan\">~~сладкие~~</span> <span class=\"P-a-cat\">:3</span>и!!!";
        let mut head_preproc = HeadPreproc::new_by_board("a", false);
        let output = head_preproc.preproc(input);
        assert_eq!(output, expected_output);
    }
}
