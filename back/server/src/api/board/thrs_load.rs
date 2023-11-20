use crate::api::header_use::*;
use crate::post::Post;
use std::collections::HashSet;

// http://127.0.0.1:5173/api/board/thrs_load?board_url=b&from=0&to=10

pub const REQ_METHOD: Method = Method::POST;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct HandlerParams{
    known_n: HashSet<u64>,
    board_url: String,
    load_n: usize,
}

#[derive(Serialize, Debug)]
pub struct ResultOk {
    thrs: Vec<SingleThreadView>,
    all_loaded: bool,
}

#[derive(Serialize, Debug)]
struct SingleThreadView {
    /// inner form: `[OP, last - n, last - (n - 1), ..., last - 1, last]` 
    posts: Vec<Post>,
    posts_qty: usize,
    // TODO: hidden_imgs_qty
    header: String,
}

pub async fn handler(
    State(state): State<HandlerState>,
    Json(params): Json<HandlerParams>,
) -> Json<ResultOk>
{
    crate::delay_ms(1500);
    // println!("TODO:DEL:thrs_load:[{params:?}]");

    pub const THR_FIRST_POSTS_QTY: usize = 3;
    pub const MIN_LOAD: usize = 5;
    pub const MAX_LOAD: usize = 30;

    let load_n = params.load_n.max(MIN_LOAD).min(MAX_LOAD);
    let mut all_loaded = false;

    {
        let r_state = state.read().unwrap();
        let board = r_state.open_boards().board(&params.board_url);

        let mut thrs = Vec::with_capacity(load_n + 1);
        if let Some(board) = board {
            for thr in board.top_k_thrs_by_usage(load_n, &params.known_n) {

                let mut posts = Vec::with_capacity(THR_FIRST_POSTS_QTY + 1);
                let posts_qty = thr.post_qty();

                if let Some(op_post) = thr.post(0) {
                    posts.push(op_post.clone())
                } else {
                    println!("WARN: ALGO ERROR: THREAD WO OP")
                }

                let from = if posts_qty > THR_FIRST_POSTS_QTY { posts_qty - THR_FIRST_POSTS_QTY } else { 1 };
                thr.posts(from, THR_FIRST_POSTS_QTY)
                    .into_iter()
                    .for_each(|x|posts.push(x.clone()));
                
                thrs.push(SingleThreadView{
                    posts,
                    posts_qty,
                    header: thr.header().into(),
                })
            }

            all_loaded = (params.known_n.len() + thrs.len()) >= board.thrs_qty();
        }

        Json(ResultOk {
            thrs,
            all_loaded,
        })
    }
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/thrs_load", routing::post(handler).with_state(Arc::clone(state)))
}