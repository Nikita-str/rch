use super::{PostImg, ImgType, ImgLoadInfo};
use super::base_to_img;

struct SingleImgProc {
    pub bytes: Vec<u8>,
    pub ty: ImgType,
}
impl SingleImgProc {
    fn new(bytes: Vec<u8>, ty: ImgType) -> Self {
        Self { bytes, ty }
    }
}

struct ImgProccesed {
    pub img: SingleImgProc,
    pub compressed_img: Option<SingleImgProc>,
}

impl ImgProccesed {
    fn save_prepared(img: &SingleImgProc, dir: &str, n: u64, compress: bool) -> bool {
        use std::io::Write;

        let bytes = &img.bytes;
        let ty = img.ty;

        let postfix = if compress { "_c" } else { "" };
        let ext = ty.to_format();
        let path = format!("{dir}/{n}{postfix}.{ext}");
        
        let Ok(mut f) = std::fs::File::create(path) else { return false };
        let Ok(_) = f.write_all(&bytes) else { return false };
        true
    }

    #[must_use]
    pub fn save(&self, dir: &str, n: u64) -> bool {
        let compressed_save = self.compressed_img.as_ref().map_or(true, |img|Self::save_prepared(&img, dir, n, true));
        compressed_save && Self::save_prepared(&self.img, dir, n, false)
    }

    fn try_by_post_img(img: &PostImg)  -> Option<ImgProccesed> {
        if !img.size_verify() { return None }

        let compressed_img = if img.spoiler { None } else {
            let Some((ty, bytes)) = base_to_img(&img.compressed_file) else { return None };
            Some(SingleImgProc::new(bytes, ty))
        };

        let Some((ty, bytes)) = base_to_img(&img.file) else { return None };
        let img = SingleImgProc::new(bytes, ty);

        return Some(ImgProccesed{
            img,
            compressed_img,
        })
    }

    fn byte_sz(&self) -> u32 {
        self.img.bytes.len() as u32
    }
}


struct SinglePreparedImg {
    img_p: ImgProccesed,
    name: String,
    w: u16,
    h: u16,
}
impl SinglePreparedImg {
    fn new(img_p: ImgProccesed, img: &PostImg) -> Self {
        Self {
            img_p,
            name: img.take_valid_name(),
            w: img.orig_w,
            h: img.orig_h,
        }
    }
    fn save(&self, dir: &str, n: u64) -> bool {
        self.img_p.save(dir, n)
    }
}

pub struct ImgsPreparerSealed {
    valid: Vec<SinglePreparedImg>,
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
            valid.push(SinglePreparedImg::new(img_p, &img))
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
        for (n, img) in pic_n.zip(valid.into_iter()) {
            if img.save(&pic_dir, n) {
                imgs.push(ImgLoadInfo{
                    name: img.name,
                    n,
                    w: img.w,
                    h: img.h,
                    byte_sz: img.img_p.byte_sz(),
                    f_ty: img.img_p.img.ty.to_char(),
                    cf_ty: img.img_p.compressed_img.map_or('#', |x|x.ty.to_char()),
                })
            }
        }
        
        imgs
    }
}
