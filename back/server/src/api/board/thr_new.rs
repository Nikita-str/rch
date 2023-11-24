use crate::api::header_use::*;
use crate::post::Post;
use crate::KB;
use crate::utility::img::{PostImg, ImgLoadInfo};

// http://127.0.0.1:5173/api/board/thr_new?board_url=b&post_header=Post%20Header&post_text=tuturu

const MAX_PIC_AMOUNT: usize = 4;

pub const REQ_METHOD: Method = Method::POST;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct HandlerParams {
    board_url: String,
    post_header: Option<String>,
    post_text: String,
    post_imgs: Vec<PostImg>,
}

#[derive(Serialize, Debug)]
pub struct ResultOk {
    thrs: Vec<Vec<Post>>,
}

pub struct ImgProccesed {
    pub bytes: Vec<u8>,
    pub compressed_bytes: Vec<u8>,
    pub f_ty: crate::utility::img::ImgType,
    pub cf_ty: crate::utility::img::ImgType,
}


impl ImgProccesed {
    fn save_prepared(bytes: &Vec<u8>, ty: crate::utility::img::ImgType, dir: &str, n: u64, compress: bool) -> bool {
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
}


pub async fn handler(
    // Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
    Json(mut params): Json<HandlerParams>,
) -> Json<Option<u64>> {
    // static PIC_PATH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    crate::delay_ms(300);

    let board_url = &params.board_url;

    {
        let r_state = state.read().unwrap();
        if !r_state.open_boards().is_board_exist(board_url) {
            return Json(None) // ERROR
        }
    }

    params.post_imgs.truncate(MAX_PIC_AMOUNT);
    let mut valid = Vec::with_capacity(MAX_PIC_AMOUNT);
    // let in_valid = Vec::with_capacity(params.post_imgs.len()); //TODO: for notify about unloaded imgs
    for img in &params.post_imgs {
        if valid.len() == MAX_PIC_AMOUNT { break }

        if img.size_verify() {
            let Some((cf_ty, compressed_bytes)) = crate::utility::img::base_to_img(&img.compressed_file) else { continue };
            let Some((f_ty, bytes)) = crate::utility::img::base_to_img(&img.file) else { continue };
            valid.push((ImgProccesed{
                bytes,
                compressed_bytes,
                f_ty,
                cf_ty,
            }, img.take_valid_name()))
        }
    }

    let (pic_dir, n) = {
        let mut w_state = state.write().unwrap();
        w_state.use_n_pic(valid.len() as u64)
    };

    let mut imgs = Vec::with_capacity(valid.len());
    for (n, (img, name)) in n.zip(valid.into_iter()) {
        if img.save(&pic_dir, n) {
            imgs.push(ImgLoadInfo{
                name,
                n,
                f_ty: img.f_ty.to_char(),
                cf_ty: img.cf_ty.to_char(),
            })
        }
    }

    let post_header = {
        macro_rules! make_valid_s {
            ($s:expr) => {{
                let to = $s.chars().take(crate::thread::MAX_HEADER_LEN).fold(0, |acc, c|acc + c.len_utf8());
                &$s[0..to]   
            }};
        }

        // TODO: Pool of preproc?!
        let mut preproc = crate::preproc::HeadPreproc::new_by_board(board_url, true);
        let header = if let Some(header) = &params.post_header {
            if header.len() > crate::thread::MAX_HEADER_LEN {
                make_valid_s!(header)
            } else {
                header
            }
        } else {
            make_valid_s!(&params.post_text)
        };
        preproc.preproc(&header)
    };

    let post_text = {
        // TODO: Pool of preproc?!
        let mut preproc = crate::preproc::HeadPreproc::new_by_board(board_url, false);
        preproc.preproc(&params.post_text)
    };

    // if params.post_img.is_some() {
    //     todo!("img case")
    // }
    // let op_post = Post::new_anon(post_text, params.post_img);
    let op_post = Post::new_anon(post_text, imgs);
    let infinity = false;


    let n = {
        let mut w_state = state.write().unwrap();
        w_state
            .mut_open_boards()
            .board_mut(board_url)
            .map(|board| board.new_thr_preproced(post_header, op_post, infinity))
    };

    Json(n)
}

pub fn router(state: &HandlerState) -> Router {
    macro_rules! base64_coef { () => { 4 / 3 }; }
    const MAX_ADDITIONAL_SZ: usize = 25 * KB;
    const MAX_TOTAL_SZ: usize = (PostImg::MAX_PIC_SZ + PostImg::MAX_MINI_PIC_SZ) * MAX_PIC_AMOUNT * base64_coef!()  + MAX_ADDITIONAL_SZ; 

    Router::new().route(
        "/thr_new",
        routing::post(handler).with_state(Arc::clone(state)),
    ).layer(tower_http::limit::RequestBodyLimitLayer::new(MAX_TOTAL_SZ))
}
