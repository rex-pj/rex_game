use std::env;

use config::{Config, File};
use rex_game_domain::helpers::configuration_helper_trait::ConfigurationHelperTrait;

#[derive(Clone)]
pub struct ConfigurationHelper {
    _config: Config,
}

impl ConfigurationHelper {
    fn get_host() -> String {
        let args: Vec<String> = env::args().collect();
        if args.len() == 0 {
            return String::from("");
        }

        match args.iter().find(|&x| String::from(x).contains("host=")) {
            Some(value) => {
                let stripped_value = value.strip_prefix("host=").unwrap_or("");
                let host_value = ".".to_owned() + stripped_value;

                return host_value;
            }
            None => String::from(""),
        }
    }

    fn get_config() -> Config {
        let host = Self::get_host();
        let config_path = "src/config".to_owned() + host.as_str() + ".toml";
        let config_file = File::with_name(config_path.as_str());
        let settings = Config::builder()
            .add_source(config_file)
            .build()
            .expect("Failed to load configuration");

        settings
    }

    pub fn new() -> Self {
        Self {
            _config: Self::get_config(),
        }
    }

    pub fn get_value(&self, key: &str) -> String {
        ConfigurationHelperTrait::get_value(self, key)
    }

    pub fn get_array(&self, key: &str) -> Vec<String> {
        ConfigurationHelperTrait::get_array(self, key)
    }
}

impl ConfigurationHelperTrait for ConfigurationHelper {
    fn get_value(&self, key: &str) -> String {
        let error_message = key.to_owned() + "'s value is missing";
        self._config.get_string(key).expect(error_message.as_str())
    }

    fn get_array(&self, key: &str) -> Vec<String> {
        let error_message = key.to_owned() + "'s value is missing";
        self._config
            .get_array(key)
            .expect(&error_message)
            .into_iter()
            .map(|value| value.to_string())
            .collect()
    }
}
