/**
 * Game Card Model for Flashcard Matching Game
 * Represents a single card in the matching game
 */
export interface GameCard {
  /** Unique ID for this card instance: "flashcardId-1" or "flashcardId-2" */
  id: string;

  /** Original flashcard ID from backend - used to match pairs */
  flashcardId: number;

  /** Card display name */
  name: string;

  /** Card image URL */
  imageUrl: string;

  /** Whether this card has been successfully matched */
  matched: boolean;

  /** Whether this card is currently flipped (showing front) */
  flipped: boolean;
}

/**
 * Game State enum
 * Represents the current state of the game
 */
export type GameState =
  | 'loading'    // Loading flashcards from API
  | 'idle'       // Ready to flip cards
  | 'selecting'  // First card selected
  | 'checking'   // Checking if two cards match
  | 'completed'  // Level completed
  | 'error';     // Error loading flashcards

/**
 * Game Statistics
 * Tracks player performance metrics
 */
export interface GameStats {
  /** Current level (affects grid size) */
  level: number;

  /** Total number of moves (pair flips) made */
  moves: number;

  /** Number of successful matches */
  matches: number;

  /** Time elapsed in current level (seconds) */
  timeElapsed: number;

  /** Current score */
  score: number;
}

/**
 * Game Configuration
 * Settings for game behavior
 */
export interface GameConfig {
  /** Starting level (default: 3 for 3x3 grid) */
  initialLevel: number;

  /** Number of additional pairs per level increase */
  pairsPerLevelIncrease: number;

  /** Time bonus multiplier for quick completion */
  timeBonusMultiplier: number;

  /** Base points per match */
  basePointsPerMatch: number;

  /** Delay before flipping cards back (ms) */
  mismatchDelay: number;

  /** Delay before progressing to next level (ms) */
  levelCompleteDelay: number;
}

/**
 * Default game configuration
 */
export const DEFAULT_GAME_CONFIG: GameConfig = {
  initialLevel: 3,
  pairsPerLevelIncrease: 2,
  timeBonusMultiplier: 10,
  basePointsPerMatch: 100,
  mismatchDelay: 1000,
  levelCompleteDelay: 3000,
};
