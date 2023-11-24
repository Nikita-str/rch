use base64::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImgType {
    Webp,
    Png,
    Jpg,
}
impl ImgType {
    pub fn base64_seems_like(base64_data: &str) -> Option<Self> {
        const MIN_LEN: usize = 16;
        if base64_data.len() < MIN_LEN {
            return None;
        }

        let mut first_bytes = [0u8; 12]; // 6 * 16 / 8 = 12.0 == 12  =>  len estimate must be correct and there will be no error when call `decode_slice` 
        if let Err(_) = base64::engine::general_purpose::STANDARD.decode_slice(&base64_data[0..MIN_LEN], &mut first_bytes) {
            return None;
        };
        
        // PNG
        const PNG: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        if &first_bytes[0..PNG.len()] == PNG {
            return Some(Self::Png);
        }

        // WEBP
        const WEBP: &[u8] = "RIFF".as_bytes();
        if &first_bytes[0..WEBP.len()] == WEBP {
            let len_bytes = (&first_bytes[WEBP.len()..(WEBP.len() + 4)])
                .try_into()
                .unwrap();
            let len = u32::from_le_bytes(len_bytes);

            const MAX_BYTE_ERR: u32 = 20; // 10 + 2 = 12 ... ~20 ://
            if u32::abs_diff(len, base64::decoded_len_estimate(base64_data.len()) as u32) > MAX_BYTE_ERR {
                return None;
            }

            const WEBP: &[u8] = "WEBP".as_bytes();
            if &first_bytes[8..12] == WEBP {
                return Some(Self::Webp);
            }
        }

        // JPG
        const JPG: &[u8] = &[0xFF, 0xD8, 0xFF,];
        if &first_bytes[0..JPG.len()] == JPG {
            if first_bytes[JPG.len()] >= 0xD0 {
                return Some(Self::Jpg);
            }
        }

        return None;
    }

    #[allow(unused)]
    pub const fn to_format(self) -> &'static str {
        match self {
            ImgType::Webp => "webp",
            ImgType::Png => "png",
            ImgType::Jpg => "jpg",
        }
    }

    #[allow(unused)]
    pub const fn to_char(self) -> char {
        match self {
            ImgType::Webp => 'w',
            ImgType::Png => 'p',
            ImgType::Jpg => 'j',
        }
    }
}
