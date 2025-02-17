use std::string::FromUtf8Error;

use argon2::password_hash::rand_core::OsRng;
use argon2::{password_hash::SaltString, Argon2};
use rex_game_domain::identities::password_hasher_trait::PasswordHasherTrait;

impl IdentityPasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasherTrait for IdentityPasswordHasher {
    fn hash(&self, password: &str, key_size: usize, salt: String) -> Result<String, FromUtf8Error> {
        let mut key = vec![0u8; key_size];
        Argon2::default()
            .hash_password_into(password.as_bytes(), &salt.into_bytes(), &mut key)
            .unwrap();

        match String::from_utf8(key) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    fn generate_salt(&self) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let salt_str = salt.to_string();

        salt_str
    }
}

#[derive(Clone)]
pub struct IdentityPasswordHasher;
