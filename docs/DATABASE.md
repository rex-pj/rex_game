# qHortus Database Schema

This document describes the database schema and entity relationships for qHortus.

---

## Table of Contents

- [Overview](#overview)
- [Database Technology](#database-technology)
- [Entity Relationship Diagram](#entity-relationship-diagram)
- [Tables](#tables)
  - [User Management](#user-management)
  - [Authorization](#authorization)
  - [Flashcards](#flashcards)
  - [Games & Scoring](#games--scoring)
  - [System](#system)
- [Indexes](#indexes)
- [Migrations](#migrations)

---

## 1. Overview

qHortus uses PostgreSQL as its primary database with SeaORM as the ORM layer. The schema follows Domain-Driven Design principles with clear separation between different bounded contexts.

### Database Statistics

- **Total Tables**: 20+
- **Relationships**: One-to-Many, Many-to-Many
- **Constraints**: Foreign Keys, Unique Constraints, Check Constraints
- **Indexes**: Primary Keys, Foreign Keys, Custom Indexes

---

## 2. Database Technology

| Component | Technology | Version |
|-----------|-----------|---------|
| **Database** | PostgreSQL | 15+ |
| **ORM** | SeaORM | 1.1 |
| **Migration Tool** | SeaORM Migration | 1.1 |
| **Connection Pool** | SQLx | - |

---

## 3. Entity Relationship Diagram

```
┌─────────────┐       ┌──────────────┐       ┌─────────────────┐
│    User     │──────<│  UserRole    │>──────│      Role       │
└─────────────┘       └──────────────┘       └─────────────────┘
      │                                              │
      │                                              │
      │ 1:N                                          │ 1:N
      │                                              │
      ├────────────┐                     ┌───────────┤
      │            │                     │           │
      ▼            ▼                     ▼           ▼
┌──────────┐  ┌─────────────┐   ┌──────────────┐  ┌────────────────┐
│UserToken │  │UserPermission│  │RolePermission │  │  Permission    │
└──────────┘  └─────────────┘   └──────────────┘  └────────────────┘


┌─────────────┐       ┌──────────────────┐       ┌────────────────┐
│    User     │──────<│  GameSession     │>──────│   GameType     │
└─────────────┘       └──────────────────┘       └────────────────┘
      │                       │
      │ 1:N                   │ N:1
      │                       │
      ▼                       ▼
┌──────────────┐       ┌────────────────┐
│  UserStats   │       │   Flashcard    │
└──────────────┘       └────────────────┘
      │                       │
      │ 1:N                   │ N:1
      │                       │
      ▼                       ▼
┌──────────────────┐   ┌─────────────────┐
│UserAchievement   │   │ FlashcardType   │
└──────────────────┘   └─────────────────┘
      │
      │ N:1
      │
      ▼
┌──────────────┐
│ Achievement  │
└──────────────┘


┌──────────────┐       ┌─────────────────────────┐
│    User      │──────<│  UserGameProgress       │
└──────────────┘       └─────────────────────────┘
                               │
                               │ N:1
                               │
                               ▼
                        ┌────────────────┐
                        │   GameType     │
                        └────────────────┘
```

---

## 4. Tables

### User Management

#### `users`

Stores user account information.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | User unique identifier |
| `email` | VARCHAR(255) | UNIQUE, NOT NULL | User email address |
| `username` | VARCHAR(100) | UNIQUE, NOT NULL | Username for display |
| `password_hash` | TEXT | NOT NULL | Bcrypt hashed password |
| `is_confirmed` | BOOLEAN | DEFAULT FALSE | Email confirmation status |
| `confirmation_token` | VARCHAR(255) | NULLABLE | Email confirmation token |
| `reset_token` | VARCHAR(255) | NULLABLE | Password reset token |
| `reset_token_expires_at` | TIMESTAMP | NULLABLE | Reset token expiration |
| `created_at` | TIMESTAMP | NOT NULL | Account creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |
| `deleted_at` | TIMESTAMP | NULLABLE | Soft delete timestamp |

**Indexes:**
- `idx_users_email` on `email`
- `idx_users_username` on `username`
- `idx_users_confirmation_token` on `confirmation_token`

---

#### `user_tokens`

Stores JWT refresh tokens for authentication.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Token unique identifier |
| `user_id` | UUID | FOREIGN KEY → users(id), NOT NULL | Owner user |
| `token` | TEXT | NOT NULL | Refresh token value |
| `expires_at` | TIMESTAMP | NOT NULL | Token expiration |
| `created_at` | TIMESTAMP | NOT NULL | Token creation time |

**Indexes:**
- `idx_user_tokens_user_id` on `user_id`
- `idx_user_tokens_token` on `token`

---

### Authorization

#### `roles`

Defines user roles in the system.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Role unique identifier |
| `name` | VARCHAR(100) | UNIQUE, NOT NULL | Role name (e.g., ROLE_ROOT_ADMIN) |
| `description` | TEXT | NULLABLE | Role description |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |

**Default Roles:**
- `ROLE_ROOT_ADMIN` - System administrator
- `ROLE_USER` - Regular user

---

#### `permissions`

Defines granular permissions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Permission unique identifier |
| `code` | VARCHAR(100) | UNIQUE, NOT NULL | Permission code (e.g., flashcard:create) |
| `description` | TEXT | NULLABLE | Permission description |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |

**Permission Format:** `<resource>:<action>`

Examples:
- `flashcard:create`
- `flashcard:read`
- `flashcard:update`
- `flashcard:delete`
- `user:read`
- `role:update`

---

#### `user_roles`

Many-to-many relationship between users and roles.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Relation unique identifier |
| `user_id` | UUID | FOREIGN KEY → users(id), NOT NULL | User reference |
| `role_id` | UUID | FOREIGN KEY → roles(id), NOT NULL | Role reference |
| `created_at` | TIMESTAMP | NOT NULL | Assignment time |

**Unique Constraint:** `(user_id, role_id)`

---

#### `role_permissions`

Many-to-many relationship between roles and permissions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Relation unique identifier |
| `role_id` | UUID | FOREIGN KEY → roles(id), NOT NULL | Role reference |
| `permission_id` | UUID | FOREIGN KEY → permissions(id), NOT NULL | Permission reference |
| `created_at` | TIMESTAMP | NOT NULL | Assignment time |

**Unique Constraint:** `(role_id, permission_id)`

---

#### `user_permissions`

Direct permission assignments to users (override role permissions).

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Relation unique identifier |
| `user_id` | UUID | FOREIGN KEY → users(id), NOT NULL | User reference |
| `permission_id` | UUID | FOREIGN KEY → permissions(id), NOT NULL | Permission reference |
| `created_at` | TIMESTAMP | NOT NULL | Assignment time |

**Unique Constraint:** `(user_id, permission_id)`

---

### Flashcards

#### `flashcard_types`

Categories for organizing flashcards.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Type unique identifier |
| `name` | VARCHAR(100) | UNIQUE, NOT NULL | Type name (e.g., Math, Science) |
| `description` | TEXT | NULLABLE | Type description |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |
| `deleted_at` | TIMESTAMP | NULLABLE | Soft delete timestamp |

---

#### `flashcards`

Individual flashcard questions and answers.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Flashcard unique identifier |
| `question` | TEXT | NOT NULL | Question text (supports Markdown) |
| `answer` | TEXT | NOT NULL | Answer text (supports Markdown) |
| `flashcard_type_id` | UUID | FOREIGN KEY → flashcard_types(id), NOT NULL | Category |
| `difficulty` | INTEGER | DEFAULT 1 | Difficulty level (1-5) |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |
| `deleted_at` | TIMESTAMP | NULLABLE | Soft delete timestamp |

**Indexes:**
- `idx_flashcards_type_id` on `flashcard_type_id`

---

#### `flashcard_files`

Stores flashcard images and attachments.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | File unique identifier |
| `flashcard_id` | UUID | FOREIGN KEY → flashcards(id), NOT NULL | Parent flashcard |
| `file_path` | VARCHAR(255) | NOT NULL | File path on disk |
| `file_name` | VARCHAR(255) | NOT NULL | Original file name |
| `mime_type` | VARCHAR(100) | NOT NULL | File MIME type |
| `file_size` | INTEGER | NOT NULL | File size in bytes |
| `created_at` | TIMESTAMP | NOT NULL | Upload time |

**Indexes:**
- `idx_flashcard_files_flashcard_id` on `flashcard_id`

---

#### `flashcard_type_relations`

Hierarchical relationships between flashcard types (parent-child).

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Relation unique identifier |
| `parent_type_id` | UUID | FOREIGN KEY → flashcard_types(id), NOT NULL | Parent type |
| `child_type_id` | UUID | FOREIGN KEY → flashcard_types(id), NOT NULL | Child type |
| `created_at` | TIMESTAMP | NOT NULL | Relation creation time |

---

### Games & Scoring

#### `game_types`

Available game modes.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | VARCHAR(50) | PRIMARY KEY | Game type identifier |
| `name` | VARCHAR(100) | NOT NULL | Display name |
| `description` | TEXT | NULLABLE | Game description |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |

**Default Game Types:**
- `multiple_choice` - Multiple Choice Quiz
- `matching` - Flashcard Matching
- `fill_blank` - Fill in the Blanks

---

#### `game_sessions`

Records of completed games.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Session unique identifier |
| `user_id` | UUID | FOREIGN KEY → users(id), NOT NULL | Player |
| `game_type_id` | VARCHAR(50) | FOREIGN KEY → game_types(id), NOT NULL | Game mode |
| `flashcard_type_id` | UUID | FOREIGN KEY → flashcard_types(id), NULLABLE | Flashcard category |
| `score` | INTEGER | NOT NULL | Final score (0-100) |
| `correct_answers` | INTEGER | NOT NULL | Number of correct answers |
| `total_questions` | INTEGER | NOT NULL | Total questions asked |
| `time_taken` | INTEGER | NOT NULL | Time in seconds |
| `xp_earned` | INTEGER | DEFAULT 0 | Experience points earned |
| `started_at` | TIMESTAMP | NOT NULL | Session start time |
| `completed_at` | TIMESTAMP | NOT NULL | Session completion time |
| `created_at` | TIMESTAMP | NOT NULL | Record creation time |

**Indexes:**
- `idx_game_sessions_user_id` on `user_id`
- `idx_game_sessions_game_type_id` on `game_type_id`
- `idx_game_sessions_score` on `score` (for leaderboard queries)
- `idx_game_sessions_completed_at` on `completed_at`

---

#### `user_game_progress`

Saves in-progress games (pause/resume functionality).

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Progress unique identifier |
| `user_id` | UUID | FOREIGN KEY → users(id), NOT NULL | Player |
| `game_type_id` | VARCHAR(50) | FOREIGN KEY → game_types(id), NOT NULL | Game mode |
| `flashcard_type_id` | UUID | FOREIGN KEY → flashcard_types(id), NULLABLE | Flashcard category |
| `progress_data` | JSONB | NOT NULL | Serialized game state |
| `current_question` | INTEGER | NOT NULL | Current question index |
| `score` | INTEGER | DEFAULT 0 | Current score |
| `started_at` | TIMESTAMP | NOT NULL | Game start time |
| `saved_at` | TIMESTAMP | NOT NULL | Last save time |

**Unique Constraint:** `(user_id, game_type_id, flashcard_type_id)`

**Indexes:**
- `idx_user_game_progress_user_id` on `user_id`

---

#### `user_stats`

Aggregate statistics for each user.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Stats unique identifier |
| `user_id` | UUID | FOREIGN KEY → users(id), UNIQUE, NOT NULL | User reference |
| `total_games` | INTEGER | DEFAULT 0 | Total games played |
| `total_score` | INTEGER | DEFAULT 0 | Sum of all scores |
| `average_score` | DECIMAL(5,2) | DEFAULT 0 | Average score |
| `highest_score` | INTEGER | DEFAULT 0 | Best score achieved |
| `total_time_played` | INTEGER | DEFAULT 0 | Total seconds played |
| `level` | INTEGER | DEFAULT 1 | User level |
| `xp` | INTEGER | DEFAULT 0 | Experience points |
| `current_streak` | INTEGER | DEFAULT 0 | Current daily streak |
| `longest_streak` | INTEGER | DEFAULT 0 | Longest daily streak |
| `last_played_at` | TIMESTAMP | NULLABLE | Last game time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |

**Indexes:**
- `idx_user_stats_level` on `level`
- `idx_user_stats_xp` on `xp`

---

#### `achievements`

Available achievements in the game.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Achievement unique identifier |
| `code` | VARCHAR(100) | UNIQUE, NOT NULL | Achievement code |
| `name` | VARCHAR(100) | NOT NULL | Display name |
| `description` | TEXT | NOT NULL | Achievement description |
| `icon` | VARCHAR(50) | NULLABLE | Icon/emoji |
| `requirement_type` | VARCHAR(50) | NOT NULL | Type of requirement |
| `requirement_value` | INTEGER | NOT NULL | Threshold value |
| `xp_reward` | INTEGER | DEFAULT 0 | XP awarded |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |

**Requirement Types:**
- `games_played` - Play N games
- `score_threshold` - Achieve score ≥ N
- `streak` - Maintain N day streak
- `time_based` - Complete game in ≤ N seconds
- `perfect_score` - Get 100% score

---

#### `user_achievements`

Tracks unlocked achievements for users.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Record unique identifier |
| `user_id` | UUID | FOREIGN KEY → users(id), NOT NULL | User reference |
| `achievement_id` | UUID | FOREIGN KEY → achievements(id), NOT NULL | Achievement reference |
| `progress` | INTEGER | DEFAULT 0 | Progress percentage (0-100) |
| `unlocked_at` | TIMESTAMP | NULLABLE | Unlock time |
| `created_at` | TIMESTAMP | NOT NULL | Record creation time |

**Unique Constraint:** `(user_id, achievement_id)`

**Indexes:**
- `idx_user_achievements_user_id` on `user_id`

---

### System

#### `mail_templates`

Email templates for system notifications.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Template unique identifier |
| `code` | VARCHAR(100) | UNIQUE, NOT NULL | Template code |
| `subject` | VARCHAR(255) | NOT NULL | Email subject |
| `body` | TEXT | NOT NULL | Email body (supports variables) |
| `variables` | TEXT[] | NULLABLE | Available variables |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |

**Template Variables:**
- `{username}` - User's username
- `{email}` - User's email
- `{token}` - Verification/reset token
- `{url}` - Action URL
- `{platform_name}` - Application name

---

#### `system_settings`

Application-wide configuration.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Setting unique identifier |
| `key` | VARCHAR(100) | UNIQUE, NOT NULL | Setting key |
| `value` | TEXT | NOT NULL | Setting value |
| `description` | TEXT | NULLABLE | Setting description |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |

---

## 5. Indexes

### Performance Indexes

```sql
-- User lookups
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- Game sessions (leaderboard queries)
CREATE INDEX idx_game_sessions_score ON game_sessions(score DESC);
CREATE INDEX idx_game_sessions_completed_at ON game_sessions(completed_at DESC);

-- Foreign key indexes
CREATE INDEX idx_game_sessions_user_id ON game_sessions(user_id);
CREATE INDEX idx_user_achievements_user_id ON user_achievements(user_id);
CREATE INDEX idx_flashcards_type_id ON flashcards(flashcard_type_id);
```

---

## 6. Migrations

### Running Migrations

```bash
cd backend

# Run all pending migrations
cargo run --bin migration up

# Rollback last migration
cargo run --bin migration down

# Check migration status
cargo run --bin migration status

# Refresh database (down + up)
cargo run --bin migration fresh
```

### Creating a New Migration

```bash
cd backend/migration

# Create migration file
sea-orm-cli migrate generate add_new_table
```

### Migration File Structure

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create table / add column / etc.
        manager
            .create_table(/* ... */)
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Rollback changes
        manager
            .drop_table(/* ... */)
            .await
    }
}
```

---

## 7. Database Relationships Summary

### One-to-Many
- User → UserTokens
- User → GameSessions
- User → UserGameProgress
- FlashcardType → Flashcards
- Flashcard → FlashcardFiles

### Many-to-Many (via join tables)
- User ↔ Role (via user_roles)
- User ↔ Permission (via user_permissions)
- Role ↔ Permission (via role_permissions)

### One-to-One
- User → UserStats

---

## 8. Data Integrity

### Foreign Key Constraints

All foreign keys use `ON DELETE CASCADE` or `ON DELETE RESTRICT` to maintain referential integrity.

### Soft Deletes

The following tables support soft deletes (have `deleted_at` column):
- `users`
- `flashcards`
- `flashcard_types`

Soft deleted records are excluded from queries by default.

---

## 9. Performance Considerations

1. **Indexes**: All foreign keys have indexes for faster joins
2. **JSONB**: `user_game_progress.progress_data` uses JSONB for efficient querying
3. **Partitioning**: Consider partitioning `game_sessions` by date for large datasets
4. **Connection Pooling**: Configured via SeaORM connection pool settings
5. **Query Optimization**: Use `EXPLAIN ANALYZE` to optimize slow queries

---

## 10. Seed Data

Initial seed data includes:
- Default roles (ROLE_ROOT_ADMIN, ROLE_USER)
- Default permissions
- System mail templates
- Default game types
- Sample achievements

See `backend/migration/src/seed.rs` for details.

---

## Additional Resources

- [API Documentation](API.md)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)

---

*Last updated: January 2026*
