use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub auth_tokens: Vec<String>,
    pub auth_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth_tokens: vec![],
            auth_enabled: true,
        }
    }
}
