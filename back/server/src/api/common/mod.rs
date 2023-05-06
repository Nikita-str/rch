pub mod all;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::all;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(state_all: &all::HandlerState) -> Router {
        let router = Router::new();
        let router = router.merge(all::router(state_all));
        let router = Router::new().nest("/common", router);
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(all::REQ_METHOD);
    }
}
