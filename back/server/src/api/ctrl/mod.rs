pub mod full_save;
pub mod full_load;
pub mod add_board;
pub mod del_post;
pub mod del_thr;
pub mod shutdown;

mod error_type;

use fns::State;
pub use fns::{router, upd_allow_methods, init_args};
mod fns {
    use super::*;
    use axum::Router;
    use axum::http::Method;
    use std::sync::{Arc, RwLock, RwLockWriteGuard};
    use std::collections::HashSet;

    use crate::app_state::common_info_state::save_load::StateInitArgs;
    
    use crate::api::error::{ApiError, ErrType};
    use error_type::E;
    type StateError = ApiError<E>;
    use crate::security::SingleUsePwds as Secure;

    use crate::api::header_use::HandlerStateCommon;

    pub struct StateInner {
        pub state: HandlerStateCommon,
        pub secure: Secure,
    }
    #[derive(Clone)]
    pub struct State {
        pub state: Arc<RwLock<StateInner>>,
    }
    impl State {
        pub fn secure_verify(
            &self, 
            pwd_hash: String, 
            act_nonce: &str,
            act: crate::security::Action,
        ) -> anyhow::Result<RwLockWriteGuard<StateInner>, StateError> {
            let hash_expected = hex::decode(&pwd_hash)
            .map_err(|_|StateError::new_detailed_x(E::BadHash, pwd_hash))?;
    
            let mut x = self.state.write().map_err(|_|E::StateAccess(1))?;
            
            let ok = x.secure.use_pwd(act, act_nonce, &hash_expected)
                .map_err(|e|E::SecureInner.detailed(e))?;
            if !ok { return Err(E::SecureInvalid.into()) }

            // let state: RwLockWriteGuard<'x, _> = x.state.write().map_err(|_|E::StateAccess(2))?;
            Ok(x)
        }
    }

    #[inline]
    fn save_dir() -> &'static str {
        &crate::config::Config::saves().dir
    }

    pub fn init_args(save_name: String, single_file: bool) -> StateInitArgs<'static> {
        StateInitArgs {
            save_dir: save_dir(),
            save_name,
            single_file,
        }
    }

    pub fn router(common_info_state: &HandlerStateCommon) -> Router {
        let save_dir = save_dir();
        let aux_dir = &crate::config::Config::saves().aux_dir;
        let pwd_file = &crate::config::Config::saves().single_use_pwd_file;

        let _ = std::fs::create_dir_all(format!("{save_dir}/{aux_dir}", ));
        let output_path = format!("{save_dir}/{aux_dir}/{pwd_file}");

        let state = Arc::new(RwLock::new(StateInner {
            state: Arc::clone(common_info_state),
            secure: Secure::new(&output_path).unwrap(),
        }));
        let state = State { state };

        let router = Router::new();
        let router = router.merge(full_save::router(&state));
        let router = router.merge(full_load::router(&state));
        let router = router.merge(add_board::router(&state));
        let router = router.merge(del_post::router(&state));
        let router = router.merge(del_thr::router(&state));
        let router = router.merge(shutdown::router(&state));
        Router::new().nest("/~~ctrl~~", router)
    }
    
    pub fn upd_allow_methods(methods: &mut HashSet<Method>) {
        methods.insert(full_save::REQ_METHOD);
        methods.insert(full_load::REQ_METHOD);
        methods.insert(add_board::REQ_METHOD);
        methods.insert(del_post::REQ_METHOD);
        methods.insert(del_thr::REQ_METHOD);
        methods.insert(shutdown::REQ_METHOD);
    }
}
