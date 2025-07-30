use std::{collections::HashMap, sync::Arc};

use crate::{
    app_state::AppStateTrait,
    view_models::{
        permissions::permission_create_request::PermissionCreateRequest,
        users::current_user::CurrentUser,
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
    roles::role_usecase_trait::RoleUseCaseTrait,
    users::{
        role_permission_dto::RolePermissionDto, roles::ROLE_ROOT_ADMIN,
        user_permission_dto::UserPermissionDto, user_usecase_trait::UserUseCaseTrait,
    },
};
use serde::Deserialize;

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
    ) -> Result<Json<PageListDto<PermissionDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let page = params.page.unwrap_or(1);
        let permissions = _state
            .permission_usecase()
            .get_permissions(params.name, params.description, page, params.page_size)
            .await;
        return match permissions {
            Ok(data) => Ok(Json(data)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    }

    pub async fn get_permission_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> Result<Json<PermissionDto>, StatusCode> {
        let permission = _state
            .permission_usecase()
            .get_permission_by_id(id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(permission))
    }

    pub async fn create_permission<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Json(payload): Json<Option<PermissionCreateRequest>>,
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

        let existing_permission: Option<PermissionDto> = _state
            .permission_usecase()
            .get_permission_by_code(&req.code)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Some(_) = existing_permission {
            return Err(StatusCode::CONFLICT);
        }

        let existing_permission_by_name: Option<PermissionDto> = _state
            .permission_usecase()
            .get_permission_by_name(&req.name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Some(_) = existing_permission_by_name {
            return Err(StatusCode::CONFLICT);
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
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn update_permission<T: AppStateTrait>(
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
            .permission_usecase()
            .get_permission_by_id(id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let mut updating = PermissionUpdationDto {
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
            } else if key.to_lowercase() == "code" {
                updating.code = Some(value.to_string())
            } else if key.to_lowercase() == "module" {
                updating.module = Some(value.to_string())
            }
        }

        let result = _state
            .permission_usecase()
            .update_permission(id, updating)
            .await;
        return match result {
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_permission<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
    ) -> Result<Json<bool>, StatusCode> {
        let deletion_req = PermissionDeletionDto {
            updated_by_id: current_user.id,
        };

        let existing = _state
            .permission_usecase()
            .get_permission_by_id(id)
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
            .permission_usecase()
            .delete_permission_by_id(id, deletion_req)
            .await;

        match is_succeed {
            Some(u) => Ok(Json(u)),
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_user_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
    ) -> Result<Json<Vec<UserPermissionDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let user_permissions = _state.user_usecase().get_user_permissions().await;

        match user_permissions {
            Ok(u) => Ok(Json(u)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_role_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
    ) -> Result<Json<Vec<RolePermissionDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let role_permissions = _state.role_usecase().get_role_permissions().await;

        match role_permissions {
            Ok(u) => Ok(Json(u)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct PermissionHandler {}
