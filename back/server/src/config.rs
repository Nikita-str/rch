use serde::Deserialize;
use anyhow::Result;
use crate::{KB, MB};

const CONFIG_PATH: &'static str = "Config.toml";
static CONFIG: std::sync::OnceLock<Config> = std::sync::OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Vue {
    pub dist_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Imageboard {
    pub bump_limit: usize,
    pub max_header_len: usize,
    pub max_pic_qty: usize,
    max_pic_size: Option<usize>,
    max_mini_pic_size: Option<usize>,
    pub max_board_url_len: usize,
    pub saves: Saves,
    pub api: Api,
    pub loops: LoopInfo,
}

#[derive(Debug, Deserialize)]
pub struct LoopInfo {
    pub auto_save_dt_sec: u64,
    pub auto_del_dt_sec: u64,
    pub rate_post_dt_sec: u64,
}

#[derive(Debug, Deserialize)]
pub struct Saves {
    pub dir: String,
    pub aux_dir: String,
    pub single_use_pwd_file: String,
    pub save_names: Vec<String>,
    pub close_save_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Api {
    pub thrs_load: ApiThrsLoad,
    pub thr_load: ApiThrLoad,
}

#[derive(Debug, Deserialize)]
pub struct ApiThrsLoad {
    pub min_thrs_load: usize,
    pub max_thrs_load: usize,
    pub qty_show_post: usize,
}

#[derive(Debug, Deserialize)]
pub struct ApiThrLoad {
    pub min_posts_load: usize,
    pub max_posts_load: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub vue: Vue,
    pub imageboard: Imageboard,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_str = std::fs::read_to_string(CONFIG_PATH)?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }
    pub fn init() {
        let _ = CONFIG.set(Self::new().unwrap());
    }   

    pub fn config() -> &'static Self {
        if let Some(config) = CONFIG.get() { return config; }
        Self::init();
        return CONFIG.get().unwrap()
    }

    pub fn saves() -> &'static Saves {
        &config().imageboard.saves
    }
    pub fn api() -> &'static Api {
        &config().imageboard.api
    }
    pub fn loops() -> &'static LoopInfo {
        &config().imageboard.loops
    }

    pub fn vue_dist_path() -> &'static str {
        &Self::config().vue.dist_path
    }
    
    pub fn max_pic_sz() -> usize {
        pub const DEFAULT: usize = 2 * MB;
        config().imageboard.max_pic_size.map(|kb|kb * KB).unwrap_or(DEFAULT)
    }
    pub fn max_mini_pic_sz() -> usize {
        pub const DEFAULT: usize = 50 * KB;
        config().imageboard.max_mini_pic_size.map(|kb|kb * KB).unwrap_or(DEFAULT)
    }
}

pub fn addr() -> std::net::SocketAddr {
    let config = rch_config::Config::open(CONFIG_PATH).unwrap();
    config.socket_addr().unwrap()
}

pub fn config() -> &'static Config {
    Config::config()
}

pub trait ConfigCtor<Args = ()> {
    fn config_new(args: Args) -> Self;
}

// pub trait ConfigEmptyCtor: ConfigCtor<()> + Sized {
//     fn config_new() -> Self {
//         <Self as ConfigCtor>::config_new(())
//     }
// }
