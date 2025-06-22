use crate::app_state::AppStateTrait;
use axum::{
    extract::{Query, State},
    Json,
};
use rex_game_application::roles::{role_dto::RoleDto, role_usecase_trait::RoleUseCaseTrait};
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
        State(_state): State<T>,
        Query(params): Query<RoleQuery>,
    ) -> Json<Option<Vec<RoleDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let roles = _state
            .role_usecase()
            .get_roles(params.name, params.description, page, page_size)
            .await;
        return match roles {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }
}

pub struct RoleHandler {}
