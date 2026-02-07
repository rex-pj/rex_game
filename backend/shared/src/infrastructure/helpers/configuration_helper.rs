use crate::domain::helpers::configuration_helper_trait::{ConfigurationHelperTrait, FromConfig};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::RwLock;

/// Default values for configuration
static DEFAULTS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("SERVER_HOST", "0.0.0.0");
    m.insert("SERVER_PORT", "3400");
    m.insert("APP_NAME", "Rex Game");
    m.insert("JWT_CLIENT_ID", "rex_game");
    m.insert("JWT_EXPIRATION", "10000000000");
    m.insert("JWT_REFRESH_EXPIRATION", "1000000000");
    m.insert("EMAIL_PROVIDER", "smtp");
    m.insert("SMTP_HOST", "smtp.gmail.com");
    m.insert("SMTP_PORT", "587");
    m.insert("PLATFORM_NAME", "Rex Game");
    m
});

/// Cache for loaded environment variables
static ENV_CACHE: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone)]
pub struct ConfigurationHelper {
    _initialized: bool,
}

impl ConfigurationHelper {
    pub fn new() -> Self {
        Self { _initialized: true }
    }

    /// Initialize the configuration by loading .env files from environments/ folder
    ///
    /// Load order (later files override earlier ones):
    /// 1. environments/.env.{APP_ENV} (environment-specific: .env.dev, .env.prod, .env.staging)
    /// 2. .env.local (local overrides at root, always gitignored)
    ///
    /// Set APP_ENV via command line or system environment:
    /// - `APP_ENV=prod cargo run`
    /// - `cargo run` (defaults to "dev")
    pub fn init() {
        // Get environment from APP_ENV, default to "dev"
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

        // 1. Load environment-specific file from environments/ folder
        let env_file = format!("environments/.env.{}", app_env);
        if Path::new(&env_file).exists() {
            match dotenvy::from_filename(&env_file) {
                Ok(_) => eprintln!("📦 Loaded environment: {} (APP_ENV={})", env_file, app_env),
                Err(e) => eprintln!("❌ Failed to load {}: {}", env_file, e),
            }
        } else {
            eprintln!("⚠️  Environment file not found: {}", env_file);
        }

        // 2. Load local overrides (at root, always gitignored)
        if Path::new(".env.local").exists() {
            match dotenvy::from_filename(".env.local") {
                Ok(_) => eprintln!("📦 Loaded local overrides: .env.local"),
                Err(e) => eprintln!("❌ Failed to load .env.local: {}", e),
            }
        }
    }

    /// Get current environment name
    pub fn get_env() -> String {
        env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string())
    }

    /// Check if running in production
    pub fn is_production() -> bool {
        Self::get_env() == "prod" || Self::get_env() == "production"
    }

    /// Check if running in development
    pub fn is_development() -> bool {
        let env = Self::get_env();
        env == "dev" || env == "development"
    }

    /// Get value from environment with caching
    fn get_env_value(key: &str) -> Option<String> {
        // Check cache first
        if let Ok(cache) = ENV_CACHE.read() {
            if let Some(value) = cache.get(key) {
                return Some(value.clone());
            }
        }

        // Try to get from environment
        if let Ok(value) = env::var(key) {
            if let Ok(mut cache) = ENV_CACHE.write() {
                cache.insert(key.to_string(), value.clone());
            }
            return Some(value);
        }

        // Check for default value
        if let Some(&default) = DEFAULTS.get(key) {
            return Some(default.to_string());
        }

        None
    }

    /// Get required value - panics if not found
    pub fn get(&self, key: &str) -> String {
        Self::get_env_value(key)
            .unwrap_or_else(|| panic!("Environment variable '{}' is not set", key))
    }

    /// Get optional value - returns empty string if not found
    pub fn get_optional(&self, key: &str) -> String {
        Self::get_env_value(key).unwrap_or_default()
    }

    /// Get array value (comma-separated)
    pub fn get_array(&self, key: &str) -> Vec<String> {
        let value = Self::get_env_value(key)
            .unwrap_or_else(|| panic!("Environment variable '{}' is not set", key));

        value
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Check if a key exists
    pub fn has(&self, key: &str) -> bool {
        Self::get_env_value(key).is_some()
    }
}

impl Default for ConfigurationHelper {
    fn default() -> Self {
        Self::new()
    }
}

// Trait implementation for generic usage
impl ConfigurationHelperTrait for ConfigurationHelper {
    fn get_value<T: FromConfig>(&self, key: &str) -> T {
        let value = Self::get_env_value(key)
            .unwrap_or_else(|| panic!("Environment variable '{}' is not set", key));
        T::from_config(value).unwrap()
    }

    fn get_raw_value(&self, key: &str) -> String {
        Self::get_env_value(key)
            .unwrap_or_else(|| panic!("Environment variable '{}' is not set", key))
    }

    fn get_array(&self, key: &str) -> Vec<String> {
        self.get_array(key)
    }
}
