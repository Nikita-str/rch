use serde::Deserialize;
use super::Base64;
use crate::{KB, MB};

#[derive(Deserialize, Debug)]
pub struct PostImg {
    pub file: Base64,
    pub compressed_file: Base64,
    pub name: String,
}

impl PostImg {
    pub const MAX_PIC_SZ: usize = 2 * MB;
    pub const MAX_MINI_PIC_SZ: usize = 50 * KB;

    pub fn size_verify(&self) -> bool {
        if Self::MAX_MINI_PIC_SZ < crate::utility::img::base64_img_sz(&self.compressed_file) {
            return false
        }
        if Self::MAX_PIC_SZ < crate::utility::img::base64_img_sz(&self.file) {
            return false
        }
        return true
    }

    /// **!** name, not a file path
    pub fn take_valid_name(&self) -> String {
        let name = match self.name.rsplit_once('.') {
            Some((left, _)) => left,
            _ => &self.name, 
        };

        const MAX_NAME_LEN: usize = 100;
        name.chars()
            .filter(|c| !c.is_ascii_control())
            .take(MAX_NAME_LEN)
            .collect()
    }
}
