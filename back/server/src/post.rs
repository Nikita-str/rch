use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Poster {
    Anon,
}

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    text: String,
    img: Option<String>,
    time: std::time::SystemTime,
    n: u64,
    poster: Poster,
}

impl Post {
    pub fn text(&self) -> &str {
        &self.text
    }
}
