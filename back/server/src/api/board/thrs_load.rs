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
    thrs: Vec<Vec<Post>>,
}

pub async fn handler(
    Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
) -> Json<ResultOk>
{
    crate::delay_ms(300);

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

                let mut first_posts = Vec::with_capacity(THR_FIRST_POSTS_QTY + 1);
                for post_n in 0..=THR_FIRST_POSTS_QTY {
                    let Some(post) = thr.post(post_n) else { break };
                    first_posts.push(post.clone())
                }

                thrs.push(first_posts)
            }
        }
        Json(ResultOk { thrs })
    }
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/thrs_load", routing::get(handler).with_state(Arc::clone(state)))
}