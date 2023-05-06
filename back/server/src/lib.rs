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

    fn upd_speed_post_loop(state: &api::common::all::HandlerState, upd_dt_sec: u32) -> tokio::task::JoinHandle<()> {
        let state = Arc::clone(state);

        tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(upd_dt_sec as u64));
            loop {
                interval.tick().await;
                {
                    let mut w_state = state.write().unwrap();
                    w_state.upd_speed_post();
                }
            }
        })
    }

    pub async fn server() {
        let deleted_board_post = 42;
        let dt_sec = 60;
        let open_boards = {
            let mut open_boards = crate::app_state::OpenBoards::new();
            let tag = crate::app_state::open_boards::BoardTag { tag: "Разное".into() };
            open_boards.add_board(
                crate::app_state::open_boards::Board{
                    url: "b".into(),
                    name: "Бред".into(),
                    descr: "Бредим вместе!".into(),
                    post_qty: 243,
                },
                Some(tag.clone()),
            );
            open_boards.add_board(
                crate::app_state::open_boards::Board{
                    url: "soc".into(),
                    name: "Общение".into(),
                    descr: "бла-бла-бла и подобное".into(),
                    post_qty: 107,
                },
                Some(tag.clone()),
            );

            let tag = crate::app_state::open_boards::BoardTag { tag: "IT".into() };
            open_boards.add_board(
                crate::app_state::open_boards::Board{
                    url: "pr".into(),
                    name: "Погроммммирование".into(),
                    descr: "Сидим. Кодим".into(),
                    post_qty: 23,
                },
                Some(tag.clone()),
            );

            open_boards.add_board(
                crate::app_state::open_boards::Board{
                    url: "touhou".into(),
                    name: "Touhou".into(),
                    descr: "Выесняем ~~baaaka~~ ли Сырник?".into(),
                    post_qty: 2,
                },
                None,
            );

            open_boards
        };
        let speed_post = crate::app_state::SpeedPost::new(dt_sec, 0);
        let state_all = app_state::CommonInfoState::new(deleted_board_post, open_boards, speed_post);

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

        let upd_speed_post_loop = upd_speed_post_loop(&state_all, dt_sec / 2);

        tokio::select!{
            _ = server => { return }
            _ = upd_speed_post_loop => {
                println!("smth weird is occurs: turned out that inf loop isn't inf :|");
            }
        };
    }
}