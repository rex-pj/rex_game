import { writable, derived, get } from "svelte/store";
import type { Writable, Readable } from "svelte/store";
import type { Flashcard } from "$lib/models/flashcard";
import {
  type GameCard,
  type GameState,
  type GameStats,
  type GameConfig,
  DEFAULT_GAME_CONFIG,
} from "$lib/models/game-card";
import {
  generateGameCards,
  getPairsForLevel,
  calculateScore,
  calculateLevelScore,
} from "$lib/utils/game-helpers";

/**
 * Flashcard Game Store
 * Centralized state management for the matching game
 */

// Game state
export const gameState: Writable<GameState> = writable("loading");

// Available flashcards from API
export const flashcards: Writable<Flashcard[]> = writable([]);

// Current game cards
export const gameCards: Writable<GameCard[]> = writable([]);

// Selected cards (max 2)
export const selectedCards: Writable<GameCard[]> = writable([]);

// Game statistics
export const gameStats: Writable<GameStats> = writable({
  level: 1,
  moves: 0,
  matches: 0,
  timeElapsed: 0,
  score: 0,
});

// Game configuration
export const gameConfig: Writable<GameConfig> = writable(DEFAULT_GAME_CONFIG);

// Timer interval ID
let timerInterval: number | null = null;

/**
 * Derived store: Check if all cards are matched
 */
export const isLevelComplete: Readable<boolean> = derived(gameCards, ($gameCards) => {
  return $gameCards.length > 0 && $gameCards.every((card) => card.matched);
});

/**
 * Derived store: Number of pairs needed for current level
 */
export const pairsNeeded: Readable<number> = derived(gameStats, ($gameStats) => {
  return getPairsForLevel($gameStats.level);
});

/**
 * Initialize the game with flashcards
 */
export function initializeGame(loadedFlashcards: Flashcard[], config?: Partial<GameConfig>) {
  flashcards.set(loadedFlashcards);

  if (config) {
    gameConfig.update((current) => ({ ...current, ...config }));
  }

  const currentConfig = get(gameConfig);
  gameStats.update((stats) => ({
    ...stats,
    level: currentConfig.initialLevel,
  }));

  startNewLevel();
}

/**
 * Start a new level
 */
export async function startNewLevel() {
  const currentFlashcards = get(flashcards);
  const currentStats = get(gameStats);

  const pairsForLevel = getPairsForLevel(currentStats.level);

  try {
    const cards = await generateGameCards(currentFlashcards, pairsForLevel);

    gameCards.set(cards);
    selectedCards.set([]);

    gameStats.update((stats) => ({
      ...stats,
      moves: 0,
      matches: 0,
      timeElapsed: 0,
    }));

    gameState.set("idle");
    startTimer();
  } catch (error) {
    console.error("Failed to start new level:", error);
    setError();
  }
}

/**
 * Flip a card
 */
export function flipCard(card: GameCard) {
  const currentState = get(gameState);
  const currentSelectedCards = get(selectedCards);
  const currentCards = get(gameCards);

  // Validate flip action
  if (
    (currentState !== "idle" && currentState !== "selecting") ||
    card.matched ||
    card.flipped ||
    currentSelectedCards.length >= 2
  ) {
    return;
  }

  // Update card to flipped state
  gameCards.update((cards) => cards.map((c) => (c.id === card.id ? { ...c, flipped: true } : c)));

  // Add to selected cards
  const newSelectedCards = [...currentSelectedCards, card];
  selectedCards.set(newSelectedCards);

  // Update game state
  if (newSelectedCards.length === 1) {
    gameState.set("selecting");
  } else if (newSelectedCards.length === 2) {
    gameState.set("checking");
    gameStats.update((stats) => ({ ...stats, moves: stats.moves + 1 }));
    checkMatch(newSelectedCards);
  }
}

/**
 * Check if two selected cards match
 */
function checkMatch(cards: GameCard[]) {
  const [card1, card2] = cards;
  const isMatch = card1.flashcardId === card2.flashcardId;
  const config = get(gameConfig);
  const stats = get(gameStats);

  setTimeout(() => {
    gameCards.update((allCards) =>
      allCards.map((c) => {
        if (c.id === card1.id || c.id === card2.id) {
          return isMatch ? { ...c, matched: true, flipped: true } : { ...c, flipped: false };
        }
        return c;
      })
    );

    if (isMatch) {
      const matchScore = calculateScore(stats.level, stats.timeElapsed, config);
      gameStats.update((s) => ({
        ...s,
        matches: s.matches + 1,
        score: s.score + matchScore,
      }));
    }

    selectedCards.set([]);

    // Check if level is complete
    const currentCards = get(gameCards);
    if (currentCards.every((c) => c.matched)) {
      handleLevelComplete();
    } else {
      gameState.set("idle");
    }
  }, config.mismatchDelay);
}

/**
 * Handle level completion
 */
function handleLevelComplete() {
  stopTimer();
  gameState.set("completed");

  const stats = get(gameStats);
  const config = get(gameConfig);

  const levelScore = calculateLevelScore(stats.level, stats.moves, stats.timeElapsed, config);

  gameStats.update((s) => ({
    ...s,
    score: s.score + levelScore,
  }));

  // Progress to next level after delay
  setTimeout(() => {
    gameStats.update((s) => ({
      ...s,
      level: s.level + 1,
    }));
    startNewLevel();
  }, config.levelCompleteDelay);
}

/**
 * Start the timer
 */
function startTimer() {
  stopTimer(); // Clear any existing timer

  timerInterval = window.setInterval(() => {
    gameStats.update((stats) => ({
      ...stats,
      timeElapsed: stats.timeElapsed + 1,
    }));
  }, 1000);
}

/**
 * Stop the timer
 */
function stopTimer() {
  if (timerInterval !== null) {
    clearInterval(timerInterval);
    timerInterval = null;
  }
}

/**
 * Reset the game to initial state
 */
export function resetGame() {
  stopTimer();
  const config = get(gameConfig);

  gameStats.set({
    level: config.initialLevel,
    moves: 0,
    matches: 0,
    timeElapsed: 0,
    score: 0,
  });

  startNewLevel();
}

/**
 * Cleanup function - call when component unmounts
 */
export function cleanup() {
  stopTimer();
}

/**
 * Set error state
 */
export function setError() {
  stopTimer();
  gameState.set("error");
}
