pub mod is_exist; /* deprecated */ 
pub mod name;
pub mod thrs_load;
pub mod thr_new;
pub mod ctlg_load;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &crate::api::header_use::HandlerStateCommon) -> Router {
        let router = Router::new();
        let router = router.merge(is_exist::router(common_info_state));
        let router = router.merge(name::router(common_info_state));
        let router = router.merge(thrs_load::router(common_info_state));
        let router = router.merge(thr_new::router(common_info_state));
        let router = router.merge(ctlg_load::router(common_info_state));
        Router::new().nest("/board", router)
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(is_exist::REQ_METHOD);
        methods.insert(name::REQ_METHOD);
        methods.insert(thrs_load::REQ_METHOD);
        methods.insert(thr_new::REQ_METHOD);
        methods.insert(ctlg_load::REQ_METHOD);
    }
}

