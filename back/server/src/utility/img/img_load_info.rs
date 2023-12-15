use serde::Serialize;
use super::ImgType;

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
impl ImgLoadInfo {
    pub fn to_del_info(&self) -> ImgDelInfo {
        ImgDelInfo {
            n: self.n,
            f_ty: ImgType::from_char(self.f_ty).unwrap(),
            cf_ty: ImgType::from_char(self.cf_ty).unwrap(),
        }
    }
}

pub struct ImgDelInfo {
    pub n: u64,
    pub f_ty: ImgType,
    pub cf_ty: ImgType,
}