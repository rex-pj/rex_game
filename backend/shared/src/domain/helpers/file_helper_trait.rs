use std::io::ErrorKind;

pub trait FileHelperTrait {
    fn get_string(&self, path: &str) -> Option<String>;
    fn save_string(&self, path: &str, value: &str) -> Result<(), ErrorKind>;
    fn has_file(&self, path: &str) -> bool;
    fn get_content_type(&self, data: &[u8]) -> Option<String>;
}
