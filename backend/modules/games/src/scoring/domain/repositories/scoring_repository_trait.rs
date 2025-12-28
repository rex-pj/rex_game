use async_trait::async_trait;
use sea_orm::DbErr;

use crate::scoring::domain::models::{
    AchievementModel, GameProgressModel, GameSessionModel, GameTypeModel, LeaderboardEntry,
    UserAchievementModel, UserStatsModel,
};

#[async_trait]
pub trait ScoringRepositoryTrait: Send + Sync {
    // Game Types
    async fn get_game_types(&self) -> Result<Vec<GameTypeModel>, DbErr>;
    async fn get_game_type_by_code(&self, code: &str) -> Result<Option<GameTypeModel>, DbErr>;

    // Game Sessions
    async fn create_game_session(
        &self,
        user_id: i32,
        game_type_id: i32,
        flashcard_type_id: Option<i32>,
    ) -> Result<i32, DbErr>;

    async fn complete_game_session(
        &self,
        session_id: i32,
        score: i32,
        max_score: Option<i32>,
        correct_answers: i32,
        wrong_answers: i32,
        combo_max: i32,
        time_spent_seconds: i32,
    ) -> Result<(), DbErr>;

    async fn get_user_game_sessions(
        &self,
        user_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<GameSessionModel>, DbErr>;

    async fn get_user_best_sessions(
        &self,
        user_id: i32,
        game_type_id: Option<i32>,
        limit: u64,
    ) -> Result<Vec<GameSessionModel>, DbErr>;

    // User Stats
    async fn get_user_stats(&self, user_id: i32) -> Result<Option<UserStatsModel>, DbErr>;
    async fn update_user_stats(&self, user_id: i32, session: &GameSessionModel)
        -> Result<(), DbErr>;
    async fn ensure_user_stats_exists(&self, user_id: i32) -> Result<(), DbErr>;

    // Leaderboard
    async fn get_leaderboard(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<LeaderboardEntry>, DbErr>;

    async fn get_user_rank(&self, user_id: i32) -> Result<Option<i32>, DbErr>;

    // Achievements
    async fn get_achievements(&self) -> Result<Vec<AchievementModel>, DbErr>;
    async fn get_user_achievements(&self, user_id: i32)
        -> Result<Vec<UserAchievementModel>, DbErr>;
    async fn unlock_achievement(&self, user_id: i32, achievement_id: i32) -> Result<(), DbErr>;
    async fn check_achievement_unlocked(
        &self,
        user_id: i32,
        achievement_code: &str,
    ) -> Result<bool, DbErr>;

    // Game Progress
    async fn get_game_progress(
        &self,
        user_id: i32,
        game_type_id: i32,
    ) -> Result<Option<GameProgressModel>, DbErr>;

    async fn save_game_progress(
        &self,
        user_id: i32,
        game_type_id: i32,
        current_level: i32,
        total_score: i64,
    ) -> Result<GameProgressModel, DbErr>;

    async fn reset_game_progress(&self, user_id: i32, game_type_id: i32) -> Result<(), DbErr>;
}
