pub use bold::BoldPreproc as Bold;
pub use italic::ItalicPreproc as Italic;
pub use spoiler::SpoilerPreproc as Spoiler;

mod bold { crate::preproc::generic::define_opcl!{BoldPreproc ["b"; "bold"] "<b>" / "</b>"} }
mod italic { crate::preproc::generic::define_opcl!{ItalicPreproc ["i"; "italic"] "<i>" / "</i>"} }
mod spoiler { crate::preproc::generic::define_opcl!{SpoilerPreproc ["spoiler"] "<span class=\"P-sp\">" / "</span>"} }
