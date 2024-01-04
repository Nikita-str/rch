mod app_state;
mod post;
mod api;
mod thread;
mod thread_usage_rate;
mod preproc;
pub mod utility;
mod security;
mod config;

const KB: usize = 1024;
const MB: usize = 1024 * KB;

static SHUTDOWN: std::sync::OnceLock<tokio::sync::mpsc::UnboundedSender<()>> = std::sync::OnceLock::new();

#[allow(unused)] use crate::utility::general::{delay, delay_ms};
pub use fns::server;
mod fns {
    use super::{api, app_state};
    use axum::{Server, Router};
    use tower_http::services::{ServeDir, ServeFile};
    use tower_http::cors::CorsLayer;
    use std::sync::{Arc, RwLock};
    use crate::config::Config;

    pub async fn server() {
        let vue_dist_path = Config::vue_dist_path();

        let deleted_board_post = 0;
        let dt_sec = 60;
        let open_boards = crate::app_state::OpenBoards::new(vue_dist_path);
        let speed_post = crate::app_state::SpeedPost::new(dt_sec, 0);
        let state_all = app_state::CommonInfoState::new(deleted_board_post, open_boards, speed_post);
        let pic_path_parent = state_all.pic_path_parent().to_owned();
        let state_all = Arc::new(RwLock::new(state_all));

        let mut loop_acts = crate::utility::action_loop::LoopActs::new();
        let act = crate::utility::action_loop::SpeedPostUpdater::new(&state_all);
        let dur =  std::time::Duration::from_secs((dt_sec / 2) as u64);
        loop_acts.add(act, dur);
        let act = crate::utility::action_loop::file_deleter::FileDelState::new(pic_path_parent);
        let dur = crate::utility::action_loop::file_deleter::DELETE_LOOP_DUR;
        loop_acts.add(act, dur);
        let act = crate::utility::action_loop::auto_saver::AutoSaver::new_std(&state_all);
        let dur = crate::utility::action_loop::auto_saver::SAVE_LOOP_DUR;
        loop_acts.add(act, dur);
        loop_acts.init();
        let cmd_loop_ctrl = crate::utility::ActionLooper::new(loop_acts);

        let (shoutdown_sx, mut shoutdown_rx) = tokio::sync::mpsc::unbounded_channel();
        crate::SHUTDOWN.set(shoutdown_sx).unwrap();

        // std::fs::create_dir(PIC_PATH).unwrap();
        let index_file = ServeFile::new(format!("{}/index.html", vue_dist_path));
        let serve_dir = ServeDir::new(vue_dist_path).fallback(index_file);

        let mut methods = std::collections::HashSet::new();
        api::upd_allow_methods(&mut methods);
        let cors = CorsLayer::new().allow_methods(methods.drain().collect::<Vec<_>>());

        let router = Router::new();
        let router = router.nest_service("/", serve_dir);
        let router = router.nest("/api", api::router(&state_all));
        let router = router.layer(cors).into_make_service();

        let addr = crate::config::addr();
        let server = Server::bind(&addr);
        let server = server.serve(router);

        let graceful = server.with_graceful_shutdown(async { shoutdown_rx.recv().await; });
        let _ = graceful.await;
        println!("[SHOUTDOWN]");
        let _ = cmd_loop_ctrl.close_async().await;
        println!("[CLOSED]");
    }
}

macro_rules! define_id {
    ([IMPL] $name:ident) => {
        impl $name {
            pub fn first() -> Self { Self(1) }
            pub fn next(self) -> Self { Self(self.0 + 1) }
            pub fn inc(&mut self) -> Self { 
                let ret = *self; 
                *self = self.next(); 
                ret 
            }
        }
    };
    ([INNER] $name:ident: $ty:ty) => {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        pub struct $name($ty);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#{}", self.0)
            }
        }
        impl Into<$ty> for $name {
            fn into(self) -> $ty {
                self.0
            }
        }
        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }
    };
    ($name:ident: $ty:ty) => {
        crate::define_id!([INNER] $name: $ty);
        crate::define_id!([IMPL] $name);
    };
    ($name:ident: $ty:ty [NO IMPL]) => {
        crate::define_id!([INNER] $name: $ty);
    };
    ($name:ident) => { crate::define_id!($name: usize); };
}
pub(crate) use define_id;

macro_rules! some_or_ret {
    ($test:expr => $err:expr) => {
        match $test {
            Some(x) => x,
            None => return Err($err.into()),
        }
    };
}
use some_or_ret;
