use crate::{
    app_state::AppState,
    validators::validation_helper::ValidationHelper,
    view_models::{
        roles::role_create_request::RoleCreateRequest,
        users::{assign_permission_request::AssignPermissionRequest, current_user::CurrentUser},
        HandlerError, HandlerResult,
    },
};
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use hyper::StatusCode;
use rex_game_identity::application::usecases::{
    role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto, role_dto::RoleDto,
    role_permission_creation_dto::RolePermissionCreationDto,
    role_permission_dto::RolePermissionDto, role_updation_dto::RoleUpdationDto,
    roles::ROLE_ROOT_ADMIN, user_role_dto::UserRoleDto, PermissionUseCaseTrait, RoleUseCaseTrait,
};
use rex_game_shared::domain::models::PageListModel;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use validator::{Validate, ValidationErrors};

#[derive(Deserialize)]
pub struct RoleQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    description: Option<String>,
}

impl RoleHandler {
    pub async fn get_roles(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Query(params): Query<RoleQuery>,
    ) -> Result<Json<PageListModel<RoleDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }
        let page = params.page.unwrap_or(1);
        let roles = _state
            .usecases
            .role
            .get_roles(params.name, params.description, page, params.page_size)
            .await;
        return match roles {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    }

    pub async fn get_role_by_id(
        Path(id): Path<i32>,
        State(_state): State<AppState>,
    ) -> HandlerResult<Json<RoleDto>> {
        let role = _state
            .usecases
            .role
            .get_role_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Role not found: {}", err),
                ..Default::default()
            })?;
        Ok(Json(role))
    }

    pub async fn create_role(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Json(payload): Json<Option<RoleCreateRequest>>,
    ) -> HandlerResult<Json<i32>> {
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

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to create roles".to_string(),
                ..Default::default()
            });
        }

        let existing_role = _state
            .usecases
            .role
            .get_role_by_name(req.name.as_str())
            .await;

        if let Some(_) = existing_role {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "Role with the same name already exists".to_string(),
                ..Default::default()
            });
        }

        let new_role = RoleCreationDto {
            name: req.name,
            description: req.description,
            created_by_id: Some(current_user.id),
            updated_by_id: Some(current_user.id),
            ..Default::default()
        };
        let created_result = _state.usecases.role.create_role(new_role).await;
        match created_result {
            Ok(created_id) => Ok(Json(created_id)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to create role".to_string(),
                ..Default::default()
            }),
        }
    }

    pub async fn update_role(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
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
                message: "Request payload cannot be empty".to_string(),
                ..Default::default()
            });
        }

        if requests.get("name").is_none() && requests.get("description").is_none() {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "At least one of 'name' or 'description' must be provided".to_string(),
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
                message: "You do not have permission to update roles".to_string(),
                ..Default::default()
            });
        }

        let existing = _state
            .usecases
            .role
            .get_role_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Role not found: {}", err),
                ..Default::default()
            })?;

        if existing.name == ROLE_ROOT_ADMIN {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "The ROOT_ADMIN role cannot be modified".to_string(),
                ..Default::default()
            });
        }

        let mut updating = RoleUpdationDto {
            updated_by_id: current_user.id,
            ..Default::default()
        };

        for (key, value) in &requests {
            if key.to_lowercase() == "name" {
                let name = value.to_string();
                if name.len() < 1 || name.len() > 255 {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Title must be between 1 and 255 characters".to_string(),
                        ..Default::default()
                    });
                }

                if let Some(_) = _state.usecases.role.get_role_by_name(&name).await {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "This name already exists. Please choose a different one."
                            .to_string(),
                        ..Default::default()
                    });
                };

                updating.name = Some(value.to_string());
            } else if key.to_lowercase() == "description" {
                updating.description = Some(value.to_string())
            }
        }

        let result = _state.usecases.role.update_role(id, updating).await;
        return match result {
            None => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update role".to_string(),
                ..Default::default()
            }),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_role(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        let deletion_req = RoleDeletionDto {
            updated_by_id: current_user.id,
        };

        let existing = _state
            .usecases
            .role
            .get_role_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Role not found: {}", err),
                ..Default::default()
            })?;

        if existing.name == ROLE_ROOT_ADMIN {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "The ROOT_ADMIN role cannot be deleted".to_string(),
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
                message: "You do not have permission to delete roles".to_string(),
                ..Default::default()
            });
        }

        let is_succeed = _state
            .usecases
            .role
            .delete_role_by_id(id, deletion_req)
            .await;

        match is_succeed {
            Some(u) => Ok(Json(u)),
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to delete role".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    pub async fn assign_permissions(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Path(role_id): Path<i32>,
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
                    message: "Permission codes cannot be empty".to_string(),
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
                message: "You do not have permission to assign permissions".to_string(),
                ..Default::default()
            });
        }

        _state
            .usecases
            .role
            .get_role_by_id(role_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Role not found: {}", err),
                ..Default::default()
            })?;

        let incomming_permissions = _state
            .usecases
            .permission
            .get_permission_by_codes(permission_codes)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: format!("Failed to fetch permissions: {}", err),
                ..Default::default()
            })?;

        let existing_assignments = _state
            .usecases
            .role
            .get_role_permissions_by_role_id(role_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch existing role permissions: {}", err),
                ..Default::default()
            })?;

        // Assign permissons that are not already assigned
        let to_be_assigned_permissons: Vec<RolePermissionCreationDto> = incomming_permissions
            .clone()
            .into_iter()
            .filter(|permission| {
                existing_assignments
                    .iter()
                    .all(|r| r.permission_id != permission.id)
            })
            .map(|permission| RolePermissionCreationDto {
                created_by_id: current_user.id,
                updated_by_id: current_user.id,
                permission_id: permission.id,
            })
            .collect::<Vec<RolePermissionCreationDto>>();

        _state
            .usecases
            .role
            .assign_permissions(role_id, to_be_assigned_permissons.clone())
            .await
            .ok();

        // Unassign permissions that are not in the incoming permissions
        let to_be_deleted_permissions: Vec<RolePermissionDto> = existing_assignments
            .into_iter()
            .filter(|r| {
                !incomming_permissions
                    .iter()
                    .any(|permission| permission.id == r.permission_id)
            })
            .collect();

        _state
            .usecases
            .role
            .unassign_permissions(role_id, to_be_deleted_permissions)
            .await
            .ok();

        Ok(Json(to_be_assigned_permissons.len() as i32))
    }

    pub async fn get_permissions(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Path(role_id): Path<i32>,
    ) -> HandlerResult<Json<Vec<RolePermissionDto>>> {
        _state
            .usecases
            .role
            .get_role_by_id(role_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Role not found: {}", err),
                ..Default::default()
            })?;

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to view role permissions".to_string(),
                ..Default::default()
            });
        }

        let role_permissions = _state
            .usecases
            .role
            .get_role_permissions_by_role_id(role_id)
            .await;

        match role_permissions {
            Ok(u) => Ok(Json(u)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch role permissions".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    pub async fn get_user_roles(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
    ) -> HandlerResult<Json<Vec<UserRoleDto>>> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to view user roles".to_string(),
                ..Default::default()
            });
        }

        let user_roles = _state.usecases.role.get_user_roles().await;
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
}

pub struct RoleHandler {}
