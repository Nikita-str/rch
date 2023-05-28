pub mod post_new;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &crate::api::header_use::HandlerStateCommon) -> Router {
        let router = Router::new();
        let router = router.merge(post_new::router(common_info_state));
        let router = Router::new().nest("/thread", router);
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(post_new::REQ_METHOD);
    }
}
