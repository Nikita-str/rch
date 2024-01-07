
#[tokio::main]
async fn main() {
    let cli = rch_server::Cli::new();
    let state = match cli.state() {
        Ok(state) => state,
        Err(e) => panic!("{e}"),
    };
    rch_server::server(state).await
}
