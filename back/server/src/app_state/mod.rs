pub(in crate) mod speed_post;
pub use speed_post::SpeedPost;

pub(in crate) mod common_info_state;
pub use common_info_state::CommonInfoState;

pub(in crate) mod open_boards;
pub use open_boards::OpenBoards;

use std::collections::VecDeque;
use crate::post::Post;

pub struct BoardState {
    thread_open_post: VecDeque<Post>,
}

pub struct ThreadState {
    posts: Vec<Post>,
}