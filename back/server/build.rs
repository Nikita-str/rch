const CONFIG_PATH: &str = "Config.toml";
const REST_PY_CONFIG_PATH: &str = "test/REST/rest.py";

fn main() {
    println!("cargo:rerun-if-changed=Config.toml");
    let config = rch_config::Config::open(CONFIG_PATH).unwrap();
    config.gen_rest_test_py(REST_PY_CONFIG_PATH).unwrap();
}