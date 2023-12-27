mod app_state;
mod post;
mod api;
mod thread;
mod thread_usage_rate;
mod preproc;
pub mod utility;
mod config;
mod security;

const VUE_DIST_PATH: &str = "../../front/vue_x/dist";
// const PIC_PATH: &str = "../../front/vue_x/dist/imgs/pp"; // pp stands for post pics 

const KB: usize = 1024;
const MB: usize = 1024 * KB;

static SHUTDOWN: std::sync::OnceLock<tokio::sync::mpsc::UnboundedSender<()>> = std::sync::OnceLock::new();

#[allow(unused)] use fns::{delay, delay_ms};
pub use fns::server;
mod fns {
    use crate::app_state::open_boards::Board;
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
        #[allow(unused_must_use)]
        let open_boards = {
            let std_thr_qty = 10; // 100;
            let small_thr_qty = 5; // 20;

            let mut open_boards = crate::app_state::OpenBoards::new(VUE_DIST_PATH);
            let tag = crate::app_state::open_boards::BoardTag { tag: "Разное".into() };
            let url = "b".into();
            let name = "Бред".into();
            let descr = "Бредим вместе!".into();
            let post_qty = 243;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, std_thr_qty),
                Some(tag.clone()),
            );
            let url = "soc".into();
            let name = "Общение".into();
            let descr = "бла-бла-бла и подобное".into();
            let post_qty = 107;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, std_thr_qty),
                Some(tag.clone()),
            );

            let tag = crate::app_state::open_boards::BoardTag { tag: "IT".into() };
            let url = "pr".into();
            let name = "Погроммммирование".into();
            let descr = "Сидим. Кодим".into();
            let post_qty = 23;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, std_thr_qty),
                Some(tag.clone()),
            );

            let tag = crate::app_state::open_boards::BoardTag { tag: "Игры".into() };
            let url = "bg".into();
            let name = "Настольные игры".into();
            let descr = "Игры на столе o_O?".into();
            let post_qty = 6;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );
            let url = "vn".into();
            let name = "Визуальные новеллы".into();
            let descr = "*> baka\n*> ...".into();
            let post_qty = 7;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );
            let url = "rp".into();
            let name = "role play".into();
            let descr = "кидани d20 на капчевание в часах!".into();
            let post_qty = 7;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );
            
            let tag = crate::app_state::open_boards::BoardTag { tag: "Всякое".into() };
            let url = "car".into();
            let name = "Автомобили".into();
            let descr = "авто<s>боты</s>мобили".into();
            let post_qty = 4;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );
            let url = "bik".into();
            let name = "Велосипеды".into();
            let descr = "А чо тут сказать? Ну педали крутим.".into();
            let post_qty = 18;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );
            let url = "sp".into();
            let name = "Спорт".into();
            let descr = "Делай 200 отжиманий раз прочитал".into();
            let post_qty = 18;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );
            
            let tag = crate::app_state::open_boards::BoardTag { tag: "Япония".into() };
            let url = "a".into();
            let name = "Аниме".into();
            let descr = "Японская анимация(мультики(для детей(детские)))\n( ´ ▽ ` )".into();
            let post_qty = 29;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, std_thr_qty),
                Some(tag.clone()),
            );
            let url = "ma".into();
            let name = "Манга".into();
            let descr = "Серьезное чтиво\n(；⌣̀_⌣́)".into();
            let post_qty = 18;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );
            let url = "ja".into();
            let name = "Японская культура".into();
            let descr = "Школьницы школьницы школьницы".into();
            let post_qty = 18;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                Some(tag.clone()),
            );

            let tag = crate::app_state::open_boards::BoardTag { tag: "Политика".into() };
            let url = "po".into();
            let name = "П<s>о</s>лит<s>и</s>ка".into();
            let descr = "Перекладываем".into();
            let post_qty = 23;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, std_thr_qty),
                Some(tag.clone()),
            );

            let url = "touhou".into();
            let name = "Touhou".into();
            let descr = "Выясняем ~~baaaka~~ ли Сырник?".into();
            let post_qty = 2;
            open_boards.add_board(
                Board::new(url, name, descr, post_qty, small_thr_qty),
                None,
            );

            open_boards
        };
        let speed_post = crate::app_state::SpeedPost::new(dt_sec, 0);
        let state_all = app_state::CommonInfoState::new(deleted_board_post, open_boards, speed_post);
        let pic_path_parent = state_all.pic_path_parent().to_owned();
        let state_all = Arc::new(RwLock::new(state_all));

        let mut loop_acts = crate::utility::action_loop::LoopActs::new();
        let act = crate::utility::action_loop::file_deleter::FileDelState::new(pic_path_parent);
        let dur = crate::utility::action_loop::file_deleter::DELETE_LOOP_DUR;
        loop_acts.add(act, dur);
        let act = crate::utility::action_loop::auto_saver::AutoSaver::new_std(&state_all);
        let dur = crate::utility::action_loop::auto_saver::SAVE_LOOP_DUR;
        loop_acts.add(act, dur);
        loop_acts.init();
        let cmd_loop_ctrl = crate::utility::ActionLooper::new(loop_acts);

        let (shoutdown_sx, mut shoutdown_rx) = tokio::sync::mpsc::unbounded_channel();
        crate::SHUTDOWN.set(shoutdown_sx).unwrap();

        // std::fs::create_dir(PIC_PATH).unwrap();
        let index_file = ServeFile::new(format!("{}/index.html", VUE_DIST_PATH));
        let serve_dir = ServeDir::new(VUE_DIST_PATH).fallback(index_file);

        let mut methods = std::collections::HashSet::new();
        api::upd_allow_methods(&mut methods);
        let cors = CorsLayer::new().allow_methods(methods.drain().collect::<Vec<_>>());

        let router = Router::new();
        let router = router.nest_service("/", serve_dir);
        let router = router.nest("/api", api::router(&state_all));
        let router = router.layer(cors).into_make_service();


        let config = crate::config::Config::open().unwrap();
        config.gen_rest_test_py().unwrap();
        let addr = config.socket_addr().unwrap();
        let server = Server::bind(&addr);
        let server = server.serve(router);

        let upd_speed_post_loop = upd_speed_post_loop(&state_all, dt_sec / 2);

        let graceful = server.with_graceful_shutdown(async { shoutdown_rx.recv().await; });
        tokio::select!{
            // _ = shoutdown_rx.recv() => { println!("[SHOUTDOWN]"); }
            // _ = server => { }
            _ = graceful => { println!("[SHOUTDOWN]"); }
            _ = upd_speed_post_loop => {
                println!("smth weird is occurs: turned out that inf loop isn't inf :|");
            }
        };
        let _ = cmd_loop_ctrl.close_async().await;
        println!("[CLOSED]");
    }
}

macro_rules! define_id {
    ([IMPL] $name:ident) => {
        impl $name {
            pub fn first() -> Self { Self(1) }
            pub fn next(self) -> Self { Self(self.0 + 1) }
            pub fn inc(&mut self) -> Self { 
                let ret = *self; 
                *self = self.next(); 
                ret 
            }
        }
    };
    ([INNER] $name:ident: $ty:ty) => {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        pub struct $name($ty);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#{}", self.0)
            }
        }
        impl Into<$ty> for $name {
            fn into(self) -> $ty {
                self.0
            }
        }
        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }
    };
    ($name:ident: $ty:ty) => {
        crate::define_id!([INNER] $name: $ty);
        crate::define_id!([IMPL] $name);
    };
    ($name:ident: $ty:ty [NO IMPL]) => {
        crate::define_id!([INNER] $name: $ty);
    };
    ($name:ident) => { crate::define_id!($name: usize); };
}
pub(crate) use define_id;

macro_rules! some_or_ret {
    ($test:expr => $err:expr) => {
        match $test {
            Some(x) => x,
            None => return Err($err.into()),
        }
    };
}
use some_or_ret;
