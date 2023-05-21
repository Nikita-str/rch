
pub enum Poster {
    Anon,
}

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