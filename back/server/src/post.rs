use serde::Serialize;
use crate::utility::img::ImgLoadInfo;
use crate::utility::general as general;

pub type PostN = u64;

#[derive(Serialize, Debug, Clone)]
pub enum Poster {
    Anon,
}

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    text: String,
    imgs: Vec<ImgLoadInfo>,
    time: general::Timestamp,
    n: PostN,
    poster: Poster,
    replies: Vec<PostN>,
}

impl Post {
    pub fn new_anon(
        text: String,
        imgs: Vec<ImgLoadInfo>,
    ) -> Self {
        Self {
            text,
            imgs,
            time: general::timestamp(),
            n: 0,
            poster: Poster::Anon,
            replies: Vec::new(),
        }
    }

    pub fn upd_n(&mut self, n: PostN) -> PostN {
        if self.n != 0 { panic!("post n already setted") }
        self.n = n;
        self.n
    } 

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn n(&self) -> PostN {
        self.n
    }

    pub fn time(&self) -> general::Timestamp {
        self.time
    }

    pub fn dt(&self, prev: &Self) -> f32 {
        (prev.time - self.time) as f32
    }

    pub fn add_reply(&mut self, reply_from: PostN) {
        self.replies.push(reply_from)
    }
}
