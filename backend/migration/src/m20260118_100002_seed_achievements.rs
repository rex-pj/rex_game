use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::enums::user_achievement::Achievement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = chrono::Utc::now();

        let insert = Query::insert()
            .into_table(Achievement::Table)
            .columns([
                Achievement::Code,
                Achievement::Name,
                Achievement::Description,
                Achievement::Icon,
                Achievement::Points,
                Achievement::Category,
                Achievement::IsActived,
                Achievement::CreatedOn,
                Achievement::UpdatedOn,
            ])
            // First Steps
            .values_panic([
                "first_game".into(),
                "First Steps".into(),
                "Complete your first game".into(),
                "fa-solid fa-baby".into(),
                10.into(),
                "beginner".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            // Perfect Score
            .values_panic([
                "perfect_score".into(),
                "Perfect Score".into(),
                "Achieve 100% accuracy in a game".into(),
                "fa-solid fa-star".into(),
                50.into(),
                "accuracy".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            // Speed Demon
            .values_panic([
                "speed_demon".into(),
                "Speed Demon".into(),
                "Complete a game in under 30 seconds".into(),
                "fa-solid fa-bolt".into(),
                30.into(),
                "speed".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            // Combo Master
            .values_panic([
                "combo_master".into(),
                "Combo Master".into(),
                "Achieve a 10x combo".into(),
                "fa-solid fa-fire".into(),
                40.into(),
                "combo".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            // Streak Keeper
            .values_panic([
                "streak_7".into(),
                "Week Warrior".into(),
                "Play games for 7 consecutive days".into(),
                "fa-solid fa-calendar-check".into(),
                100.into(),
                "streak".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            // Game Collector
            .values_panic([
                "games_10".into(),
                "Getting Started".into(),
                "Play 10 games".into(),
                "fa-solid fa-gamepad".into(),
                20.into(),
                "games".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .values_panic([
                "games_50".into(),
                "Dedicated Player".into(),
                "Play 50 games".into(),
                "fa-solid fa-trophy".into(),
                50.into(),
                "games".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .values_panic([
                "games_100".into(),
                "Game Master".into(),
                "Play 100 games".into(),
                "fa-solid fa-crown".into(),
                100.into(),
                "games".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            // Score Milestones
            .values_panic([
                "score_1000".into(),
                "Rising Star".into(),
                "Reach 1,000 total points".into(),
                "fa-solid fa-medal".into(),
                25.into(),
                "score".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .values_panic([
                "score_10000".into(),
                "Champion".into(),
                "Reach 10,000 total points".into(),
                "fa-solid fa-award".into(),
                75.into(),
                "score".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Achievement::Table)
            .and_where(Expr::col(Achievement::Code).is_in([
                "first_game",
                "perfect_score",
                "speed_demon",
                "combo_master",
                "streak_7",
                "games_10",
                "games_50",
                "games_100",
                "score_1000",
                "score_10000",
            ]))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
