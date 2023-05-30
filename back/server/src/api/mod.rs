pub mod common;
pub mod board;
pub mod thread;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::{common, board, thread};
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &crate::api::header_use::HandlerStateCommon) -> Router {
        let router = Router::new();
        let router = router.merge(common::router(common_info_state));
        let router = router.merge(board::router(common_info_state));
        let router = router.merge(thread::router(common_info_state));
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        common::upd_allow_methods(methods);
        board::upd_allow_methods(methods);
        thread::upd_allow_methods(methods);
    }
}

mod header_use {
    pub use axum::{routing, Router,http::Method, Json};
    pub use serde::{Serialize, Deserialize};
    pub use axum::extract::{State, Query, Form};
    pub use std::sync::{Arc, RwLock, Mutex};

    pub type HandlerStateCommon = Arc<RwLock<crate::app_state::CommonInfoState>>;
}