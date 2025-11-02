use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub users: HashMap<String, user::ConfigUser>,
    #[serde(default)]
    pub db_url: String,
}

impl Config {
    pub fn load_from_env() -> anyhow::Result<Arc<Self>> {
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL var not set")?;
        let config_path_string = std::env::var("CONFIG_PATH").context("CONFIG_PATH var not set")?;
        let config_path = PathBuf::from(config_path_string);
        let config_string = std::fs::read_to_string(config_path)?;

        let mut config: Config = toml::from_str(&config_string)?;
        config.db_url = db_url;
        Ok(Arc::new(config))
    }

    pub fn get_user_or_err(&self, user_id: &str) -> anyhow::Result<&user::ConfigUser> {
        self.users
            .get(user_id)
            .context(format!("User '{user_id}' does not exist in config."))
    }
}
