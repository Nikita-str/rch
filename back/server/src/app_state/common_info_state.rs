use super::SpeedPost;

// pub struct CommonInfoState<'a> {
//     total_post: &'a std::sync::atomic::AtomicU64,
//     open_board: &'a std::sync::atomic::AtomicU32,
//     speed_post: &'a std::sync::atomic::AtomicU32,
// }
pub struct CommonInfoState {
    total_post: u64,
    open_board: u32,
    speed_post: SpeedPost,
}

impl CommonInfoState {
    pub fn new(total_post: u64, speed_post: u32, speed_post_dt_sec: u32, open_board: u32) -> Self {
        Self {
            total_post,
            open_board,
            speed_post: SpeedPost::new(speed_post_dt_sec, speed_post),
        }
    }

    pub fn total_post(&self) -> u64 {
        self.total_post
    }
    pub fn speed_post(&self) -> u32 {
        self.speed_post.speed_post()
    }
    pub fn open_board_qty(&self) -> u32 {
        self.open_board
    }

    pub fn inc_post(&mut self) {
        self.speed_post.inc_post();
        self.total_post += 1;
    }
    pub fn open_new_board(&mut self) {
        self.open_board += 1;
    }
    pub fn upd_speed_post(&mut self) {
        self.speed_post.upd_speed_post()
    }
}