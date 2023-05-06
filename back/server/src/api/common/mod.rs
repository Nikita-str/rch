pub mod all;
pub mod pop_boards;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(state_all: &all::HandlerState) -> Router {
        let router = Router::new();
        let router = router.merge(all::router(state_all));
        let router = router.merge(pop_boards::router(state_all));
        let router = Router::new().nest("/common", router);
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(all::REQ_METHOD);
        methods.insert(pop_boards::REQ_METHOD);
    }
}
