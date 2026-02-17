use crate::{
    app_state::AppState,
    validators::validation_helper::ValidationHelper,
    view_models::{users::signup_request::SignupRequest, HandlerError, HandlerResult},
};
use axum::{extract::State, Json};
use hyper::StatusCode;
use migration::{LockBehavior, LockType, Migrator, MigratorTrait};
use rex_game_entities::entities::system_settings;
use rex_game_identity::application::usecases::{
    auth::{user_creation_dto::UserCreationDto, IdentityUserUseCaseTrait},
    roles::ROLE_ROOT_ADMIN,
    user_role_creation_dto::UserRoleCreationDto,
    RoleUseCaseTrait, UserUseCaseTrait,
};
use rex_game_identity::domain::models::user_statuses::UserStatuses;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, QuerySelect, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize)]
pub struct SetupStatus {
    pub database_connected: bool,
    pub migrations_pending: bool,
    pub is_installed: bool,
    pub pending_migrations: Vec<String>,
}

const INSTALLATION_KEY: &str = "is_installed";

// Helper functions for database-based installation state
async fn get_installation_status_with_lock(
    txn: &DatabaseTransaction,
) -> Result<bool, sea_orm::DbErr> {
    // Query with FOR UPDATE lock to prevent concurrent setup
    let setting = system_settings::Entity::find()
        .filter(system_settings::Column::Key.eq(INSTALLATION_KEY))
        .lock_with_behavior(LockType::Update, LockBehavior::Nowait)
        .one(txn)
        .await?;

    match setting {
        Some(model) => Ok(model.value == "true"),
        None => Ok(false),
    }
}

async fn set_installation_status(
    txn: &DatabaseTransaction,
    installed: bool,
) -> Result<(), sea_orm::DbErr> {
    let value = if installed { "true" } else { "false" };

    // Find the existing record
    let setting = system_settings::Entity::find()
        .filter(system_settings::Column::Key.eq(INSTALLATION_KEY))
        .one(txn)
        .await?;

    if let Some(model) = setting {
        // Update existing record
        let mut active_model: system_settings::ActiveModel = model.into();
        active_model.value = Set(value.to_string());
        active_model.updated_on = Set(chrono::Utc::now().into());
        active_model.created_on = Set(chrono::Utc::now().into());
        active_model.update(txn).await?;
    }
    Ok(())
}

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

        // Run database migrations before setup
        println!("Running database migrations...");
        Migrator::up(&*_state.db_connection, None)
            .await
            .map_err(|e| {
                eprintln!("Migration failed: {:?}", e);
                HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Database migration failed: {}", e),
                    ..Default::default()
                }
            })?;
        println!("Database migrations completed successfully");

        // Start transaction with database lock for installation status
        let txn = _state.db_connection.begin().await.map_err(|e| {
            eprintln!("Failed to start transaction: {:?}", e);
            HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to start setup transaction".to_string(),
                ..Default::default()
            }
        })?;

        // Check installation status with row-level lock (FOR UPDATE)
        // This prevents concurrent setup requests
        let is_installed = get_installation_status_with_lock(&txn).await.map_err(|e| {
            eprintln!("Failed to check installation status: {:?}", e);
            HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to check installation status".to_string(),
                ..Default::default()
            }
        })?;

        if is_installed {
            // Rollback transaction and return error
            let _ = txn.rollback().await;
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "Application is already installed".to_string(),
                ..Default::default()
            });
        }

        // Check if user with email already exists
        if let Ok(_) = _state.usecases.user.get_user_by_email(&payload.email).await {
            let _ = txn.rollback().await;
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "User with this email already exists".to_string(),
                ..Default::default()
            });
        }

        let new_user = UserCreationDto {
            email: payload.email.clone(),
            name: payload.name.clone(),
            display_name: payload.display_name.clone(),
            status_id: UserStatuses::Actived as i32,
            ..Default::default()
        };

        // Create admin user
        let created_user = _state
            .usecases
            .identity_user
            .create_user(new_user, &payload.password)
            .await
            .map_err(|e| {
                eprintln!("Failed to create user: {:?}", e);
                HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to create admin user: {}", e),
                    ..Default::default()
                }
            })?;

        // Get root admin role (should exist from seeding migration)
        let role = _state
            .usecases
            .role
            .get_role_by_name(ROLE_ROOT_ADMIN)
            .await
            .ok_or_else(|| {
                eprintln!("Root admin role not found - migration seeding may have failed");
                HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Root admin role does not exist. Please check database migrations."
                        .to_string(),
                    ..Default::default()
                }
            })?;

        // Assign root admin role to user
        let user_role_req = vec![UserRoleCreationDto {
            role_id: role.id,
            created_by_id: created_user.id,
            updated_by_id: created_user.id,
        }];

        _state
            .usecases
            .user
            .assign_roles(created_user.id, user_role_req)
            .await
            .map_err(|e| {
                eprintln!("Failed to assign role: {:?}", e);
                HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to assign admin role: {}", e),
                    ..Default::default()
                }
            })?;

        // Mark installation as complete in database
        set_installation_status(&txn, true).await.map_err(|e| {
            eprintln!("Failed to update installation status: {:?}", e);
            HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update installation status".to_string(),
                ..Default::default()
            }
        })?;

        // Commit transaction
        txn.commit().await.map_err(|e| {
            eprintln!("Failed to commit setup transaction: {:?}", e);
            HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to complete setup process".to_string(),
                ..Default::default()
            }
        })?;

        println!("Setup completed successfully for user: {}", payload.email);
        Ok(Json(true))
    }

    pub async fn get_status(State(_state): State<AppState>) -> HandlerResult<Json<SetupStatus>> {
        // Check database connection and pending migrations
        let (database_connected, migrations_pending, pending_migrations) =
            match Migrator::get_pending_migrations(&*_state.db_connection).await {
                Ok(pending) => {
                    let pending_count = pending.len();
                    let pending_names: Vec<String> =
                        pending.iter().map(|m| m.name().to_string()).collect();
                    (true, pending_count > 0, pending_names)
                }
                Err(e) => {
                    eprintln!("Failed to check migrations: {:?}", e);
                    (false, false, vec![])
                }
            };

        // Check installation state from database (preferred) or fallback to file
        let is_installed = if database_connected && !migrations_pending {
            // Query database for installation status using entity API
            match system_settings::Entity::find()
                .filter(system_settings::Column::Key.eq(INSTALLATION_KEY))
                .one(&*_state.db_connection)
                .await
            {
                Ok(Some(model)) => model.value == "true",
                Ok(None) => false,
                Err(_) => false,
            }
        } else {
            false
        };

        Ok(Json(SetupStatus {
            database_connected,
            migrations_pending,
            is_installed,
            pending_migrations,
        }))
    }
}

pub struct SetupHandler {}
