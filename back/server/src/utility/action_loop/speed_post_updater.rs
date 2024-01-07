use std::sync::Arc;
use crate::api::common::all::HandlerState;
use super::help::*;
use super::LoopDur;

const UPD_SECS: u64 = 30;
pub const UPD_LOOP_DUR: Duration = Duration::from_secs(UPD_SECS);

pub struct SpeedPostUpdater {
    state: HandlerState,
}

impl SpeedPostUpdater {
    pub fn new(state: &HandlerState) -> Self {
        Self { state: Arc::clone(state) }
    }
    fn upd(&self) -> anyhow::Result<()> {
        let Ok(mut w_state) = self.state.write() else {
            anyhow::bail!("state is poisoned")
        };
        w_state.upd_speed_post();
        Ok(())
    }
}

impl LoopActor for SpeedPostUpdater {
    fn act_step(&mut self) {
        self.upd().unwrap()
    }

    fn close_step(&mut self) {
        self.upd().unwrap()
    }

    fn init(&self) {
        self.upd().unwrap()
    }
}

impl LoopDur for SpeedPostUpdater {
    fn config_loop_dur() -> Duration {
        let secs = crate::config::Config::loops().rate_post_dt_sec;
        Duration::from_secs(secs / 2)
    }
}
