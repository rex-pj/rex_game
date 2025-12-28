use crate::{
    app_state::AppState,
    validators::validation_helper::ValidationHelper,
    view_models::{users::signup_request::SignupRequest, HandlerError, HandlerResult},
};
use axum::{extract::State, Json};
use hyper::StatusCode;
use rex_game_identity::application::usecases::{
    auth::{user_creation_dto::UserCreationDto, IdentityUserUseCaseTrait},
    roles::ROLE_ROOT_ADMIN,
    user_role_creation_dto::UserRoleCreationDto,
    RoleUseCaseTrait, UserUseCaseTrait,
};
use rex_game_identity::domain::models::user_statuses::UserStatuses;
use rex_game_shared_kernel::infrastructure::helpers::file_helper_object_trait::FileHelperObjectTrait;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize)]
struct InstallationState {
    is_installed: bool,
}

const INSTALLATION_PATH: &str = "./src/app_data/installation_state.json";

impl SetupHandler {
    pub async fn setup(
        State(_state): State<AppState>,
        Json(payload): Json<SignupRequest>,
    ) -> HandlerResult<Json<bool>> {
        payload.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        match _state
            .helpers.file
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

        if let Ok(_) = _state
            .usecases.user
            .get_user_by_email(&payload.email)
            .await
        {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "User with this email already exists".to_string(),
                ..Default::default()
            });
        };

        let new_user = UserCreationDto {
            email: payload.email,
            name: payload.name,
            display_name: payload.display_name,
            status_id: UserStatuses::Actived as i32,
            ..Default::default()
        };

        // Create user (simplified without transaction for setup)
        let created_user = _state
            .usecases.identity_user
            .create_user(new_user, &payload.password)
            .await
            .map_err(|_| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to create user".to_string(),
                ..Default::default()
            })?;

        // Get role
        let role = _state
            .usecases.role
            .get_role_by_name(ROLE_ROOT_ADMIN)
            .await
            .ok_or_else(|| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Root admin role does not exist".to_string(),
                ..Default::default()
            })?;

        // Assign role
        let user_role_req = vec![UserRoleCreationDto {
            role_id: role.id,
            created_by_id: created_user.id,
            updated_by_id: created_user.id,
        }];

        _state
            .usecases.user
            .assign_roles(created_user.id, user_role_req)
            .await
            .map_err(|_| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to assign role to user".to_string(),
                ..Default::default()
            })?;

        // Save installation state
        _state
            .helpers.file
            .save_object(INSTALLATION_PATH, &InstallationState { is_installed: true })
            .map_err(|_| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update installation state".to_string(),
                ..Default::default()
            })?;

        Ok(Json(true))
    }
}

pub struct SetupHandler {}
