
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use config::{Config, ConfigError, File,};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub debug_mode: bool,
    pub api_endpoints: ApiEndpoints,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiEndpoints {
    pub solana_rpc: String,
   
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        // Determine config file path
        let config_dir = dirs::config_dir()
            .expect("Could not find configuration directory")
            .join("solana-token-analyzer");

        // Ensure config directory exists
        std::fs::create_dir_all(&config_dir)
            .expect("Could not create configuration directory");

        // Config file path
        let config_path = config_dir.join("config.toml");

        // Create a default configuration if not exists
        if !config_path.exists() {
            let default_config = Self::default();
            let config_str = toml::to_string(&default_config)
                .expect("Failed to serialize default config");
            std::fs::write(&config_path, config_str)
                .expect("Could not write default configuration");
        }

        // Build configuration
        let settings = Config::builder()
            .add_source(File::from(config_path))
            .build()?;

        // Deserialize configuration
        settings.try_deserialize()
    }

    // Default configuration
    fn default() -> Self {
        AppConfig {
            app_name: "Solana Token Analyzer".to_string(),
            debug_mode: false,
            api_endpoints: ApiEndpoints {
                solana_rpc: "https://api.mainnet-beta.solana.com".to_string(),
                
            },
        }
    }

    // Method to get config directory
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .expect("Could not find configuration directory")
            .join("solana-token-analyzer")
    }
}