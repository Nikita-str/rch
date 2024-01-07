pub mod post_new;
pub mod thr_load;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &crate::api::header_use::HandlerStateCommon) -> Router {
        let router = Router::new();
        let router = router.merge(post_new::router(common_info_state));
        let router = router.merge(thr_load::router(common_info_state));
        Router::new().nest("/thread", router)
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(post_new::REQ_METHOD);
        methods.insert(thr_load::REQ_METHOD);
    }
}
