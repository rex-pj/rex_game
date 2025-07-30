use std::collections::HashMap;
use std::sync::Arc;

use crate::view_models::users::assign_permission_request::AssignPermissionRequest;
use crate::view_models::users::assign_role_request::AssignRoleRequest;
use crate::view_models::users::current_user::CurrentUser;
use crate::{app_state::AppStateTrait, view_models::users::signup_request::SignupRequest};
use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use axum::Extension;
use axum::{extract::State, Json};
use hyper::StatusCode;
use rex_game_application::page_list_dto::PageListDto;
use rex_game_application::permissions::permission_usecase_trait::PermissionUseCaseTrait;
use rex_game_application::roles::role_usecase_trait::RoleUseCaseTrait;
use rex_game_application::users::roles::ROLE_ROOT_ADMIN;
use rex_game_application::users::user_deletion_dto::UserDeletionDto;
use rex_game_application::users::user_dto::UserDto;
use rex_game_application::users::user_permission_creation_dto::UserPermissionCreationDto;
use rex_game_application::users::user_permission_dto::UserPermissionDto;
use rex_game_application::users::user_role_creation_dto::UserRoleCreationDto;
use rex_game_application::users::user_role_dto::UserRoleDto;
use rex_game_application::users::user_updation_dto::UserUpdationDto;
use rex_game_application::users::user_usecase_trait::UserUseCaseTrait;
use rex_game_application::{
    identities::{
        identity_user_usecase_trait::IdentityUserUseCaseTrait, user_creation_dto::UserCreationDto,
    },
    users::loggedin_user_dto::LoggedInUserDto,
};
use rex_game_domain::models::user_statuses::UserStatuses;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserQuery {
    page: Option<u64>,
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
    ) -> Result<Json<PageListDto<UserDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
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
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
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
    ) -> Result<Json<i32>, StatusCode> {
        let req = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };
        let new_user = UserCreationDto {
            email: req.email,
            name: req.name,
            display_name: req.display_name,
            status_id: UserStatuses::Actived as i32,
            ..Default::default()
        };
        let signup_result = _state
            .identity_user_usecase()
            .create_user(new_user, &req.password)
            .await;
        match signup_result {
            Ok(created) => Ok(Json(created.id)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_current_user<T: AppStateTrait>(
        headers: HeaderMap,
        State(_state): State<T>,
    ) -> Result<Json<LoggedInUserDto>, StatusCode> {
        let access_token_header = match headers.get("authorization") {
            Some(authorization) => authorization,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let access_token = match access_token_header.to_str() {
            Ok(authorization) => authorization,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        };

        let access_token = access_token.strip_prefix("Bearer ").unwrap();
        let logged_in_user_result = _state
            .identity_user_usecase()
            .get_logged_in_user(access_token)
            .await;

        match logged_in_user_result {
            Ok(user) => Ok(Json(user)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn update_user<T: AppStateTrait>(
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

        let existing = _state
            .user_usecase()
            .get_user_by_id(current_user.id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        if requests.get("name").is_none()
            && requests.get("display_name").is_none()
            && requests.get("email").is_none()
        {
            return Err(StatusCode::BAD_REQUEST);
        }

        if existing.id != current_user.id
            && !current_user
                .roles
                .iter()
                .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::UNAUTHORIZED);
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
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_user<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
    ) -> Result<Json<bool>, StatusCode> {
        let deletion_req = UserDeletionDto {
            updated_by_id: Some(current_user.id),
        };

        let existing = _state
            .user_usecase()
            .get_user_by_id(current_user.id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        if existing.id == current_user.id {
            return Err(StatusCode::CONFLICT);
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let is_succeed = _state
            .user_usecase()
            .delete_user_by_id(id, deletion_req)
            .await;

        match is_succeed {
            Some(u) => Ok(Json(u)),
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn assign_roles<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(user_id): Path<i32>,
        Json(payload): Json<Option<AssignRoleRequest>>,
    ) -> Result<Json<i32>, StatusCode> {
        let requests = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let role_ids = match requests.role_ids {
            Some(id) => id,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        if role_ids.len() == 0 {
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
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let incomming_roles = _state
            .role_usecase()
            .get_roles_by_ids(role_ids)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let existing_assignments = _state
            .user_usecase()
            .get_user_roles_by_user_id(user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    ) -> Result<Json<Vec<UserRoleDto>>, StatusCode> {
        _state
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let user_roles = _state
            .user_usecase()
            .get_user_roles_by_user_id(user_id)
            .await;

        match user_roles {
            Ok(u) => Ok(Json(u)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn assign_permissions<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(user_id): Path<i32>,
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
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let incomming_permissions = _state
            .permission_usecase()
            .get_permission_by_codes(permission_codes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let existing_assignments = _state
            .user_usecase()
            .get_user_permissions_by_user_id(user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    ) -> Result<Json<Vec<UserPermissionDto>>, StatusCode> {
        _state
            .user_usecase()
            .get_user_by_id(user_id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let user_permissions = _state
            .user_usecase()
            .get_user_permissions_by_user_id(user_id)
            .await;

        match user_permissions {
            Ok(u) => Ok(Json(u)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct UserHandler {}
