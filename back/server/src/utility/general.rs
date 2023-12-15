
pub type Timestamp = i64;

pub fn timestamp() -> Timestamp {
    chrono::offset::Utc::now().timestamp()
}

pub fn create_dir<P: AsRef<std::path::Path>>(path: P) -> bool {
    match std::fs::create_dir(path) {
        Ok(()) => { return true }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => { return true }
        _ => { return false }
    }
}