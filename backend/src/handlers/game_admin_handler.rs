use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use rex_game_games::{
    AchievementCreationDto, AchievementUpdationDto, AdminAchievementDto, AdminGameSessionDto,
    AdminUserStatsDto, GameTypeCreationDto, GameTypeDto, GameTypeUpdationDto, ScoringUseCaseTrait,
};
use rex_game_shared::domain::models::PageListModel;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    view_models::{HandlerError, HandlerResult},
};

#[derive(Deserialize)]
pub struct AdminListQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
}

pub struct GameAdminHandler {}

impl GameAdminHandler {
    // ---- Game Types ----

    pub async fn get_game_types(
        State(state): State<AppState>,
        Query(params): Query<AdminListQuery>,
    ) -> Result<Json<PageListModel<GameTypeDto>>, StatusCode> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        match state
            .usecases
            .scoring
            .admin_get_game_types(params.name, page, page_size)
            .await
        {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_game_type_by_id(
        Path(id): Path<i32>,
        State(state): State<AppState>,
    ) -> HandlerResult<Json<GameTypeDto>> {
        match state.usecases.scoring.admin_get_game_type_by_id(id).await {
            Ok(Some(data)) => Ok(Json(data)),
            Ok(None) => Err(HandlerError {
                status: StatusCode::NOT_FOUND,
                message: "Game type not found".to_string(),
                ..Default::default()
            }),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to fetch game type".to_string(),
                ..Default::default()
            }),
        }
    }

    pub async fn create_game_type(
        State(state): State<AppState>,
        Json(payload): Json<Option<GameTypeCreationDto>>,
    ) -> HandlerResult<Json<i32>> {
        let dto = match payload {
            Some(d) => d,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        if dto.code.is_empty() || dto.name.is_empty() {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Code and name are required".to_string(),
                ..Default::default()
            });
        }

        match state.usecases.scoring.admin_create_game_type(dto).await {
            Ok(id) => Ok(Json(id)),
            Err(e) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to create game type: {}", e),
                ..Default::default()
            }),
        }
    }

    pub async fn update_game_type(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<Option<HashMap<String, serde_json::Value>>>,
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

        let mut dto = GameTypeUpdationDto::default();
        for (key, value) in &requests {
            match key.as_str() {
                "code" => dto.code = value.as_str().map(|s| s.to_string()),
                "name" => dto.name = value.as_str().map(|s| s.to_string()),
                "description" => dto.description = value.as_str().map(|s| s.to_string()),
                "icon" => dto.icon = value.as_str().map(|s| s.to_string()),
                _ => {}
            }
        }

        match state.usecases.scoring.admin_update_game_type(id, dto).await {
            Ok(updated) => Ok(Json(updated)),
            Err(e) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to update game type: {}", e),
                ..Default::default()
            }),
        }
    }

    pub async fn delete_game_type(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<u64>> {
        match state.usecases.scoring.admin_delete_game_type(id).await {
            Ok(count) => Ok(Json(count)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to delete game type".to_string(),
                ..Default::default()
            }),
        }
    }

    pub async fn toggle_game_type_active(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        match state
            .usecases
            .scoring
            .admin_toggle_game_type_active(id)
            .await
        {
            Ok(new_status) => Ok(Json(new_status)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to toggle game type status".to_string(),
                ..Default::default()
            }),
        }
    }

    // ---- Achievements ----

    pub async fn get_achievements(
        State(state): State<AppState>,
        Query(params): Query<AdminListQuery>,
    ) -> Result<Json<PageListModel<AdminAchievementDto>>, StatusCode> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        match state
            .usecases
            .scoring
            .admin_get_achievements(params.name, page, page_size)
            .await
        {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_achievement_by_id(
        Path(id): Path<i32>,
        State(state): State<AppState>,
    ) -> HandlerResult<Json<AdminAchievementDto>> {
        match state.usecases.scoring.admin_get_achievement_by_id(id).await {
            Ok(Some(data)) => Ok(Json(data)),
            Ok(None) => Err(HandlerError {
                status: StatusCode::NOT_FOUND,
                message: "Achievement not found".to_string(),
                ..Default::default()
            }),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to fetch achievement".to_string(),
                ..Default::default()
            }),
        }
    }

    pub async fn create_achievement(
        State(state): State<AppState>,
        Json(payload): Json<Option<AchievementCreationDto>>,
    ) -> HandlerResult<Json<i32>> {
        let dto = match payload {
            Some(d) => d,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        if dto.code.is_empty() || dto.name.is_empty() {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Code and name are required".to_string(),
                ..Default::default()
            });
        }

        match state.usecases.scoring.admin_create_achievement(dto).await {
            Ok(id) => Ok(Json(id)),
            Err(e) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to create achievement: {}", e),
                ..Default::default()
            }),
        }
    }

    pub async fn update_achievement(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<Option<HashMap<String, serde_json::Value>>>,
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

        let mut dto = AchievementUpdationDto::default();
        for (key, value) in &requests {
            match key.as_str() {
                "code" => dto.code = value.as_str().map(|s| s.to_string()),
                "name" => dto.name = value.as_str().map(|s| s.to_string()),
                "description" => dto.description = value.as_str().map(|s| s.to_string()),
                "icon" => dto.icon = value.as_str().map(|s| s.to_string()),
                "points" => dto.points = value.as_i64().map(|v| v as i32),
                "category" => dto.category = value.as_str().map(|s| s.to_string()),
                _ => {}
            }
        }

        match state
            .usecases
            .scoring
            .admin_update_achievement(id, dto)
            .await
        {
            Ok(updated) => Ok(Json(updated)),
            Err(e) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to update achievement: {}", e),
                ..Default::default()
            }),
        }
    }

    pub async fn delete_achievement(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<u64>> {
        match state.usecases.scoring.admin_delete_achievement(id).await {
            Ok(count) => Ok(Json(count)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to delete achievement".to_string(),
                ..Default::default()
            }),
        }
    }

    pub async fn toggle_achievement_active(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        match state
            .usecases
            .scoring
            .admin_toggle_achievement_active(id)
            .await
        {
            Ok(new_status) => Ok(Json(new_status)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to toggle achievement status".to_string(),
                ..Default::default()
            }),
        }
    }

    // ---- Game Sessions ----

    pub async fn get_game_sessions(
        State(state): State<AppState>,
        Query(params): Query<AdminListQuery>,
    ) -> Result<Json<PageListModel<AdminGameSessionDto>>, StatusCode> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        match state
            .usecases
            .scoring
            .admin_get_game_sessions(page, page_size)
            .await
        {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn delete_game_session(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<u64>> {
        match state.usecases.scoring.admin_delete_game_session(id).await {
            Ok(count) => Ok(Json(count)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to delete game session".to_string(),
                ..Default::default()
            }),
        }
    }

    // ---- User Stats ----

    pub async fn get_user_stats_list(
        State(state): State<AppState>,
        Query(params): Query<AdminListQuery>,
    ) -> Result<Json<PageListModel<AdminUserStatsDto>>, StatusCode> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        match state
            .usecases
            .scoring
            .admin_get_user_stats(page, page_size)
            .await
        {
            Ok(data) => Ok(Json(data)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn reset_user_stats(
        State(state): State<AppState>,
        Path(user_id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        match state.usecases.scoring.admin_reset_user_stats(user_id).await {
            Ok(result) => Ok(Json(result)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to reset user stats".to_string(),
                ..Default::default()
            }),
        }
    }
}
