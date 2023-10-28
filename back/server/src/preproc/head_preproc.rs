use std::collections::HashSet;

use super::{Span, Preproc, PreprocVerdict, Tokenizer};

pub struct HeadPreproc {
    preprocers: Vec<Box<dyn Preproc>>, // OR `Vec<enum AllPreproc>`,
}

impl HeadPreproc {
    pub fn new() -> Self {
        Self {
            preprocers: Vec::new(),
        }
    }

    pub(in crate::preproc)
    fn add_preproc(&mut self, preproc: Box<dyn Preproc>) {
        self.preprocers.push(preproc);
    }

    fn init_output(input: &str) -> String {
        const OUT_COEF_99_PROC_CASE: f64 = 1.2;
        const OUT_ADDITIONAL_BYTES: usize = 128;
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
                        // TODO: pass full_match into `action` by `(unwrited_span U span).extract_str(input)`
                        preproc.action(&mut output, state);
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
                                PreprocVerdict::Matched => on_matched!(),
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
    use crate::preproc::general::{Bold, Italic, Strike, Spoiler};
    use crate::preproc::general::{SubText, SupText};
    use crate::preproc::general::{NewLinePreproc, ReservedSymbsPreproc};
    use crate::preproc::general::{Random};


    fn help_only_italic(input: &str, expected_output: &str) {
        let mut head_preproc = HeadPreproc::new();
        let italic = Italic::default();
        head_preproc.add_preproc(Box::new(italic));
        let output = head_preproc.preproc(input);

        assert_eq!(output, expected_output);
    }

    fn help_all(input: &str, expected_output: &str) {
        let mut head_preproc = HeadPreproc::new();

        let bold = Bold::default();
        let italic = Italic::default();
        let strike = Strike::default();
        let spoiler = Spoiler::default();
        head_preproc.add_preproc(Box::new(bold));
        head_preproc.add_preproc(Box::new(italic));
        head_preproc.add_preproc(Box::new(strike));
        head_preproc.add_preproc(Box::new(spoiler));

        let sup = SupText::default();
        let sub = SubText::default();
        head_preproc.add_preproc(Box::new(sup));
        head_preproc.add_preproc(Box::new(sub));

        let new_line = NewLinePreproc::default();
        let reserved = ReservedSymbsPreproc::default();
        head_preproc.add_preproc(Box::new(new_line));
        head_preproc.add_preproc(Box::new(reserved));

        // is non-necessary
        let random = Random::default();
        head_preproc.add_preproc(Box::new(random));

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
        let mut head_preproc = HeadPreproc::new();
        
        let random = Random::default();
        head_preproc.add_preproc(Box::new(random));

        for _ in 0..100 {
            let input = "[random( -42,  50  )]";
            let output = head_preproc.preproc(input);
            let x = output.parse().unwrap();
            assert!(-42 <= x && x <= 50);
        }
        
        for _ in 0..10 {
            let input = "[random( -1245,  -1245  )]";
            let output = head_preproc.preproc(input);
            let x: i32 = output.parse().unwrap();
            assert_eq!(x, -1245);
        }
        
        for _ in 0..10 {
            let input = "[random(-103:-101)]";
            let output = head_preproc.preproc(input);
            let x: i32 = output.parse().unwrap();
            assert!(x == -103 || x == -102 || x == -101);
        }

        for i in 0..10 {
            let from = 4200 + i;
            let to = 4200 + i * 2;
            let delim = if i % 2 == 0 { ":" } else { ", " };
            let input = format!("[random({from}{delim}{to})]");
            let output = head_preproc.preproc(&input);
            let x: i32 = output.parse().unwrap();
            assert!(from <= x && x <= to);
        }
    }
}
