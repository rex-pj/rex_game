use crate::{
    errors::application_error::{ApplicationError, ErrorKind},
    users::{user_creation_dto::UserCreationDto, user_usecase_trait::UserUseCaseTrait},
};

use super::{
    identity_user_trait::IdentityUserTrait, identity_user_usecase_trait::IdentityUserUseCaseTrait,
};
use rex_game_domain::{
    identities::password_hasher_trait::PasswordHasherTrait,
    transaction_manager_trait::TransactionWrapperTrait,
};

#[derive(Clone)]
pub struct IdentityUserUseCase<PH, US>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
{
    _password_hasher: PH,
    _user_usecase: US,
}

impl<PH: PasswordHasherTrait, US: UserUseCaseTrait> IdentityUserUseCase<PH, US> {
    pub fn new(password_hasher: PH, user_usecase: US) -> Self {
        Self {
            _password_hasher: password_hasher,
            _user_usecase: user_usecase,
        }
    }
}

impl<PH: PasswordHasherTrait, US: UserUseCaseTrait> IdentityUserUseCaseTrait
    for IdentityUserUseCase<PH, US>
{
    async fn create_user_with_transaction<UT: IdentityUserTrait<K>, K>(
        &self,
        mut user: UT,
        password: &str,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> Result<UT, ApplicationError> {
        let salt = self._password_hasher.generate_salt();
        user.set_security_stamp(&salt);
        let password_hash_result = self._password_hasher.hash(password, salt);
        user.set_password_hash(&password_hash_result);

        let created_id = match self
            ._user_usecase
            .create_user_with_transaction(
                UserCreationDto {
                    display_name: user.display_name().map(|f| String::from(f)),
                    email: String::from(user.email()),
                    name: String::from(user.name()),
                    password: String::from(user.password_hash()),
                    security_stamp: String::from(user.security_stamp()),
                },
                transaction,
            )
            .await
        {
            Ok(id) => id,
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::InvalidInput,
                    "Create user failed",
                    None,
                ))
            }
        };

        user.set_id(created_id);
        Ok(user)
    }

    async fn create_user<UT: IdentityUserTrait<K>, K>(
        &self,
        mut user: UT,
        password: &str,
    ) -> Result<UT, ApplicationError> {
        let salt = self._password_hasher.generate_salt();
        user.set_security_stamp(&salt);
        let password_hash_result = self._password_hasher.hash(password, salt);
        user.set_password_hash(&password_hash_result);

        let created_id = match self
            ._user_usecase
            .create_user(UserCreationDto {
                display_name: user.display_name().map(|f| String::from(f)),
                email: String::from(user.email()),
                name: String::from(user.name()),
                password: String::from(user.password_hash()),
                security_stamp: String::from(user.security_stamp()),
            })
            .await
        {
            Ok(id) => id,
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::InvalidInput,
                    "Create user failed",
                    None,
                ))
            }
        };

        user.set_id(created_id);
        Ok(user)
    }
}
