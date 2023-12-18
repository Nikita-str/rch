use serde::{Serialize, Deserialize};
use super::ImgType;

#[derive(Serialize, Deserialize, Clone, Debug)]
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
            f_ty: ImgType::from_char(self.f_ty),
            cf_ty: ImgType::from_char(self.cf_ty),
        }
    }
}

pub struct ImgDelInfo {
    pub n: u64,
    pub f_ty: Option<ImgType>, // here must be always valid... but.. k
    pub cf_ty: Option<ImgType>,
}

impl ImgDelInfo {
    pub fn del(self, path: &str) {
        let n = self.n;
        if let Some(f_ty) = self.f_ty {
            let f_path = format!("{path}/{}.{}", n, f_ty.to_format());
            let _ = std::fs::remove_file(f_path);
        }
        if let Some(cf_ty) = self.cf_ty {
            let cf_path = format!("{path}/{}_c.{}", n, cf_ty.to_format());
            let _ = std::fs::remove_file(cf_path);
        }
    }
}