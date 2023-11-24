use serde::Serialize;
use crate::utility::img::ImgLoadInfo;

pub type PostN = u64;

#[derive(Serialize, Debug, Clone)]
pub enum Poster {
    Anon,
}

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    text: String,
    img: Vec<ImgLoadInfo>,
    time: i64,
    n: PostN,
    poster: Poster,
}

impl Post {
    pub fn new_anon(
        text: String,
        img: Vec<ImgLoadInfo>,
    ) -> Self {
        Self {
            text,
            img,
            time: chrono::offset::Utc::now().timestamp(),
            n: 0,
            poster: Poster::Anon,
        }
    }

    pub fn upd_n(&mut self, n: PostN) {
        if self.n != 0 { panic!("post n already setted") }
        self.n = n;
    } 

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn n(&self) -> PostN {
        self.n
    }

    pub fn dt(&self, prev: &Self) -> f32 {
        (prev.time - self.time) as f32
    }
}
