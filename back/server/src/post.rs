use serde::{Serialize, Deserialize};
use crate::utility::img::{ImgLoadInfo, ImgDelInfo};
use crate::utility::general as general;

pub type PostN = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Poster {
    Anon,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn add_del_info(&self, add_to: &mut Vec<ImgDelInfo>) {
        for img_info in &self.imgs {
            add_to.push(img_info.to_del_info())
        }
    }

    pub fn del_content<S>(&mut self, board_url: S, new_content: Option<String>)
    where S: Into<String> + AsRef<str>
    {
        self.text = new_content.unwrap_or_else(||crate::preproc::pkg_del(board_url.as_ref()));

        let pics_info = std::mem::take(&mut self.imgs)
            .into_iter()
            .map(|x|x.to_del_info())
            .collect();
        if let Err(_) = crate::utility::global_file_deleter::add_del_pics_act(board_url.into(), pics_info) {
            println!("[WARN] cant add del pic act")
        }
    }
}
