use async_trait::async_trait;
use sea_orm::DbErr;

use rex_game_shared::domain::models::page_list_model::PageListModel;

use super::scoring_dto::{
    AchievementCreationDto, AchievementDto, AchievementUpdationDto, AdminAchievementDto,
    AdminGameSessionDto, AdminUserStatsDto, CompleteGameSessionDto, GameCompleteResponseDto,
    GameProgressDto, GameSessionDto, GameTypeCreationDto, GameTypeDto, GameTypeUpdationDto,
    LeaderboardEntryDto, SaveGameProgressDto, StartGameSessionDto, UserStatsDto,
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

    // Admin: Game Types
    async fn admin_get_game_types(&self, name: Option<String>, page: u64, page_size: u64) -> Result<PageListModel<GameTypeDto>, DbErr>;
    async fn admin_get_game_type_by_id(&self, id: i32) -> Result<Option<GameTypeDto>, DbErr>;
    async fn admin_create_game_type(&self, dto: GameTypeCreationDto) -> Result<i32, DbErr>;
    async fn admin_update_game_type(&self, id: i32, dto: GameTypeUpdationDto) -> Result<bool, DbErr>;
    async fn admin_delete_game_type(&self, id: i32) -> Result<u64, DbErr>;
    async fn admin_toggle_game_type_active(&self, id: i32) -> Result<bool, DbErr>;

    // Admin: Achievements
    async fn admin_get_achievements(&self, name: Option<String>, page: u64, page_size: u64) -> Result<PageListModel<AdminAchievementDto>, DbErr>;
    async fn admin_get_achievement_by_id(&self, id: i32) -> Result<Option<AdminAchievementDto>, DbErr>;
    async fn admin_create_achievement(&self, dto: AchievementCreationDto) -> Result<i32, DbErr>;
    async fn admin_update_achievement(&self, id: i32, dto: AchievementUpdationDto) -> Result<bool, DbErr>;
    async fn admin_delete_achievement(&self, id: i32) -> Result<u64, DbErr>;
    async fn admin_toggle_achievement_active(&self, id: i32) -> Result<bool, DbErr>;

    // Admin: Game Sessions
    async fn admin_get_game_sessions(&self, page: u64, page_size: u64) -> Result<PageListModel<AdminGameSessionDto>, DbErr>;
    async fn admin_delete_game_session(&self, id: i32) -> Result<u64, DbErr>;

    // Admin: User Stats
    async fn admin_get_user_stats(&self, page: u64, page_size: u64) -> Result<PageListModel<AdminUserStatsDto>, DbErr>;
    async fn admin_reset_user_stats(&self, user_id: i32) -> Result<bool, DbErr>;
}
