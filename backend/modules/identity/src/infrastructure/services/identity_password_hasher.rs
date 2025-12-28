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
        // OWASP recommended parameters for Argon2id:
        // - Memory cost: 65536 KB (64 MB) - strong protection against GPU attacks
        // - Time cost: 3 iterations - balance between security and performance
        // - Parallelism: 4 threads - utilize multi-core CPUs
        // - Algorithm: Argon2id - hybrid mode resistant to both side-channel and GPU attacks
        let params = ParamsBuilder::new()
            .m_cost(65536) // 64 MB memory (increased from 32 KB for better security)
            .t_cost(3)     // 3 iterations (increased from 2 for better security)
            .p_cost(4)     // 4 parallel threads (increased from 3 for better performance)
            .data(AssociatedData::new(&[0x0f; 6]).unwrap())
            .keyid(KeyId::new(&[0xf0; 4]).unwrap())
            .build()
            .unwrap();

        // Use Argon2id instead of Argon2d for better security against side-channel attacks
        // Version V0x13 is the latest and recommended version
        let ctx = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
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
