use super::{
    identity_user_trait::IdentityUserTrait, identity_user_usecase_trait::IdentityUserUseCaseTrait,
};
use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    roles::role_usecase_trait::RoleUseCaseTrait,
    users::{
        loggedin_user_dto::{LoggedInUserDto, LoggedInUserPermissonDto, LoggedInUserRoleDto},
        user_creation_dto::UserCreationDto,
        user_usecase_trait::UserUseCaseTrait,
    },
};
use rex_game_domain::{
    identities::{
        password_hasher_trait::PasswordHasherTrait, token_helper_trait::TokenHelperTrait,
    },
    transaction_manager_trait::TransactionWrapperTrait,
};
use rex_game_shared::utils::types::BoxFuture;

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
    US: UserUseCaseTrait + Send + Sync + Clone + 'static,
    RS: RoleUseCaseTrait + Send + Sync + Clone + 'static,
    TH: TokenHelperTrait + Send + Sync + Clone + 'static,
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
                    status_id: user.status_id(),
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

        let created_id = self
            ._user_usecase
            .create_user(UserCreationDto {
                display_name: user.display_name().map(|f| String::from(f)),
                email: String::from(user.email()),
                name: String::from(user.name()),
                password: String::from(user.password_hash()),
                security_stamp: String::from(user.security_stamp()),
                status_id: user.status_id(),
            })
            .await
            .map_err(|_| {
                ApplicationError::new(
                    ApplicationErrorKind::InvalidInput,
                    "Create user failed",
                    None,
                )
            })?;

        user.set_id(created_id);
        Ok(user)
    }

    fn get_logged_in_user<'a>(
        &'a self,
        access_token: &'a str,
    ) -> BoxFuture<'a, Result<LoggedInUserDto, ApplicationError>> {
        let token_helper = self._token_helper.clone();
        let user_usecase = self._user_usecase.clone();
        let role_usecase = self._role_usecase.clone();
        Box::pin(async move {
            let claims = token_helper.validate_token(access_token).map_err(|err| {
                ApplicationError::new(ApplicationErrorKind::InvalidInput, &err.message, None)
            })?;

            let email = claims.email.ok_or_else(|| {
                ApplicationError::new(ApplicationErrorKind::NotFound, "No email found", None)
            })?;

            let user = match user_usecase.get_user_by_email(&email).await {
                Err(_) => {
                    return Err(ApplicationError::new(
                        ApplicationErrorKind::InvalidInput,
                        "Failed to get the user by email",
                        None,
                    ));
                }
                Ok(user) => user,
            };

            let roles = user_usecase
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

            let mut permissions = user_usecase
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
                if let Ok(role_perms) = role_usecase
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
        })
    }
}
