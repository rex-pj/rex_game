# Rex Game

A web-based educational flashcard game designed for interactive learning through gamified experiences.

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-5.0-ff3e00.svg)](https://svelte.dev/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15-blue.svg)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

---

## 1. About

Rex Game is an interactive web-based flashcard learning platform that makes education fun through gamification. Originally created as a learning tool, it features multiple game modes, progress tracking, achievements, and leaderboards to encourage continuous learning.

### Key Features

- **Multiple Game Modes**
  - Multiple Choice Quiz
  - Flashcard Matching
  - Fill in the Blanks

- **Progress Tracking**
  - Real-time scoring system
  - Session history
  - Performance analytics
  - Personal statistics dashboard

- **Gamification**
  - Achievement system with unlockable badges
  - Global leaderboard
  - XP and level progression
  - Streak tracking

- **User Management**
  - Secure authentication with JWT
  - Email verification
  - Password reset functionality
  - Profile customization

- **Flashcard Management**
  - Custom flashcard categories
  - Rich text support (Markdown)
  - Image attachments
  - Import/Export capabilities

---

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Client (Browser)                      │
│                     SvelteKit 2 + Svelte 5                   │
└────────────────────┬────────────────────────────────────────┘
                     │ REST API (JSON)
                     │
┌────────────────────┴────────────────────────────────────────┐
│                      Backend Server                          │
│                   Rust + Axum Framework                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Modules (DDD Architecture)              │  │
│  ├──────────────┬──────────────┬──────────────┬─────────┤  │
│  │   Identity   │    Games     │   Mail       │ Shared  │  │
│  │   (Auth)     │  (Scoring)   │ Templates    │ Utils   │  │
│  └──────────────┴──────────────┴──────────────┴─────────┘  │
└────────────────────┬────────────────────────────────────────┘
                     │ SeaORM
                     │
┌────────────────────┴────────────────────────────────────────┐
│                    PostgreSQL Database                       │
│         (Users, Flashcards, Sessions, Achievements)          │
└──────────────────────────────────────────────────────────────┘
```

### Module Structure (Domain-Driven Design)

```
backend/
├── entities/           # Database models (SeaORM entities)
├── migration/          # Database migrations
├── modules/            # Business logic modules
│   ├── identity/       # Authentication & authorization
│   ├── games/          # Game logic & scoring
│   └── mail_templates/ # Email templates
├── shared/             # Shared utilities & infrastructure
└── src/                # API handlers & routing
```

---

## 3. Tech Stack

### Backend

| Component          | Technology | Version |
| ------------------ | ---------- | ------- |
| **Language**       | Rust       | 1.70+   |
| **Web Framework**  | Axum       | 0.7     |
| **ORM**            | SeaORM     | 1.1     |
| **Database**       | PostgreSQL | 15+     |
| **Authentication** | JWT        | -       |
| **Email**          | Resend API | -       |

### Frontend

| Component       | Technology   | Version |
| --------------- | ------------ | ------- |
| **Framework**   | SvelteKit    | 2.x     |
| **UI Library**  | Svelte       | 5.x     |
| **Styling**     | Bootstrap 5  | 5.3     |
| **Icons**       | Font Awesome | 6.x     |
| **HTTP Client** | Fetch API    | -       |

### DevOps

- **CI/CD**: GitHub Actions
- **Deployment**: Google Compute Engine
- **Web Server**: Nginx (reverse proxy)
- **SSL**: Let's Encrypt

---

## 4. Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** 1.70 or higher ([Install Rust](https://www.rust-lang.org/tools/install))
- **Node.js** 20.x or higher ([Install Node](https://nodejs.org/))
- **PostgreSQL** 15 or higher ([Install PostgreSQL](https://www.postgresql.org/download/))
- **Git** ([Install Git](https://git-scm.com/downloads))

---

## 5. Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/rex-pj/rex_game.git
cd rex_game
```

### 2. Database Setup

Create a PostgreSQL database:

```bash
# Connect to PostgreSQL
psql -U postgres

# Create database
CREATE DATABASE rex_game_db;

# Create user (optional)
CREATE USER rex_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE rex_game_db TO rex_user;

# Exit
\q
```

### 3. Backend Setup

```bash
cd backend

# Copy environment file
cp environments/.env.example environments/.env.dev

# Edit .env.dev with your configuration
# - DATABASE_URL
# - JWT_CLIENT_SECRET (generate with: openssl rand -base64 32)
# - RESEND_API_KEY (get from https://resend.com)

# Run migrations
cargo run --bin migration

# Start the backend server
cargo run
```

The backend will start at `http://localhost:8080`

### 4. Frontend Setup

```bash
cd client-app

# Install dependencies
npm install

# Start development server
npm run dev
```

The frontend will start at `http://localhost:5173`

### 5. Access the Application

Open your browser and navigate to:

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:8080

---

## 6. Project Structure

```
rex_game/
├── backend/                    # Rust backend
│   ├── entities/              # Database entities (SeaORM models)
│   ├── environments/          # Environment configuration files
│   │   ├── .env.example      # Environment template
│   │   ├── .env.dev          # Development config
│   │   └── .env.prod         # Production config
│   ├── migration/            # Database migrations
│   ├── modules/              # Business logic modules
│   │   ├── identity/         # Authentication module
│   │   ├── games/            # Game & scoring module
│   │   └── mail_templates/   # Email templates
│   ├── shared/               # Shared utilities
│   │   ├── domain/           # Domain logic
│   │   └── infrastructure/   # Infrastructure (DB, email, etc.)
│   └── src/                  # API layer
│       ├── handlers/         # Request handlers
│       ├── middlewares/      # Auth, CORS, etc.
│       └── routings/         # Route definitions
│
├── client-app/                # SvelteKit frontend
│   ├── src/
│   │   ├── components/       # Reusable UI components
│   │   │   ├── atoms/        # Basic elements
│   │   │   ├── molecules/    # Composite components
│   │   │   └── organisms/    # Complex components
│   │   ├── lib/              # Utilities & stores
│   │   │   ├── api/          # API client functions
│   │   │   ├── models/       # TypeScript types
│   │   │   └── stores/       # Svelte stores
│   │   └── routes/           # SvelteKit routes
│   │       ├── (app)/        # Authenticated routes
│   │       └── (auth)/       # Public routes
│   └── static/               # Static assets
│
├── docs/                      # Documentation
│   ├── DEPLOYMENT.md         # Deployment guide
│   ├── IMPROVEMENTS_SUMMARY.md
│   └── RATE_LIMITING_IMPLEMENTATION.md
│
├── .github/
│   └── workflows/
│       └── deploy.yml        # CI/CD pipeline
│
└── README.md                 # This file
```

---

## 7. Documentation

| Document                               | Description                                |
| -------------------------------------- | ------------------------------------------ |
| [Deployment Guide](docs/DEPLOYMENT.md) | Complete guide for deploying to production |
| [API Documentation](docs/API.md)       | REST API endpoints reference               |
| [Database Schema](docs/DATABASE.md)    | Database structure and relationships       |
| [Contributing Guide](CONTRIBUTING.md)  | How to contribute to the project           |

---

## 8. Environment Variables

### Backend Configuration

Create `backend/environments/.env.dev` with the following:

```bash
# Database
DATABASE_URL=postgres://postgres:admin@localhost:5432/rex_game_db

# JWT (generate with: openssl rand -base64 32)
JWT_CLIENT_SECRET=your-generated-secret-here

# CORS
CORS_ALLOW_ORIGINS=http://localhost:5173,https://localhost:5173

# Email - Resend (https://resend.com)
EMAIL_PROVIDER=resend
RESEND_API_KEY=re_xxxxxxxxxx
EMAIL_FROM_NAME=Rex Game
EMAIL_FROM_ADDRESS=onboarding@resend.dev

# Platform URLs
PLATFORM_URL=http://localhost:5173
SIGNUP_VERIFICATION_URL=http://localhost:5173/account/confirm?token=[token]
RESET_PASSWORD_URL=http://localhost:5173/account/reset-password?token=[token]
```

For production configuration, see [Deployment Guide](docs/DEPLOYMENT.md).

---

## 9. Running Tests

```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd client-app
npm test
```

---

## 10. Building for Production

### Backend

```bash
cd backend
cargo build --release
```

The binary will be at `backend/target/release/rex_game`

### Frontend

```bash
cd client-app
npm run build
```

The build output will be in `client-app/build/`

---

## 11. Deployment

For detailed deployment instructions, see [Deployment Guide](docs/DEPLOYMENT.md).

Quick deployment options:

- **Google Cloud Platform** (recommended, includes free tier)
- **AWS EC2**
- **DigitalOcean Droplets**
- **Any VPS with Ubuntu 22.04+**

---

## 12. Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions and best practices
- Write tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting PR

---

## 13. Authors

- **Original Author** - [Rex-PJ](https://github.com/rex-pj)

---

## 14. Acknowledgments

- Built with ❤️ for educational purposes
- Inspired by the desire to make learning fun
- Thanks to the Rust and Svelte communities

---

## 15. Support

For issues, questions, or suggestions:

- Open an issue on [GitHub Issues](https://github.com/rex-pj/rex_game/issues)
- Email: support@your-domain.com

---

## 16. Roadmap

- [ ] Mobile app (React Native)
- [ ] Multiplayer mode
- [ ] AI-generated flashcards
- [ ] Voice recognition for language learning
- [ ] Integration with popular learning platforms
- [ ] Teacher dashboard for classroom use
