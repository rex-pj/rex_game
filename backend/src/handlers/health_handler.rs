use crate::app_state::AppState;
use axum::{extract::State, Json};
use rex_game_entities::entities::system_settings;
use sea_orm::{EntityTrait, QuerySelect};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: bool,
}

pub struct HealthHandler;

impl HealthHandler {
    pub async fn check(State(state): State<AppState>) -> Json<HealthResponse> {
        let db_ok = system_settings::Entity::find()
            .limit(1)
            .one(state.db_connection.as_ref())
            .await
            .is_ok();

        Json(HealthResponse {
            status: if db_ok {
                "ok".to_string()
            } else {
                "degraded".to_string()
            },
            database: db_ok,
        })
    }
}
