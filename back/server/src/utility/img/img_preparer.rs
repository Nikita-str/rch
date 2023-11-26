use super::{PostImg, ImgType, ImgLoadInfo};
use super::base_to_img;

struct ImgProccesed {
    pub bytes: Vec<u8>,
    pub compressed_bytes: Vec<u8>,
    pub f_ty: ImgType,
    pub cf_ty: ImgType,
}

impl ImgProccesed {
    fn save_prepared(bytes: &Vec<u8>, ty: ImgType, dir: &str, n: u64, compress: bool) -> bool {
        use std::io::Write;

        let postfix = if compress { "_c" } else { "" };
        let ext = ty.to_format();
        let path = format!("{dir}/{n}{postfix}.{ext}");
        
        let Ok(mut f) = std::fs::File::create(path) else { return false };
        let Ok(_) = f.write_all(&bytes) else { return false };
        true
    }

    #[must_use]
    pub fn save(&self, dir: &str, n: u64) -> bool {
        Self::save_prepared(&self.compressed_bytes, self.cf_ty, dir, n, true)
        && Self::save_prepared(&self.bytes, self.f_ty, dir, n, false)
    }

    fn try_by_post_img(img: &PostImg)  -> Option<ImgProccesed> {
        if !img.size_verify() { return None }

        let Some((cf_ty, compressed_bytes)) = base_to_img(&img.compressed_file) else { return None };
        let Some((f_ty, bytes)) = base_to_img(&img.file) else { return None };
        return Some(ImgProccesed{
            bytes,
            compressed_bytes,
            f_ty,
            cf_ty,
        })
    }
}

pub struct ImgsPreparerSealed {
    valid: Vec<(ImgProccesed, /* name: */String)>,
}
impl ImgsPreparerSealed {
    /// # why `max_imgs_n`
    /// used for case when user can pass more pics than the user can post\
    /// otherwise we can change `max_imgs_n` to `imgs.len()`
    pub fn new_by_imgs(imgs: &Vec<PostImg>, max_imgs_n: usize) -> Self {
        let mut valid = Vec::with_capacity(max_imgs_n); 
        // let in_valid = Vec::with_capacity(params.post_imgs.len()); //TODO: for notify about unloaded imgs
        for img in imgs {
            if valid.len() == max_imgs_n { break }
            let Some(img_p) = ImgProccesed::try_by_post_img(img) else { continue };
            valid.push((img_p, img.take_valid_name()))
        }
        
        return Self{ valid }
    }

    /// return how many pic slots we need
    pub fn n_pics(&self) -> u64 {
        self.valid.len() as u64
    }

    pub fn to_img_load_info(self, pic_dir: &str, pic_n: std::ops::Range<u64>) -> Vec<ImgLoadInfo> {
        let valid = self.valid;
        let mut imgs = Vec::with_capacity(valid.len());
        for (n, (img, name)) in pic_n.zip(valid.into_iter()) {
            if img.save(&pic_dir, n) {
                imgs.push(ImgLoadInfo{
                    name,
                    n,
                    f_ty: img.f_ty.to_char(),
                    cf_ty: img.cf_ty.to_char(),
                })
            }
        }
        
        imgs
    }
}
