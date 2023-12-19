pub mod full_save;
pub mod full_load;

use fns::{State, init_args};
pub use fns::{router, upd_allow_methods};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::sync::{Arc, RwLock};
    use std::collections::HashSet;

    use crate::app_state::common_info_state::save_load::StateInitArgs;

    use crate::security::SingleUsePwds as Secure;
    use crate::api::header_use::HandlerStateCommon;

    pub struct StateInner {
        pub state: HandlerStateCommon,
        pub secure: Secure,
    }
    pub type State = Arc<RwLock<StateInner>>;
    pub const SAVE_DIR: &str = "./saves";

    pub fn init_args(save_name: String, single_file: bool) -> StateInitArgs<'static> {
        StateInitArgs {
            save_dir: SAVE_DIR,
            save_name,
            single_file,
        }
    }

    pub fn router(common_info_state: &HandlerStateCommon) -> Router {
        const SAVE_AUX_DIR: &str = "aux";
        const PWD_FILE_NAME: &str = "single_pwds.txt";

        let _ = std::fs::create_dir_all(format!("{SAVE_DIR}/{SAVE_AUX_DIR}"));
        let output_path = format!("{SAVE_DIR}/{SAVE_AUX_DIR}/{PWD_FILE_NAME}");

        let state = Arc::new(RwLock::new(StateInner {
            state: Arc::clone(common_info_state),
            secure: Secure::new(&output_path).unwrap(),
        }));

        let router = Router::new();
        let router = router.merge(full_save::router(&state));
        let router = router.merge(full_load::router(&state));
        let router = Router::new().nest("/~~ctrl~~", router);
        router
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(full_save::REQ_METHOD);
        methods.insert(full_load::REQ_METHOD);
    }
}
