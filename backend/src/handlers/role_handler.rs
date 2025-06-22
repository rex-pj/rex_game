use std::{collections::HashMap, sync::Arc};

use crate::{
    app_state::AppStateTrait,
    view_models::{
        roles::role_create_request::RoleCreateRequest, users::current_user::CurrentUser,
    },
};
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use hyper::StatusCode;
use rex_game_application::{
    page_list_dto::PageListDto,
    roles::{
        role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto, role_dto::RoleDto,
        role_updation_dto::RoleUpdationDto, role_usecase_trait::RoleUseCaseTrait,
    },
    users::roles::{ROLE_ADMIN, ROLE_ROOT_ADMIN},
};
use serde::Deserialize;

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
            .any(|role| role == ROLE_ROOT_ADMIN || role == ROLE_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let roles = _state
            .role_usecase()
            .get_roles(params.name, params.description, page, page_size)
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
            .any(|role| role == ROLE_ROOT_ADMIN || role == ROLE_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
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
            .any(|role| role == ROLE_ROOT_ADMIN || role == ROLE_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let existing = _state
            .role_usecase()
            .get_role_by_id(id)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

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
            .any(|role| role == ROLE_ROOT_ADMIN || role == ROLE_ADMIN)
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
}

pub struct RoleHandler {}
