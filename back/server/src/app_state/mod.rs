mod speed_post;
pub use speed_post::SpeedPost;
mod common_info_state;
pub use common_info_state::CommonInfoState;


use std::collections::VecDeque;
use crate::post::Post;

pub struct BoardState {
    thread_open_post: VecDeque<Post>,
}

pub struct ThreadState {
    posts: Vec<Post>,
}