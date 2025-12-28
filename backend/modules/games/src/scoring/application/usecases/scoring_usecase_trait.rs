use async_trait::async_trait;
use sea_orm::DbErr;

use super::scoring_dto::{
    AchievementDto, CompleteGameSessionDto, GameCompleteResponseDto, GameProgressDto,
    GameSessionDto, GameTypeDto, LeaderboardEntryDto, SaveGameProgressDto, StartGameSessionDto,
    UserStatsDto,
};

#[async_trait]
pub trait ScoringUseCaseTrait: Send + Sync {
    // Game Types
    async fn get_game_types(&self) -> Result<Vec<GameTypeDto>, DbErr>;

    // Game Sessions
    async fn start_game_session(
        &self,
        user_id: i32,
        dto: StartGameSessionDto,
    ) -> Result<i32, DbErr>;

    async fn complete_game_session(
        &self,
        user_id: i32,
        dto: CompleteGameSessionDto,
    ) -> Result<GameCompleteResponseDto, DbErr>;

    async fn get_user_game_history(
        &self,
        user_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<GameSessionDto>, DbErr>;

    async fn get_user_best_games(
        &self,
        user_id: i32,
        game_type_code: Option<String>,
        limit: u64,
    ) -> Result<Vec<GameSessionDto>, DbErr>;

    // User Stats
    async fn get_user_stats(&self, user_id: i32) -> Result<Option<UserStatsDto>, DbErr>;

    // Leaderboard
    async fn get_leaderboard(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<LeaderboardEntryDto>, DbErr>;

    // Achievements
    async fn get_all_achievements(&self, user_id: Option<i32>) -> Result<Vec<AchievementDto>, DbErr>;
    async fn get_user_achievements(&self, user_id: i32) -> Result<Vec<AchievementDto>, DbErr>;

    // Game Progress
    async fn get_game_progress(
        &self,
        user_id: i32,
        game_type_code: &str,
    ) -> Result<Option<GameProgressDto>, DbErr>;

    async fn save_game_progress(
        &self,
        user_id: i32,
        dto: SaveGameProgressDto,
    ) -> Result<GameProgressDto, DbErr>;

    async fn reset_game_progress(&self, user_id: i32, game_type_code: &str) -> Result<(), DbErr>;
}
