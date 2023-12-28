use rand::Rng;

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

pub fn rand_string(len: usize) -> String {
    const CHARSET: &[u8] = b"AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz0123456789";
    const RANGE: std::ops::Range<usize> = 0..CHARSET.len();

    let mut rng = rand::thread_rng();
    let rand_char = |_|CHARSET[rng.gen_range(RANGE)] as char;
    (0..len).into_iter().map(rand_char).collect()
}

pub fn rand<T: rand::distributions::uniform::SampleUniform + PartialOrd>(min: T, max: T) -> T {
    let mut rng = rand::thread_rng();
    let range = min..=max;
    rng.gen_range(range)
}
