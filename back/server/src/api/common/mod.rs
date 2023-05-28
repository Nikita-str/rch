pub mod all;
pub mod pop_boards;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &crate::api::header_use::HandlerStateCommon) -> Router {
        let router = Router::new();
        let router = router.merge(all::router(common_info_state));
        let router = router.merge(pop_boards::router(common_info_state));
        let router = Router::new().nest("/common", router);
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(all::REQ_METHOD);
        methods.insert(pop_boards::REQ_METHOD);
    }
}
