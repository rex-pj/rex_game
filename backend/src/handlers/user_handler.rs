use crate::validators::validation_helper::ValidationHelper;
use crate::view_models::users::assign_permission_request::AssignPermissionRequest;
use crate::view_models::users::assign_role_request::AssignRoleRequest;
use crate::view_models::users::confirm_user_request::ConfirmUserRequest;
use crate::view_models::users::current_user::CurrentUser;
use crate::view_models::users::forgot_password_request::ForgotPasswordRequest;
use crate::view_models::users::reset_password_request::ResetPasswordRequest;
use crate::view_models::{HandlerError, HandlerResult};
use crate::{app_state::AppStateTrait, view_models::users::signup_request::SignupRequest};
use axum::extract::{Path, Query};
use axum::Extension;
use axum::{extract::State, Json};
use chrono::{DateTime, Duration, Utc};
use hyper::StatusCode;
use rex_game_application::identities::identity_user_token_usecase_trait::IdentityUserTokenUseCaseTrait;
use rex_game_application::identities::identity_user_usecase_trait::IdentityUserUseCaseTrait;
use rex_game_application::identities::user_creation_dto::UserCreationDto;
use rex_game_application::identities::user_token_creation_dto::UserTokenCreationDto;
use rex_game_application::identities::user_token_updation_dto::UserTokenUpdationDto;
use rex_game_application::mail_templates::mail_template_usecase_trait::MailTemplateUseCaseTrait;
use rex_game_application::page_list_dto::PageListDto;
use rex_game_application::permissions::permission_usecase_trait::PermissionUseCaseTrait;
use rex_game_application::roles::role_usecase_trait::RoleUseCaseTrait;
use rex_game_application::roles::roles::ROLE_ROOT_ADMIN;
use rex_game_application::users::user_deletion_dto::UserDeletionDto;
use rex_game_application::users::user_dto::UserDto;
use rex_game_application::users::user_permission_creation_dto::UserPermissionCreationDto;
use rex_game_application::users::user_permission_dto::UserPermissionDto;
use rex_game_application::users::user_role_creation_dto::UserRoleCreationDto;
use rex_game_application::users::user_role_dto::UserRoleDto;
use rex_game_application::users::user_updation_dto::UserUpdationDto;
use rex_game_application::users::user_usecase_trait::UserUseCaseTrait;
use rex_game_domain::helpers::configuration_helper_trait::ConfigurationHelperTrait;
use rex_game_domain::helpers::email_helper_trait::{EmailHelperTrait, EmailMessage};
use rex_game_domain::identities::token_helper_trait::TokenHelperTrait;
use rex_game_domain::identities::TokenGenerationOptions;
use rex_game_domain::models::user_statuses::UserStatuses;
use rex_game_infrastructure::helpers::configuration_helper::ConfigurationHelper;
use rex_game_shared::enums::mail_template_names::MailTemplateNames;
use rex_game_shared::enums::user_token_porposes::UserTokenPurposes;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use validator::{Validate, ValidationErrors};

#[derive(Deserialize, Validate)]
pub struct UserQuery {
    #[validate(range(min = 1))]
    page: Option<u64>,
    #[validate(range(min = 5, max = 100))]
    page_size: Option<u64>,
    display_name: Option<String>,
    name: Option<String>,
    email: Option<String>,
    role_name: Option<String>,
}

impl UserHandler {
    pub async fn get_users<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Query(params): Query<UserQuery>,
    ) -> HandlerResult<Json<PageListDto<UserDto>>> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Access denied".to_string(),
                ..Default::default()
            });
        }
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let users = _state
            .user_usecase()
            .get_users(
                params.display_name,
                params.name,
                params.email,
                params.role_name,
                page,
                page_size,
            )
            .await;
        return match users {
            Ok(data) => Ok(Json(data)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch users".to_string(),
                    ..Default::default()
                })
            }
        };
    }

    pub async fn get_user_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> Result<Json<UserDto>, StatusCode> {
        let user = _state
            .user_usecase()
            .get_user_by_id(id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(user))
    }

    pub async fn create_user<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<SignupRequest>>,
    ) -> HandlerResult<Json<i32>> {
        let req = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                });
            }
        };

        req.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        let existing_user = _state
            .user_usecase()
            .get_user_by_email(&req.email)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

        if existing_user.is_ok() {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "Email already in use".to_string(),
                ..Default::default()
            });
        }

        let existing_user = _state
            .user_usecase()
            .get_user_by_name(&req.name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

        if existing_user.is_ok() {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "Username already in use".to_string(),
                ..Default::default()
            });
        }

        let new_user = UserCreationDto {
            email: req.email.to_owned(),
            name: req.name.to_owned(),
            display_name: req.display_name,
            status_id: UserStatuses::Pending as i32,
            ..Default::default()
        };

        let signup_result = _state
            .identity_user_usecase()
            .create_user(new_user, &req.password)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        let generated_token_options = TokenGenerationOptions {
            email: Some(signup_result.email),
            user_id: signup_result.id,
            exp_secs: Duration::days(1).num_seconds(),
            purpose: UserTokenPurposes::SignupConfirmation.to_string(),
            iat: Some(Utc::now().timestamp()),
        };
        let generated_token_option = _state
            .identity_token_helper()
            .generate_token(generated_token_options);

        let generated_token = match generated_token_option {
            Some(token) => token,
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to generate signup token".to_string(),
                    ..Default::default()
                })
            }
        };

        let token_creation = UserTokenCreationDto {
            created_by_id: signup_result.id,
            expiration: generated_token.exp as i32,
            purpose: UserTokenPurposes::SignupConfirmation as i32,
            token: generated_token.token.clone(),
            user_id: signup_result.id,
            updated_by_id: signup_result.id,
        };

        _state
            .identity_user_token_usecase()
            .create_user_token(token_creation)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        let register_mail_template = match _state
            .mail_template_usecase()
            .get_mail_template_by_name(MailTemplateNames::USER_REGISTRATION_CONFIRMATION)
            .await
        {
            Some(template) => template,
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to get mail template".to_string(),
                    ..Default::default()
                });
            }
        };

        if register_mail_template.is_enabled {
            let configuration_helper = Arc::new(ConfigurationHelper::new());
            let verification_url = configuration_helper.get_value("email.signup_verification_url");
            let from_name = configuration_helper.get_value("email.from_name");
            let username = configuration_helper.get_value("email.username");
            let platform_name = configuration_helper.get_value("email.platform_name");
            let platform_url = configuration_helper.get_value("email.platform_url");
            let expiration_date = DateTime::from_timestamp(generated_token.exp as i64, 0)
                .unwrap()
                .format("%d/%m/%Y %H:%M")
                .to_string();

            let verification_url = verification_url.replace("[token]", &generated_token.token);
            let email_body = register_mail_template
                .body
                .replace("[user_name]", &req.name)
                .replace("[confirmation_url]", &verification_url)
                .replace("[expiration_date]", &expiration_date)
                .replace("[platform_name]", &platform_name)
                .replace("[platform_url]", &platform_url);

            let subject = register_mail_template
                .subject
                .replace("[platform_name]", &platform_name);

            _state
                .email_helper()
                .send_email(EmailMessage {
                    to_name: Some(req.name.to_owned()),
                    to_email: req.email,
                    from_name: Some(from_name.to_owned()),
                    from_email: username.to_owned(),
                    subject: subject,
                    text_body: None,
                    html_body: Some(email_body),
                })
                .await
                .map_err(|err| HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.message,
                    ..Default::default()
                })?;
        }

        Ok(Json(signup_result.id))
    }

    pub async fn confirm_user<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<ConfirmUserRequest>>,
    ) -> HandlerResult<Json<bool>> {
        let request = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        request.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        let signup_token = match &request.token {
            Some(token) => token.as_str(),
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Token is required".to_string(),
                    ..Default::default()
                })
            }
        };

        let token_validation_result = _state.identity_token_helper().validate_token(signup_token);
        let token_validation = match token_validation_result {
            Ok(info) => info,
            Err(err) => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: err.message,
                    ..Default::default()
                })
            }
        };

        let user_token = _state
            .identity_user_token_usecase()
            .get_user_token_by_token(signup_token)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        if !user_token.is_actived
            || user_token.purpose != UserTokenPurposes::SignupConfirmation as i32
        {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Invalid token".to_string(),
                ..Default::default()
            });
        }

        let existing_user = _state
            .user_usecase()
            .get_user_by_id(token_validation.sub)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        _state
            .identity_user_token_usecase()
            .update_user_token(
                user_token.id,
                UserTokenUpdationDto {
                    updated_by_id: existing_user.id,
                    is_actived: Some(false),
                },
            )
            .await
            .map_err(|_| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: "Failed to deactive the token".to_string(),
                ..Default::default()
            })?;

        if existing_user.status_id == (UserStatuses::Actived as i32) {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "The user is already actived".to_string(),
                ..Default::default()
            });
        }

        if existing_user.status_id == (UserStatuses::Deleted as i32) {
            return Err(HandlerError {
                status: StatusCode::NOT_FOUND,
                message: "The user is not valid".to_string(),
                ..Default::default()
            });
        }

        let updating = UserUpdationDto {
            status_id: Some(UserStatuses::Actived as i32),
            ..Default::default()
        };

        let result = _state
            .user_usecase()
            .update_user(token_validation.sub, updating)
            .await;

        return match result {
            None => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to verify user".to_string(),
                ..Default::default()
            }),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn forgot_password<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<ForgotPasswordRequest>>,
    ) -> HandlerResult<Json<bool>> {
        let req = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                });
            }
        };

        req.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        let existing_user = _state
            .user_usecase()
            .get_user_by_email(&req.email)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: err.message,
                ..Default::default()
            })?;

        let generated_token_options = TokenGenerationOptions {
            email: None,
            user_id: existing_user.id,
            exp_secs: Duration::days(1).num_seconds(),
            purpose: UserTokenPurposes::ForgotPassword.to_string(),
            iat: None,
        };
        let generated_token_option = _state
            .identity_token_helper()
            .generate_token(generated_token_options);

        let generated_token = match generated_token_option {
            Some(token) => token,
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to generate forgot password token".to_string(),
                    ..Default::default()
                })
            }
        };

        let token_creation = UserTokenCreationDto {
            created_by_id: existing_user.id,
            expiration: generated_token.exp as i32,
            purpose: UserTokenPurposes::ForgotPassword as i32,
            token: generated_token.token.clone(),
            user_id: existing_user.id,
            updated_by_id: existing_user.id,
        };

        _state
            .identity_user_token_usecase()
            .create_user_token(token_creation)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        let forgot_password_mail_template = match _state
            .mail_template_usecase()
            .get_mail_template_by_name(MailTemplateNames::PASSWORD_RESET_REQUEST)
            .await
        {
            Some(template) => template,
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to get mail template".to_string(),
                    ..Default::default()
                });
            }
        };

        if forgot_password_mail_template.is_enabled {
            let configuration_helper = _state.configuration_helper();
            let reset_password_url: String =
                configuration_helper.get_value("email.reset_password_url");
            let from_name: String = configuration_helper.get_value("email.from_name");
            let username: String = configuration_helper.get_value("email.username");
            let platform_name: String = configuration_helper.get_value("email.platform_name");
            let platform_url: String = configuration_helper.get_value("email.platform_url");
            let expiration_date: String = DateTime::from_timestamp(generated_token.exp as i64, 0)
                .unwrap()
                .format("%d/%m/%Y %H:%M")
                .to_string();

            let reset_password_url = reset_password_url.replace("[token]", &generated_token.token);
            let email_body = forgot_password_mail_template
                .body
                .replace("[user_name]", &existing_user.name)
                .replace("[reset_url]", &reset_password_url)
                .replace("[expiration_date]", &expiration_date)
                .replace("[platform_name]", &platform_name)
                .replace("[platform_url]", &platform_url);

            let subject = forgot_password_mail_template
                .subject
                .replace("[platform_name]", &platform_name);

            _state
                .email_helper()
                .send_email(EmailMessage {
                    to_name: Some(existing_user.name.to_owned()),
                    to_email: req.email,
                    from_name: Some(from_name.to_owned()),
                    from_email: username.to_owned(),
                    subject: subject,
                    text_body: None,
                    html_body: Some(email_body),
                })
                .await
                .map_err(|err| HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.message,
                    ..Default::default()
                })?;
        }

        Ok(Json(true))
    }

    pub async fn reset_password<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<ResetPasswordRequest>>,
    ) -> HandlerResult<Json<bool>> {
        let request = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        request.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        let reset_password_token = match &request.token {
            Some(token) => token.as_str(),
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Token is required".to_string(),
                    ..Default::default()
                })
            }
        };

        let user_token = _state
            .identity_user_token_usecase()
            .get_user_token_by_token(reset_password_token)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        if !user_token.is_actived || user_token.purpose != UserTokenPurposes::ForgotPassword as i32
        {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Invalid token".to_string(),
                ..Default::default()
            });
        }

        let token_validation_result = _state
            .identity_token_helper()
            .validate_token(reset_password_token);
        let token_validation = match token_validation_result {
            Ok(info) => info,
            Err(err) => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: err.message,
                    ..Default::default()
                })
            }
        };

        let existing_user = _state
            .user_usecase()
            .get_user_by_id(token_validation.sub)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        if existing_user.status_id == (UserStatuses::Deleted as i32) {
            return Err(HandlerError {
                status: StatusCode::NOT_FOUND,
                message: "The user is already deleted".to_string(),
                ..Default::default()
            });
        }

        _state
            .identity_user_token_usecase()
            .update_user_token(
                user_token.id,
                UserTokenUpdationDto {
                    updated_by_id: existing_user.id,
                    is_actived: Some(false),
                },
            )
            .await
            .map_err(|_| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: "Failed to deactive the token".to_string(),
                ..Default::default()
            })?;

        let updating = UserUpdationDto {
            password: Some(request.password),
            ..Default::default()
        };

        let result = _state
            .user_usecase()
            .update_user(token_validation.sub, updating)
            .await;
        return match result {
            None => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to reset password".to_string(),
                ..Default::default()
            }),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn get_current_user<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
    ) -> Result<Json<CurrentUser>, StatusCode> {
        Ok(Json((*current_user).clone()))
    }

    pub async fn update_user<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
        Json(payload): Json<Option<HashMap<String, String>>>,
    ) -> HandlerResult<Json<bool>> {
        let requests = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        if requests.is_empty() {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "No fields to update".to_string(),
                ..Default::default()
            });
        }

        let existing = _state
            .user_usecase()
            .get_user_by_id(current_user.id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        if requests.get("name").is_none()
            && requests.get("display_name").is_none()
            && requests.get("email").is_none()
        {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "No valid fields to update".to_string(),
                ..Default::default()
            });
        }

        if existing.id != current_user.id
            && !current_user
                .roles
                .iter()
                .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Access denied".to_string(),
                ..Default::default()
            });
        }

        let mut updating = UserUpdationDto {
            updated_by_id: Some(current_user.id),
            ..Default::default()
        };

        for (key, value) in &requests {
            if key.to_lowercase() == "name" {
                updating.name = Some(value.to_string());
            } else if key.to_lowercase() == "display_name" {
                updating.display_name = Some(value.to_string())
            } else if key.to_lowercase() == "email" {
                updating.email = Some(value.to_string())
            }
        }

        let result = _state.user_usecase().update_user(id, updating).await;
        return match result {
            None => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update user".to_string(),
                ..Default::default()
            }),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_user<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        let deletion_req = UserDeletionDto {
            updated_by_id: Some(current_user.id),
        };

        let existing = _state
            .user_usecase()
            .get_user_by_id(current_user.id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        if existing.id == current_user.id {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "You cannot delete yourself".to_string(),
                ..Default::default()
            });
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Access denied".to_string(),
                ..Default::default()
            });
        }

        let is_succeed = _state
            .user_usecase()
            .delete_user_by_id(id, deletion_req)
            .await;

        match is_succeed {
            Some(u) => Ok(Json(u)),
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to delete user".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    pub async fn assign_roles<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(user_id): Path<i32>,
        Json(payload): Json<Option<AssignRoleRequest>>,
    ) -> HandlerResult<Json<i32>> {
        let requests = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        let role_ids = match requests.role_ids {
            Some(id) => id,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Role IDs are required".to_string(),
                    ..Default::default()
                })
            }
        };

        if role_ids.len() == 0 {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Role IDs cannot be empty".to_string(),
                ..Default::default()
            });
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Access denied".to_string(),
                ..Default::default()
            });
        }

        _state
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        let incomming_roles = _state
            .role_usecase()
            .get_roles_by_ids(role_ids)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        let existing_assignments = _state
            .user_usecase()
            .get_user_roles_by_user_id(user_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        // Assign roles that are not already assigned
        let to_be_assigned_roles: Vec<UserRoleCreationDto> = incomming_roles
            .clone()
            .into_iter()
            .filter(|role| {
                role.name != ROLE_ROOT_ADMIN
                    && existing_assignments.iter().all(|r| r.role_id != role.id)
            })
            .map(|role| UserRoleCreationDto {
                created_by_id: current_user.id,
                updated_by_id: current_user.id,
                role_id: role.id,
            })
            .collect::<Vec<UserRoleCreationDto>>();

        _state
            .user_usecase()
            .assign_roles(user_id, to_be_assigned_roles.clone())
            .await
            .ok();

        // Unassign roles that are not in the incoming roles
        let to_be_deleted_roles: Vec<UserRoleDto> = existing_assignments
            .into_iter()
            .filter(|r| {
                r.role_name != ROLE_ROOT_ADMIN
                    && !incomming_roles.iter().any(|role| role.id == r.role_id)
            })
            .collect();

        _state
            .user_usecase()
            .unassign_roles(user_id, to_be_deleted_roles)
            .await
            .ok();

        Ok(Json(to_be_assigned_roles.len() as i32))
    }

    pub async fn get_roles<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(user_id): Path<i32>,
    ) -> HandlerResult<Json<Vec<UserRoleDto>>> {
        _state
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Access denied".to_string(),
                ..Default::default()
            });
        }

        let user_roles = _state
            .user_usecase()
            .get_user_roles_by_user_id(user_id)
            .await;

        match user_roles {
            Ok(u) => Ok(Json(u)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch user roles".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    pub async fn assign_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(user_id): Path<i32>,
        Json(payload): Json<Option<AssignPermissionRequest>>,
    ) -> HandlerResult<Json<i32>> {
        let requests = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        let permission_codes = match requests.permission_codes {
            Some(code) => code,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Permission codes are required".to_string(),
                    ..Default::default()
                })
            }
        };

        if permission_codes.len() == 0 {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Permission codes cannot be empty".to_string(),
                ..Default::default()
            });
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Access denied".to_string(),
                ..Default::default()
            });
        }

        _state
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        let incomming_permissions = _state
            .permission_usecase()
            .get_permission_by_codes(permission_codes)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        let existing_assignments = _state
            .user_usecase()
            .get_user_permissions_by_user_id(user_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.message,
                ..Default::default()
            })?;

        // Assign permissons that are not already assigned
        let to_be_assigned_permissons: Vec<UserPermissionCreationDto> = incomming_permissions
            .clone()
            .into_iter()
            .filter(|permission| {
                existing_assignments
                    .iter()
                    .all(|r| r.permission_id != permission.id)
            })
            .map(|permission| UserPermissionCreationDto {
                created_by_id: current_user.id,
                updated_by_id: current_user.id,
                permission_id: permission.id,
            })
            .collect::<Vec<UserPermissionCreationDto>>();

        _state
            .user_usecase()
            .assign_permissions(user_id, to_be_assigned_permissons.clone())
            .await
            .ok();

        // Unassign permissions that are not in the incoming permissions
        let to_be_deleted_permissions: Vec<UserPermissionDto> = existing_assignments
            .into_iter()
            .filter(|r| {
                !incomming_permissions
                    .iter()
                    .any(|permission| permission.id == r.permission_id)
            })
            .collect();

        _state
            .user_usecase()
            .unassign_permissions(user_id, to_be_deleted_permissions)
            .await
            .ok();

        Ok(Json(to_be_assigned_permissons.len() as i32))
    }

    pub async fn get_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(user_id): Path<i32>,
    ) -> HandlerResult<Json<Vec<UserPermissionDto>>> {
        _state
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: err.message,
                ..Default::default()
            })?;

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Access denied".to_string(),
                ..Default::default()
            });
        }

        let user_permissions = _state
            .user_usecase()
            .get_user_permissions_by_user_id(user_id)
            .await;

        match user_permissions {
            Ok(u) => Ok(Json(u)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch user permissions".to_string(),
                    ..Default::default()
                })
            }
        }
    }
}

pub struct UserHandler {}
