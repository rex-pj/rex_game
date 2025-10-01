use std::{collections::HashMap, sync::Arc};

use crate::{
    app_state::AppStateTrait,
    validators::validation_helper::ValidationHelper,
    view_models::{
        permissions::permission_create_request::PermissionCreateRequest,
        users::current_user::CurrentUser, HandlerError, HandlerResult,
    },
};
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use hyper::StatusCode;
use rex_game_application::{
    page_list_dto::PageListDto,
    permissions::{
        permission_creation_dto::PermissionCreationDto,
        permission_deletion_dto::PermissionDeletionDto, permission_dto::PermissionDto,
        permission_updation_dto::PermissionUpdationDto,
        permission_usecase_trait::PermissionUseCaseTrait,
    },
    roles::{
        role_permission_dto::RolePermissionDto, role_usecase_trait::RoleUseCaseTrait,
        roles::ROLE_ROOT_ADMIN,
    },
    users::{user_permission_dto::UserPermissionDto, user_usecase_trait::UserUseCaseTrait},
};
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

#[derive(Deserialize)]
pub struct PermissionQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    description: Option<String>,
}

impl PermissionHandler {
    pub async fn get_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Query(params): Query<PermissionQuery>,
    ) -> HandlerResult<Json<PageListDto<PermissionDto>>> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to view permissions".to_string(),
                ..Default::default()
            });
        }

        let page = params.page.unwrap_or(1);
        let permissions = _state
            .permission_usecase()
            .get_permissions(params.name, params.description, page, params.page_size)
            .await;
        return match permissions {
            Ok(data) => Ok(Json(data)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch permissions".to_string(),
                    ..Default::default()
                })
            }
        };
    }

    pub async fn get_permission_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> HandlerResult<Json<PermissionDto>> {
        let permission = _state
            .permission_usecase()
            .get_permission_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Permission not found: {}", err.message),
                ..Default::default()
            })?;
        Ok(Json(permission))
    }

    pub async fn create_permission<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Json(payload): Json<Option<PermissionCreateRequest>>,
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
                message: "You do not have permission to create permissions".to_string(),
                ..Default::default()
            });
        }

        let existing_permission: Option<PermissionDto> = _state
            .permission_usecase()
            .get_permission_by_code(&req.code)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to check existing permission: {}", err.message),
                ..Default::default()
            })?;

        if let Some(_) = existing_permission {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "Permission with the same code already exists".to_string(),
                ..Default::default()
            });
        }

        let existing_permission_by_name: Option<PermissionDto> = _state
            .permission_usecase()
            .get_permission_by_name(&req.name)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to check existing permission: {}", err.message),
                ..Default::default()
            })?;

        if let Some(_) = existing_permission_by_name {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "Permission with the same name already exists".to_string(),
                ..Default::default()
            });
        }

        let new_permission = PermissionCreationDto {
            name: req.name,
            code: req.code,
            module: req.module,
            description: req.description,
            created_by_id: Some(current_user.id),
            updated_by_id: Some(current_user.id),
            ..Default::default()
        };
        let created_result = _state
            .permission_usecase()
            .create_permission(new_permission)
            .await;
        match created_result {
            Ok(created_id) => Ok(Json(created_id)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to create permission".to_string(),
                ..Default::default()
            }),
        }
    }

    pub async fn update_permission<T: AppStateTrait>(
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
                message: "You do not have permission to update permissions".to_string(),
                ..Default::default()
            });
        }

        let existing = _state
            .permission_usecase()
            .get_permission_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Permission not found: {}", err.message),
                ..Default::default()
            })?;

        let mut updating = PermissionUpdationDto {
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

                if existing.name == ROLE_ROOT_ADMIN {
                    return Err(HandlerError {
                        status: StatusCode::FORBIDDEN,
                        message: String::from(
                            "You cannot change the name of the root admin permission",
                        ),
                        ..Default::default()
                    });
                }

                updating.name = Some(value.to_string());
            } else if key.to_lowercase() == "description" {
                updating.description = Some(value.to_string())
            } else if key.to_lowercase() == "code" {
                let name = value.to_string();
                if name.len() < 1 || name.len() > 100 {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Code must be between 1 and 100 characters".to_string(),
                        ..Default::default()
                    });
                }
                updating.code = Some(value.to_string())
            } else if key.to_lowercase() == "module" {
                let name = value.to_string();
                if name.len() < 1 || name.len() > 100 {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Module must be between 1 and 100 characters".to_string(),
                        ..Default::default()
                    });
                }
                updating.module = Some(value.to_string())
            }
        }

        let result = _state
            .permission_usecase()
            .update_permission(id, updating)
            .await;
        return match result {
            None => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update permission".to_string(),
                ..Default::default()
            }),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_permission<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        let deletion_req = PermissionDeletionDto {
            updated_by_id: current_user.id,
        };

        let existing = _state
            .permission_usecase()
            .get_permission_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Permission not found: {}", err.message),
                ..Default::default()
            })?;

        if existing.name == ROLE_ROOT_ADMIN {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You cannot delete the root admin permission".to_string(),
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
                message: "You do not have permission to delete permissions".to_string(),
                ..Default::default()
            });
        }

        let is_succeed = _state
            .permission_usecase()
            .delete_permission_by_id(id, deletion_req)
            .await;

        match is_succeed {
            Some(u) => Ok(Json(u)),
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to delete permission".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    pub async fn get_user_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
    ) -> HandlerResult<Json<Vec<UserPermissionDto>>> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to view user permissions".to_string(),
                ..Default::default()
            });
        }

        let user_permissions = _state.user_usecase().get_user_permissions().await;

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

    pub async fn get_role_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
    ) -> HandlerResult<Json<Vec<RolePermissionDto>>> {
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

        let role_permissions = _state.role_usecase().get_role_permissions().await;

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
}

pub struct PermissionHandler {}
