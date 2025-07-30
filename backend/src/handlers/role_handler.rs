use crate::{
    app_state::AppStateTrait,
    view_models::{
        roles::role_create_request::RoleCreateRequest,
        users::{assign_permission_request::AssignPermissionRequest, current_user::CurrentUser},
    },
};
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use hyper::StatusCode;
use rex_game_application::{
    page_list_dto::PageListDto,
    permissions::permission_usecase_trait::PermissionUseCaseTrait,
    roles::{
        role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto, role_dto::RoleDto,
        role_updation_dto::RoleUpdationDto, role_usecase_trait::RoleUseCaseTrait,
    },
    users::{
        role_permission_creation_dto::RolePermissionCreationDto,
        role_permission_dto::RolePermissionDto, roles::ROLE_ROOT_ADMIN, user_role_dto::UserRoleDto,
    },
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

#[derive(Deserialize)]
pub struct RoleQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    description: Option<String>,
}

impl RoleHandler {
    pub async fn get_roles<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Query(params): Query<RoleQuery>,
    ) -> Result<Json<PageListDto<RoleDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }
        let page = params.page.unwrap_or(1);
        let roles = _state
            .role_usecase()
            .get_roles(params.name, params.description, page, params.page_size)
            .await;
        return match roles {
            Ok(data) => Ok(Json(data)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    }

    pub async fn get_role_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> Result<Json<RoleDto>, StatusCode> {
        let role = _state
            .role_usecase()
            .get_role_by_id(id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(role))
    }

    pub async fn create_role<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Json(payload): Json<Option<RoleCreateRequest>>,
    ) -> Result<Json<i32>, StatusCode> {
        let req = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let existing_role = _state
            .role_usecase()
            .get_role_by_name(req.name.as_str())
            .await;

        if let Some(_) = existing_role {
            return Err(StatusCode::CONFLICT);
        }

        let new_role = RoleCreationDto {
            name: req.name,
            description: req.description,
            created_by_id: Some(current_user.id),
            updated_by_id: Some(current_user.id),
            ..Default::default()
        };
        let created_result = _state.role_usecase().create_role(new_role).await;
        match created_result {
            Ok(created_id) => Ok(Json(created_id)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn update_role<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
        Json(payload): Json<Option<HashMap<String, String>>>,
    ) -> Result<Json<bool>, StatusCode> {
        let requests = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };
        if requests.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        if requests.get("name").is_none() && requests.get("description").is_none() {
            return Err(StatusCode::BAD_REQUEST);
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let existing = _state
            .role_usecase()
            .get_role_by_id(id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        if existing.name == ROLE_ROOT_ADMIN {
            return Err(StatusCode::FORBIDDEN);
        }

        let mut updating = RoleUpdationDto {
            updated_by_id: current_user.id,
            ..Default::default()
        };

        for (key, value) in &requests {
            if key.to_lowercase() == "name" {
                if existing.name == ROLE_ROOT_ADMIN {
                    return Err(StatusCode::FORBIDDEN);
                }
                updating.name = Some(value.to_string());
            } else if key.to_lowercase() == "description" {
                updating.description = Some(value.to_string())
            }
        }

        let result = _state.role_usecase().update_role(id, updating).await;
        return match result {
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_role<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
    ) -> Result<Json<bool>, StatusCode> {
        let deletion_req = RoleDeletionDto {
            updated_by_id: current_user.id,
        };

        let existing = _state
            .role_usecase()
            .get_role_by_id(id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        if existing.name == ROLE_ROOT_ADMIN {
            return Err(StatusCode::FORBIDDEN);
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let is_succeed = _state
            .role_usecase()
            .delete_role_by_id(id, deletion_req)
            .await;

        match is_succeed {
            Some(u) => Ok(Json(u)),
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn assign_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(role_id): Path<i32>,
        Json(payload): Json<Option<AssignPermissionRequest>>,
    ) -> Result<Json<i32>, StatusCode> {
        let requests = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let permission_codes = match requests.permission_codes {
            Some(code) => code,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        if permission_codes.len() == 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        _state
            .role_usecase()
            .get_role_by_id(role_id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let incomming_permissions = _state
            .permission_usecase()
            .get_permission_by_codes(permission_codes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let existing_assignments = _state
            .role_usecase()
            .get_role_permissions_by_role_id(role_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
            .role_usecase()
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
            .role_usecase()
            .unassign_permissions(role_id, to_be_deleted_permissions)
            .await
            .ok();

        Ok(Json(to_be_assigned_permissons.len() as i32))
    }

    pub async fn get_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(role_id): Path<i32>,
    ) -> Result<Json<Vec<RolePermissionDto>>, StatusCode> {
        _state
            .role_usecase()
            .get_role_by_id(role_id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let role_permissions = _state
            .role_usecase()
            .get_role_permissions_by_role_id(role_id)
            .await;

        match role_permissions {
            Ok(u) => Ok(Json(u)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_user_roles<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
    ) -> Result<Json<Vec<UserRoleDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let user_roles = _state.role_usecase().get_user_roles().await;

        match user_roles {
            Ok(u) => Ok(Json(u)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct RoleHandler {}
