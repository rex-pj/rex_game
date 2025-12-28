use crate::{
    app_state::AppState,
    view_models::{users::current_user::CurrentUser, HandlerError, HandlerResult},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use rex_game_games::{
    AchievementDto, CompleteGameSessionDto, GameCompleteResponseDto, GameProgressDto,
    GameSessionDto, GameTypeDto, LeaderboardEntryDto, SaveGameProgressDto, ScoringUseCaseTrait,
    StartGameSessionDto, UserStatsDto,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PaginationQuery {
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct BestGamesQuery {
    game_type: Option<String>,
    limit: Option<u64>,
}

#[derive(Deserialize)]
pub struct GameProgressQuery {
    game_type: String,
}

pub struct ScoringHandler {}

impl ScoringHandler {
    /// GET /game-types - Get all active game types
    pub async fn get_game_types(
        State(state): State<AppState>,
    ) -> HandlerResult<Json<Vec<GameTypeDto>>> {
        let game_types = state
            .usecases
            .scoring
            .get_game_types()
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch game types: {}", err),
                ..Default::default()
            })?;

        Ok(Json(game_types))
    }

    /// POST /games/sessions - Start a new game session
    pub async fn start_game_session(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Json(dto): Json<StartGameSessionDto>,
    ) -> HandlerResult<Json<i32>> {
        let session_id = state
            .usecases
            .scoring
            .start_game_session(current_user.id, dto)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: format!("Failed to start game session: {}", err),
                ..Default::default()
            })?;

        Ok(Json(session_id))
    }

    /// POST /games/sessions/complete - Complete a game session
    pub async fn complete_game_session(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Json(dto): Json<CompleteGameSessionDto>,
    ) -> HandlerResult<Json<GameCompleteResponseDto>> {
        let response = state
            .usecases
            .scoring
            .complete_game_session(current_user.id, dto)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: format!("Failed to complete game session: {}", err),
                ..Default::default()
            })?;

        Ok(Json(response))
    }

    /// GET /games/history - Get current user's game history
    pub async fn get_game_history(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Query(params): Query<PaginationQuery>,
    ) -> HandlerResult<Json<Vec<GameSessionDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);

        let sessions = state
            .usecases
            .scoring
            .get_user_game_history(current_user.id, page, page_size)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch game history: {}", err),
                ..Default::default()
            })?;

        Ok(Json(sessions))
    }

    /// GET /games/best - Get current user's best games
    pub async fn get_best_games(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Query(params): Query<BestGamesQuery>,
    ) -> HandlerResult<Json<Vec<GameSessionDto>>> {
        let limit = params.limit.unwrap_or(10);

        let sessions = state
            .usecases
            .scoring
            .get_user_best_games(current_user.id, params.game_type, limit)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch best games: {}", err),
                ..Default::default()
            })?;

        Ok(Json(sessions))
    }

    /// GET /users/me/stats - Get current user's stats
    pub async fn get_my_stats(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
    ) -> HandlerResult<Json<Option<UserStatsDto>>> {
        let stats = state
            .usecases
            .scoring
            .get_user_stats(current_user.id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch user stats: {}", err),
                ..Default::default()
            })?;

        Ok(Json(stats))
    }

    /// GET /users/{user_id}/stats - Get specific user's stats (public)
    pub async fn get_user_stats(
        State(state): State<AppState>,
        Path(user_id): Path<i32>,
    ) -> HandlerResult<Json<Option<UserStatsDto>>> {
        let stats = state
            .usecases
            .scoring
            .get_user_stats(user_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch user stats: {}", err),
                ..Default::default()
            })?;

        Ok(Json(stats))
    }

    /// GET /leaderboard - Get global leaderboard
    pub async fn get_leaderboard(
        State(state): State<AppState>,
        Query(params): Query<PaginationQuery>,
    ) -> HandlerResult<Json<Vec<LeaderboardEntryDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);

        let entries = state
            .usecases
            .scoring
            .get_leaderboard(page, page_size)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch leaderboard: {}", err),
                ..Default::default()
            })?;

        Ok(Json(entries))
    }

    /// GET /achievements - Get all achievements (with user progress if authenticated)
    pub async fn get_achievements(
        State(state): State<AppState>,
    ) -> HandlerResult<Json<Vec<AchievementDto>>> {
        let achievements = state
            .usecases
            .scoring
            .get_all_achievements(None)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch achievements: {}", err),
                ..Default::default()
            })?;

        Ok(Json(achievements))
    }

    /// GET /users/me/achievements - Get current user's achievements
    pub async fn get_my_achievements(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
    ) -> HandlerResult<Json<Vec<AchievementDto>>> {
        let achievements = state
            .usecases
            .scoring
            .get_all_achievements(Some(current_user.id))
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch achievements: {}", err),
                ..Default::default()
            })?;

        Ok(Json(achievements))
    }

    /// GET /games/progress - Get current user's game progress
    pub async fn get_game_progress(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Query(params): Query<GameProgressQuery>,
    ) -> HandlerResult<Json<Option<GameProgressDto>>> {
        let progress = state
            .usecases
            .scoring
            .get_game_progress(current_user.id, &params.game_type)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch game progress: {}", err),
                ..Default::default()
            })?;

        Ok(Json(progress))
    }

    /// POST /games/progress - Save game progress
    pub async fn save_game_progress(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Json(dto): Json<SaveGameProgressDto>,
    ) -> HandlerResult<Json<GameProgressDto>> {
        let progress = state
            .usecases
            .scoring
            .save_game_progress(current_user.id, dto)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: format!("Failed to save game progress: {}", err),
                ..Default::default()
            })?;

        Ok(Json(progress))
    }

    /// DELETE /games/progress - Reset game progress
    pub async fn reset_game_progress(
        State(state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Query(params): Query<GameProgressQuery>,
    ) -> HandlerResult<StatusCode> {
        state
            .usecases
            .scoring
            .reset_game_progress(current_user.id, &params.game_type)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: format!("Failed to reset game progress: {}", err),
                ..Default::default()
            })?;

        Ok(StatusCode::NO_CONTENT)
    }
}
