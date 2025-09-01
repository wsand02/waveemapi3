use serde::{Deserialize, Serialize};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "data");

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub auth_tokens: Vec<String>,
    pub auth_enabled: bool,
    pub data_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth_tokens: vec![],
            auth_enabled: true,
            data_path: ROOT.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.data_path, ROOT);
        assert!(config.auth_enabled);
        assert!(config.auth_tokens.is_empty());
    }
}
