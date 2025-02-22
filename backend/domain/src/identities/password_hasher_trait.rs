use super::IdentityError;

pub trait PasswordHasherTrait {
    fn hash(&self, password: &str, salt: String) -> String;
    fn generate_salt(&self) -> String;
    fn verify_password(&self, password: &str, password_hash: &str) -> Result<(), IdentityError>;
}
