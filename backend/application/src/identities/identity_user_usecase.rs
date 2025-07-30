use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    users::{
        loggedin_user_dto::{LoggedInUserDto, LoggedInUserRoleDto},
        user_creation_dto::UserCreationDto,
        user_usecase_trait::UserUseCaseTrait,
    },
};

use super::{
    identity_user_trait::IdentityUserTrait, identity_user_usecase_trait::IdentityUserUseCaseTrait,
};
use rex_game_domain::{
    identities::{
        password_hasher_trait::PasswordHasherTrait, token_helper_trait::TokenHelperTrait,
    },
    transaction_manager_trait::TransactionWrapperTrait,
};

#[derive(Clone)]
pub struct IdentityUserUseCase<PH, US, TH>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    TH: TokenHelperTrait,
{
    _password_hasher: PH,
    _user_usecase: US,
    _token_helper: TH,
}

impl<PH, US, TH> IdentityUserUseCase<PH, US, TH>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    TH: TokenHelperTrait,
{
    pub fn new(password_hasher: PH, user_usecase: US, token_helper: TH) -> Self {
        Self {
            _password_hasher: password_hasher,
            _user_usecase: user_usecase,
            _token_helper: token_helper,
        }
    }
}

impl<PH, US, TH> IdentityUserUseCaseTrait for IdentityUserUseCase<PH, US, TH>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    TH: TokenHelperTrait,
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
                    ApplicationErrorKind::InvalidInput,
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
                    ApplicationErrorKind::InvalidInput,
                    "Create user failed",
                    None,
                ))
            }
        };

        user.set_id(created_id);
        Ok(user)
    }

    async fn get_logged_in_user(
        &self,
        access_token: &str,
    ) -> Result<LoggedInUserDto, ApplicationError> {
        let access_token_info = match self._token_helper.get_access_token_info(access_token) {
            Some(claims) => claims,
            None => {
                return Err(ApplicationError {
                    kind: ApplicationErrorKind::InvalidInput,
                    message: String::from("Failed to get token info"),
                    details: None,
                })
            }
        };

        let user = self
            ._user_usecase
            .get_user_by_email(access_token_info.email)
            .await
            .map_err(|_| ApplicationError {
                kind: ApplicationErrorKind::InvalidInput,
                message: String::from("Failed to get the user by email"),
                details: None,
            })?;

        let assigned_roles: Vec<LoggedInUserRoleDto> = self
            ._user_usecase
            .get_user_roles_by_user_id(user.id)
            .await
            .map_err(|_| ApplicationError {
                kind: ApplicationErrorKind::InvalidInput,
                message: String::from("Failed to get the assigned roles"),
                details: None,
            })?
            .iter()
            .map(|f| LoggedInUserRoleDto {
                role_name: f.role_name.to_owned(),
                role_id: f.role_id,
            })
            .collect();

        let logged_in_result = LoggedInUserDto {
            email: user.email,
            name: user.name,
            display_name: user.display_name,
            id: user.id,
            roles: assigned_roles,
            ..Default::default()
        };

        return Ok(logged_in_result);
    }
}
