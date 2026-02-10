import { writable, derived, get } from "svelte/store";
import type { Writable, Readable } from "svelte/store";
import type { Flashcard } from "$lib/models/flashcard";
import {
  type SpeedMatchPair,
  type SpeedMatchState,
  type SpeedMatchStats,
  type SpeedMatchConfig,
  DEFAULT_SPEED_MATCH_CONFIG,
} from "$lib/models/speed-match-card";
import {
  generateSpeedMatchPairs,
  getPairsForLevel,
  calculateSpeedMatchScore,
} from "$lib/helpers/speedMatchHelpers";
import {
  ScoringApi,
  type GameCompleteResponse,
  type Achievement,
  type GameProgress,
} from "$lib/api/scoringApi";
import { ACCESS_TOKEN } from "$lib/common/contants";
import Cookies from "js-cookie";

/**
 * Speed Match Game Store
 * Centralized state management for the speed match game
 */

// Game state
export const speedMatchState: Writable<SpeedMatchState> = writable("loading");

// Available flashcards from API
export const speedMatchFlashcards: Writable<Flashcard[]> = writable([]);

// Current speed match pairs
export const speedMatchPairs: Writable<SpeedMatchPair[]> = writable([]);

// Current pair index
export const currentPairIndex: Writable<number> = writable(0);

// Speed match statistics
export const speedMatchStats: Writable<SpeedMatchStats> = writable({
  level: 1,
  correct: 0,
  wrong: 0,
  timeRemaining: DEFAULT_SPEED_MATCH_CONFIG.timePerRound,
  score: 0,
  streak: 0,
});

// Speed match configuration
export const speedMatchConfig: Writable<SpeedMatchConfig> = writable(
  DEFAULT_SPEED_MATCH_CONFIG
);

// Session tracking for backend
export const speedMatchSessionId: Writable<number | null> = writable(null);
export const speedMatchSessionStartTime: Writable<number | null> =
  writable(null);
export const speedMatchComboCount: Writable<number> = writable(0);
export const speedMatchMaxCombo: Writable<number> = writable(0);
export const speedMatchCurrentFlashcardTypeId: Writable<number | undefined> =
  writable(undefined);

// New achievements unlocked
export const speedMatchNewAchievements: Writable<Achievement[]> = writable([]);

// Game complete response from backend
export const speedMatchLastGameResult: Writable<GameCompleteResponse | null> =
  writable(null);

// Saved game progress from backend
export const speedMatchSavedProgress: Writable<GameProgress | null> =
  writable(null);

// Timer interval ID
let timerInterval: number | null = null;

// Scoring API instance
let scoringApi: ScoringApi | null = null;

/**
 * Check if user is logged in
 */
function isUserLoggedIn(): boolean {
  const token = Cookies.get(ACCESS_TOKEN.USER_ACCESS_TOKEN);
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
 * Derived store: Current pair
 */
export const currentPair: Readable<SpeedMatchPair | null> = derived(
  [speedMatchPairs, currentPairIndex],
  ([$pairs, $index]) => {
    if ($pairs.length === 0 || $index >= $pairs.length) {
      return null;
    }
    return $pairs[$index];
  }
);

/**
 * Derived store: Check if speed match round is complete
 */
export const isSpeedMatchComplete: Readable<boolean> = derived(
  [speedMatchPairs, currentPairIndex, speedMatchStats],
  ([$pairs, $index, $stats]) => {
    return (
      ($pairs.length > 0 && $index >= $pairs.length) ||
      $stats.timeRemaining <= 0
    );
  }
);

/**
 * Derived store: Progress percentage
 */
export const speedMatchProgress: Readable<number> = derived(
  [currentPairIndex, speedMatchPairs],
  ([$index, $pairs]) => {
    if ($pairs.length === 0) return 0;
    return Math.round(($index / $pairs.length) * 100);
  }
);

/**
 * Initialize the speed match game with flashcards
 */
export async function initializeSpeedMatch(
  loadedFlashcards: Flashcard[],
  config?: Partial<SpeedMatchConfig> & { flashcardTypeId?: number }
) {
  speedMatchFlashcards.set(loadedFlashcards);

  if (config) {
    speedMatchConfig.update((current) => ({ ...current, ...config }));
  }

  // Store flashcardTypeId for later use
  speedMatchCurrentFlashcardTypeId.set(config?.flashcardTypeId);

  const finalConfig = get(speedMatchConfig);

  // Reset stats
  speedMatchStats.set({
    level: 1,
    correct: 0,
    wrong: 0,
    timeRemaining: finalConfig.timePerRound,
    score: 0,
    streak: 0,
  });

  // Reset session tracking
  speedMatchComboCount.set(0);
  speedMatchMaxCombo.set(0);
  speedMatchNewAchievements.set([]);
  speedMatchLastGameResult.set(null);
  speedMatchSessionStartTime.set(Date.now());
  currentPairIndex.set(0);

  // Start game session on backend (only if logged in)
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "speed_match",
        flashcard_type_id: config?.flashcardTypeId,
      });
      speedMatchSessionId.set(sessionId);
      console.log("[SpeedMatch] Game session started:", sessionId);
    } catch (error) {
      console.error("[SpeedMatch] Failed to start game session:", error);
      speedMatchSessionId.set(null);
    }
  } else {
    speedMatchSessionId.set(null);
  }

  await startNewRound();
}

/**
 * Start a new round of pairs
 */
export async function startNewRound() {
  const currentFlashcards = get(speedMatchFlashcards);
  const stats = get(speedMatchStats);
  const config = get(speedMatchConfig);

  const numPairs = getPairsForLevel(stats.level);

  try {
    speedMatchState.set("loading");

    const pairs = await generateSpeedMatchPairs(currentFlashcards, numPairs);

    speedMatchPairs.set(pairs);
    currentPairIndex.set(0);

    speedMatchStats.update((s) => ({
      ...s,
      correct: 0,
      wrong: 0,
      timeRemaining: config.timePerRound,
      streak: 0,
    }));

    speedMatchState.set("idle");
    startCountdownTimer();
  } catch (error) {
    console.error("[SpeedMatch] Failed to start new round:", error);
    setSpeedMatchError();
  }
}

/**
 * Answer the current pair (same or different)
 */
export async function answerSpeedMatch(userSaysMatch: boolean) {
  const currentState = get(speedMatchState);
  if (currentState !== "idle") return;

  const pairs = get(speedMatchPairs);
  const index = get(currentPairIndex);
  const pair = pairs[index];
  const config = get(speedMatchConfig);
  const stats = get(speedMatchStats);

  if (!pair) return;

  // Check if answer is correct
  const isCorrect = userSaysMatch === pair.isMatch;

  // Calculate score
  const points = calculateSpeedMatchScore(
    isCorrect,
    stats.timeRemaining,
    isCorrect ? stats.streak : 0,
    stats.level,
    config
  );

  // Update stats
  if (isCorrect) {
    speedMatchStats.update((s) => ({
      ...s,
      correct: s.correct + 1,
      score: s.score + points,
      streak: s.streak + 1,
    }));

    // Update combo
    speedMatchComboCount.update((c) => c + 1);
    const currentCombo = get(speedMatchComboCount);
    const currentMax = get(speedMatchMaxCombo);
    if (currentCombo > currentMax) {
      speedMatchMaxCombo.set(currentCombo);
    }
  } else {
    speedMatchStats.update((s) => ({
      ...s,
      wrong: s.wrong + 1,
      streak: 0,
    }));

    // Reset combo
    speedMatchComboCount.set(0);
  }

  // Show result state briefly
  speedMatchState.set("answered");

  // Move to next pair after delay
  setTimeout(() => {
    const newIndex = index + 1;
    const totalPairs = get(speedMatchPairs).length;
    const currentStats = get(speedMatchStats);

    if (newIndex >= totalPairs || currentStats.timeRemaining <= 0) {
      // Round complete
      handleRoundComplete();
    } else {
      // Next pair
      currentPairIndex.set(newIndex);
      speedMatchState.set("idle");
    }
  }, config.answerFeedbackDelay);
}

/**
 * Handle round completion
 */
async function handleRoundComplete() {
  stopTimer();
  speedMatchState.set("completed");

  // Save session to backend
  if (isUserLoggedIn() && get(speedMatchSessionId)) {
    await saveSpeedMatchSession();
  }
}

/**
 * Progress to next level
 */
export async function nextSpeedMatchLevel() {
  const stats = get(speedMatchStats);
  const newLevel = stats.level + 1;
  const newScore = stats.score;

  speedMatchStats.update((s) => ({
    ...s,
    level: newLevel,
  }));

  // Reset combo for new level
  speedMatchComboCount.set(0);
  speedMatchMaxCombo.set(0);

  // Save progress and start new session
  if (isUserLoggedIn()) {
    await saveSpeedMatchProgress(newLevel, newScore);
    await startNewSpeedMatchSession();
  }

  await startNewRound();
}

/**
 * Save current session to backend
 */
async function saveSpeedMatchSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(speedMatchSessionId);
  if (!sessionId) return null;

  const stats = get(speedMatchStats);
  const startTime = get(speedMatchSessionStartTime);
  const currentMaxCombo = get(speedMatchMaxCombo);
  const config = get(speedMatchConfig);

  const timeSpentSeconds = startTime
    ? Math.floor((Date.now() - startTime) / 1000)
    : config.timePerRound - stats.timeRemaining;

  try {
    const api = getScoringApi();
    const result = await api.completeGameSession(fetch, {
      session_id: sessionId,
      score: stats.score,
      correct_answers: stats.correct,
      wrong_answers: stats.wrong,
      combo_max: currentMaxCombo,
      time_spent_seconds: timeSpentSeconds,
    });

    speedMatchLastGameResult.set(result);

    if (result.new_achievements && result.new_achievements.length > 0) {
      speedMatchNewAchievements.set(result.new_achievements);
    }

    console.log("[SpeedMatch] Session saved:", result);
    return result;
  } catch (error) {
    console.error("[SpeedMatch] Failed to save session:", error);
    return null;
  }
}

/**
 * Start a new session for next level
 */
async function startNewSpeedMatchSession(): Promise<void> {
  speedMatchSessionStartTime.set(Date.now());

  try {
    const api = getScoringApi();
    const flashcardTypeId = get(speedMatchCurrentFlashcardTypeId);
    const sessionId = await api.startGameSession(fetch, {
      game_type_code: "speed_match",
      flashcard_type_id: flashcardTypeId,
    });
    speedMatchSessionId.set(sessionId);
    console.log("[SpeedMatch] New session started:", sessionId);
  } catch (error) {
    console.warn("[SpeedMatch] Failed to start new session:", error);
    speedMatchSessionId.set(null);
  }
}

/**
 * End game session
 */
export async function endSpeedMatchSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(speedMatchSessionId);
  if (!sessionId) {
    console.warn("No active speed match session to end");
    return null;
  }

  stopTimer();

  const result = await saveSpeedMatchSession();
  speedMatchSessionId.set(null);

  return result;
}

/**
 * Start the countdown timer (counts down from timePerRound to 0)
 */
function startCountdownTimer() {
  stopTimer();

  timerInterval = window.setInterval(() => {
    speedMatchStats.update((stats) => {
      const newTime = stats.timeRemaining - 1;

      if (newTime <= 0) {
        // Time's up — end the round
        stopTimer();
        setTimeout(() => handleRoundComplete(), 0);
        return { ...stats, timeRemaining: 0 };
      }

      return { ...stats, timeRemaining: newTime };
    });
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
 * Reset the speed match game
 */
export async function resetSpeedMatch() {
  if (get(speedMatchSessionId)) {
    await endSpeedMatchSession();
  }

  stopTimer();

  const config = get(speedMatchConfig);

  speedMatchStats.set({
    level: 1,
    correct: 0,
    wrong: 0,
    timeRemaining: config.timePerRound,
    score: 0,
    streak: 0,
  });

  speedMatchComboCount.set(0);
  speedMatchMaxCombo.set(0);
  speedMatchNewAchievements.set([]);
  speedMatchLastGameResult.set(null);
  currentPairIndex.set(0);

  // Start new session
  speedMatchSessionStartTime.set(Date.now());
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "speed_match",
      });
      speedMatchSessionId.set(sessionId);
    } catch (error) {
      console.warn("[SpeedMatch] Failed to start new session:", error);
      speedMatchSessionId.set(null);
    }
  }

  await startNewRound();
}

/**
 * Cleanup function
 */
export async function cleanupSpeedMatch() {
  stopTimer();
  if (get(speedMatchSessionId)) {
    await endSpeedMatchSession();
  }
}

/**
 * Set error state
 */
export function setSpeedMatchError() {
  stopTimer();
  speedMatchState.set("error");
}

/**
 * Clear achievements notification
 */
export function clearSpeedMatchAchievements() {
  speedMatchNewAchievements.set([]);
}

/**
 * Get saved game progress
 */
export async function getSpeedMatchProgress(): Promise<GameProgress | null> {
  if (!isUserLoggedIn()) return null;

  try {
    const api = getScoringApi();
    const progress = await api.getGameProgress(fetch, "speed_match");
    speedMatchSavedProgress.set(progress);
    return progress;
  } catch (error) {
    console.error("[SpeedMatch] Failed to get progress:", error);
    return null;
  }
}

/**
 * Save game progress
 */
async function saveSpeedMatchProgress(
  level: number,
  score: number
): Promise<void> {
  if (!isUserLoggedIn()) return;

  try {
    const api = getScoringApi();
    const progress = await api.saveGameProgress(fetch, {
      game_type_code: "speed_match",
      current_level: level,
      total_score: score,
    });
    speedMatchSavedProgress.set(progress);
    console.log("[SpeedMatch] Progress saved:", progress);
  } catch (error) {
    console.error("[SpeedMatch] Failed to save progress:", error);
  }
}

/**
 * Continue from saved progress
 */
export async function continueSpeedMatchFromProgress(
  progress: GameProgress
): Promise<void> {
  speedMatchStats.update((stats) => ({
    ...stats,
    level: progress.current_level,
    score: progress.total_score,
  }));

  speedMatchSavedProgress.set(null);
}

/**
 * Start new game (reset progress)
 */
export async function startNewSpeedMatchGame(): Promise<void> {
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      await api.resetGameProgress(fetch, "speed_match");
      console.log("[SpeedMatch] Progress reset");
    } catch (error) {
      console.error("[SpeedMatch] Failed to reset progress:", error);
    }
  }

  speedMatchSavedProgress.set(null);
}
