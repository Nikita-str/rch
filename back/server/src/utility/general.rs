
pub type Timestamp = i64;

pub fn timestamp() -> Timestamp {
    chrono::offset::Utc::now().timestamp()
}