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
} from "$lib/helpers/gameHelpers";
import { ScoringApi, type GameCompleteResponse, type Achievement, type GameProgress } from "$lib/api/scoringApi";
import { ACCESS_TOKEN } from "$lib/common/contants";
import Cookies from "js-cookie";

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

// Session tracking for backend
export const currentSessionId: Writable<number | null> = writable(null);
export const sessionStartTime: Writable<number | null> = writable(null);
export const comboCount: Writable<number> = writable(0);
export const maxCombo: Writable<number> = writable(0);
export const wrongAnswers: Writable<number> = writable(0);
export const totalCorrectAnswers: Writable<number> = writable(0); // Total correct matches across all levels
export const currentFlashcardTypeId: Writable<number | undefined> = writable(undefined);

// New achievements unlocked
export const newAchievements: Writable<Achievement[]> = writable([]);

// Game complete response from backend
export const lastGameResult: Writable<GameCompleteResponse | null> = writable(null);

// Saved game progress from backend
export const savedProgress: Writable<GameProgress | null> = writable(null);

// Timer interval ID
let timerInterval: number | null = null;

// Scoring API instance
let scoringApi: ScoringApi | null = null;

/**
 * Check if user is logged in
 */
function isUserLoggedIn(): boolean {
  const token = Cookies.get(ACCESS_TOKEN.USER_ACCESS_TOKEN);
  console.log("[Scoring] Checking login status, token key:", ACCESS_TOKEN.USER_ACCESS_TOKEN, "token exists:", !!token);
  return !!token;
}

/**
 * Initialize Scoring API
 */
function getScoringApi(): ScoringApi {
  if (!scoringApi) {
    scoringApi = new ScoringApi({
      cookies: Cookies,
      tokenKey: ACCESS_TOKEN.USER_ACCESS_TOKEN,
    });
  }
  return scoringApi;
}

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
export async function initializeGame(
  loadedFlashcards: Flashcard[],
  config?: Partial<GameConfig> & { flashcardTypeId?: number }
) {
  flashcards.set(loadedFlashcards);

  if (config) {
    gameConfig.update((current) => ({ ...current, ...config }));
  }

  // Store flashcardTypeId for later use when starting new sessions
  currentFlashcardTypeId.set(config?.flashcardTypeId);

  const currentConfig = get(gameConfig);
  gameStats.update((stats) => ({
    ...stats,
    level: currentConfig.initialLevel,
  }));

  // Reset session tracking
  comboCount.set(0);
  maxCombo.set(0);
  wrongAnswers.set(0);
  totalCorrectAnswers.set(0);
  newAchievements.set([]);
  lastGameResult.set(null);
  sessionStartTime.set(Date.now());

  // Start game session on backend (only if logged in)
  const loggedIn = isUserLoggedIn();
  console.log("[Scoring] initializeGame - user logged in:", loggedIn);

  if (loggedIn) {
    try {
      console.log("[Scoring] Starting game session...");
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "memory_match",
        flashcard_type_id: config?.flashcardTypeId,
      });
      currentSessionId.set(sessionId);
      console.log("[Scoring] Game session started:", sessionId);
    } catch (error) {
      console.error("[Scoring] Failed to start game session on backend:", error);
      // Continue playing even if backend fails
      currentSessionId.set(null);
    }
  } else {
    console.log("[Scoring] User not logged in, skipping session creation");
    currentSessionId.set(null);
  }

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

      // Track total correct answers across all levels
      totalCorrectAnswers.update((c) => c + 1);

      // Update combo tracking
      comboCount.update((c) => c + 1);
      const currentCombo = get(comboCount);
      const currentMax = get(maxCombo);
      if (currentCombo > currentMax) {
        maxCombo.set(currentCombo);
      }
    } else {
      // Reset combo on wrong answer
      comboCount.set(0);
      wrongAnswers.update((w) => w + 1);
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
 * Handle level completion - auto-save session and start new one for next level
 */
async function handleLevelComplete() {
  stopTimer();
  gameState.set("completed");

  const stats = get(gameStats);
  const config = get(gameConfig);

  const levelScore = calculateLevelScore(stats.level, stats.moves, stats.timeElapsed, config);

  gameStats.update((s) => ({
    ...s,
    score: s.score + levelScore,
  }));

  // Auto-save current level session
  if (isUserLoggedIn() && get(currentSessionId)) {
    await saveLevelSession();
  }

  // Progress to next level after delay
  setTimeout(async () => {
    const newLevel = stats.level + 1;
    const newScore = get(gameStats).score;

    gameStats.update((s) => ({
      ...s,
      level: newLevel,
    }));

    // Save game progress to database
    if (isUserLoggedIn()) {
      await saveGameProgress(newLevel, newScore);
      // Start new session for next level
      await startNewSession();
    }

    startNewLevel();
  }, config.levelCompleteDelay);
}

/**
 * Save current level session to backend (without ending the game UI)
 */
async function saveLevelSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(currentSessionId);
  if (!sessionId) {
    return null;
  }

  const stats = get(gameStats);
  const startTime = get(sessionStartTime);
  const currentMaxCombo = get(maxCombo);
  const currentWrongAnswers = get(wrongAnswers);
  const currentTotalCorrectAnswers = get(totalCorrectAnswers);

  const timeSpentSeconds = startTime ? Math.floor((Date.now() - startTime) / 1000) : stats.timeElapsed;

  console.log("[Scoring] Auto-saving level session:", {
    session_id: sessionId,
    level: stats.level,
    score: stats.score,
    correct_answers: currentTotalCorrectAnswers,
    wrong_answers: currentWrongAnswers,
    combo_max: currentMaxCombo,
    time_spent_seconds: timeSpentSeconds,
  });

  try {
    const api = getScoringApi();
    const result = await api.completeGameSession(fetch, {
      session_id: sessionId,
      score: stats.score,
      correct_answers: currentTotalCorrectAnswers,
      wrong_answers: currentWrongAnswers,
      combo_max: currentMaxCombo,
      time_spent_seconds: timeSpentSeconds,
    });

    // Store the result
    lastGameResult.set(result);

    // Store new achievements for display
    if (result.new_achievements && result.new_achievements.length > 0) {
      newAchievements.set(result.new_achievements);
    }

    console.log("[Scoring] Level session saved:", result);
    return result;
  } catch (error) {
    console.error("[Scoring] Failed to save level session:", error);
    return null;
  }
}

/**
 * Start a new session for the next level
 */
async function startNewSession(): Promise<void> {
  // Reset session tracking for new level
  comboCount.set(0);
  maxCombo.set(0);
  wrongAnswers.set(0);
  totalCorrectAnswers.set(0);
  sessionStartTime.set(Date.now());

  try {
    const api = getScoringApi();
    const flashcardTypeId = get(currentFlashcardTypeId);
    const sessionId = await api.startGameSession(fetch, {
      game_type_code: "memory_match",
      flashcard_type_id: flashcardTypeId,
    });
    currentSessionId.set(sessionId);
    console.log("[Scoring] New level session started:", sessionId);
  } catch (error) {
    console.warn("[Scoring] Failed to start new level session:", error);
    currentSessionId.set(null);
  }
}

/**
 * End game and save results to backend
 * Call this when user wants to end the game session
 */
export async function endGameSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(currentSessionId);
  if (!sessionId) {
    console.warn("No active session to end");
    return null;
  }

  stopTimer();
  gameState.set("completed");

  const stats = get(gameStats);
  const startTime = get(sessionStartTime);
  const currentMaxCombo = get(maxCombo);
  const currentWrongAnswers = get(wrongAnswers);
  const currentTotalCorrectAnswers = get(totalCorrectAnswers);

  // Calculate total time spent
  const timeSpentSeconds = startTime ? Math.floor((Date.now() - startTime) / 1000) : stats.timeElapsed;

  console.log("[Scoring] Ending session with:", {
    session_id: sessionId,
    score: stats.score,
    correct_answers: currentTotalCorrectAnswers,
    wrong_answers: currentWrongAnswers,
    combo_max: currentMaxCombo,
    time_spent_seconds: timeSpentSeconds,
  });

  try {
    const api = getScoringApi();
    const result = await api.completeGameSession(fetch, {
      session_id: sessionId,
      score: stats.score,
      correct_answers: currentTotalCorrectAnswers,
      wrong_answers: currentWrongAnswers,
      combo_max: currentMaxCombo,
      time_spent_seconds: timeSpentSeconds,
    });

    // Store the result
    lastGameResult.set(result);

    // Store new achievements for display
    if (result.new_achievements && result.new_achievements.length > 0) {
      newAchievements.set(result.new_achievements);
    }

    // Clear session ID
    currentSessionId.set(null);

    console.log("Game session completed:", result);
    return result;
  } catch (error) {
    console.error("Failed to save game session:", error);
    currentSessionId.set(null);
    return null;
  }
}

/**
 * Check if there's an active game session
 */
export function hasActiveSession(): boolean {
  return get(currentSessionId) !== null;
}

/**
 * Clear achievements notification
 */
export function clearNewAchievements() {
  newAchievements.set([]);
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
export async function resetGame() {
  // End current session if exists
  if (hasActiveSession()) {
    await endGameSession();
  }

  stopTimer();
  const config = get(gameConfig);

  gameStats.set({
    level: config.initialLevel,
    moves: 0,
    matches: 0,
    timeElapsed: 0,
    score: 0,
  });

  // Reset session tracking
  comboCount.set(0);
  maxCombo.set(0);
  wrongAnswers.set(0);
  totalCorrectAnswers.set(0);
  newAchievements.set([]);
  lastGameResult.set(null);

  // Start new session (only if logged in)
  sessionStartTime.set(Date.now());
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "memory_match",
      });
      currentSessionId.set(sessionId);
    } catch (error) {
      console.warn("Failed to start new game session:", error);
      currentSessionId.set(null);
    }
  } else {
    currentSessionId.set(null);
  }

  startNewLevel();
}

/**
 * Cleanup function - call when component unmounts
 */
export async function cleanup() {
  stopTimer();
  // End session when component unmounts
  if (hasActiveSession()) {
    await endGameSession();
  }
}

/**
 * Set error state
 */
export function setError() {
  stopTimer();
  gameState.set("error");
}

/**
 * Get saved game progress from database
 */
export async function getGameProgress(): Promise<GameProgress | null> {
  if (!isUserLoggedIn()) {
    return null;
  }

  try {
    const api = getScoringApi();
    const progress = await api.getGameProgress(fetch, "memory_match");
    savedProgress.set(progress);
    return progress;
  } catch (error) {
    console.error("[Progress] Failed to get game progress:", error);
    return null;
  }
}

/**
 * Save game progress to database
 */
async function saveGameProgress(level: number, score: number): Promise<void> {
  if (!isUserLoggedIn()) {
    return;
  }

  try {
    const api = getScoringApi();
    const progress = await api.saveGameProgress(fetch, {
      game_type_code: "memory_match",
      current_level: level,
      total_score: score,
    });
    savedProgress.set(progress);
    console.log("[Progress] Game progress saved:", progress);
  } catch (error) {
    console.error("[Progress] Failed to save game progress:", error);
  }
}

/**
 * Continue game from saved progress
 */
export async function continueFromProgress(progress: GameProgress): Promise<void> {
  const currentConfig = get(gameConfig);

  gameStats.update((stats) => ({
    ...stats,
    level: progress.current_level,
    score: progress.total_score,
  }));

  // Clear saved progress indicator
  savedProgress.set(null);
}

/**
 * Start new game (reset progress)
 */
export async function startNewGame(): Promise<void> {
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      await api.resetGameProgress(fetch, "memory_match");
      console.log("[Progress] Game progress reset");
    } catch (error) {
      console.error("[Progress] Failed to reset game progress:", error);
    }
  }

  savedProgress.set(null);
}
