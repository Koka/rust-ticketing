use eyre::Report;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server_addr: String,
    pub jwt_secret: String,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, Report> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        Ok(cfg.try_into()?)
    }
}
