use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ImgLoadInfo {
    pub name: String,
    /// pic n
    pub n: u64,
    /// orig byte size
    pub byte_sz: u32,
    /// orig width
    pub w: u16,
    /// orig height
    pub h: u16,
    /// file type (see `ImgType::to_char()`)
    pub f_ty: char,
    /// compressed file type (see `ImgType::to_char()`)
    pub cf_ty: char,
}