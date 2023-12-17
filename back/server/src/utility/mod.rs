pub mod img;
pub mod general;

mod file_deleter;
pub use file_deleter::FileDeleter;
pub use file_deleter::global as global_file_deleter;

pub mod save_load;
