use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use std::sync::Arc;

use rex_game_entities::entities::{
    achievement, game_session, game_type, user, user_achievement, user_game_progress, user_stats,
};

use crate::scoring::domain::{
    models::{
        AchievementModel, GameProgressModel, GameSessionModel, GameTypeModel, LeaderboardEntry,
        UserAchievementModel, UserStatsModel,
    },
    repositories::ScoringRepositoryTrait,
};

pub struct ScoringRepository {
    db: Arc<DatabaseConnection>,
}

impl ScoringRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ScoringRepositoryTrait for ScoringRepository {
    async fn get_game_types(&self) -> Result<Vec<GameTypeModel>, sea_orm::DbErr> {
        let game_types = game_type::Entity::find()
            .filter(game_type::Column::IsActived.eq(true))
            .all(self.db.as_ref())
            .await?;

        Ok(game_types
            .into_iter()
            .map(|gt| GameTypeModel {
                id: gt.id,
                code: gt.code,
                name: gt.name,
                description: gt.description,
                icon: gt.icon,
                is_actived: gt.is_actived,
                created_on: gt.created_on.with_timezone(&Utc),
                updated_on: gt.updated_on.with_timezone(&Utc),
            })
            .collect())
    }

    async fn get_game_type_by_code(
        &self,
        code: &str,
    ) -> Result<Option<GameTypeModel>, sea_orm::DbErr> {
        let game_type = game_type::Entity::find()
            .filter(game_type::Column::Code.eq(code))
            .one(self.db.as_ref())
            .await?;

        Ok(game_type.map(|gt| GameTypeModel {
            id: gt.id,
            code: gt.code,
            name: gt.name,
            description: gt.description,
            icon: gt.icon,
            is_actived: gt.is_actived,
            created_on: gt.created_on.with_timezone(&Utc),
            updated_on: gt.updated_on.with_timezone(&Utc),
        }))
    }

    async fn create_game_session(
        &self,
        user_id: i32,
        game_type_id: i32,
        flashcard_type_id: Option<i32>,
    ) -> Result<i32, sea_orm::DbErr> {
        let now = Utc::now().fixed_offset();

        let session = game_session::ActiveModel {
            user_id: Set(user_id),
            game_type_id: Set(game_type_id),
            flashcard_type_id: Set(flashcard_type_id),
            score: Set(0),
            max_score: Set(None),
            accuracy: Set(None),
            time_spent_seconds: Set(0),
            cards_played: Set(0),
            correct_answers: Set(0),
            wrong_answers: Set(0),
            combo_max: Set(0),
            started_at: Set(now),
            completed_at: Set(None),
            created_on: Set(now),
            ..Default::default()
        };

        let result = session.insert(self.db.as_ref()).await?;
        Ok(result.id)
    }

    async fn complete_game_session(
        &self,
        session_id: i32,
        score: i32,
        max_score: Option<i32>,
        correct_answers: i32,
        wrong_answers: i32,
        combo_max: i32,
        time_spent_seconds: i32,
    ) -> Result<(), sea_orm::DbErr> {
        let now = Utc::now().fixed_offset();
        let total_answers = correct_answers + wrong_answers;
        let accuracy = if total_answers > 0 {
            Some(Decimal::from(correct_answers * 100) / Decimal::from(total_answers))
        } else {
            None
        };

        let session = game_session::Entity::find_by_id(session_id)
            .one(self.db.as_ref())
            .await?;

        if let Some(session) = session {
            let mut active: game_session::ActiveModel = session.into();
            active.score = Set(score);
            active.max_score = Set(max_score);
            active.accuracy = Set(accuracy);
            active.correct_answers = Set(correct_answers);
            active.wrong_answers = Set(wrong_answers);
            active.cards_played = Set(correct_answers + wrong_answers);
            active.combo_max = Set(combo_max);
            active.time_spent_seconds = Set(time_spent_seconds);
            active.completed_at = Set(Some(now));
            active.update(self.db.as_ref()).await?;
        }

        Ok(())
    }

    async fn get_user_game_sessions(
        &self,
        user_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<GameSessionModel>, sea_orm::DbErr> {
        let offset = (page - 1) * page_size;

        let sessions = game_session::Entity::find()
            .filter(game_session::Column::UserId.eq(user_id))
            .filter(game_session::Column::CompletedAt.is_not_null())
            .order_by_desc(game_session::Column::CreatedOn)
            .offset(offset)
            .limit(page_size)
            .find_also_related(game_type::Entity)
            .all(self.db.as_ref())
            .await?;

        Ok(sessions
            .into_iter()
            .map(|(session, game_type)| GameSessionModel {
                id: session.id,
                user_id: session.user_id,
                user_name: None,
                user_display_name: None,
                game_type_id: session.game_type_id,
                game_type_code: game_type.as_ref().map(|gt| gt.code.clone()),
                game_type_name: game_type.map(|gt| gt.name),
                flashcard_type_id: session.flashcard_type_id,
                score: session.score,
                max_score: session.max_score,
                accuracy: session.accuracy,
                time_spent_seconds: session.time_spent_seconds,
                cards_played: session.cards_played,
                correct_answers: session.correct_answers,
                wrong_answers: session.wrong_answers,
                combo_max: session.combo_max,
                started_at: session.started_at.with_timezone(&Utc),
                completed_at: session.completed_at.map(|dt| dt.with_timezone(&Utc)),
                created_on: session.created_on.with_timezone(&Utc),
            })
            .collect())
    }

    async fn get_user_best_sessions(
        &self,
        user_id: i32,
        game_type_id: Option<i32>,
        limit: u64,
    ) -> Result<Vec<GameSessionModel>, sea_orm::DbErr> {
        let mut query = game_session::Entity::find()
            .filter(game_session::Column::UserId.eq(user_id))
            .filter(game_session::Column::CompletedAt.is_not_null());

        if let Some(gt_id) = game_type_id {
            query = query.filter(game_session::Column::GameTypeId.eq(gt_id));
        }

        let sessions = query
            .order_by_desc(game_session::Column::Score)
            .limit(limit)
            .find_also_related(game_type::Entity)
            .all(self.db.as_ref())
            .await?;

        Ok(sessions
            .into_iter()
            .map(|(session, game_type)| GameSessionModel {
                id: session.id,
                user_id: session.user_id,
                user_name: None,
                user_display_name: None,
                game_type_id: session.game_type_id,
                game_type_code: game_type.as_ref().map(|gt| gt.code.clone()),
                game_type_name: game_type.map(|gt| gt.name),
                flashcard_type_id: session.flashcard_type_id,
                score: session.score,
                max_score: session.max_score,
                accuracy: session.accuracy,
                time_spent_seconds: session.time_spent_seconds,
                cards_played: session.cards_played,
                correct_answers: session.correct_answers,
                wrong_answers: session.wrong_answers,
                combo_max: session.combo_max,
                started_at: session.started_at.with_timezone(&Utc),
                completed_at: session.completed_at.map(|dt| dt.with_timezone(&Utc)),
                created_on: session.created_on.with_timezone(&Utc),
            })
            .collect())
    }

    async fn get_user_stats(&self, user_id: i32) -> Result<Option<UserStatsModel>, sea_orm::DbErr> {
        let stats = user_stats::Entity::find()
            .filter(user_stats::Column::UserId.eq(user_id))
            .find_also_related(user::Entity)
            .one(self.db.as_ref())
            .await?;

        Ok(stats.map(|(stats, user)| UserStatsModel {
            id: stats.id,
            user_id: stats.user_id,
            user_name: user.as_ref().map(|u| u.name.clone()),
            user_display_name: user.and_then(|u| u.display_name),
            total_score: stats.total_score,
            total_games_played: stats.total_games_played,
            total_time_played_seconds: stats.total_time_played_seconds,
            best_score: stats.best_score,
            best_combo: stats.best_combo,
            average_accuracy: stats.average_accuracy,
            current_streak: stats.current_streak,
            best_streak: stats.best_streak,
            last_played_at: stats.last_played_at.map(|dt| dt.with_timezone(&Utc)),
        }))
    }

    async fn ensure_user_stats_exists(&self, user_id: i32) -> Result<(), sea_orm::DbErr> {
        let existing = user_stats::Entity::find()
            .filter(user_stats::Column::UserId.eq(user_id))
            .one(self.db.as_ref())
            .await?;

        if existing.is_none() {
            let now = Utc::now().fixed_offset();
            let stats = user_stats::ActiveModel {
                user_id: Set(user_id),
                total_score: Set(0),
                total_games_played: Set(0),
                total_time_played_seconds: Set(0),
                best_score: Set(0),
                best_combo: Set(0),
                average_accuracy: Set(Decimal::ZERO),
                current_streak: Set(0),
                best_streak: Set(0),
                last_played_at: Set(None),
                updated_on: Set(now),
                ..Default::default()
            };
            stats.insert(self.db.as_ref()).await?;
        }

        Ok(())
    }

    async fn update_user_stats(
        &self,
        user_id: i32,
        session: &GameSessionModel,
    ) -> Result<(), sea_orm::DbErr> {
        self.ensure_user_stats_exists(user_id).await?;

        let stats = user_stats::Entity::find()
            .filter(user_stats::Column::UserId.eq(user_id))
            .one(self.db.as_ref())
            .await?;

        if let Some(stats) = stats {
            let now = Utc::now().fixed_offset();
            let new_total_games = stats.total_games_played + 1;
            let new_total_score = stats.total_score + session.score as i64;

            // Calculate new average accuracy
            let session_accuracy = session.accuracy.unwrap_or(Decimal::ZERO);
            let new_avg_accuracy = if stats.total_games_played == 0 {
                session_accuracy
            } else {
                (stats.average_accuracy * Decimal::from(stats.total_games_played)
                    + session_accuracy)
                    / Decimal::from(new_total_games)
            };

            // Check streak
            let (new_current_streak, new_best_streak) = {
                let today = Utc::now().date_naive();
                let last_played = stats
                    .last_played_at
                    .map(|dt| dt.with_timezone(&Utc).date_naive());

                let streak = if let Some(last) = last_played {
                    let days_diff = (today - last).num_days();
                    if days_diff == 1 {
                        stats.current_streak + 1
                    } else if days_diff == 0 {
                        stats.current_streak
                    } else {
                        1
                    }
                } else {
                    1
                };

                let best = std::cmp::max(streak, stats.best_streak);
                (streak, best)
            };

            let mut active: user_stats::ActiveModel = stats.into();
            active.total_score = Set(new_total_score);
            active.total_games_played = Set(new_total_games);
            active.total_time_played_seconds =
                Set(active.total_time_played_seconds.unwrap() + session.time_spent_seconds as i64);
            active.best_score = Set(std::cmp::max(active.best_score.unwrap(), session.score));
            active.best_combo = Set(std::cmp::max(active.best_combo.unwrap(), session.combo_max));
            active.average_accuracy = Set(new_avg_accuracy);
            active.current_streak = Set(new_current_streak);
            active.best_streak = Set(new_best_streak);
            active.last_played_at = Set(Some(now));
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?;
        }

        Ok(())
    }

    async fn get_leaderboard(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<LeaderboardEntry>, sea_orm::DbErr> {
        let offset = (page - 1) * page_size;

        let stats = user_stats::Entity::find()
            .filter(user_stats::Column::TotalGamesPlayed.gt(0))
            .order_by_desc(user_stats::Column::TotalScore)
            .offset(offset)
            .limit(page_size)
            .find_also_related(user::Entity)
            .all(self.db.as_ref())
            .await?;

        Ok(stats
            .into_iter()
            .enumerate()
            .map(|(idx, (stats, user))| LeaderboardEntry {
                rank: (offset + idx as u64 + 1) as i32,
                user_id: stats.user_id,
                user_name: user.as_ref().map(|u| u.name.clone()).unwrap_or_default(),
                user_display_name: user.and_then(|u| u.display_name),
                total_score: stats.total_score,
                total_games_played: stats.total_games_played,
                best_score: stats.best_score,
                average_accuracy: stats.average_accuracy,
            })
            .collect())
    }

    async fn get_user_rank(&self, user_id: i32) -> Result<Option<i32>, sea_orm::DbErr> {
        let user_stats = user_stats::Entity::find()
            .filter(user_stats::Column::UserId.eq(user_id))
            .one(self.db.as_ref())
            .await?;

        if let Some(stats) = user_stats {
            let rank = user_stats::Entity::find()
                .filter(user_stats::Column::TotalScore.gt(stats.total_score))
                .count(self.db.as_ref())
                .await?;

            Ok(Some(rank as i32 + 1))
        } else {
            Ok(None)
        }
    }

    async fn get_achievements(&self) -> Result<Vec<AchievementModel>, sea_orm::DbErr> {
        let achievements = achievement::Entity::find()
            .filter(achievement::Column::IsActived.eq(true))
            .order_by_asc(achievement::Column::Category)
            .order_by_asc(achievement::Column::Points)
            .all(self.db.as_ref())
            .await?;

        Ok(achievements
            .into_iter()
            .map(|a| AchievementModel {
                id: a.id,
                code: a.code,
                name: a.name,
                description: a.description,
                icon: a.icon,
                points: a.points,
                category: a.category,
                is_actived: a.is_actived,
                created_on: a.created_on.with_timezone(&Utc),
                updated_on: a.updated_on.with_timezone(&Utc),
            })
            .collect())
    }

    async fn get_user_achievements(
        &self,
        user_id: i32,
    ) -> Result<Vec<UserAchievementModel>, sea_orm::DbErr> {
        let user_achievements = user_achievement::Entity::find()
            .filter(user_achievement::Column::UserId.eq(user_id))
            .find_also_related(achievement::Entity)
            .all(self.db.as_ref())
            .await?;

        Ok(user_achievements
            .into_iter()
            .filter_map(|(ua, achievement)| {
                achievement.map(|a| UserAchievementModel {
                    id: ua.id,
                    user_id: ua.user_id,
                    achievement: AchievementModel {
                        id: a.id,
                        code: a.code,
                        name: a.name,
                        description: a.description,
                        icon: a.icon,
                        points: a.points,
                        category: a.category,
                        is_actived: a.is_actived,
                        created_on: a.created_on.with_timezone(&Utc),
                        updated_on: a.updated_on.with_timezone(&Utc),
                    },
                    unlocked_at: ua.unlocked_at.with_timezone(&Utc),
                })
            })
            .collect())
    }

    async fn unlock_achievement(
        &self,
        user_id: i32,
        achievement_id: i32,
    ) -> Result<(), sea_orm::DbErr> {
        let now = Utc::now().fixed_offset();

        let existing = user_achievement::Entity::find()
            .filter(user_achievement::Column::UserId.eq(user_id))
            .filter(user_achievement::Column::AchievementId.eq(achievement_id))
            .one(self.db.as_ref())
            .await?;

        if existing.is_none() {
            let ua = user_achievement::ActiveModel {
                user_id: Set(user_id),
                achievement_id: Set(achievement_id),
                unlocked_at: Set(now),
                ..Default::default()
            };
            ua.insert(self.db.as_ref()).await?;
        }

        Ok(())
    }

    async fn check_achievement_unlocked(
        &self,
        user_id: i32,
        achievement_code: &str,
    ) -> Result<bool, sea_orm::DbErr> {
        let achievement = achievement::Entity::find()
            .filter(achievement::Column::Code.eq(achievement_code))
            .one(self.db.as_ref())
            .await?;

        if let Some(achievement) = achievement {
            let existing = user_achievement::Entity::find()
                .filter(user_achievement::Column::UserId.eq(user_id))
                .filter(user_achievement::Column::AchievementId.eq(achievement.id))
                .one(self.db.as_ref())
                .await?;

            Ok(existing.is_some())
        } else {
            Ok(false)
        }
    }

    // Game Progress
    async fn get_game_progress(
        &self,
        user_id: i32,
        game_type_id: i32,
    ) -> Result<Option<GameProgressModel>, sea_orm::DbErr> {
        let progress = user_game_progress::Entity::find()
            .filter(user_game_progress::Column::UserId.eq(user_id))
            .filter(user_game_progress::Column::GameTypeId.eq(game_type_id))
            .one(self.db.as_ref())
            .await?;

        if let Some(p) = progress {
            // Get game type info
            let game_type = game_type::Entity::find_by_id(p.game_type_id)
                .one(self.db.as_ref())
                .await?;

            Ok(Some(GameProgressModel {
                id: p.id,
                user_id: p.user_id,
                game_type_id: p.game_type_id,
                game_type_code: game_type.as_ref().map(|gt| gt.code.clone()),
                game_type_name: game_type.as_ref().map(|gt| gt.name.clone()),
                current_level: p.current_level,
                highest_level: p.highest_level,
                total_score: p.total_score,
                last_played_at: p.last_played_at.with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    async fn save_game_progress(
        &self,
        user_id: i32,
        game_type_id: i32,
        current_level: i32,
        total_score: i64,
    ) -> Result<GameProgressModel, sea_orm::DbErr> {
        let now = Utc::now().fixed_offset();

        // Check if progress exists
        let existing = user_game_progress::Entity::find()
            .filter(user_game_progress::Column::UserId.eq(user_id))
            .filter(user_game_progress::Column::GameTypeId.eq(game_type_id))
            .one(self.db.as_ref())
            .await?;

        let progress = if let Some(p) = existing {
            // Update existing progress
            let highest_level = if current_level > p.highest_level {
                current_level
            } else {
                p.highest_level
            };

            let mut active: user_game_progress::ActiveModel = p.into();
            active.current_level = Set(current_level);
            active.highest_level = Set(highest_level);
            active.total_score = Set(total_score);
            active.last_played_at = Set(now);
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?
        } else {
            // Create new progress
            let new_progress = user_game_progress::ActiveModel {
                user_id: Set(user_id),
                game_type_id: Set(game_type_id),
                current_level: Set(current_level),
                highest_level: Set(current_level),
                total_score: Set(total_score),
                last_played_at: Set(now),
                created_on: Set(now),
                updated_on: Set(now),
                ..Default::default()
            };
            new_progress.insert(self.db.as_ref()).await?
        };

        // Get game type info
        let game_type = game_type::Entity::find_by_id(progress.game_type_id)
            .one(self.db.as_ref())
            .await?;

        Ok(GameProgressModel {
            id: progress.id,
            user_id: progress.user_id,
            game_type_id: progress.game_type_id,
            game_type_code: game_type.as_ref().map(|gt| gt.code.clone()),
            game_type_name: game_type.as_ref().map(|gt| gt.name.clone()),
            current_level: progress.current_level,
            highest_level: progress.highest_level,
            total_score: progress.total_score,
            last_played_at: progress.last_played_at.with_timezone(&Utc),
        })
    }

    async fn reset_game_progress(
        &self,
        user_id: i32,
        game_type_id: i32,
    ) -> Result<(), sea_orm::DbErr> {
        let now = Utc::now().fixed_offset();

        let existing = user_game_progress::Entity::find()
            .filter(user_game_progress::Column::UserId.eq(user_id))
            .filter(user_game_progress::Column::GameTypeId.eq(game_type_id))
            .one(self.db.as_ref())
            .await?;

        if let Some(p) = existing {
            let mut active: user_game_progress::ActiveModel = p.into();
            active.current_level = Set(1);
            active.total_score = Set(0);
            active.last_played_at = Set(now);
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?;
        }

        Ok(())
    }

    // ---- Admin: Game Types ----

    async fn get_game_types_paged(
        &self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<GameTypeModel>, u64), sea_orm::DbErr> {
        let mut query = game_type::Entity::find();

        if let Some(n) = name {
            if !n.is_empty() {
                query = query.filter(Condition::any().add(game_type::Column::Name.contains(&n)));
            }
        }

        let total_count = query.clone().count(self.db.as_ref()).await?;

        let items = query
            .order_by_desc(game_type::Column::UpdatedOn)
            .paginate(self.db.as_ref(), page_size)
            .fetch_page(page - 1)
            .await?;

        let list = items
            .into_iter()
            .map(|gt| GameTypeModel {
                id: gt.id,
                code: gt.code,
                name: gt.name,
                description: gt.description,
                icon: gt.icon,
                is_actived: gt.is_actived,
                created_on: gt.created_on.with_timezone(&Utc),
                updated_on: gt.updated_on.with_timezone(&Utc),
            })
            .collect();

        Ok((list, total_count))
    }

    async fn get_game_type_by_id(&self, id: i32) -> Result<Option<GameTypeModel>, sea_orm::DbErr> {
        let item = game_type::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await?;

        Ok(item.map(|gt| GameTypeModel {
            id: gt.id,
            code: gt.code,
            name: gt.name,
            description: gt.description,
            icon: gt.icon,
            is_actived: gt.is_actived,
            created_on: gt.created_on.with_timezone(&Utc),
            updated_on: gt.updated_on.with_timezone(&Utc),
        }))
    }

    async fn create_game_type(&self, model: GameTypeModel) -> Result<i32, sea_orm::DbErr> {
        let now = Utc::now().fixed_offset();
        let active = game_type::ActiveModel {
            code: Set(model.code),
            name: Set(model.name),
            description: Set(model.description),
            icon: Set(model.icon),
            is_actived: Set(true),
            created_on: Set(now),
            updated_on: Set(now),
            ..Default::default()
        };
        let result = active.insert(self.db.as_ref()).await?;
        Ok(result.id)
    }

    async fn update_game_type(&self, model: GameTypeModel) -> Result<bool, sea_orm::DbErr> {
        let existing = game_type::Entity::find_by_id(model.id)
            .one(self.db.as_ref())
            .await?;

        if let Some(item) = existing {
            let now = Utc::now().fixed_offset();
            let mut active: game_type::ActiveModel = item.into();
            active.code = Set(model.code);
            active.name = Set(model.name);
            active.description = Set(model.description);
            active.icon = Set(model.icon);
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_game_type(&self, id: i32) -> Result<u64, sea_orm::DbErr> {
        let result = game_type::Entity::delete_by_id(id)
            .exec(self.db.as_ref())
            .await?;
        Ok(result.rows_affected)
    }

    async fn toggle_game_type_active(&self, id: i32) -> Result<bool, sea_orm::DbErr> {
        let existing = game_type::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await?;

        if let Some(item) = existing {
            let now = Utc::now().fixed_offset();
            let new_status = !item.is_actived;
            let mut active: game_type::ActiveModel = item.into();
            active.is_actived = Set(new_status);
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?;
            Ok(new_status)
        } else {
            Err(sea_orm::DbErr::RecordNotFound(
                "Game type not found".to_string(),
            ))
        }
    }

    // ---- Admin: Achievements ----

    async fn get_achievements_paged(
        &self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<AchievementModel>, u64), sea_orm::DbErr> {
        let mut query = achievement::Entity::find();

        if let Some(n) = name {
            if !n.is_empty() {
                query = query.filter(Condition::any().add(achievement::Column::Name.contains(&n)));
            }
        }

        let total_count = query.clone().count(self.db.as_ref()).await?;

        let items = query
            .order_by_desc(achievement::Column::UpdatedOn)
            .paginate(self.db.as_ref(), page_size)
            .fetch_page(page - 1)
            .await?;

        let list = items
            .into_iter()
            .map(|a| AchievementModel {
                id: a.id,
                code: a.code,
                name: a.name,
                description: a.description,
                icon: a.icon,
                points: a.points,
                category: a.category,
                is_actived: a.is_actived,
                created_on: a.created_on.with_timezone(&Utc),
                updated_on: a.updated_on.with_timezone(&Utc),
            })
            .collect();

        Ok((list, total_count))
    }

    async fn get_achievement_by_id(
        &self,
        id: i32,
    ) -> Result<Option<AchievementModel>, sea_orm::DbErr> {
        let item = achievement::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await?;

        Ok(item.map(|a| AchievementModel {
            id: a.id,
            code: a.code,
            name: a.name,
            description: a.description,
            icon: a.icon,
            points: a.points,
            category: a.category,
            is_actived: a.is_actived,
            created_on: a.created_on.with_timezone(&Utc),
            updated_on: a.updated_on.with_timezone(&Utc),
        }))
    }

    async fn create_achievement(&self, model: AchievementModel) -> Result<i32, sea_orm::DbErr> {
        let now = Utc::now().fixed_offset();
        let active = achievement::ActiveModel {
            code: Set(model.code),
            name: Set(model.name),
            description: Set(model.description),
            icon: Set(model.icon),
            points: Set(model.points),
            category: Set(model.category),
            is_actived: Set(true),
            created_on: Set(now),
            updated_on: Set(now),
            ..Default::default()
        };
        let result = active.insert(self.db.as_ref()).await?;
        Ok(result.id)
    }

    async fn update_achievement(&self, model: AchievementModel) -> Result<bool, sea_orm::DbErr> {
        let existing = achievement::Entity::find_by_id(model.id)
            .one(self.db.as_ref())
            .await?;

        if let Some(item) = existing {
            let now = Utc::now().fixed_offset();
            let mut active: achievement::ActiveModel = item.into();
            active.code = Set(model.code);
            active.name = Set(model.name);
            active.description = Set(model.description);
            active.icon = Set(model.icon);
            active.points = Set(model.points);
            active.category = Set(model.category);
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_achievement(&self, id: i32) -> Result<u64, sea_orm::DbErr> {
        let result = achievement::Entity::delete_by_id(id)
            .exec(self.db.as_ref())
            .await?;
        Ok(result.rows_affected)
    }

    async fn toggle_achievement_active(&self, id: i32) -> Result<bool, sea_orm::DbErr> {
        let existing = achievement::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await?;

        if let Some(item) = existing {
            let now = Utc::now().fixed_offset();
            let new_status = !item.is_actived;
            let mut active: achievement::ActiveModel = item.into();
            active.is_actived = Set(new_status);
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?;
            Ok(new_status)
        } else {
            Err(sea_orm::DbErr::RecordNotFound(
                "Achievement not found".to_string(),
            ))
        }
    }

    // ---- Admin: Game Sessions ----

    async fn get_all_game_sessions_paged(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<GameSessionModel>, u64), sea_orm::DbErr> {
        let total_count = game_session::Entity::find().count(self.db.as_ref()).await?;

        let offset = (page - 1) * page_size;
        let sessions = game_session::Entity::find()
            .order_by_desc(game_session::Column::CreatedOn)
            .offset(offset)
            .limit(page_size)
            .find_also_related(game_type::Entity)
            .all(self.db.as_ref())
            .await?;

        // Collect user IDs to fetch user info
        let user_ids: Vec<i32> = sessions.iter().map(|(s, _)| s.user_id).collect();
        let users = user::Entity::find()
            .filter(user::Column::Id.is_in(user_ids))
            .all(self.db.as_ref())
            .await?;
        let user_map: std::collections::HashMap<i32, user::Model> =
            users.into_iter().map(|u| (u.id, u)).collect();

        let list = sessions
            .into_iter()
            .map(|(session, game_type)| {
                let u = user_map.get(&session.user_id);
                GameSessionModel {
                    id: session.id,
                    user_id: session.user_id,
                    user_name: u.map(|u| u.name.clone()),
                    user_display_name: u.and_then(|u| u.display_name.clone()),
                    game_type_id: session.game_type_id,
                    game_type_code: game_type.as_ref().map(|gt| gt.code.clone()),
                    game_type_name: game_type.map(|gt| gt.name),
                    flashcard_type_id: session.flashcard_type_id,
                    score: session.score,
                    max_score: session.max_score,
                    accuracy: session.accuracy,
                    time_spent_seconds: session.time_spent_seconds,
                    cards_played: session.cards_played,
                    correct_answers: session.correct_answers,
                    wrong_answers: session.wrong_answers,
                    combo_max: session.combo_max,
                    started_at: session.started_at.with_timezone(&Utc),
                    completed_at: session.completed_at.map(|dt| dt.with_timezone(&Utc)),
                    created_on: session.created_on.with_timezone(&Utc),
                }
            })
            .collect();

        Ok((list, total_count))
    }

    async fn delete_game_session(&self, id: i32) -> Result<u64, sea_orm::DbErr> {
        let result = game_session::Entity::delete_by_id(id)
            .exec(self.db.as_ref())
            .await?;
        Ok(result.rows_affected)
    }

    // ---- Admin: User Stats ----

    async fn get_all_user_stats_paged(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<UserStatsModel>, u64), sea_orm::DbErr> {
        let total_count = user_stats::Entity::find().count(self.db.as_ref()).await?;

        let items = user_stats::Entity::find()
            .order_by_desc(user_stats::Column::TotalScore)
            .paginate(self.db.as_ref(), page_size)
            .fetch_page(page - 1)
            .await?;

        // Fetch related users
        let user_ids: Vec<i32> = items.iter().map(|s| s.user_id).collect();
        let users = user::Entity::find()
            .filter(user::Column::Id.is_in(user_ids))
            .all(self.db.as_ref())
            .await?;
        let user_map: std::collections::HashMap<i32, user::Model> =
            users.into_iter().map(|u| (u.id, u)).collect();

        let list = items
            .into_iter()
            .map(|stats| {
                let u = user_map.get(&stats.user_id);
                UserStatsModel {
                    id: stats.id,
                    user_id: stats.user_id,
                    user_name: u.map(|u| u.name.clone()),
                    user_display_name: u.and_then(|u| u.display_name.clone()),
                    total_score: stats.total_score,
                    total_games_played: stats.total_games_played,
                    total_time_played_seconds: stats.total_time_played_seconds,
                    best_score: stats.best_score,
                    best_combo: stats.best_combo,
                    average_accuracy: stats.average_accuracy,
                    current_streak: stats.current_streak,
                    best_streak: stats.best_streak,
                    last_played_at: stats.last_played_at.map(|dt| dt.with_timezone(&Utc)),
                }
            })
            .collect();

        Ok((list, total_count))
    }

    async fn reset_user_stats(&self, user_id: i32) -> Result<bool, sea_orm::DbErr> {
        let existing = user_stats::Entity::find()
            .filter(user_stats::Column::UserId.eq(user_id))
            .one(self.db.as_ref())
            .await?;

        if let Some(stats) = existing {
            let now = Utc::now().fixed_offset();
            let mut active: user_stats::ActiveModel = stats.into();
            active.total_score = Set(0);
            active.total_games_played = Set(0);
            active.total_time_played_seconds = Set(0);
            active.best_score = Set(0);
            active.best_combo = Set(0);
            active.average_accuracy = Set(Decimal::ZERO);
            active.current_streak = Set(0);
            active.best_streak = Set(0);
            active.last_played_at = Set(None);
            active.updated_on = Set(now);
            active.update(self.db.as_ref()).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
