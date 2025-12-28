# Contributing to Rex Game

Thank you for your interest in contributing to Rex Game! This document provides guidelines and instructions for contributing to the project.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)

---

## 1. Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Respect differing viewpoints and experiences
- Accept responsibility and apologize for mistakes

---

## 2. Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/rex-pj/rex_game.git
cd rex_game

# Add upstream remote
git remote add upstream https://github.com/rex-pj/rex_game.git
```

### 2. Set Up Development Environment

Follow the instructions in [README.md](README.md) to set up your development environment.

### 3. Create a Branch

```bash
# Update your main branch
git checkout master
git pull upstream master

# Create a feature branch
git checkout -b feature/your-feature-name
# or for bug fixes
git checkout -b fix/bug-description
```

---

## 3. Development Workflow

### Branch Naming Convention

Use descriptive branch names following these patterns:

- `feature/add-multiplayer-mode`
- `fix/login-button-not-working`
- `refactor/simplify-auth-logic`
- `docs/update-api-documentation`
- `test/add-scoring-tests`
- `chore/update-dependencies`

### Keep Your Fork Updated

```bash
# Fetch upstream changes
git fetch upstream

# Merge upstream changes into your branch
git checkout master
git merge upstream/master

# Rebase your feature branch
git checkout feature/your-feature-name
git rebase master
```

---

## 4. Coding Standards

### Rust Backend

#### Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

- Use `rustfmt` for code formatting
- Use `clippy` for linting
- Maximum line length: 100 characters
- Use 4 spaces for indentation

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run all checks before committing
cargo fmt && cargo clippy && cargo test
```

#### Naming Conventions

```rust
// Modules and files: snake_case
mod user_service;

// Structs and Enums: PascalCase
struct UserProfile { }
enum GameType { }

// Functions and variables: snake_case
fn calculate_score() { }
let user_name = "John";

// Constants: SCREAMING_SNAKE_CASE
const MAX_ATTEMPTS: i32 = 5;

// Traits: PascalCase (often with -able or -er suffix)
trait Scoreable { }
```

#### Error Handling

- Use `Result<T, E>` for recoverable errors
- Use custom error types (see `InfraError` in `shared/src/infrastructure/errors/`)
- Avoid `unwrap()` in production code - use `?` operator or `expect()` with meaningful messages

```rust
// Good
let user = find_user(id).await?;

// Better with context
let user = find_user(id)
    .await
    .map_err(|e| InfraError::database(format!("Failed to find user {}: {}", id, e)))?;

// Acceptable for initialization
let config = ConfigHelper::new()
    .expect("Configuration must be valid at startup");
```

#### Documentation

Use Rust doc comments for public APIs:

```rust
/// Calculates the score for a game session.
///
/// # Arguments
///
/// * `correct_answers` - Number of correct answers
/// * `total_questions` - Total number of questions
/// * `time_taken` - Time taken in seconds
///
/// # Returns
///
/// The calculated score as a float between 0.0 and 100.0
///
/// # Examples
///
/// ```
/// let score = calculate_score(8, 10, 120);
/// assert_eq!(score, 80.0);
/// ```
pub fn calculate_score(correct_answers: i32, total_questions: i32, time_taken: i32) -> f64 {
    // Implementation
}
```

### TypeScript/JavaScript Frontend

#### Style Guide

- Use ESLint and Prettier for formatting
- Use TypeScript for type safety
- Maximum line length: 100 characters
- Use 2 spaces for indentation

```bash
# Format code
npm run format

# Run linter
npm run lint

# Type check
npm run check
```

#### Naming Conventions

```typescript
// Files and folders: kebab-case
// game-session.ts, user-profile.svelte

// Classes and Types: PascalCase
class UserService { }
interface GameSession { }
type ScoreData = { }

// Functions and variables: camelCase
function calculateScore() { }
const userName = "John";

// Constants: SCREAMING_SNAKE_CASE
const MAX_ATTEMPTS = 5;

// Svelte components: PascalCase
// FlashcardGame.svelte
```

#### Component Structure (Svelte)

```svelte
<script lang="ts">
  // 1. Imports
  import { onMount } from 'svelte';
  import type { GameSession } from '$lib/models';

  // 2. Props
  interface Props {
    sessionId: string;
    onComplete?: (score: number) => void;
  }
  let { sessionId, onComplete }: Props = $props();

  // 3. State
  let score = $state(0);
  let isLoading = $state(true);

  // 4. Derived state
  let percentage = $derived(score / 100);

  // 5. Functions
  function handleSubmit() {
    // Implementation
  }

  // 6. Lifecycle
  onMount(() => {
    // Initialization
  });
</script>

<!-- 7. Markup -->
<div class="game-container">
  {#if isLoading}
    <p>Loading...</p>
  {:else}
    <!-- Game UI -->
  {/if}
</div>

<!-- 8. Styles -->
<style>
  .game-container {
    padding: 1rem;
  }
</style>
```

### Database Migrations

- One migration per logical change
- Use descriptive names: `m20260123_100000_add_user_game_progress.rs`
- Always include both `up` and `down` migrations
- Test migrations before committing

```rust
// Example migration
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create table
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop table
    }
}
```

---

## 5. Commit Guidelines

### Commit Message Format

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, missing semicolons, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks (dependencies, build, etc.)
- `perf`: Performance improvements

### Examples

```bash
# Simple feature
feat(auth): add password reset functionality

# Bug fix with issue reference
fix(scoring): correct calculation for bonus points

Fixes #123

# Breaking change
feat(api)!: change authentication endpoint structure

BREAKING CHANGE: /api/auth/login now requires email instead of username

# Multiple changes
chore: update dependencies and cleanup code

- Update axum to 0.7.5
- Remove unused imports
- Run cargo fmt
```

### Commit Best Practices

- Write clear, concise commit messages
- Use present tense ("add feature" not "added feature")
- Keep commits atomic (one logical change per commit)
- Reference issues in commit messages when applicable
- Avoid committing generated files or dependencies

---

## 6. Pull Request Process

### Before Creating a PR

1. **Update your branch** with the latest main
2. **Run all tests** and ensure they pass
3. **Run linters** and fix any issues
4. **Test your changes** manually
5. **Update documentation** if needed

### Creating a Pull Request

1. **Push your branch** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create PR** on GitHub with a clear title and description

3. **Fill out the PR template** (see below)

### Pull Request Template

```markdown
## Description
Brief description of what this PR does.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Related Issue
Closes #(issue number)

## Changes Made
- Change 1
- Change 2
- Change 3

## Testing Done
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Screenshots (if applicable)
Add screenshots here

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have updated the documentation accordingly
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
```

### Review Process

- At least one maintainer approval required
- All CI checks must pass
- Address review comments promptly
- Keep discussions focused and professional

### After Approval

- Squash commits if requested
- Maintainers will merge your PR
- Delete your feature branch after merge

---

## 7. Testing Guidelines

### Backend Tests

```rust
// Unit test example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score() {
        let score = calculate_score(8, 10, 120);
        assert_eq!(score, 80.0);
    }

    #[tokio::test]
    async fn test_create_user() {
        let user = create_user("test@example.com").await.unwrap();
        assert_eq!(user.email, "test@example.com");
    }
}
```

### Frontend Tests

```typescript
// Component test example (Vitest)
import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import GameScore from './GameScore.svelte';

describe('GameScore', () => {
  it('displays the correct score', () => {
    render(GameScore, { score: 85 });
    expect(screen.getByText('85%')).toBeInTheDocument();
  });
});
```

### Test Coverage

- Aim for >80% code coverage
- Test edge cases and error conditions
- Write integration tests for critical paths
- Mock external dependencies

### Running Tests

```bash
# Backend
cd backend
cargo test
cargo test --all-features

# Frontend
cd client-app
npm test
npm run test:coverage
```

---

## 8. Documentation

### When to Update Documentation

Update documentation when you:
- Add new features
- Change existing APIs
- Modify configuration
- Update dependencies
- Fix significant bugs

### Documentation Locations

| Type | Location |
|------|----------|
| API endpoints | `docs/API.md` |
| Database schema | `docs/DATABASE.md` |
| Deployment guide | `docs/DEPLOYMENT.md` |
| Code documentation | Inline comments + rustdoc/JSDoc |
| User guide | `README.md` |

### Writing Good Documentation

- Be clear and concise
- Include code examples
- Keep it up to date
- Use proper formatting (Markdown)
- Add diagrams where helpful

---

## 9. Questions

If you have questions about contributing:

1. Check existing [issues](https://github.com/rex-pj/rex_game/issues)
2. Open a [discussion](https://github.com/rex-pj/rex_game/discussions)
3. Join our community chat (if available)
4. Email the maintainers

---

## 10. Thank You!

Your contributions make Rex Game better for everyone. We appreciate your time and effort!

---

**Happy Coding! 🚀**
