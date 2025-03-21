use crate::{app_state::AppStateTrait, view_models::users::signup_request::SignupRequest};
use axum::{extract::State, Json};
use hyper::StatusCode;
use rex_game_application::{
    identities::{
        application_user_dto::ApplicationUserDto,
        identity_user_usecase_trait::IdentityUserUseCaseTrait,
    },
    users::{
        roles::ROLE_ADMIN, user_role_creation_dto::UserRoleCreationDto,
        user_statuses::UserStatuses, user_usecase_trait::UserUseCaseTrait,
    },
};
use rex_game_infrastructure::helpers::file_helper_object_trait::FileHelperObjectTrait;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InstallationState {
    is_installed: bool,
}

const INSTALLATION_PATH: &str = "./src/app_data/installation_state.json";

impl SetupHandler {
    pub async fn setup<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<SignupRequest>>,
    ) -> Result<Json<bool>, StatusCode> {
        let req = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        match _state
            .file_helper()
            .get_object::<InstallationState>(INSTALLATION_PATH)
        {
            Some(installed) => {
                if installed.is_installed {
                    return Err(StatusCode::CONFLICT);
                }
            }
            _ => {}
        };

        if let Ok(_) = _state
            .user_usecase()
            .get_user_by_email(req.email.to_owned())
            .await
        {
            return Err(StatusCode::CONFLICT);
        };

        let new_user = ApplicationUserDto {
            email: req.email,
            name: req.name,
            display_name: req.display_name,
            status_id: UserStatuses::Actived as i32,
            ..Default::default()
        };

        let db_transaction = match _state.db_connection().begin().await {
            Ok(transaction) => transaction,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        let created_result = match _state
            .identity_user_usecase()
            .create_user(new_user, &req.password, Some(&db_transaction))
            .await
        {
            Ok(created) => created,
            Err(_) => {
                if let Err(_) = db_transaction.rollback().await {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        if let Err(_) = _state
            .user_usecase()
            .assign_role(
                UserRoleCreationDto {
                    user_id: created_result.id,
                    role_name: String::from(ROLE_ADMIN),
                    created_by_id: Some(created_result.id),
                    updated_by_id: Some(created_result.id),
                },
                Some(&db_transaction),
            )
            .await
        {
            if let Err(_) = db_transaction.rollback().await {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        if let Err(_) = _state
            .file_helper()
            .save_object(INSTALLATION_PATH, &InstallationState { is_installed: true })
        {
            if let Err(_) = db_transaction.rollback().await {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        if let Err(_) = db_transaction.commit().await {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        return Ok(Json(true));
    }
}

pub struct SetupHandler {}
