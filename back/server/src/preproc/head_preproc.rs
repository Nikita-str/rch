use super::{Span, Preproc, PreprocVerdict, Tokenizer};

pub struct HeadPreproc {
    preprocers: Vec<Box<dyn Preproc>>, // OR `Vec<enum AllPreproc>`,
}

impl HeadPreproc {
    pub fn new(input: &str) -> Self {
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
        loop {
            let token = tokenizer.next_token();
            let (token, token_span) = (token.token, token.span);
            
            if token.is_empty() {
                assert!(tokenizer.is_ended());
                break;
            }

            let mut maybe_once = false;
            let mut matched = false;
            for preproc in &mut self.preprocers {
                let mut repeat_once = false;
                'try_token_as_first_token: loop {
                    match preproc.state_upd(token) {
                        PreprocVerdict::No => {
                            preproc.reset();
                            if !repeat_once {
                                repeat_once = true;
                                continue 'try_token_as_first_token
                            }
                        }
                        PreprocVerdict::Maybe => maybe_once = true,
                        PreprocVerdict::Matched => {
                            matched = true;
                            preproc.action(&mut output, state);
                            preproc.reset();
                        }
                    }
                    break 'try_token_as_first_token
                }
            }

            if matched {
                unwrited_span = Span::new_empty(token_span.end());
            } else if maybe_once {
                unwrited_span.union(token_span);
            } else {
                output.push_str(unwrited_span.extract_str(input));
                unwrited_span = Span::new_empty(token_span.end());
            }
        }
        
        output.push_str(unwrited_span.extract_str(input));
        self.preprocers.iter_mut().for_each(|preproc|preproc.close(&mut output, state));

        output
    }
}

