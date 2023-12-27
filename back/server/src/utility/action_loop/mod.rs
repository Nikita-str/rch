mod loop_acts;
mod action_loop;
pub mod file_deleter;
pub mod auto_saver;

pub use loop_acts::LoopActs;
pub use action_loop::ActionLooper;
pub use file_deleter::global as global_file_deleter;

mod help {
    pub use std::sync::{Arc, Mutex, MutexGuard};
    pub use std::time::Duration;
    pub use tokio::sync::mpsc::UnboundedReceiver as Receiver;
    pub use tokio::sync::mpsc::UnboundedSender as Sender;
    pub use std::thread::JoinHandle;

    pub use super::LoopActor;
}

pub trait LoopActor: Send {
    fn act_step(&mut self);
    fn close_step(&mut self);
    fn init(&self);
}