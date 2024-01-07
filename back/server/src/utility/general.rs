use rand::Rng;

pub type Timestamp = i64;

pub fn timestamp() -> Timestamp {
    chrono::offset::Utc::now().timestamp()
}

pub fn create_dir<P: AsRef<std::path::Path>>(path: P) -> bool {
    match std::fs::create_dir(path) {
        Ok(()) => { true }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => { true }
        _ => { false }
    }
}


pub fn rand_string(len: usize) -> String {
    const CHARSET: &[u8] = b"AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz0123456789";
    const RANGE: std::ops::Range<usize> = 0..CHARSET.len();

    let mut rng = rand::thread_rng();
    let rand_char = |_|CHARSET[rng.gen_range(RANGE)] as char;
    (0..len).map(rand_char).collect()
}

pub fn rand<T: rand::distributions::uniform::SampleUniform + PartialOrd>(min: T, max: T) -> T {
    let mut rng = rand::thread_rng();
    let range = min..=max;
    rng.gen_range(range)
}


#[inline(always)]
pub(in crate) fn delay() {
    #[cfg(debug_assertions)]
    delay_ms(1_500);
}

#[inline]
pub(in crate) fn delay_ms(_ms: usize) {
    #[cfg(debug_assertions)] {
        std::thread::sleep(std::time::Duration::from_millis(_ms as u64));
        println!("[DELAY({_ms}ms)]");
    }
}