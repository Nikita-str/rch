use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Poster {
    Anon,
}

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    text: String,
    img: Option<String>,
    time: i64,
    n: u64,
    poster: Poster,
}

impl Post {
    pub fn new_anon(
        text: String,
        img: Option<String>,
    ) -> Self {
        Self {
            text,
            img,
            time: chrono::offset::Utc::now().timestamp(),
            n: 0,
            poster: Poster::Anon,
        }
    }

    pub fn upd_n(&mut self, n: u64) {
        if self.n != 0 { panic!("post n already setted") }
        self.n = n;
    } 

    pub fn text(&self) -> &str {
        &self.text
    }
}
