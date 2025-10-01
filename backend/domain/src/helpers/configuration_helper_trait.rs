pub trait FromConfig: Sized {
    fn from_config(s: String) -> Result<Self, String>;
}

impl FromConfig for String {
    fn from_config(s: String) -> Result<Self, String> {
        Ok(s)
    }
}

impl FromConfig for i32 {
    fn from_config(s: String) -> Result<Self, String> {
        s.parse::<i32>().map_err(|e| e.to_string())
    }
}

impl FromConfig for i64 {
    fn from_config(s: String) -> Result<Self, String> {
        s.parse::<i64>().map_err(|e| e.to_string())
    }
}

impl FromConfig for bool {
    fn from_config(s: String) -> Result<Self, String> {
        s.parse::<bool>().map_err(|e| e.to_string())
    }
}
pub trait ConfigurationHelperTrait {
    fn get_raw_value(&self, key: &str) -> String;
    fn get_value<T: FromConfig>(&self, key: &str) -> T;
    fn get_array(&self, key: &str) -> Vec<String>;
}
