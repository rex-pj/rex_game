# Rex Game API Documentation

This document provides comprehensive documentation for the Rex Game REST API.

---

## Table of Contents

- [Base URL](#base-url)
- [Authentication](#authentication)
- [Rate Limiting](#rate-limiting)
- [Error Handling](#error-handling)
- [API Endpoints](#api-endpoints)
  - [Authentication](#authentication-endpoints)
  - [Users](#user-endpoints)
  - [Flashcards](#flashcard-endpoints)
  - [Games & Scoring](#games--scoring-endpoints)
  - [Admin](#admin-endpoints)

---

## 1. Base URL

```
Development: http://localhost:8080
Production:  https://your-domain.com/api
```

All API endpoints are prefixed with `/api` in production (handled by Nginx reverse proxy).

---

## 2. Authentication

### JWT Bearer Token

Most endpoints require authentication using JWT tokens. Include the access token in the `Authorization` header:

```http
Authorization: Bearer <your_access_token>
```

### Token Lifecycle

| Token Type | Purpose | Expiration |
|------------|---------|------------|
| **Access Token** | API authentication | 1 hour (configurable) |
| **Refresh Token** | Renew access token | 7 days (configurable) |

### Obtaining Tokens

1. **Login** at `/auth/login` to receive both tokens
2. **Refresh** access token at `/auth/refresh` using refresh token
3. **Logout** at `/auth/logout` to invalidate tokens

---

## 3. Rate Limiting

Rate limits are enforced to prevent abuse:

| Route Category | Limit | Window |
|----------------|-------|--------|
| **Authentication** (`/auth/login`, `/users`) | 5 requests | 1 second |
| **Password Recovery** | 3 requests | 1 minute |
| **General API** | 30 requests | 1 second |

When rate limited, you'll receive a `429 Too Many Requests` response.

---

## 4. Error Handling

### Error Response Format

```json
{
  "error": "Error message describing what went wrong"
}
```

### HTTP Status Codes

| Code | Meaning | Description |
|------|---------|-------------|
| `200` | OK | Request successful |
| `201` | Created | Resource created successfully |
| `400` | Bad Request | Invalid request data |
| `401` | Unauthorized | Missing or invalid authentication |
| `403` | Forbidden | Insufficient permissions |
| `404` | Not Found | Resource doesn't exist |
| `409` | Conflict | Resource already exists |
| `422` | Unprocessable Entity | Validation error |
| `429` | Too Many Requests | Rate limit exceeded |
| `500` | Internal Server Error | Server error |

---

## 5. API Endpoints

### Authentication Endpoints

#### POST `/auth/login`

Authenticate user and receive tokens.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "secure_password"
}
```

**Response (200 OK):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "is_confirmed": true
  }
}
```

**Rate Limit:** 5 req/sec

---

#### POST `/auth/refresh`

Refresh access token using refresh token.

**Authentication:** Required (Refresh Token in Authorization header)

**Response (200 OK):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

---

#### DELETE `/auth/logout`

Logout and invalidate tokens.

**Authentication:** Required

**Response (200 OK):**
```json
{
  "message": "Logged out successfully"
}
```

---

### User Endpoints

#### POST `/users`

Register a new user account.

**Request Body:**
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "secure_password",
  "password_confirmation": "secure_password"
}
```

**Response (201 Created):**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "username": "johndoe",
  "is_confirmed": false,
  "created_at": "2026-01-23T10:00:00Z"
}
```

**Note:** A confirmation email will be sent.

**Rate Limit:** 5 req/sec

---

#### POST `/users/confirmations`

Confirm email address with token from email.

**Request Body:**
```json
{
  "token": "confirmation_token_from_email"
}
```

**Response (200 OK):**
```json
{
  "message": "Email confirmed successfully"
}
```

---

#### POST `/users/password`

Request password reset (forgot password).

**Request Body:**
```json
{
  "email": "user@example.com"
}
```

**Response (200 OK):**
```json
{
  "message": "Password reset email sent"
}
```

**Rate Limit:** 3 req/min

---

#### PATCH `/users/password`

Reset password with token from email.

**Request Body:**
```json
{
  "token": "reset_token_from_email",
  "password": "new_secure_password",
  "password_confirmation": "new_secure_password"
}
```

**Response (200 OK):**
```json
{
  "message": "Password reset successfully"
}
```

**Rate Limit:** 3 req/min

---

#### GET `/users/me`

Get current user profile.

**Authentication:** Required

**Response (200 OK):**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "username": "johndoe",
  "is_confirmed": true,
  "created_at": "2026-01-23T10:00:00Z",
  "updated_at": "2026-01-23T10:00:00Z"
}
```

---

#### GET `/users`

Get list of users (with pagination).

**Authentication:** Required

**Query Parameters:**
- `page` (optional, default: 1) - Page number
- `per_page` (optional, default: 10) - Items per page

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "uuid",
      "email": "user@example.com",
      "username": "johndoe",
      "is_confirmed": true
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 10,
    "total": 50,
    "total_pages": 5
  }
}
```

---

#### GET `/users/{id}`

Get user by ID.

**Response (200 OK):**
```json
{
  "id": "uuid",
  "username": "johndoe",
  "created_at": "2026-01-23T10:00:00Z"
}
```

---

#### PATCH `/users/{id}`

Update user profile.

**Authentication:** Required

**Request Body:**
```json
{
  "username": "new_username",
  "email": "newemail@example.com"
}
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "email": "newemail@example.com",
  "username": "new_username",
  "updated_at": "2026-01-23T11:00:00Z"
}
```

---

### Flashcard Endpoints

#### GET `/flashcards`

Get all flashcards.

**Query Parameters:**
- `flashcard_type_id` (optional) - Filter by flashcard type
- `page` (optional, default: 1) - Page number
- `per_page` (optional, default: 20) - Items per page

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "uuid",
      "question": "What is 2 + 2?",
      "answer": "4",
      "flashcard_type_id": "uuid",
      "image_url": "/api/flashcards/images/uuid",
      "created_at": "2026-01-23T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100
  }
}
```

---

#### GET `/flashcards/{id}`

Get flashcard by ID.

**Response (200 OK):**
```json
{
  "id": "uuid",
  "question": "What is 2 + 2?",
  "answer": "4",
  "flashcard_type_id": "uuid",
  "flashcard_type_name": "Math",
  "image_url": "/api/flashcards/images/uuid",
  "created_at": "2026-01-23T10:00:00Z"
}
```

---

#### GET `/flashcards/images/{id}`

Get flashcard image.

**Response:** Image file (PNG, JPG, etc.)

---

#### GET `/flashcard-types`

Get all flashcard categories/types.

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "uuid",
      "name": "Math",
      "description": "Mathematics flashcards",
      "flashcard_count": 50
    },
    {
      "id": "uuid",
      "name": "Science",
      "description": "Science flashcards",
      "flashcard_count": 30
    }
  ]
}
```

---

#### GET `/flashcard-types/{id}`

Get flashcard type by ID.

**Response (200 OK):**
```json
{
  "id": "uuid",
  "name": "Math",
  "description": "Mathematics flashcards",
  "flashcard_count": 50,
  "created_at": "2026-01-23T10:00:00Z"
}
```

---

### Games & Scoring Endpoints

#### GET `/game-types`

Get available game types.

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "multiple_choice",
      "name": "Multiple Choice",
      "description": "Choose the correct answer"
    },
    {
      "id": "matching",
      "name": "Flashcard Matching",
      "description": "Match questions with answers"
    },
    {
      "id": "fill_blank",
      "name": "Fill in the Blanks",
      "description": "Type the correct answer"
    }
  ]
}
```

---

#### POST `/games/sessions`

Start a new game session.

**Authentication:** Required

**Request Body:**
```json
{
  "game_type": "multiple_choice",
  "flashcard_type_id": "uuid",
  "difficulty": "medium"
}
```

**Response (201 Created):**
```json
{
  "session_id": "uuid",
  "game_type": "multiple_choice",
  "started_at": "2026-01-23T10:00:00Z"
}
```

---

#### POST `/games/sessions/complete`

Complete a game session and record score.

**Authentication:** Required

**Request Body:**
```json
{
  "session_id": "uuid",
  "score": 85,
  "correct_answers": 17,
  "total_questions": 20,
  "time_taken": 120,
  "answers": [
    {
      "flashcard_id": "uuid",
      "user_answer": "4",
      "is_correct": true,
      "time_taken": 5
    }
  ]
}
```

**Response (200 OK):**
```json
{
  "session_id": "uuid",
  "score": 85,
  "rank": 12,
  "achievements_unlocked": [
    {
      "id": "uuid",
      "name": "Quick Learner",
      "description": "Complete a game in under 2 minutes"
    }
  ],
  "xp_earned": 100,
  "new_level": 5
}
```

---

#### GET `/games/history`

Get user's game history.

**Authentication:** Required

**Query Parameters:**
- `page` (optional, default: 1)
- `per_page` (optional, default: 10)
- `game_type` (optional) - Filter by game type

**Response (200 OK):**
```json
{
  "data": [
    {
      "session_id": "uuid",
      "game_type": "multiple_choice",
      "score": 85,
      "correct_answers": 17,
      "total_questions": 20,
      "time_taken": 120,
      "played_at": "2026-01-23T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 10,
    "total": 50
  }
}
```

---

#### GET `/games/best`

Get user's best game scores.

**Authentication:** Required

**Query Parameters:**
- `game_type` (optional) - Filter by game type
- `limit` (optional, default: 10) - Number of records

**Response (200 OK):**
```json
{
  "data": [
    {
      "session_id": "uuid",
      "game_type": "multiple_choice",
      "score": 95,
      "played_at": "2026-01-23T10:00:00Z"
    }
  ]
}
```

---

#### GET `/games/progress`

Get user's game progress (resume game).

**Authentication:** Required

**Query Parameters:**
- `game_type` (required)
- `flashcard_type_id` (optional)

**Response (200 OK):**
```json
{
  "session_id": "uuid",
  "game_type": "multiple_choice",
  "current_question": 5,
  "total_questions": 20,
  "score": 40,
  "answers": [],
  "saved_at": "2026-01-23T10:00:00Z"
}
```

---

#### POST `/games/progress`

Save game progress (pause game).

**Authentication:** Required

**Request Body:**
```json
{
  "session_id": "uuid",
  "game_type": "multiple_choice",
  "flashcard_type_id": "uuid",
  "current_question": 5,
  "total_questions": 20,
  "score": 40,
  "answers": []
}
```

**Response (200 OK):**
```json
{
  "message": "Progress saved successfully"
}
```

---

#### DELETE `/games/progress`

Delete saved game progress.

**Authentication:** Required

**Query Parameters:**
- `game_type` (required)
- `flashcard_type_id` (optional)

**Response (200 OK):**
```json
{
  "message": "Progress deleted successfully"
}
```

---

#### GET `/users/me/stats`

Get current user's statistics.

**Authentication:** Required

**Response (200 OK):**
```json
{
  "total_games": 50,
  "total_score": 4250,
  "average_score": 85.0,
  "highest_score": 95,
  "total_time_played": 6000,
  "level": 5,
  "xp": 1250,
  "xp_to_next_level": 250,
  "achievements_count": 8,
  "current_streak": 5,
  "longest_streak": 10
}
```

---

#### GET `/users/{user_id}/stats`

Get user's public statistics.

**Response (200 OK):**
```json
{
  "user_id": "uuid",
  "username": "johndoe",
  "total_games": 50,
  "average_score": 85.0,
  "highest_score": 95,
  "level": 5,
  "achievements_count": 8
}
```

---

#### GET `/leaderboard`

Get global leaderboard.

**Query Parameters:**
- `period` (optional) - `daily`, `weekly`, `monthly`, `all_time` (default: all_time)
- `game_type` (optional) - Filter by game type
- `limit` (optional, default: 10) - Number of top players

**Response (200 OK):**
```json
{
  "period": "all_time",
  "data": [
    {
      "rank": 1,
      "user_id": "uuid",
      "username": "johndoe",
      "total_score": 5000,
      "average_score": 90.5,
      "total_games": 55,
      "level": 7
    },
    {
      "rank": 2,
      "user_id": "uuid",
      "username": "janedoe",
      "total_score": 4800,
      "average_score": 88.0,
      "total_games": 55,
      "level": 6
    }
  ]
}
```

---

#### GET `/achievements`

Get all available achievements.

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "uuid",
      "name": "Quick Learner",
      "description": "Complete a game in under 2 minutes",
      "icon": "⚡",
      "requirement": "time_based",
      "threshold": 120
    },
    {
      "id": "uuid",
      "name": "Perfect Score",
      "description": "Get 100% on any game",
      "icon": "💯",
      "requirement": "score_based",
      "threshold": 100
    }
  ]
}
```

---

#### GET `/users/me/achievements`

Get current user's unlocked achievements.

**Authentication:** Required

**Response (200 OK):**
```json
{
  "data": [
    {
      "achievement": {
        "id": "uuid",
        "name": "Quick Learner",
        "description": "Complete a game in under 2 minutes",
        "icon": "⚡"
      },
      "unlocked_at": "2026-01-23T10:00:00Z",
      "progress": 100
    }
  ]
}
```

---

### Admin Endpoints

All admin endpoints require authentication and the `ROLE_ROOT_ADMIN` role.

**Authentication:** Required + Admin Role

#### POST `/flashcards`

Create a new flashcard.

**Required Permission:** `flashcard:create`

**Request Body:**
```json
{
  "question": "What is 2 + 2?",
  "answer": "4",
  "flashcard_type_id": "uuid",
  "image": "base64_encoded_image" // optional
}
```

**Response (201 Created):**
```json
{
  "id": "uuid",
  "question": "What is 2 + 2?",
  "answer": "4",
  "flashcard_type_id": "uuid",
  "created_at": "2026-01-23T10:00:00Z"
}
```

---

#### PATCH `/flashcards/{id}`

Update flashcard.

**Required Permission:** `flashcard:update`

**Request Body:**
```json
{
  "question": "What is 3 + 3?",
  "answer": "6"
}
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "question": "What is 3 + 3?",
  "answer": "6",
  "updated_at": "2026-01-23T11:00:00Z"
}
```

---

#### DELETE `/flashcards/{id}`

Delete flashcard.

**Required Permission:** `flashcard:delete`

**Response (200 OK):**
```json
{
  "message": "Flashcard deleted successfully"
}
```

---

#### POST `/flashcard-types`

Create flashcard category/type.

**Required Permission:** `flashcard_type:create`

**Request Body:**
```json
{
  "name": "History",
  "description": "Historical flashcards"
}
```

**Response (201 Created):**
```json
{
  "id": "uuid",
  "name": "History",
  "description": "Historical flashcards",
  "created_at": "2026-01-23T10:00:00Z"
}
```

---

#### GET `/roles`

Get all roles.

**Required Permission:** `role:read`

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "uuid",
      "name": "ROLE_ROOT_ADMIN",
      "description": "Root administrator"
    }
  ]
}
```

---

#### GET `/permissions`

Get all permissions.

**Required Permission:** `permission:read`

**Response (200 OK):**
```json
{
  "data": [
    {
      "id": "uuid",
      "code": "flashcard:create",
      "description": "Create flashcards"
    }
  ]
}
```

---

## Additional Resources

- [Database Schema](DATABASE.md)
- [Deployment Guide](DEPLOYMENT.md)
- [Contributing Guide](../CONTRIBUTING.md)

---

## API Versioning

Currently, the API is at version 1.0. Future breaking changes will be versioned (e.g., `/api/v2/...`).

---

## Support

For API support:
- Open an issue on [GitHub](https://github.com/rex-pj/rex_game/issues)
- Email: api-support@your-domain.com

---

*Last updated: January 2026*
