use crate::api::header_use::*;

pub const REQ_METHOD: Method = Method::GET;
pub type HandlerState = Arc<Mutex<HandlerStateInner>>;
#[derive(Clone)]
pub struct HandlerStateInner {
    state: super::all::HandlerState,
    memo: ResultOk,
    pop_boards_qty: u32,

}
impl HandlerStateInner {
    pub fn new(state: &super::all::HandlerState) -> HandlerState {
        Arc::new(Mutex::new(Self {
            state: Arc::clone(state),
            memo: Vec::new(),
            pop_boards_qty: 0,
        }))
    }
}

pub type ResultOk = Vec<crate::app_state::open_boards::PopBoardsTagInfo>;

pub async fn handler(State(state): State<HandlerState>) -> Json<ResultOk> {
    crate::delay();

    let mut state = state.lock().unwrap();

    let pop_boards_qty = {
        let r_state = state.state.read().unwrap();
        r_state.open_boards().pop_boards_qty()
    };

    if pop_boards_qty != state.pop_boards_qty {
        state.memo = {
            let w_state = state.state.write().unwrap();
            w_state.open_boards().popular_boards()
        };
        state.pop_boards_qty = pop_boards_qty;
    }
    
    Json(state.memo.clone())
}

pub fn router(state: &super::all::HandlerState) -> Router {
    let state = HandlerStateInner::new(state);
    Router::new().route("/pop_boards", routing::get(handler).with_state(state))
}