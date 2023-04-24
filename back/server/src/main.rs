use axum::{routing::get, Router, Server, http::Method, Json};
use axum::extract::State;

use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::CorsLayer;
use serde::Serialize;

use std::sync::{Arc, RwLock};

const VUE_DIST_PATH: &str = "../../front/vue_x/dist";

pub fn delay() {
    std::thread::sleep(std::time::Duration::from_millis(1_500));
}

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    total_post: u64,
    open_board: u32,
    speed_post: u32,
    unused_field: u128,
}

#[derive(Serialize, Debug)]
pub struct AllCommon {
    total_post: u64,
    open_board: u32,
    speed_post: u32,
}

pub async fn handler(
    State(state): State<SharedState>,
) -> Json<AllCommon> {
    delay();
    
    {
        // change value in GET :|
        // (it's just emul so it's okey, it's okey)
        let mut w_state = state.write().unwrap();
        w_state.total_post += 10;
        w_state.open_board = 2;
        w_state.speed_post += 1;
    }

    let value = {
        let r_state = state.read().unwrap();
        AllCommon {
            total_post: r_state.total_post,
            open_board: r_state.open_board,
            speed_post: r_state.speed_post,
        }
    };

    Json(value)
 }

#[tokio::main]
async fn main() {
    let shared_state = SharedState::default();

    let cors = CorsLayer::new().allow_methods([Method::GET]);

    let router_common_all = Router::new().route("/common/all", get(handler).with_state(Arc::clone(&shared_state)));

    let index_file = ServeFile::new(format!("{}/index.html", VUE_DIST_PATH));
    
    let router = Router::new();
    let router = router.nest_service("/", ServeDir::new(VUE_DIST_PATH).fallback(index_file));
    let router = router.nest("/api", Router::new().merge(router_common_all));
    let router = router.layer(cors).into_make_service();

    let server = Server::bind(&"127.0.0.1:5173".parse().unwrap());
    let server = server.serve(router);
    server.await.unwrap();

    // TODO:CHECK:
    // * RequestBodyLimitLayer::new(1024 * 1024 * 5 /* 5mb */),
}
