use crate::security::Action;
use crate::api::header_use::*;
use super::State as StateX;

use crate::some_or_ret;

use super::error_type::E;
pub enum DelPostE {
    E(E),
    UnknownBoardUrl(String),
    UnknownThread(String, u64),
    DelError,
}
impl ErrType for DelPostE {
    const MAX_ERR_CODE: usize = E::MAX_ERR_CODE + 3;
    fn err_code(&self) -> usize {
        match self {
            Self::E(x) => x.err_code(),
            Self::UnknownBoardUrl(_) => E::MAX_ERR_CODE + 1,
            Self::UnknownThread(_, _) => E::MAX_ERR_CODE + 2,
            Self::DelError => E::MAX_ERR_CODE + 3,
        }
    }

    fn err_status(&self) -> StatusCode {
        match self {
            Self::E(x) => x.err_status(),
            Self::UnknownBoardUrl(_) => StatusCode::BAD_REQUEST,
            Self::UnknownThread(_, _) => StatusCode::BAD_REQUEST,
            Self::DelError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn err_msg(self) -> std::borrow::Cow<'static, str> {
        match self {
            Self::E(x) => x.err_msg(),
            Self::UnknownBoardUrl(url) => format!("unknown board url `/{url}/`").into(),
            Self::UnknownThread(url, op_n) => format!("unknown thread `/{url}/{op_n}/`").into(),
            Self::DelError => "error while delete post content".into(),
        }
    }
}
impl From<E> for DelPostE {
    fn from(e: E) -> Self {
        Self::E(e)
    }
}
type Error = ApiError<DelPostE>;

pub const REQ_METHOD: Method = Method::DELETE;

#[derive(Deserialize, Debug)]
pub struct Params {
    pwd_hash: String,
    board_url: String,
    op_post_n: u64,
    post_n: u64,
}

pub async fn handler(
    State(state): State<StateX>,
    Json(params): Json<Params>,
) -> anyhow::Result<(), Error> {
    let url = params.board_url;
    let post_n = params.post_n;
    let op_post_n = params.op_post_n;

    let act_nonce = format!("{url}#{post_n}");
    {
        let x = state.secure_verify(params.pwd_hash, &act_nonce, Action::DelPost).map_err(|x|x.map())?;
        let mut x = x.state.write().map_err(|_|DelPostE::E(E::StateAccess(2)))?;

        let board = some_or_ret!(x.mut_open_boards().board_mut(&url) => DelPostE::UnknownBoardUrl(url));
        let thr = some_or_ret!(board.thr_mut(op_post_n) => DelPostE::UnknownThread(url, op_post_n));
        thr.del_post_content(url, post_n).map_err(|e|DelPostE::DelError.detailed(e))?;
    }

    Ok(())
}
pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/del_post", 
        routing::delete(handler).with_state(state.clone())
    )
}
