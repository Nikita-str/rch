mod app_state;
mod post;
mod api;

const VUE_DIST_PATH: &str = "../../front/vue_x/dist";


#[allow(unused)] use fns::{delay, delay_ms};
pub use fns::{server};
mod fns {
    use super::{api, app_state, VUE_DIST_PATH};
    use axum::{Server, Router};
    use tower_http::services::{ServeDir, ServeFile};
    use tower_http::cors::CorsLayer;
    use std::sync::{Arc, RwLock};

    pub(in crate) fn delay() {
        std::thread::sleep(std::time::Duration::from_millis(1_500));
    }
        
    pub(in crate) fn delay_ms(ms: usize) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }

    pub async fn server() {
        let state_all = app_state::CommonInfoState::new(0, 0, 20, 2);
        let state_all = Arc::new(RwLock::new(state_all));

        let index_file = ServeFile::new(format!("{}/index.html", VUE_DIST_PATH));
        let serve_dir = ServeDir::new(VUE_DIST_PATH).fallback(index_file);

        let mut methods = std::collections::HashSet::new();
        api::upd_allow_methods(&mut methods);
        let cors = CorsLayer::new().allow_methods(methods.drain().collect::<Vec<_>>());

        let router = Router::new();
        let router = router.nest_service("/", serve_dir);
        let router = router.nest("/api", api::router(&state_all));
        let router = router.layer(cors).into_make_service();

        let server = Server::bind(&"127.0.0.1:5173".parse().unwrap());
        let server = server.serve(router);
        server.await.unwrap();
    }
}