use crate::api::header_use::*;
use crate::post::Post;

// http://127.0.0.1:5173/api/board/thr_new?board_url=b&post_header=Post%20Header&post_text=tuturu

pub const REQ_METHOD: Method = Method::POST;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct HandlerParams {
    board_url: String,
    post_header: Option<String>,
    post_text: String,
    post_imgs: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ResultOk {
    thrs: Vec<Vec<Post>>,
}

pub async fn handler(
    // Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
    Json(params): Json<HandlerParams>,
) -> Json<Option<u64>> {
    crate::delay_ms(300);

    let board_url = &params.board_url;

    {
        let r_state = state.read().unwrap();
        if !r_state.open_boards().is_board_exist(board_url) {
            return Json(None) // ERROR
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
    let op_post = Post::new_anon(post_text, None);
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
    use crate::{KB, MB};
    const MAX_PIC_SZ: usize = 2 * MB;
    const MAX_MINI_PIC_SZ: usize = 50 * KB;
    const MAX_PIC_AMOUNT: usize = 4;
    const MAX_ADDITIONAL_SZ: usize = 25 * KB;
    const MAX_TOTAL_SZ: usize = (MAX_PIC_SZ + MAX_MINI_PIC_SZ) * MAX_PIC_AMOUNT + MAX_ADDITIONAL_SZ; 

    Router::new().route(
        "/thr_new",
        routing::post(handler).with_state(Arc::clone(state)),
    ).layer(tower_http::limit::RequestBodyLimitLayer::new(MAX_TOTAL_SZ))
}
