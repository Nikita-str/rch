pub mod common;
pub mod board;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::{common, board};
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &common::all::HandlerState) -> Router {
        let router = Router::new();
        let router = router.merge(common::router(common_info_state));
        let router = router.merge(board::router(common_info_state));
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        common::upd_allow_methods(methods);
        board::upd_allow_methods(methods);
    }
}

mod header_use {
    pub use axum::{routing, Router,http::Method, Json};
    pub use serde::{Serialize, Deserialize};
    pub use axum::extract::{State, Query};
    pub use std::sync::{Arc, RwLock, Mutex};
}