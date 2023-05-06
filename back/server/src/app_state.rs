use std::collections::VecDeque;
use crate::post::Post;


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
}


pub struct SpeedPost {
    post_times: VecDeque<(std::time::SystemTime, u32)>, // TODO:MAYBE: RefCell<..>
    max_post_times_len: usize,
    speed_post: u32,
    // how freq update info about posts 
    dt_sec: u32,
}
impl SpeedPost {
    const H_TO_SEC: u32 = 60 * 60;
    const MIN_DT_SEC: u32 = 10;

    pub fn new(dt_sec: u32, speed_post: u32) -> Self {
        let dt_sec = dt_sec.max(Self::MIN_DT_SEC);
        
        let max_post_times_len = Self::H_TO_SEC / dt_sec;
        let max_post_times_len = if max_post_times_len * dt_sec != Self::H_TO_SEC {
            max_post_times_len + 1
        } else {
            max_post_times_len
        };

        let mut post_times = VecDeque::new();
        if speed_post != 0 {
            post_times.push_back((std::time::SystemTime::now(), speed_post));
        }

        Self {
            post_times,
            max_post_times_len: ((Self::H_TO_SEC / dt_sec) + 1) as usize, 
            speed_post,
            dt_sec,
        }
    }

    pub fn speed_post(&self) -> u32 {
        self.speed_post
    }

    fn remove_oldest(&mut self) {
        if let Some((_, old_post_qty)) = self.post_times.pop_front() {
            self.speed_post -= old_post_qty;
        }
    }

    pub fn upd_speed_post(&mut self) {
        let cur_time = std::time::SystemTime::now();

        // remove too old if they wasn't removed in next part for some reason
        loop {
            // it will end because on each iter `.len()` decrease by 1
            if self.post_times.len() <= self.max_post_times_len { break }
            // there can't be panic: `self.max_post_times_len` >= 1
            self.remove_oldest();
        }


        loop {
            let mut remove = false;
            if let Some(x) = self.post_times.get(0) {
                if let Ok(dt) = cur_time.duration_since(x.0) {
                    if dt.as_secs() as u32 > Self::H_TO_SEC {
                        remove = true;
                    }
                }
            }
            if remove { self.remove_oldest() } 
            else { break }
        }
    }

    pub fn inc_post(&mut self) {
        let cur_time = std::time::SystemTime::now();
        let mut create_new_time_seg = true;

        if let Some((prev_time, post_qty)) = self.post_times.back_mut() {
            let prev_time = *prev_time;
            let dt = cur_time.duration_since(prev_time);

            // if `Err` assume that too little time has passed (less than `self.dt_sec`)
            if dt.map(|dt| (dt.as_secs() as u32) < self.dt_sec).unwrap_or(true) {
                *post_qty += 1;
                create_new_time_seg = false;
            }
        }
        
        if create_new_time_seg { self.post_times.push_back((cur_time, 1)) }

        self.speed_post += 1;
    } 
}

pub struct BoardState {
    thread_open_post: VecDeque<Post>,
}

pub struct ThreadState {
    posts: Vec<Post>,
}