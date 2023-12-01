use crate::api::header_use::*;
use crate::api::fns::create_img_load_info;
use crate::api::MAX_PIC_AMOUNT;
use crate::utility::img::PostImg;
use crate::post::Post;

// ../api/thread/post_new?board_url=b&op_post_n=244&post_text=heheh%20in%20tuturu%20!!!!!!

pub const REQ_METHOD: Method = Method::POST;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct HandlerParams{
    board_url: String,
    op_post_n: u64,
    post_text: String,
    post_imgs: Vec<PostImg>,
}

pub async fn handler(
    State(state): State<HandlerState>,
    Json(mut params): Json<HandlerParams>,
) -> Json<()>
{
    crate::delay_ms(300);
    let board_url = &params.board_url;

    {
        let r_state = state.read().unwrap();
        let Some(board) = r_state.open_boards().board(board_url) else { 
            return Json(()) // ERROR // no board
        };
        if board.thr(params.op_post_n).is_none() {
            return Json(()) // ERROR // no OP
        }
    }

    let (post_text, reply_to) = {
        // TODO: Pool of preproc?!
        let mut preproc = crate::preproc::HeadPreproc::new_by_board(board_url, false);
        let result = preproc.preproc(&params.post_text);
        (result.output, result.reply_to)
    };

    // [+] IMGS
    params.post_imgs.truncate(MAX_PIC_AMOUNT);
    let imgs = create_img_load_info(&state, &params.post_imgs, MAX_PIC_AMOUNT);
    // [-] IMGS
    
    let post = Post::new_anon(post_text, imgs);

    'write_state: {
        let mut w_state = state.write().unwrap();
        let board = w_state.mut_open_boards().board_mut(board_url);
        if let Some(board) = board {
            let Some(new_post_n) = board.add_post(params.op_post_n, post) else { break 'write_state };
            if !reply_to.is_empty() {
                if let Some(thr) = board.thr_mut(params.op_post_n) {
                    thr.add_replies(new_post_n, reply_to)
                }
            }
        }
    }

    Json(())
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/post_new", routing::post(handler).with_state(Arc::clone(state)))
}