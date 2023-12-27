pub mod img;
pub mod general;

pub mod action_loop;
pub use action_loop::ActionLooper;
pub use action_loop::global_file_deleter;
// mod file_deleter;
// pub use file_deleter::FileDeleter;
// pub use file_deleter::global as global_file_deleter;

pub mod save_load;

mod mut_cell;
pub use mut_cell::MutCell;