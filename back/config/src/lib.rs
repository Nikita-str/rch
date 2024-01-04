use serde::Deserialize;
use anyhow::Result;
use std::io::Write;

#[derive(Debug, Deserialize)]
enum ConnType {
    Http,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    conn_type: ConnType,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    server: ServerConfig,
}

impl Config {
    pub fn open(config_path: impl AsRef<std::path::Path>) -> Result<Self> {
        let config_str = std::fs::read_to_string(config_path)?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }
    pub fn gen_rest_test_py(&self, rest_py_conf_path: impl AsRef<std::path::Path>) -> Result<()> {
        let mut py = std::fs::File::create(rest_py_conf_path)?;
        match self.server.conn_type {
            ConnType::Http => {
                let host = &self.server.host;
                let port = &self.server.port;
                writeln!(py, "import http.client")?;
                writeln!(py, "conn = http.client.HTTPConnection(\"{host}:{port}\")")?;
            }
        }
        Ok(())
    }
    pub fn socket_addr(&self) -> Result<std::net::SocketAddr> {
        let port = self.server.port;
        let host = &self.server.host;
        Ok(format!("{host}:{port}").parse()?)
    }
}
