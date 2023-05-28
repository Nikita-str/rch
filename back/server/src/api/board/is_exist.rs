use crate::api::header_use::*;

pub const REQ_METHOD: Method = Method::GET;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct HandlerParams{
    board_url: String,
}

pub type ResultOk = bool;

pub async fn handler(
    Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
) -> Json<ResultOk> {
    crate::delay_ms(1_000);

    let exist = {
        let r_state = state.read().unwrap();
        r_state.open_boards().is_board_exist(&params.board_url)
    };

    Json(exist)
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/is_exist", routing::get(handler).with_state(Arc::clone(state)))
}