use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameTypeDto {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_actived: bool,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StartGameSessionDto {
    #[validate(length(min = 1, max = 50))]
    pub game_type_code: String,
    pub flashcard_type_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CompleteGameSessionDto {
    pub session_id: i32,
    #[validate(range(min = 0))]
    pub score: i32,
    pub max_score: Option<i32>,
    #[validate(range(min = 0))]
    pub correct_answers: i32,
    #[validate(range(min = 0))]
    pub wrong_answers: i32,
    #[validate(range(min = 0))]
    pub combo_max: i32,
    #[validate(range(min = 0))]
    pub time_spent_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSessionDto {
    pub id: i32,
    pub game_type_code: Option<String>,
    pub game_type_name: Option<String>,
    pub score: i32,
    pub max_score: Option<i32>,
    pub accuracy: Option<Decimal>,
    pub time_spent_seconds: i32,
    pub cards_played: i32,
    pub correct_answers: i32,
    pub wrong_answers: i32,
    pub combo_max: i32,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatsDto {
    pub user_id: i32,
    pub user_name: Option<String>,
    pub user_display_name: Option<String>,
    pub total_score: i64,
    pub total_games_played: i32,
    pub total_time_played_seconds: i64,
    pub best_score: i32,
    pub best_combo: i32,
    pub average_accuracy: Decimal,
    pub current_streak: i32,
    pub best_streak: i32,
    pub rank: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardEntryDto {
    pub rank: i32,
    pub user_id: i32,
    pub user_name: String,
    pub user_display_name: Option<String>,
    pub total_score: i64,
    pub total_games_played: i32,
    pub best_score: i32,
    pub average_accuracy: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementDto {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub points: i32,
    pub category: Option<String>,
    pub unlocked: bool,
    pub unlocked_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameCompleteResponseDto {
    pub session: GameSessionDto,
    pub new_achievements: Vec<AchievementDto>,
    pub updated_stats: UserStatsDto,
}

// Game Progress DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct GameProgressDto {
    pub id: i32,
    pub user_id: i32,
    pub game_type_id: i32,
    pub game_type_code: Option<String>,
    pub game_type_name: Option<String>,
    pub current_level: i32,
    pub highest_level: i32,
    pub total_score: i64,
    pub last_played_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SaveGameProgressDto {
    #[validate(length(min = 1, max = 50))]
    pub game_type_code: String,
    #[validate(range(min = 1))]
    pub current_level: i32,
    #[validate(range(min = 0))]
    pub total_score: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetGameProgressDto {
    pub game_type_code: String,
}

// ---- Admin DTOs ----

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GameTypeCreationDto {
    #[validate(length(min = 1, max = 50))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameTypeUpdationDto {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminAchievementDto {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub points: i32,
    pub category: Option<String>,
    pub is_actived: bool,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AchievementCreationDto {
    #[validate(length(min = 1, max = 50))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    #[validate(range(min = 0))]
    pub points: i32,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AchievementUpdationDto {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub points: Option<i32>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminGameSessionDto {
    pub id: i32,
    pub user_id: i32,
    pub user_name: Option<String>,
    pub user_display_name: Option<String>,
    pub game_type_code: Option<String>,
    pub game_type_name: Option<String>,
    pub score: i32,
    pub max_score: Option<i32>,
    pub accuracy: Option<Decimal>,
    pub time_spent_seconds: i32,
    pub cards_played: i32,
    pub correct_answers: i32,
    pub wrong_answers: i32,
    pub combo_max: i32,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub created_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserStatsDto {
    pub id: i32,
    pub user_id: i32,
    pub user_name: Option<String>,
    pub user_display_name: Option<String>,
    pub total_score: i64,
    pub total_games_played: i32,
    pub total_time_played_seconds: i64,
    pub best_score: i32,
    pub best_combo: i32,
    pub average_accuracy: Decimal,
    pub current_streak: i32,
    pub best_streak: i32,
    pub last_played_at: Option<String>,
}
