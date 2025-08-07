use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    roles::role_usecase_trait::RoleUseCaseTrait,
    users::{
        loggedin_user_dto::{LoggedInUserDto, LoggedInUserPermissonDto, LoggedInUserRoleDto},
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
pub struct IdentityUserUseCase<PH, US, RS, TH>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    RS: RoleUseCaseTrait,
    TH: TokenHelperTrait,
{
    _password_hasher: PH,
    _user_usecase: US,
    _role_usecase: RS,
    _token_helper: TH,
}

impl<PH, US, RS, TH> IdentityUserUseCase<PH, US, RS, TH>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    RS: RoleUseCaseTrait,
    TH: TokenHelperTrait,
{
    pub fn new(password_hasher: PH, user_usecase: US, role_usecase: RS, token_helper: TH) -> Self {
        Self {
            _password_hasher: password_hasher,
            _user_usecase: user_usecase,
            _role_usecase: role_usecase,
            _token_helper: token_helper,
        }
    }
}

impl<PH, US, RS, TH> IdentityUserUseCaseTrait for IdentityUserUseCase<PH, US, RS, TH>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    RS: RoleUseCaseTrait,
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
        let claims = self
            ._token_helper
            .get_access_token_info(access_token)
            .ok_or_else(|| {
                ApplicationError::new(
                    ApplicationErrorKind::InvalidInput,
                    "Failed to get token info",
                    None,
                )
            })?;

        let user = self
            ._user_usecase
            .get_user_by_email(claims.email)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::InvalidInput,
                    "Failed to get the user by email",
                    None,
                )
            })?;

        let roles = self
            ._user_usecase
            .get_user_roles_by_user_id(user.id)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::InvalidInput,
                    "Failed to get the assigned roles",
                    None,
                )
            })?
            .into_iter()
            .map(|r| LoggedInUserRoleDto {
                role_name: r.role_name,
                role_id: r.role_id,
            })
            .collect::<Vec<_>>();

        let mut permissions = self
            ._user_usecase
            .get_user_permissions_by_user_id(user.id)
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::InvalidInput,
                    "Failed to get the assigned permissions",
                    None,
                )
            })?
            .into_iter()
            .map(|p| LoggedInUserPermissonDto {
                permisson_code: p.permission_code,
                permisson_id: p.permission_id,
                permisson_name: p.permission_name,
            })
            .collect::<Vec<_>>();

        if !roles.is_empty() {
            if let Ok(role_perms) = self
                ._role_usecase
                .get_roles_permissions_by_role_ids(roles.iter().map(|r| r.role_id).collect())
                .await
            {
                permissions.extend(role_perms.into_iter().map(|p| LoggedInUserPermissonDto {
                    permisson_code: p.permission_code,
                    permisson_id: p.permission_id,
                    permisson_name: p.permission_name,
                }));
            }
        }

        Ok(LoggedInUserDto {
            email: user.email,
            name: user.name,
            display_name: user.display_name,
            id: user.id,
            roles,
            permissions,
            ..Default::default()
        })
    }
}
