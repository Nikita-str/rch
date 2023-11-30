use crate::preproc::general::{Bold, Italic, Strike, Spoiler};
use crate::preproc::general::{SubText, SupText};
use crate::preproc::general::{NewLinePreproc as NewLine, ReservedSymbsPreproc as ReservedSymbs};
use crate::preproc::general::Random;
use crate::preproc::{Preproc, PreprocVerdict};
use crate::preproc::inner::PreprocVerdictInfo;

use crate::preproc::board_specific as board;
use board::a::Cat as CatFromA;
use board::a::Nyan as NyanFromA;
use board::a::Kawaii as KawaiiFromA;
use board::rp::StdDices as StdDicesFromRp;

macro_rules! all_gen {
    ($name:ident $type_enum_name:ident [$( $preproc:ident; )*]) => {
        pub enum $name {
            $( $preproc($preproc), )*
        }

        impl $name {
            pub fn preproc_type(&self) -> $type_enum_name {
                match self {
                    $( Self::$preproc(_) => $type_enum_name::$preproc, )*
                }
            }
        }

        impl Preproc for $name {
            fn close(&mut self, output: &mut String, state: ()) {
                match self {
                    $( Self::$preproc(x) => x.close(output, state), )*
                }
            }

            fn reset(&mut self) {
                match self {
                    $( Self::$preproc(x) => x.reset(), )*
                }
            }

            fn action(&mut self, output: &mut String, matched_tokens: &str, state: ()) {
                match self {
                    $( Self::$preproc(x) => x.action(output, matched_tokens, state), )*
                }
            }

            fn state_upd_str(&mut self, token: &str) -> PreprocVerdict {
                match self {
                    $( Self::$preproc(x) => x.state_upd_str(token), )*
                }
            }
            
            fn state_upd_simple_token(&mut self, token: &crate::preproc::tokenizer::SimpleToken) -> PreprocVerdict {
                match self {
                    $( Self::$preproc(x) => x.state_upd_simple_token(token), )*
                }
            }
            
            fn state_upd_multi_token(&mut self, token: &crate::preproc::tokenizer::MultiToken) -> PreprocVerdictInfo {
                match self {
                    $( Self::$preproc(x) => x.state_upd_multi_token(token), )*
                }
            }
        }
        
        #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
        pub enum $type_enum_name {
            $( $preproc, )*
        }
    };
}

all_gen!{
    AllPreproc AllPreprocType [
        Bold; Italic; Strike; Spoiler;
        SupText; SubText;
        NewLine; ReservedSymbs;
        Random;

        // board = `/a/`
        CatFromA; NyanFromA; KawaiiFromA;
        
        // board = `/rp/`
        StdDicesFromRp;
    ]
}

pub enum AllPreprocCtor {
    Bold{ ignore: bool },
    Italic{ ignore: bool },
    Strike{ ignore: bool },
    Spoiler{ ignore: bool },
    
    SupText{ ignore: bool },
    SubText{ ignore: bool },

    NewLine{ space_mode: bool },
    ReservedSymbs,

    Random{ mode: crate::preproc::general::RandomMode },

    CatFromA,
    NyanFromA,
    KawaiiFromA,

    StdDicesFromRp,
}

impl AllPreprocCtor {
    pub fn create(self) -> AllPreproc {
        AllPreproc::new(self)
    }
}


macro_rules! all_preproc_new {
    (IGN $case:ident $ignore:ident) => {
        Self::$case(if $ignore { $case::new_ignore_mode() } else { $case::default() })
    };
}

impl AllPreproc {
    pub fn new(ctor: AllPreprocCtor) -> Self {
        match ctor {
            AllPreprocCtor::Bold { ignore } => all_preproc_new!(IGN Bold ignore),
            AllPreprocCtor::Italic { ignore } => all_preproc_new!(IGN Italic ignore),
            AllPreprocCtor::Strike { ignore } => all_preproc_new!(IGN Strike ignore),
            AllPreprocCtor::Spoiler { ignore } => all_preproc_new!(IGN Spoiler ignore),

            AllPreprocCtor::SupText { ignore } => all_preproc_new!(IGN SupText ignore),
            AllPreprocCtor::SubText { ignore } => all_preproc_new!(IGN SubText ignore),

            AllPreprocCtor::ReservedSymbs => Self::ReservedSymbs(ReservedSymbs::default()),
            AllPreprocCtor::NewLine { space_mode } => {
                let mut new_line = NewLine::default();
                new_line.set_space_mode(space_mode);
                Self::NewLine(new_line)
            }

            AllPreprocCtor::Random { mode } => {
                let mut random = Random::default();
                match mode {
                    super::general::RandomMode::Std => {}
                    super::general::RandomMode::Simple => random.simple_mode_on(),
                    super::general::RandomMode::HeaderClass => random.header_mode_on(),
                };
                Self::Random(random)
            }

            AllPreprocCtor::CatFromA => Self::CatFromA(CatFromA::default()),
            AllPreprocCtor::NyanFromA => Self::NyanFromA(NyanFromA::default()),
            AllPreprocCtor::KawaiiFromA => Self::KawaiiFromA(KawaiiFromA::default()),

            AllPreprocCtor::StdDicesFromRp => Self::StdDicesFromRp(StdDicesFromRp::default()),
        }
    }
}
