mod app_state;
mod post;
mod api;
mod thread;
mod thread_usage_rate;
mod preproc;
pub mod utility;
mod security;
mod config;
mod cli;
pub use cli::Cli;

const KB: usize = 1024;
const MB: usize = 1024 * KB;

static SHUTDOWN: std::sync::OnceLock<tokio::sync::mpsc::UnboundedSender<()>> = std::sync::OnceLock::new();

#[allow(unused)] use crate::utility::general::{delay, delay_ms};
pub use fns::server;
mod fns {
    use axum::{Server, Router};
    use futures::Future;
    use tower_http::services::{ServeDir, ServeFile};
    use tower_http::cors::CorsLayer;

    use crate::api;

    use crate::config::{Config, ConfigCtor};
    use crate::utility::action_loop::LoopActs;

    fn init_shutdown() -> tokio::sync::mpsc::UnboundedReceiver<()> {
        let (shutdown_sx, shutdown_rx) = tokio::sync::mpsc::unbounded_channel();
        crate::SHUTDOWN.set(shutdown_sx).unwrap();
        shutdown_rx
    }

    fn serve_dir() -> ServeDir<ServeFile> {
        let vue_dist_path = Config::vue_dist_path();
        let index_file = ServeFile::new(format!("{}/index.html", vue_dist_path));
        ServeDir::new(vue_dist_path).fallback(index_file)
    }

    fn cors_layer() -> CorsLayer {
        let mut methods = std::collections::HashSet::new();
        api::upd_allow_methods(&mut methods);
        let cors = CorsLayer::new().allow_methods(methods.drain().collect::<Vec<_>>());
        cors
    }
    
    fn router(state: &api::StdState) -> Router {
        let router = Router::new();
        let router = router.nest_service("/", serve_dir());
        let router = router.nest("/api", api::router(state));
        router
    }

    fn start_server(state: &api::StdState) -> impl Future {
        let cors = cors_layer();
        let router = router(state);
        let router = router.layer(cors).into_make_service();

        let addr = crate::config::addr();
        let server = Server::bind(&addr);
        let server = server.serve(router);

        let mut shutdown_rx = init_shutdown();
        let awaiter = async move { shutdown_rx.recv().await; };
        let graceful = server.with_graceful_shutdown(awaiter);
        graceful
    }

    pub async fn server(state: api::StdState) {
        let loop_acts = LoopActs::config_new(&state);
        let cmd_loop_ctrl = crate::utility::ActionLooper::new(loop_acts);

        let _ = start_server(&state).await;
        println!("[SHUTDOWN]");
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
        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }
        // impl Into<$ty> for $name {
        impl From<$name> for $ty {
            fn from(x: $name) -> $ty {
                x.0
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
