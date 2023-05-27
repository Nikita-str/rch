use crate::api::header_use::*;
use crate::app_state::CommonInfoState;
use crate::post::Post;

// http://127.0.0.1:5173/api/board/thr_new?board_url=b&header=Post%20Header&post_text=tuturu

pub const REQ_METHOD: Method = Method::POST;

pub type HandlerState = Arc<RwLock<CommonInfoState>>;

#[derive(Deserialize, Debug)]
pub struct HandlerParams{
    board_url: String,
    post_header: Option<String>,
    post_text: String,
    post_img: Option<String /* [?] Type [\?] */>,
}

#[derive(Serialize, Debug)]
pub struct ResultOk {
    thrs: Vec<Vec<Post>>,
}

pub async fn handler(
    Query(params): Query<HandlerParams>,
    State(state): State<HandlerState>,
) -> Json<()>
{
    crate::delay_ms(300);

    if params.post_img.is_some() { todo!("img case") }
    let op_post = Post::new_anon(params.post_text, params.post_img);
    let infinity = false;

    {
        let mut w_state = state.write().unwrap();
        w_state.mut_open_boards().board_mut(&params.board_url)
            .map(|board|board.new_thr(params.post_header, op_post, infinity));    
    }

    Json(())
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/thr_new", routing::post(handler).with_state(Arc::clone(state)))
}