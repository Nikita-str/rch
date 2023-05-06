use axum::{routing, Router,http::Method, Json};
use serde::Serialize;
use axum::extract::State;
use std::sync::{Arc, RwLock};
use crate::app_state::CommonInfoState;

pub type HandlerState = Arc<RwLock<CommonInfoState>>;
pub const REQ_METHOD: Method = Method::GET;

#[derive(Serialize, Debug)]
pub struct ResultOk {
    total_post: u64,
    open_board: u32,
    speed_post: u32,
}

pub async fn handler(
    State(state): State<HandlerState>,
) -> Json<ResultOk> {
    crate::delay();

    {
        // change value in GET :|
        // (it's just emul so it's okey, it's okey)
        let mut w_state = state.write().unwrap();
        (0..10).for_each(|_|w_state.inc_post());
    }

    let value = {
        let r_state = state.read().unwrap();
        ResultOk {
            total_post: r_state.total_post(),
            open_board: r_state.open_board_qty(),
            speed_post: r_state.speed_post(),
        }
    };

    Json(value)
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/all", routing::get(handler).with_state(Arc::clone(state)))
}