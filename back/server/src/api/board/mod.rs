pub mod is_exist;
pub mod thrs_load;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &is_exist::HandlerState) -> Router {
        let router = Router::new();
        let router = router.merge(is_exist::router(common_info_state));
        let router = router.merge(thrs_load::router(common_info_state));
        let router = Router::new().nest("/board", router);
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(is_exist::REQ_METHOD);
        methods.insert(thrs_load::REQ_METHOD);
    }
}

