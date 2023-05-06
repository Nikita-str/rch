pub mod common;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::common;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(state_all: &common::all::HandlerState) -> Router {
        let router = Router::new();
        let router = router.merge(common::router(state_all));
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        common::upd_allow_methods(methods);
    }
}