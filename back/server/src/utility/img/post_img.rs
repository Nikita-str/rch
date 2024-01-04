use serde::Deserialize;
use super::Base64;
use crate::config::Config;

#[derive(Deserialize, Debug)]
pub struct PostImg {
    pub file: Base64,
    pub compressed_file: Base64,
    pub name: String,
    pub orig_w: u16,
    pub orig_h: u16,
    pub spoiler: bool,
}

impl PostImg {
    pub fn size_verify(&self) -> bool {
        let max_sz = Config::max_mini_pic_sz();
        if max_sz < crate::utility::img::base64_img_sz(&self.compressed_file) {
            return false
        }
        let max_sz = Config::max_pic_sz();
        if max_sz < crate::utility::img::base64_img_sz(&self.file) {
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
