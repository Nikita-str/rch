use crate::api::header_use::*;
use crate::app_state::CommonInfoState;


pub const REQ_METHOD: Method = Method::GET;

pub type HandlerState = Arc<RwLock<CommonInfoState>>;

#[derive(Deserialize, Debug)]
pub struct HandlerParams{
    board_url: String,
}

pub type ResultOk = bool;

pub async fn handler(
    Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
) -> Json<ResultOk> {
    crate::delay_ms(300);

    let exist = {
        let r_state = state.read().unwrap();
        r_state.open_boards().is_board_exist(&params.board_url)
    };

    Json(exist)
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/is_exist", routing::get(handler).with_state(Arc::clone(state)))
}