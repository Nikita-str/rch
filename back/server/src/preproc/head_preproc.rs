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
    use crate::preproc::general::italic::ItalicPreproc;


    fn help_only_italic(input: &str, expected_output: &str) {
        let mut head_preproc = HeadPreproc::new();
        let italic = ItalicPreproc::default();
        head_preproc.add_preproc(Box::new(italic));
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
}
