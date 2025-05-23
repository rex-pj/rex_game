pub trait ConfigurationHelperTrait {
    fn get_value(&self, key: &str) -> String;
    fn get_array(&self, key: &str) -> Vec<String>;
}
