use eyre::Report;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, Report> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        Ok(cfg.try_into()?)
    }
}
