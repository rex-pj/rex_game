import { writable, derived, get } from "svelte/store";
import type { Writable, Readable } from "svelte/store";
import type { Flashcard } from "$lib/models/flashcard";
import {
  type SpellingQuestion,
  type SpellingState,
  type SpellingStats,
  type SpellingConfig,
  DEFAULT_SPELLING_CONFIG,
} from "$lib/models/spelling-card";
import {
  generateSpellingQuestions,
  getQuestionsForLevel,
  calculateSpellingScore,
  normalizeAnswer,
} from "$lib/helpers/spellingHelpers";
import {
  ScoringApi,
  type GameCompleteResponse,
  type Achievement,
  type GameProgress,
} from "$lib/api/scoringApi";
import { ACCESS_TOKEN } from "$lib/common/contants";
import Cookies from "js-cookie";

/**
 * Spelling Game Store
 * Centralized state management for the spelling game
 */

// Game state
export const spellingState: Writable<SpellingState> = writable("loading");

// Available flashcards from API
export const spellingFlashcards: Writable<Flashcard[]> = writable([]);

// Current spelling questions
export const spellingQuestions: Writable<SpellingQuestion[]> = writable([]);

// Current question index
export const currentSpellingIndex: Writable<number> = writable(0);

// Spelling statistics
export const spellingStats: Writable<SpellingStats> = writable({
  currentQuestion: 1,
  totalQuestions: 5,
  correct: 0,
  wrong: 0,
  timeElapsed: 0,
  score: 0,
  level: 1,
  hintsUsed: 0,
});

// Spelling configuration
export const spellingConfig: Writable<SpellingConfig> = writable(
  DEFAULT_SPELLING_CONFIG
);

// Session tracking for backend
export const spellingSessionId: Writable<number | null> = writable(null);
export const spellingSessionStartTime: Writable<number | null> = writable(null);
export const spellingComboCount: Writable<number> = writable(0);
export const spellingMaxCombo: Writable<number> = writable(0);
export const spellingCurrentFlashcardTypeId: Writable<number | undefined> =
  writable(undefined);

// Question timer (tracks time per question)
export const questionStartTime: Writable<number> = writable(Date.now());

// Hints revealed for current question
export const hintsRevealed: Writable<number> = writable(0);

// New achievements unlocked
export const spellingNewAchievements: Writable<Achievement[]> = writable([]);

// Game complete response from backend
export const spellingLastGameResult: Writable<GameCompleteResponse | null> =
  writable(null);

// Saved game progress from backend
export const spellingSavedProgress: Writable<GameProgress | null> =
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
 * Derived store: Current question
 */
export const currentSpellingQuestion: Readable<SpellingQuestion | null> =
  derived(
    [spellingQuestions, currentSpellingIndex],
    ([$questions, $index]) => {
      if ($questions.length === 0 || $index >= $questions.length) {
        return null;
      }
      return $questions[$index];
    }
  );

/**
 * Derived store: Check if spelling is complete
 */
export const isSpellingComplete: Readable<boolean> = derived(
  [spellingQuestions, currentSpellingIndex],
  ([$questions, $index]) => {
    return $questions.length > 0 && $index >= $questions.length;
  }
);

/**
 * Derived store: Progress percentage
 */
export const spellingProgress: Readable<number> = derived(
  [currentSpellingIndex, spellingQuestions],
  ([$index, $questions]) => {
    if ($questions.length === 0) return 0;
    return Math.round(($index / $questions.length) * 100);
  }
);

/**
 * Initialize the spelling game with flashcards
 */
export async function initializeSpelling(
  loadedFlashcards: Flashcard[],
  config?: Partial<SpellingConfig> & { flashcardTypeId?: number }
) {
  spellingFlashcards.set(loadedFlashcards);

  if (config) {
    spellingConfig.update((current) => ({ ...current, ...config }));
  }

  // Store flashcardTypeId for later use
  spellingCurrentFlashcardTypeId.set(config?.flashcardTypeId);

  // Reset stats
  spellingStats.set({
    currentQuestion: 1,
    totalQuestions: getQuestionsForLevel(1),
    correct: 0,
    wrong: 0,
    timeElapsed: 0,
    score: 0,
    level: 1,
    hintsUsed: 0,
  });

  // Reset session tracking
  spellingComboCount.set(0);
  spellingMaxCombo.set(0);
  spellingNewAchievements.set([]);
  spellingLastGameResult.set(null);
  spellingSessionStartTime.set(Date.now());
  currentSpellingIndex.set(0);
  hintsRevealed.set(0);

  // Start game session on backend (only if logged in)
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "spelling",
        flashcard_type_id: config?.flashcardTypeId,
      });
      spellingSessionId.set(sessionId);
      console.log("[Spelling] Game session started:", sessionId);
    } catch (error) {
      console.error("[Spelling] Failed to start game session:", error);
      spellingSessionId.set(null);
    }
  } else {
    spellingSessionId.set(null);
  }

  await startNewRound();
}

/**
 * Start a new round of questions
 */
export async function startNewRound() {
  const currentFlashcards = get(spellingFlashcards);
  const stats = get(spellingStats);

  const numQuestions = getQuestionsForLevel(stats.level);

  try {
    spellingState.set("loading");

    const questions = await generateSpellingQuestions(
      currentFlashcards,
      numQuestions
    );

    spellingQuestions.set(questions);
    currentSpellingIndex.set(0);

    spellingStats.update((s) => ({
      ...s,
      currentQuestion: 1,
      totalQuestions: questions.length,
      correct: 0,
      wrong: 0,
      timeElapsed: 0,
      hintsUsed: 0,
    }));

    hintsRevealed.set(0);
    questionStartTime.set(Date.now());
    spellingState.set("idle");
    startTimer();
  } catch (error) {
    console.error("[Spelling] Failed to start new round:", error);
    setSpellingError();
  }
}

/**
 * Submit an answer for the current question
 */
export async function submitSpellingAnswer(input: string) {
  const currentState = get(spellingState);
  if (currentState !== "idle") return;

  const questions = get(spellingQuestions);
  const index = get(currentSpellingIndex);
  const question = questions[index];
  const config = get(spellingConfig);
  const stats = get(spellingStats);

  if (!question) return;

  // Calculate time for this question
  const questionTime = Math.floor(
    (Date.now() - get(questionStartTime)) / 1000
  );

  // Check if answer is correct
  const isCorrect = normalizeAnswer(input, question.correctAnswer);

  // Calculate score
  const currentHints = get(hintsRevealed);
  const points = calculateSpellingScore(
    isCorrect,
    stats.level,
    questionTime,
    currentHints,
    config
  );

  // Update stats
  if (isCorrect) {
    spellingStats.update((s) => ({
      ...s,
      correct: s.correct + 1,
      score: s.score + points,
    }));

    // Update combo
    spellingComboCount.update((c) => c + 1);
    const currentCombo = get(spellingComboCount);
    const currentMax = get(spellingMaxCombo);
    if (currentCombo > currentMax) {
      spellingMaxCombo.set(currentCombo);
    }
  } else {
    spellingStats.update((s) => ({
      ...s,
      wrong: s.wrong + 1,
    }));

    // Reset combo
    spellingComboCount.set(0);
  }

  // Show result state briefly
  spellingState.set("answered");

  // Move to next question after delay
  setTimeout(() => {
    const newIndex = index + 1;
    const totalQuestions = get(spellingQuestions).length;

    if (newIndex >= totalQuestions) {
      // Round complete
      handleRoundComplete();
    } else {
      // Next question
      currentSpellingIndex.set(newIndex);
      spellingStats.update((s) => ({
        ...s,
        currentQuestion: newIndex + 1,
      }));
      hintsRevealed.set(0);
      questionStartTime.set(Date.now());
      spellingState.set("idle");
    }
  }, config.answerFeedbackDelay);
}

/**
 * Use a hint for the current question
 */
export function useHint() {
  const question = get(currentSpellingQuestion);
  if (!question) return;

  const currentRevealed = get(hintsRevealed);
  if (currentRevealed >= question.correctAnswer.length - 1) return;

  hintsRevealed.update((h) => h + 1);
  spellingStats.update((s) => ({
    ...s,
    hintsUsed: s.hintsUsed + 1,
  }));
}

/**
 * Handle round completion
 */
async function handleRoundComplete() {
  stopTimer();
  spellingState.set("completed");

  // Save session to backend
  if (isUserLoggedIn() && get(spellingSessionId)) {
    await saveSpellingSession();
  }
}

/**
 * Progress to next level
 */
export async function nextSpellingLevel() {
  const stats = get(spellingStats);
  const newLevel = stats.level + 1;
  const newScore = stats.score;

  spellingStats.update((s) => ({
    ...s,
    level: newLevel,
  }));

  // Reset combo for new level
  spellingComboCount.set(0);
  spellingMaxCombo.set(0);

  // Save progress and start new session
  if (isUserLoggedIn()) {
    await saveSpellingProgress(newLevel, newScore);
    await startNewSpellingSession();
  }

  await startNewRound();
}

/**
 * Save current session to backend
 */
async function saveSpellingSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(spellingSessionId);
  if (!sessionId) return null;

  const stats = get(spellingStats);
  const startTime = get(spellingSessionStartTime);
  const currentMaxCombo = get(spellingMaxCombo);

  const timeSpentSeconds = startTime
    ? Math.floor((Date.now() - startTime) / 1000)
    : stats.timeElapsed;

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

    spellingLastGameResult.set(result);

    if (result.new_achievements && result.new_achievements.length > 0) {
      spellingNewAchievements.set(result.new_achievements);
    }

    console.log("[Spelling] Session saved:", result);
    return result;
  } catch (error) {
    console.error("[Spelling] Failed to save session:", error);
    return null;
  }
}

/**
 * Start a new session for next level
 */
async function startNewSpellingSession(): Promise<void> {
  spellingSessionStartTime.set(Date.now());

  try {
    const api = getScoringApi();
    const flashcardTypeId = get(spellingCurrentFlashcardTypeId);
    const sessionId = await api.startGameSession(fetch, {
      game_type_code: "spelling",
      flashcard_type_id: flashcardTypeId,
    });
    spellingSessionId.set(sessionId);
    console.log("[Spelling] New session started:", sessionId);
  } catch (error) {
    console.warn("[Spelling] Failed to start new session:", error);
    spellingSessionId.set(null);
  }
}

/**
 * End game session
 */
export async function endSpellingSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(spellingSessionId);
  if (!sessionId) {
    console.warn("No active spelling session to end");
    return null;
  }

  stopTimer();

  const result = await saveSpellingSession();
  spellingSessionId.set(null);

  return result;
}

/**
 * Start the timer (counts up)
 */
function startTimer() {
  stopTimer();

  timerInterval = window.setInterval(() => {
    spellingStats.update((stats) => ({
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
 * Reset the spelling game
 */
export async function resetSpelling() {
  if (get(spellingSessionId)) {
    await endSpellingSession();
  }

  stopTimer();

  spellingStats.set({
    currentQuestion: 1,
    totalQuestions: 5,
    correct: 0,
    wrong: 0,
    timeElapsed: 0,
    score: 0,
    level: 1,
    hintsUsed: 0,
  });

  spellingComboCount.set(0);
  spellingMaxCombo.set(0);
  spellingNewAchievements.set([]);
  spellingLastGameResult.set(null);
  currentSpellingIndex.set(0);
  hintsRevealed.set(0);

  // Start new session
  spellingSessionStartTime.set(Date.now());
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "spelling",
      });
      spellingSessionId.set(sessionId);
    } catch (error) {
      console.warn("[Spelling] Failed to start new session:", error);
      spellingSessionId.set(null);
    }
  }

  await startNewRound();
}

/**
 * Cleanup function
 */
export async function cleanupSpelling() {
  stopTimer();
  if (get(spellingSessionId)) {
    await endSpellingSession();
  }
}

/**
 * Set error state
 */
export function setSpellingError() {
  stopTimer();
  spellingState.set("error");
}

/**
 * Clear achievements notification
 */
export function clearSpellingAchievements() {
  spellingNewAchievements.set([]);
}

/**
 * Get saved game progress
 */
export async function getSpellingProgress(): Promise<GameProgress | null> {
  if (!isUserLoggedIn()) return null;

  try {
    const api = getScoringApi();
    const progress = await api.getGameProgress(fetch, "spelling");
    spellingSavedProgress.set(progress);
    return progress;
  } catch (error) {
    console.error("[Spelling] Failed to get progress:", error);
    return null;
  }
}

/**
 * Save game progress
 */
async function saveSpellingProgress(
  level: number,
  score: number
): Promise<void> {
  if (!isUserLoggedIn()) return;

  try {
    const api = getScoringApi();
    const progress = await api.saveGameProgress(fetch, {
      game_type_code: "spelling",
      current_level: level,
      total_score: score,
    });
    spellingSavedProgress.set(progress);
    console.log("[Spelling] Progress saved:", progress);
  } catch (error) {
    console.error("[Spelling] Failed to save progress:", error);
  }
}

/**
 * Continue from saved progress
 */
export async function continueSpellingFromProgress(
  progress: GameProgress
): Promise<void> {
  spellingStats.update((stats) => ({
    ...stats,
    level: progress.current_level,
    score: progress.total_score,
  }));

  spellingSavedProgress.set(null);
}

/**
 * Start new game (reset progress)
 */
export async function startNewSpellingGame(): Promise<void> {
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      await api.resetGameProgress(fetch, "spelling");
      console.log("[Spelling] Progress reset");
    } catch (error) {
      console.error("[Spelling] Failed to reset progress:", error);
    }
  }

  spellingSavedProgress.set(null);
}
