use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSessionModel {
    pub id: i32,
    pub user_id: i32,
    pub game_type_id: i32,
    pub game_type_code: Option<String>,
    pub game_type_name: Option<String>,
    pub flashcard_type_id: Option<i32>,
    pub score: i32,
    pub max_score: Option<i32>,
    pub accuracy: Option<Decimal>,
    pub time_spent_seconds: i32,
    pub cards_played: i32,
    pub correct_answers: i32,
    pub wrong_answers: i32,
    pub combo_max: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTypeModel {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_actived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsModel {
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
    pub last_played_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementModel {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub points: i32,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAchievementModel {
    pub id: i32,
    pub user_id: i32,
    pub achievement: AchievementModel,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i32,
    pub user_id: i32,
    pub user_name: String,
    pub user_display_name: Option<String>,
    pub total_score: i64,
    pub total_games_played: i32,
    pub best_score: i32,
    pub average_accuracy: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProgressModel {
    pub id: i32,
    pub user_id: i32,
    pub game_type_id: i32,
    pub game_type_code: Option<String>,
    pub game_type_name: Option<String>,
    pub current_level: i32,
    pub highest_level: i32,
    pub total_score: i64,
    pub last_played_at: DateTime<Utc>,
}
