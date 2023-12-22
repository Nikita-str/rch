pub mod common;
pub mod board;
pub mod thread;
pub mod ctrl;

mod error;

pub use fns::{router, upd_allow_methods};
mod fns {
    use super::{common, board, thread, ctrl};
    use axum::Router;
    use axum::http::Method;
    use std::collections::HashSet;

    pub fn router(common_info_state: &crate::api::header_use::HandlerStateCommon) -> Router {
        let router = Router::new();
        let router = router.merge(common::router(common_info_state));
        let router = router.merge(board::router(common_info_state));
        let router = router.merge(thread::router(common_info_state));
        let router = router.merge(ctrl::router(common_info_state));
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        common::upd_allow_methods(methods);
        board::upd_allow_methods(methods);
        thread::upd_allow_methods(methods);
        ctrl::upd_allow_methods(methods);
    }

    pub fn create_img_load_info(
        state: &super::header_use::HandlerStateCommon, 
        board_url: &str,
        imgs: &Vec<crate::utility::img::PostImg>, 
        max_imgs_n: usize
    ) -> Vec<crate::utility::img::ImgLoadInfo> {
        let img_preparer = crate::utility::img::ImgsPreparerSealed::new_by_imgs(imgs, max_imgs_n);

        let (pic_dir, pic_n) = {
            let mut w_state = state.write().unwrap();
            w_state.use_n_pic(img_preparer.n_pics(), board_url)
        };
    
        let imgs = img_preparer.to_img_load_info(&pic_dir, pic_n);
        imgs
    }
}

mod header_use {
    pub use serde_json::json;
    pub use serde::{Serialize, Deserialize};

    pub use axum::{routing, Router, Json};
    pub use axum::http::{StatusCode, Method};
    pub use axum::extract::{State, Query, Form};
    pub use axum::response::{IntoResponse, Response};
    
    pub use std::sync::{Arc, RwLock, Mutex};

    pub type HandlerStateCommon = Arc<RwLock<crate::app_state::CommonInfoState>>;

    pub use crate::api::error::*;
}

const MAX_PIC_AMOUNT: usize = 4;
