use clap::Parser;
use std::sync::{Arc, RwLock};
use crate::api::StdState;
use crate::utility::save_load::Load;
use crate::config::ConfigCtor;

#[derive(Debug, Parser)]
pub struct Cli {
    /// load specified save (by it's name)
    #[clap(long)]
    load: Option<String>,
    /// is the loaded save stored as a single file?
    #[clap(long)]
    load_single_file: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn state(&self) -> anyhow::Result<StdState> {
        let state = match &self.load {
            Some(save_name) => {
                let mut args = crate::api::ctrl::init_args(save_name.clone(), self.load_single_file);
                crate::app_state::CommonInfoState::load(&mut args)
                    .map_err(|e|anyhow::anyhow!("Error during save loading(save: {:?}): {e}", save_name))?
            }
            None => {
                crate::app_state::CommonInfoState::config_new(())
            }
        };
        Ok(Arc::new(RwLock::new(state)))
    }
}