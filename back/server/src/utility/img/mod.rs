mod img_type;
mod post_img;
mod img_load_info;
mod img_preparer;

pub use img_type::ImgType;
pub use post_img::PostImg;
pub use img_load_info::ImgLoadInfo;
pub use img_load_info::ImgDelInfo;
pub use img_preparer::ImgsPreparerSealed;

type Base64 = String;

pub fn base_to_img(base64_data: &str) -> Option<(ImgType, Vec<u8>)> {
    use base64::prelude::*;

    let img_ty = ImgType::base64_seems_like(base64_data)?;
    let mut bytes = Vec::with_capacity(base64::decoded_len_estimate(base64_data.len()));
    let r = base64::engine::general_purpose::STANDARD.decode_vec(base64_data, &mut bytes);
    if r.is_err() { return None }
    
    Some((img_ty, bytes))
}

#[inline]
pub fn base64_img_sz(base64_data: &str) -> usize {
    base64::decoded_len_estimate(base64_data.len())
}