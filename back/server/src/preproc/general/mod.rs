pub use bold::BoldPreproc as Bold;
pub use italic::ItalicPreproc as Italic;
pub use strike::StrikePreproc as Strike;
pub use spoiler::SpoilerPreproc as Spoiler;

mod bold { crate::preproc::generic::define_opcl!{BoldPreproc ["b"; "bold"] "<b>" / "</b>"} }
mod italic { crate::preproc::generic::define_opcl!{ItalicPreproc ["i"; "italic"] "<i>" / "</i>"} }
mod strike { crate::preproc::generic::define_opcl!{StrikePreproc ["s"; "strike"] "<s>" / "</s>"} }
mod spoiler { crate::preproc::generic::define_opcl!{SpoilerPreproc ["spoiler"] "<span class=\"P-sp\">" / "</span>"} }

mod new_line;
pub use new_line::NewLinePreproc;

mod reserved_symbs;
pub use reserved_symbs::ReservedSymbsPreproc;