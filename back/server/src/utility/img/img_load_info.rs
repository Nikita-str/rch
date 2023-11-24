use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ImgLoadInfo {
    pub name: String,
    pub n: u64,
    pub f_ty: char,
    pub cf_ty: char,
}