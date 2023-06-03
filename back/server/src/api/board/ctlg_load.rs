use crate::{api::header_use::*};
use std::collections::{/* HashMap, */ HashSet};
use crate::post::Post;
use crate::thread::Thread;

pub const REQ_METHOD: Method = Method::POST;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct QueryParams{
    board_url: String,
}
#[derive(Deserialize, Debug)]
pub struct BodyParams{
    known_n: HashSet<u64>,
}

#[derive(Serialize, Debug)]
pub struct ResultOk {
    // new_thrs: HashMap<u64, SingleThreadView>,
    new_thrs: Vec<SingleThreadView>,
    removed_thrs: Vec<u64>,
    rate_thrs: Vec<u64>,
}

#[derive(Serialize, Debug)]
struct SingleThreadView {
    open_post: Post,
    posts_qty: usize,
    // TODO: hidden_imgs_qty
    header: String,
}


pub async fn handler(
    Query(q_params): Query<QueryParams>,
    State(state): State<HandlerState>,
    Json(body_params): Json<BodyParams>,
) -> Result<Json<ResultOk>, ()> {
    crate::delay_ms(1_000);

    let board_url = q_params.board_url;
    let known_n = body_params.known_n;

    {
        let r_state = state.read().unwrap();
        let Some(board) = r_state.open_boards().board(&board_url) else { return Err(()) };
        
        let removed = |op_post_n: &&u64|!board.is_thr_exist(**op_post_n);
        let removed_thrs = known_n.iter().filter(removed).map(|x|*x).collect();
        
        let only_new = |thr: &&Thread|!known_n.contains(&thr.op_n().into());
        let new_thrs = board.thr_iter().filter(only_new).map(|thr|{
            // let n: u64 = thr.op_n().into();
            let single_thr = SingleThreadView {
                open_post: thr.open_post().clone(),
                posts_qty: thr.post_qty(),
                header: thr.header().into(),
            };
            single_thr // (n, single_thr)
        }).collect();
        
        let rate_thrs = board.thrs_rate();
        
        Ok(Json(ResultOk{
            new_thrs,
            removed_thrs,
            rate_thrs,
        }))
    }
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/ctlg_load", routing::post(handler).with_state(Arc::clone(state)))
}