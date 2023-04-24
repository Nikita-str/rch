use axum::{routing::get, Router, Server, http::Method, Json};

use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::CorsLayer;
use serde::Serialize;

const VUE_DIST_PATH: &str = "../../front/vue_x/dist";

pub fn delay() {
    std::thread::sleep(std::time::Duration::from_millis(1_500));
}

#[derive(Serialize, Debug)]
pub struct AllCommon {
    total_post: u64,
    open_board: u32,
    speed_post: u32,
}

pub async fn handler() -> Json<AllCommon> {
    delay();
    Json(AllCommon {
        total_post: 10,
        open_board: 20,
        speed_post: 30,
    })
 }

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_methods([Method::GET]);

    let router_common_all = Router::new().route("/common/all", get(handler));

    let index_file = ServeFile::new(format!("{}/index.html", VUE_DIST_PATH));
    
    let router = Router::new();
    let router = router.nest_service("/", ServeDir::new(VUE_DIST_PATH).fallback(index_file));
    let router = router.nest("/api", Router::new().merge(router_common_all));
    let router = router.layer(cors).into_make_service();

    let server = Server::bind(&"127.0.0.1:5173".parse().unwrap());
    let server = server.serve(router);
    server.await.unwrap();
}
