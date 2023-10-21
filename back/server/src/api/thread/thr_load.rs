use crate::api::header_use::*;
use crate::post::Post;

pub const REQ_METHOD: Method = Method::GET;
pub type HandlerState = HandlerStateCommon;

#[derive(Clone, Copy)]
pub enum Error {
    UnknBoard = 1,
    NoThr = 2,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, code, msg) = match self {
            Error::UnknBoard => (StatusCode::NOT_FOUND, self as usize, "unknown board"),
            Error::NoThr => (StatusCode::NOT_FOUND, self as usize, "invalid op thread"),
        };

        let body = Json(json!({
            "code": code,
            "msg": msg,
        }));

        (status, body).into_response()
    }
}

#[derive(Deserialize, Debug)]
pub struct HandlerParams {
    board_url: String,
    op_post_n: u64,
    from: usize,
    n_load: usize,
}

#[derive(Serialize, Debug)]
pub struct ResultOk {
    loaded_posts: Vec<Post>,
    header: Option<String>,
    all_loaded: bool,
}

pub async fn handler(
    Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
) -> Result<Json<ResultOk>, Error> {
    pub const N_LOAD_MIN: usize = 10;
    pub const N_LOAD_MAX: usize = 30;

    let op_post_n = params.op_post_n;
    let from = params.from;
    let n_load = params.n_load.min(N_LOAD_MAX).max(N_LOAD_MIN);

    crate::delay_ms(1000);

    {
        let r_state = state.read().unwrap();
        let Some(board) = r_state.open_boards().board(&params.board_url) else {
            return Err(Error::UnknBoard);
        };
        let Some(thr) = board.thr(op_post_n) else {
            return Err(Error::NoThr);
        };

        let loaded_posts = thr
            .posts(from, n_load)
            .into_iter()
            .map(|x| x.clone())
            .collect::<Vec<_>>();

        let all_loaded = from + n_load > thr.post_qty();

        Ok(Json(ResultOk {
            loaded_posts,
            header: (from == 0).then(||thr.header().to_owned()),
            all_loaded,
        }))
    }
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route(
        "/thr_load",
        routing::get(handler).with_state(Arc::clone(state)),
    )
}
