use crate::security::Action;
use crate::api::header_use::*;
use super::State as StateX;

use crate::some_or_ret;

use super::error_type::E;
use super::del_post::DelPostE;
type Error = ApiError<DelPostE>;

pub const REQ_METHOD: Method = Method::DELETE;

#[derive(Deserialize, Debug)]
pub struct Params {
    pwd_hash: String,
    board_url: String,
    op_post_n: u64,
}

pub async fn handler(
    State(state): State<StateX>,
    Json(params): Json<Params>,
) -> anyhow::Result<(), Error> {
    let url = params.board_url;
    let op_post_n = params.op_post_n;

    let act_nonce = format!("{url}#{op_post_n}");
    {
        let x = state.secure_verify(params.pwd_hash, &act_nonce, Action::AddBoard).map_err(|x|x.map())?;
        let mut x = x.state.write().map_err(|_|DelPostE::E(E::StateAccess(2)))?;

        let board = some_or_ret!(x.mut_open_boards().board_mut(&url) => DelPostE::UnknownBoardUrl(url));
        board.remove_thr(op_post_n.into());
    }

    Ok(())
}
pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/del_thr", 
        routing::delete(handler).with_state(state.clone())
    )
}
