use super::{SpeedPost, OpenBoards};

pub struct CommonInfoState {
    total_post: u64,
    open_boards: OpenBoards,
    speed_post: SpeedPost,
}

impl CommonInfoState {
    /// # params
    /// * `deleted_board_post`: qty of posts that was on board that already deleted 
    pub fn new(deleted_board_post: u64, open_boards: OpenBoards, speed_post: SpeedPost) -> Self {
        Self {
            total_post: open_boards.x_post_qty() + deleted_board_post,
            open_boards,
            speed_post,
        }
    }

    pub fn total_post(&self) -> u64 {
        self.total_post
    }
    pub fn speed_post(&self) -> u32 {
        self.speed_post.speed_post()
    }
    pub fn open_board_qty(&self) -> u32 {
        self.open_boards.open_boards_qty()
    }

    pub fn inc_post(&mut self) {
        self.speed_post.inc_post();
        self.total_post += 1;
    }
    pub fn upd_speed_post(&mut self) {
        self.speed_post.upd_speed_post()
    }
    
    pub fn mut_open_boards(&mut self) -> &mut OpenBoards {
        &mut self.open_boards
    }
    pub fn open_boards(&self) -> &OpenBoards {
        &self.open_boards
    }

    // pub fn pic_path(&self) -> String {
    //     self.open_boards.pic_path()
    // }    
    pub fn use_n_pic(&mut self, n: u64, board_url: &str) -> (String, std::ops::Range<u64>) {
        (self.open_boards.pic_path_unchecked(board_url), self.open_boards.use_n_pic(n))
    }

    pub fn pic_path_parent(&self) -> &str {
        self.open_boards.pic_path_parent()
    }
}