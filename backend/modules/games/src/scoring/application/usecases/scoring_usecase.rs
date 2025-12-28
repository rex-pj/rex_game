use async_trait::async_trait;
use sea_orm::DbErr;
use std::sync::Arc;

use crate::scoring::domain::repositories::ScoringRepositoryTrait;

use super::scoring_dto::{
    AchievementDto, CompleteGameSessionDto, GameCompleteResponseDto, GameProgressDto,
    GameSessionDto, GameTypeDto, LeaderboardEntryDto, SaveGameProgressDto, StartGameSessionDto,
    UserStatsDto,
};
use super::scoring_usecase_trait::ScoringUseCaseTrait;

#[derive(Clone)]
pub struct ScoringUseCase {
    repository: Arc<dyn ScoringRepositoryTrait>,
}

impl ScoringUseCase {
    pub fn new(repository: Arc<dyn ScoringRepositoryTrait>) -> Self {
        Self { repository }
    }

    async fn check_and_unlock_achievements(
        &self,
        user_id: i32,
        session_score: i32,
        session_accuracy: Option<rust_decimal::Decimal>,
        session_time: i32,
        combo_max: i32,
    ) -> Result<Vec<AchievementDto>, DbErr> {
        let mut new_achievements = Vec::new();
        let stats = self.repository.get_user_stats(user_id).await?;
        let achievements = self.repository.get_achievements().await?;

        for achievement in achievements {
            let already_unlocked = self
                .repository
                .check_achievement_unlocked(user_id, &achievement.code)
                .await?;

            if already_unlocked {
                continue;
            }

            let should_unlock = match achievement.code.as_str() {
                "first_game" => stats.as_ref().map(|s| s.total_games_played >= 1).unwrap_or(false),
                "perfect_score" => session_accuracy
                    .map(|acc| acc >= rust_decimal::Decimal::from(100))
                    .unwrap_or(false),
                "speed_demon" => session_time <= 30 && session_score > 0,
                "combo_master" => combo_max >= 10,
                "streak_7" => stats.as_ref().map(|s| s.current_streak >= 7).unwrap_or(false),
                "games_10" => stats.as_ref().map(|s| s.total_games_played >= 10).unwrap_or(false),
                "games_50" => stats.as_ref().map(|s| s.total_games_played >= 50).unwrap_or(false),
                "games_100" => stats.as_ref().map(|s| s.total_games_played >= 100).unwrap_or(false),
                "score_1000" => stats.as_ref().map(|s| s.total_score >= 1000).unwrap_or(false),
                "score_10000" => stats.as_ref().map(|s| s.total_score >= 10000).unwrap_or(false),
                _ => false,
            };

            if should_unlock {
                self.repository
                    .unlock_achievement(user_id, achievement.id)
                    .await?;

                new_achievements.push(AchievementDto {
                    id: achievement.id,
                    code: achievement.code,
                    name: achievement.name,
                    description: achievement.description,
                    icon: achievement.icon,
                    points: achievement.points,
                    category: achievement.category,
                    unlocked: true,
                    unlocked_at: Some(chrono::Utc::now().to_rfc3339()),
                });
            }
        }

        Ok(new_achievements)
    }
}

#[async_trait]
impl ScoringUseCaseTrait for ScoringUseCase {
    async fn get_game_types(&self) -> Result<Vec<GameTypeDto>, DbErr> {
        let game_types = self.repository.get_game_types().await?;

        Ok(game_types
            .into_iter()
            .map(|gt| GameTypeDto {
                id: gt.id,
                code: gt.code,
                name: gt.name,
                description: gt.description,
                icon: gt.icon,
            })
            .collect())
    }

    async fn start_game_session(
        &self,
        user_id: i32,
        dto: StartGameSessionDto,
    ) -> Result<i32, DbErr> {
        let game_type = self
            .repository
            .get_game_type_by_code(&dto.game_type_code)
            .await?
            .ok_or(DbErr::Custom(format!(
                "Game type not found: {}",
                dto.game_type_code
            )))?;

        self.repository.ensure_user_stats_exists(user_id).await?;

        let session_id = self
            .repository
            .create_game_session(user_id, game_type.id, dto.flashcard_type_id)
            .await?;

        Ok(session_id)
    }

    async fn complete_game_session(
        &self,
        user_id: i32,
        dto: CompleteGameSessionDto,
    ) -> Result<GameCompleteResponseDto, DbErr> {
        // Complete the session
        self.repository
            .complete_game_session(
                dto.session_id,
                dto.score,
                dto.max_score,
                dto.correct_answers,
                dto.wrong_answers,
                dto.combo_max,
                dto.time_spent_seconds,
            )
            .await?;

        // Get completed session
        let sessions = self.repository.get_user_game_sessions(user_id, 1, 1).await?;
        let session = sessions.into_iter().find(|s| s.id == dto.session_id);

        let session_model = session.ok_or(DbErr::Custom("Session not found".to_string()))?;

        // Update user stats
        self.repository
            .update_user_stats(user_id, &session_model)
            .await?;

        // Check achievements
        let new_achievements = self
            .check_and_unlock_achievements(
                user_id,
                dto.score,
                session_model.accuracy,
                dto.time_spent_seconds,
                dto.combo_max,
            )
            .await?;

        // Get updated stats
        let stats = self.repository.get_user_stats(user_id).await?;
        let rank = self.repository.get_user_rank(user_id).await?;

        let stats_dto = stats
            .map(|s| UserStatsDto {
                user_id: s.user_id,
                user_name: s.user_name,
                user_display_name: s.user_display_name,
                total_score: s.total_score,
                total_games_played: s.total_games_played,
                total_time_played_seconds: s.total_time_played_seconds,
                best_score: s.best_score,
                best_combo: s.best_combo,
                average_accuracy: s.average_accuracy,
                current_streak: s.current_streak,
                best_streak: s.best_streak,
                rank,
            })
            .unwrap_or(UserStatsDto {
                user_id,
                user_name: None,
                user_display_name: None,
                total_score: 0,
                total_games_played: 0,
                total_time_played_seconds: 0,
                best_score: 0,
                best_combo: 0,
                average_accuracy: rust_decimal::Decimal::ZERO,
                current_streak: 0,
                best_streak: 0,
                rank: None,
            });

        Ok(GameCompleteResponseDto {
            session: GameSessionDto {
                id: session_model.id,
                game_type_code: session_model.game_type_code,
                game_type_name: session_model.game_type_name,
                score: session_model.score,
                max_score: session_model.max_score,
                accuracy: session_model.accuracy,
                time_spent_seconds: session_model.time_spent_seconds,
                cards_played: session_model.cards_played,
                correct_answers: session_model.correct_answers,
                wrong_answers: session_model.wrong_answers,
                combo_max: session_model.combo_max,
                completed_at: session_model.completed_at.map(|dt| dt.to_rfc3339()),
            },
            new_achievements,
            updated_stats: stats_dto,
        })
    }

    async fn get_user_game_history(
        &self,
        user_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<GameSessionDto>, DbErr> {
        let sessions = self
            .repository
            .get_user_game_sessions(user_id, page, page_size)
            .await?;

        Ok(sessions
            .into_iter()
            .map(|s| GameSessionDto {
                id: s.id,
                game_type_code: s.game_type_code,
                game_type_name: s.game_type_name,
                score: s.score,
                max_score: s.max_score,
                accuracy: s.accuracy,
                time_spent_seconds: s.time_spent_seconds,
                cards_played: s.cards_played,
                correct_answers: s.correct_answers,
                wrong_answers: s.wrong_answers,
                combo_max: s.combo_max,
                completed_at: s.completed_at.map(|dt| dt.to_rfc3339()),
            })
            .collect())
    }

    async fn get_user_best_games(
        &self,
        user_id: i32,
        game_type_code: Option<String>,
        limit: u64,
    ) -> Result<Vec<GameSessionDto>, DbErr> {
        let game_type_id = if let Some(code) = game_type_code {
            self.repository
                .get_game_type_by_code(&code)
                .await?
                .map(|gt| gt.id)
        } else {
            None
        };

        let sessions = self
            .repository
            .get_user_best_sessions(user_id, game_type_id, limit)
            .await?;

        Ok(sessions
            .into_iter()
            .map(|s| GameSessionDto {
                id: s.id,
                game_type_code: s.game_type_code,
                game_type_name: s.game_type_name,
                score: s.score,
                max_score: s.max_score,
                accuracy: s.accuracy,
                time_spent_seconds: s.time_spent_seconds,
                cards_played: s.cards_played,
                correct_answers: s.correct_answers,
                wrong_answers: s.wrong_answers,
                combo_max: s.combo_max,
                completed_at: s.completed_at.map(|dt| dt.to_rfc3339()),
            })
            .collect())
    }

    async fn get_user_stats(&self, user_id: i32) -> Result<Option<UserStatsDto>, DbErr> {
        let stats = self.repository.get_user_stats(user_id).await?;
        let rank = self.repository.get_user_rank(user_id).await?;

        Ok(stats.map(|s| UserStatsDto {
            user_id: s.user_id,
            user_name: s.user_name,
            user_display_name: s.user_display_name,
            total_score: s.total_score,
            total_games_played: s.total_games_played,
            total_time_played_seconds: s.total_time_played_seconds,
            best_score: s.best_score,
            best_combo: s.best_combo,
            average_accuracy: s.average_accuracy,
            current_streak: s.current_streak,
            best_streak: s.best_streak,
            rank,
        }))
    }

    async fn get_leaderboard(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<LeaderboardEntryDto>, DbErr> {
        let entries = self.repository.get_leaderboard(page, page_size).await?;

        Ok(entries
            .into_iter()
            .map(|e| LeaderboardEntryDto {
                rank: e.rank,
                user_id: e.user_id,
                user_name: e.user_name,
                user_display_name: e.user_display_name,
                total_score: e.total_score,
                total_games_played: e.total_games_played,
                best_score: e.best_score,
                average_accuracy: e.average_accuracy,
            })
            .collect())
    }

    async fn get_all_achievements(&self, user_id: Option<i32>) -> Result<Vec<AchievementDto>, DbErr> {
        let achievements = self.repository.get_achievements().await?;

        let user_achievements = if let Some(uid) = user_id {
            self.repository.get_user_achievements(uid).await?
        } else {
            Vec::new()
        };

        Ok(achievements
            .into_iter()
            .map(|a| {
                let user_ach = user_achievements
                    .iter()
                    .find(|ua| ua.achievement.id == a.id);

                AchievementDto {
                    id: a.id,
                    code: a.code,
                    name: a.name,
                    description: a.description,
                    icon: a.icon,
                    points: a.points,
                    category: a.category,
                    unlocked: user_ach.is_some(),
                    unlocked_at: user_ach.map(|ua| ua.unlocked_at.to_rfc3339()),
                }
            })
            .collect())
    }

    async fn get_user_achievements(&self, user_id: i32) -> Result<Vec<AchievementDto>, DbErr> {
        let user_achievements = self.repository.get_user_achievements(user_id).await?;

        Ok(user_achievements
            .into_iter()
            .map(|ua| AchievementDto {
                id: ua.achievement.id,
                code: ua.achievement.code,
                name: ua.achievement.name,
                description: ua.achievement.description,
                icon: ua.achievement.icon,
                points: ua.achievement.points,
                category: ua.achievement.category,
                unlocked: true,
                unlocked_at: Some(ua.unlocked_at.to_rfc3339()),
            })
            .collect())
    }

    // Game Progress
    async fn get_game_progress(
        &self,
        user_id: i32,
        game_type_code: &str,
    ) -> Result<Option<GameProgressDto>, DbErr> {
        let game_type = self.repository.get_game_type_by_code(game_type_code).await?;

        let game_type = match game_type {
            Some(gt) => gt,
            None => return Ok(None),
        };

        let progress = self
            .repository
            .get_game_progress(user_id, game_type.id)
            .await?;

        Ok(progress.map(|p| GameProgressDto {
            id: p.id,
            user_id: p.user_id,
            game_type_id: p.game_type_id,
            game_type_code: p.game_type_code,
            game_type_name: p.game_type_name,
            current_level: p.current_level,
            highest_level: p.highest_level,
            total_score: p.total_score,
            last_played_at: p.last_played_at.to_rfc3339(),
        }))
    }

    async fn save_game_progress(
        &self,
        user_id: i32,
        dto: SaveGameProgressDto,
    ) -> Result<GameProgressDto, DbErr> {
        let game_type = self
            .repository
            .get_game_type_by_code(&dto.game_type_code)
            .await?;

        let game_type = match game_type {
            Some(gt) => gt,
            None => {
                return Err(DbErr::Custom(format!(
                    "Game type not found: {}",
                    dto.game_type_code
                )))
            }
        };

        let progress = self
            .repository
            .save_game_progress(user_id, game_type.id, dto.current_level, dto.total_score)
            .await?;

        Ok(GameProgressDto {
            id: progress.id,
            user_id: progress.user_id,
            game_type_id: progress.game_type_id,
            game_type_code: progress.game_type_code,
            game_type_name: progress.game_type_name,
            current_level: progress.current_level,
            highest_level: progress.highest_level,
            total_score: progress.total_score,
            last_played_at: progress.last_played_at.to_rfc3339(),
        })
    }

    async fn reset_game_progress(&self, user_id: i32, game_type_code: &str) -> Result<(), DbErr> {
        let game_type = self
            .repository
            .get_game_type_by_code(game_type_code)
            .await?;

        let game_type = match game_type {
            Some(gt) => gt,
            None => {
                return Err(DbErr::Custom(format!(
                    "Game type not found: {}",
                    game_type_code
                )))
            }
        };

        self.repository
            .reset_game_progress(user_id, game_type.id)
            .await
    }
}
