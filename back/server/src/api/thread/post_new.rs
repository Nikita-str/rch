use crate::api::header_use::*;
use crate::post::Post;

// ../api/thread/post_new?board_url=b&op_post_n=244&post_text=heheh%20in%20tuturu%20!!!!!!

pub const REQ_METHOD: Method = Method::POST;
pub type HandlerState = HandlerStateCommon;

#[derive(Deserialize, Debug)]
pub struct HandlerParams{
    board_url: String,
    op_post_n: u64,
    post_text: String,
    post_img: Option<String /* [?] Type [\?] */>,
}

pub async fn handler(
    State(state): State<HandlerState>,
    Json(params): Json<HandlerParams>,
) -> Json<()>
{
    crate::delay_ms(300);
    let board_url = &params.board_url;

    {
        let r_state = state.read().unwrap();
        if !r_state.open_boards().is_board_exist(board_url) {
            return Json(()) // ERROR
        }
    }

    let post_text = {
        // TODO: Pool of preproc?!
        let mut preproc = crate::preproc::HeadPreproc::new_by_board(board_url, false);
        preproc.preproc(&params.post_text)
    };

    if params.post_img.is_some() { todo!("img case") }
    let post = Post::new_anon(post_text, vec![]);

    {
        let mut w_state = state.write().unwrap();
        w_state.mut_open_boards()
            .board_mut(board_url)
            .map(|board|board.add_post(params.op_post_n, post));
    }

    Json(())
}

pub fn router(state: &HandlerState) -> Router {
    Router::new().route("/post_new", routing::post(handler).with_state(Arc::clone(state)))
}