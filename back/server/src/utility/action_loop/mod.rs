mod loop_acts;
mod act_looper;
pub mod file_deleter;
pub mod auto_saver;
pub mod speed_post_updater;

pub use loop_acts::LoopActs;
pub use loop_acts::ConfigCtorArgs as LoopActsArgs;
pub use act_looper::ActionLooper;
pub use speed_post_updater::SpeedPostUpdater;
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

trait LoopDur {
    fn config_loop_dur() -> std::time::Duration;
}