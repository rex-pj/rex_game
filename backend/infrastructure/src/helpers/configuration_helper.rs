use config::{Config, File};
pub struct ConfigurationHelper;

impl ConfigurationHelper {
    pub fn get_value(key: &str) -> String {
        let config_file = File::with_name("src/config.toml");
        let settings = Config::builder()
            .add_source(config_file)
            .build()
            .expect("Failed to load configuration");

        let error_message = key.to_owned() + "'s value is missing";
        settings.get_string(key).expect(error_message.as_str())
    }
}
