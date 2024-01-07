use crate::api::header_use::*;
use crate::api::fns::create_img_load_info;
use crate::api::max_pic_qty;
use crate::utility::img::PostImg;
use crate::post::Post;
use crate::KB;

// http://127.0.0.1:5173/api/board/thr_new?board_url=b&post_header=Post%20Header&post_text=tuturu

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
        let Some(_) = r_state.open_boards().board_id(board_url) else {
            return Json(None) // ERROR
        };
    }

    // [+] IMGS
    params.post_imgs.truncate(max_pic_qty());
    let imgs = create_img_load_info(&state, board_url, &params.post_imgs, max_pic_qty());
    // [-] IMGS

    // [+] HEADER
    let post_header = {
        macro_rules! make_valid_s {
            ($s:expr) => {{
                let to = $s.chars().take(crate::thread::max_header_len()).fold(0, |acc, c|acc + c.len_utf8());
                &$s[0..to]   
            }};
        }

        // TODO: Pool of preproc?!
        let mut preproc = crate::preproc::HeadPreproc::new_by_board(board_url, true);
        let header = if let Some(header) = &params.post_header {
            if header.len() > crate::thread::max_header_len() {
                make_valid_s!(header)
            } else {
                header
            }
        } else {
            make_valid_s!(&params.post_text)
        };
        preproc.preproc(header).output
    };
    // [-] HEADER

    let post_text = {
        // TODO: Pool of preproc?!
        let mut preproc = crate::preproc::HeadPreproc::new_by_board(board_url, false);
        preproc.preproc(&params.post_text).output
    };

    // if params.post_img.is_some() {
    //     todo!("img case")
    // }
    // let op_post = Post::new_anon(post_text, params.post_img);
    let op_post = Post::new_anon(post_text, imgs);
    let infinity = false;


    let n = {
        let mut w_state = state.write().unwrap();
        let post_n = w_state
            .mut_open_boards()
            .board_mut(board_url)
            .map(|board| board.new_thr_preproced(post_header, op_post, infinity));
        
        if post_n.is_some() {
            w_state.inc_post();
        }

        post_n
    };

    Json(n)
}

pub fn max_total_sz() -> usize {
    let base64_coef = |x| x * 4 / 3;
    const MAX_ADDITIONAL_SZ: usize = 25 * KB;

    let max_pic_sz = crate::config::Config::max_pic_sz();
    let max_mnini_pic_sz = crate::config::Config::max_mini_pic_sz();
    base64_coef((max_pic_sz + max_mnini_pic_sz) * max_pic_qty())  + MAX_ADDITIONAL_SZ
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route(
        "/thr_new",
        routing::post(handler).with_state(Arc::clone(state)),
    )
    .layer(axum::extract::DefaultBodyLimit::disable())
    .layer(tower_http::limit::RequestBodyLimitLayer::new(max_total_sz()))
}
