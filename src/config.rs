use serde::{Deserialize, Serialize};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "data");

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub auth_tokens: Vec<String>,
    pub auth_enabled: bool,
    pub data_path: String,
    pub cleanup_interval_minutes: u64,
    pub file_expiry_minutes: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth_tokens: vec![],
            auth_enabled: true,
            data_path: ROOT.to_string(),
            cleanup_interval_minutes: 15,
            file_expiry_minutes: 60,
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
        assert!(config.cleanup_interval_minutes == 15);
        assert!(config.file_expiry_minutes == 60);
    }
}
