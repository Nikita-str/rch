use crate::api::header_use::*;
use crate::post::Post;

// http://127.0.0.1:5173/api/board/thrs_load?board_url=b&from=0&to=10

pub const REQ_METHOD: Method = Method::GET;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct HandlerParams{
    board_url: String,
    from: usize,
    to: usize,
}

#[derive(Serialize, Debug)]
pub struct ResultOk {
    thrs: Vec<SingleThreadView>,
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
    Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
) -> Json<ResultOk>
{
    crate::delay_ms(1500);

    pub const THR_FIRST_POSTS_QTY: usize = 3;
    pub const MAX_LOAD: usize = 30;

    let from = params.from.min(params.to);
    let to = params.from.max(params.to);
    let to = to.min(from + MAX_LOAD); 

    {
        let r_state = state.read().unwrap();
        let board = r_state.open_boards().board(&params.board_url);

        let mut thrs = Vec::with_capacity(to - from + 1); 
        if let Some(board) = board {
            for thr_n in from..=to {
                let Some(thr) = board.top_thr_by_usage_n(thr_n) else { break };

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
        }
        Json(ResultOk { thrs })
    }
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/thrs_load", routing::get(handler).with_state(Arc::clone(state)))
}