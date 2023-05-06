
#[tokio::main]
async fn main() {
    rch_server::server().await

    // TODO:CHECK:
    // * RequestBodyLimitLayer::new(1024 * 1024 * 5 /* 5mb */),
}
