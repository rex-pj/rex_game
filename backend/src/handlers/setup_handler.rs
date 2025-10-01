use crate::{
    app_state::AppStateTrait,
    validators::validation_helper::ValidationHelper,
    view_models::{users::signup_request::SignupRequest, HandlerError, HandlerResult},
};
use axum::{extract::State, Json};
use hyper::StatusCode;
use rex_game_application::{
    identities::{
        identity_user_usecase_trait::IdentityUserUseCaseTrait, user_creation_dto::UserCreationDto,
    },
    roles::{role_usecase_trait::RoleUseCaseTrait, roles::ROLE_ROOT_ADMIN},
    users::{user_role_creation_dto::UserRoleCreationDto, user_usecase_trait::UserUseCaseTrait},
};
use rex_game_domain::{
    models::user_statuses::UserStatuses, transaction_manager_trait::TransactionManagerTrait,
};
use rex_game_infrastructure::helpers::file_helper_object_trait::FileHelperObjectTrait;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize)]
struct InstallationState {
    is_installed: bool,
}

const INSTALLATION_PATH: &str = "./src/app_data/installation_state.json";

impl SetupHandler {
    pub async fn setup<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<SignupRequest>>,
    ) -> HandlerResult<Json<bool>> {
        let req = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
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

        match _state
            .file_helper()
            .get_object::<InstallationState>(INSTALLATION_PATH)
        {
            Some(installed) => {
                if installed.is_installed {
                    return Err(HandlerError {
                        status: StatusCode::CONFLICT,
                        message: "Application is already installed".to_string(),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        };

        if let Ok(_) = _state.user_usecase().get_user_by_email(&req.email).await {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "User with this email already exists".to_string(),
                ..Default::default()
            });
        };

        let new_user = UserCreationDto {
            email: req.email,
            name: req.name,
            display_name: req.display_name,
            status_id: UserStatuses::Actived as i32,
            ..Default::default()
        };

        let transaction_manager = _state.transaction_manager();
        let transaction_wrapper =
            transaction_manager
                .begin()
                .await
                .map_err(|err| HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to begin transaction: {}", err.message),
                    ..Default::default()
                })?;
        let created_result = match _state
            .identity_user_usecase()
            .create_user_with_transaction(new_user, &req.password, Box::new(&transaction_wrapper))
            .await
        {
            Ok(created) => created,
            Err(_) => {
                if let Err(_) = transaction_manager.rollback(transaction_wrapper).await {
                    return Err(HandlerError {
                        status: StatusCode::INTERNAL_SERVER_ERROR,
                        message: "Failed to rollback transaction".to_string(),
                        ..Default::default()
                    });
                }
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to create user".to_string(),
                    ..Default::default()
                });
            }
        };

        let existing_role = _state
            .role_usecase()
            .get_role_by_name(ROLE_ROOT_ADMIN)
            .await;
        let role = match existing_role {
            Some(role_model) => role_model,
            None => {
                if let Err(_) = transaction_manager.rollback(transaction_wrapper).await {
                    return Err(HandlerError {
                        status: StatusCode::INTERNAL_SERVER_ERROR,
                        message: "Failed to rollback transaction".to_string(),
                        ..Default::default()
                    });
                }
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Root admin role does not exist".to_string(),
                    ..Default::default()
                });
            }
        };

        if let Err(_) = _state
            .user_usecase()
            .assign_role_with_transaction(
                created_result.id,
                UserRoleCreationDto {
                    role_id: role.id,
                    created_by_id: created_result.id,
                    updated_by_id: created_result.id,
                },
                Box::new(&transaction_wrapper),
            )
            .await
        {
            if let Err(_) = transaction_manager.rollback(transaction_wrapper).await {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to rollback transaction".to_string(),
                    ..Default::default()
                });
            }
            return Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to assign role to user".to_string(),
                ..Default::default()
            });
        }

        if let Err(_) = _state
            .file_helper()
            .save_object(INSTALLATION_PATH, &InstallationState { is_installed: true })
        {
            if let Err(_) = transaction_manager.rollback(transaction_wrapper).await {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to rollback transaction".to_string(),
                    ..Default::default()
                });
            }
            return Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update installation state".to_string(),
                ..Default::default()
            });
        }

        if let Err(_) = transaction_manager.commit(transaction_wrapper).await {
            return Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to commit transaction".to_string(),
                ..Default::default()
            });
        }
        return Ok(Json(true));
    }
}

pub struct SetupHandler {}
