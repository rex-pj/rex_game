use std::string::FromUtf8Error;

pub trait PasswordHasherTrait {
    fn hash(&self, password: &str, key_size: usize, salt: String) -> Result<String, FromUtf8Error>;
    fn generate_salt(&self) -> String;
}
