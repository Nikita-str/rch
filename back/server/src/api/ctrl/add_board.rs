use crate::app_state::open_boards::{Board, BoardTag};
use crate::security::Action;
use crate::api::header_use::*;
use super::State as StateX;

use super::error_type::E;
#[derive(Clone, Copy)]
pub enum AddBoardE {
    E(E),
    AddBoardErr,
}
impl ErrType for AddBoardE {
    const MAX_ERR_CODE: usize = E::MAX_ERR_CODE + 1;
    fn err_code(&self) -> usize {
        match self {
            Self::E(x) => x.err_code(),
            Self::AddBoardErr => E::MAX_ERR_CODE + 1,
        }
    }

    fn err_status(&self) -> StatusCode {
        match self {
            Self::E(x) => x.err_status(),
            Self::AddBoardErr => StatusCode::BAD_REQUEST,
        }
    }

    fn err_msg(self) -> std::borrow::Cow<'static, str> {
        match self {
            Self::E(x) => x.err_msg(),
            Self::AddBoardErr => "error during board adding".into(),
        }
    }
}
impl From<E> for AddBoardE {
    fn from(e: E) -> Self {
        Self::E(e)
    }
}
type Error = ApiError<AddBoardE>;

pub const REQ_METHOD: Method = Method::POST;

#[derive(Deserialize, Debug)]
pub struct Params {
    pwd_hash: String,
    tag: Option<String>,
    url: String,
    name: String,
    descr: String,
    max_thr_qty: usize,
}

pub async fn handler(
    State(state): State<StateX>,
    Json(params): Json<Params>,
) -> anyhow::Result<(), Error> {
    let url = params.url;
    let name = params.name;
    let tag = params.tag;

    let act_nonce = format!("{url}#{name}");
    {
        let x = state.secure_verify(params.pwd_hash, &act_nonce, Action::AddBoard).map_err(|x|x.map())?;
        let mut x = x.state.write().map_err(|_|AddBoardE::E(E::StateAccess(2)))?;

        let board = Board::new(url, name, params.descr, 0, params.max_thr_qty);
        let tag = tag.map(|tag|BoardTag{ tag });
        x.mut_open_boards().add_board(board, tag).map_err(|e|AddBoardE::AddBoardErr.detailed(e))?;
    }

    Ok(())
}
pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/add_board", 
        routing::post(handler).with_state(state.clone())
    )
}