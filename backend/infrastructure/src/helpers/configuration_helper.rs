use config::{Config, File};
use rex_game_domain::helpers::configuration_helper_trait::ConfigurationHelperTrait;

#[derive(Clone)]
pub struct ConfigurationHelper;

impl ConfigurationHelper {
    pub fn new() -> Self {
        Self
    }

    pub fn get_value(&self, key: &str) -> String {
        ConfigurationHelperTrait::get_value(self, key)
    }
}

impl ConfigurationHelperTrait for ConfigurationHelper {
    fn get_value(&self, key: &str) -> String {
        let config_file = File::with_name("src/config.toml");
        let settings = Config::builder()
            .add_source(config_file)
            .build()
            .expect("Failed to load configuration");

        let error_message = key.to_owned() + "'s value is missing";
        settings.get_string(key).expect(error_message.as_str())
    }
}
