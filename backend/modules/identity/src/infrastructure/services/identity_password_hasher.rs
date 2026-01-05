use crate::domain::services::password_hasher_trait::PasswordHasherTrait;
use argon2::password_hash::rand_core::OsRng;
use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use argon2::{Algorithm, AssociatedData, KeyId, ParamsBuilder, PasswordHash, Version};
use rex_game_shared::ApplicationError;

impl IdentityPasswordHasher {
    pub fn new() -> Self {
        Self
    }

    fn hasher_context(&self) -> Argon2<'_> {
        let params = ParamsBuilder::new()
            .m_cost(32)
            .t_cost(2)
            .p_cost(3)
            .data(AssociatedData::new(&[0x0f; 6]).unwrap())
            .keyid(KeyId::new(&[0xf0; 4]).unwrap())
            .build()
            .unwrap();

        let ctx = Argon2::new(Algorithm::Argon2d, Version::V0x10, params);
        ctx
    }
}

impl PasswordHasherTrait for IdentityPasswordHasher {
    fn hash(&self, password: &str, salt: String) -> String {
        let salt_string = SaltString::encode_b64(&salt.as_bytes()).unwrap();
        self.hasher_context()
            .hash_password(password.as_bytes(), &salt_string)
            .unwrap()
            .to_string()
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> Result<(), ApplicationError> {
        if password.is_empty() || password_hash.is_empty() {
            return Err(ApplicationError::invalid_input(
                "Password verification failed",
            ));
        }

        let parsed_hash = PasswordHash::new(&password_hash).unwrap();
        match self
            .hasher_context()
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(ApplicationError::InvalidCredentials),
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
