import { writable, derived, get } from "svelte/store";
import type { Writable, Readable } from "svelte/store";
import type { Flashcard } from "$lib/models/flashcard";
import {
  type QuizQuestion,
  type QuizGameState,
  type QuizStats,
  type QuizConfig,
  DEFAULT_QUIZ_CONFIG,
} from "$lib/models/quiz-card";
import {
  generateQuizQuestions,
  getQuestionsForLevel,
  calculateQuizScore,
  calculateRoundBonus,
} from "$lib/helpers/quizHelpers";
import {
  ScoringApi,
  type GameCompleteResponse,
  type Achievement,
  type GameProgress,
} from "$lib/api/scoringApi";
import { ACCESS_TOKEN } from "$lib/common/contants";
import Cookies from "js-cookie";

/**
 * Quiz Game Store
 * Centralized state management for the quiz game
 */

// Game state
export const quizGameState: Writable<QuizGameState> = writable("loading");

// Available flashcards from API
export const quizFlashcards: Writable<Flashcard[]> = writable([]);

// Current quiz questions
export const quizQuestions: Writable<QuizQuestion[]> = writable([]);

// Current question index
export const currentQuestionIndex: Writable<number> = writable(0);

// Quiz statistics
export const quizStats: Writable<QuizStats> = writable({
  currentQuestion: 1,
  totalQuestions: 5,
  correctAnswers: 0,
  wrongAnswers: 0,
  timeElapsed: 0,
  score: 0,
  level: 1,
});

// Quiz configuration
export const quizConfig: Writable<QuizConfig> = writable(DEFAULT_QUIZ_CONFIG);

// Session tracking for backend
export const quizSessionId: Writable<number | null> = writable(null);
export const quizSessionStartTime: Writable<number | null> = writable(null);
export const quizComboCount: Writable<number> = writable(0);
export const quizMaxCombo: Writable<number> = writable(0);
export const quizCurrentFlashcardTypeId: Writable<number | undefined> =
  writable(undefined);

// Question timer
export const questionStartTime: Writable<number> = writable(Date.now());

// New achievements unlocked
export const quizNewAchievements: Writable<Achievement[]> = writable([]);

// Game complete response from backend
export const quizLastGameResult: Writable<GameCompleteResponse | null> =
  writable(null);

// Saved game progress from backend
export const quizSavedProgress: Writable<GameProgress | null> = writable(null);

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
export const currentQuestion: Readable<QuizQuestion | null> = derived(
  [quizQuestions, currentQuestionIndex],
  ([$questions, $index]) => {
    if ($questions.length === 0 || $index >= $questions.length) {
      return null;
    }
    return $questions[$index];
  }
);

/**
 * Derived store: Check if quiz is complete
 */
export const isQuizComplete: Readable<boolean> = derived(
  [quizQuestions, currentQuestionIndex],
  ([$questions, $index]) => {
    return $questions.length > 0 && $index >= $questions.length;
  }
);

/**
 * Derived store: Progress percentage
 */
export const quizProgress: Readable<number> = derived(
  [currentQuestionIndex, quizQuestions],
  ([$index, $questions]) => {
    if ($questions.length === 0) return 0;
    return Math.round(($index / $questions.length) * 100);
  }
);

/**
 * Initialize the quiz with flashcards
 */
export async function initializeQuiz(
  loadedFlashcards: Flashcard[],
  config?: Partial<QuizConfig> & { flashcardTypeId?: number }
) {
  quizFlashcards.set(loadedFlashcards);

  if (config) {
    quizConfig.update((current) => ({ ...current, ...config }));
  }

  // Store flashcardTypeId for later use
  quizCurrentFlashcardTypeId.set(config?.flashcardTypeId);

  // Reset stats
  quizStats.set({
    currentQuestion: 1,
    totalQuestions: getQuestionsForLevel(1),
    correctAnswers: 0,
    wrongAnswers: 0,
    timeElapsed: 0,
    score: 0,
    level: 1,
  });

  // Reset session tracking
  quizComboCount.set(0);
  quizMaxCombo.set(0);
  quizNewAchievements.set([]);
  quizLastGameResult.set(null);
  quizSessionStartTime.set(Date.now());
  currentQuestionIndex.set(0);

  // Start game session on backend (only if logged in)
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "quiz",
        flashcard_type_id: config?.flashcardTypeId,
      });
      quizSessionId.set(sessionId);
      console.log("[Quiz] Game session started:", sessionId);
    } catch (error) {
      console.error("[Quiz] Failed to start game session:", error);
      quizSessionId.set(null);
    }
  } else {
    quizSessionId.set(null);
  }

  await startNewRound();
}

/**
 * Start a new round of questions
 */
export async function startNewRound() {
  const currentFlashcards = get(quizFlashcards);
  const stats = get(quizStats);
  const config = get(quizConfig);

  const numQuestions = getQuestionsForLevel(stats.level);

  try {
    const questions = await generateQuizQuestions(
      currentFlashcards,
      numQuestions,
      config.optionsPerQuestion
    );

    quizQuestions.set(questions);
    currentQuestionIndex.set(0);

    quizStats.update((s) => ({
      ...s,
      currentQuestion: 1,
      totalQuestions: questions.length,
      correctAnswers: 0,
      wrongAnswers: 0,
      timeElapsed: 0,
    }));

    questionStartTime.set(Date.now());
    quizGameState.set("idle");
    startTimer();
  } catch (error) {
    console.error("Failed to start new round:", error);
    setQuizError();
  }
}

/**
 * Answer the current question
 */
export async function answerQuestion(selectedAnswer: string) {
  const currentState = get(quizGameState);
  if (currentState !== "idle") return;

  const questions = get(quizQuestions);
  const index = get(currentQuestionIndex);
  const question = questions[index];
  const config = get(quizConfig);
  const stats = get(quizStats);

  if (!question) return;

  // Calculate time for this question
  const questionTime = Math.floor(
    (Date.now() - get(questionStartTime)) / 1000
  );

  // Check if answer is correct
  const isCorrect = selectedAnswer === question.correctAnswer;

  // Update question state
  quizQuestions.update((qs) =>
    qs.map((q, i) =>
      i === index
        ? {
            ...q,
            answered: true,
            isCorrect,
            selectedAnswer,
          }
        : q
    )
  );

  // Update stats
  if (isCorrect) {
    const questionScore = calculateQuizScore(stats.level, questionTime, config);
    quizStats.update((s) => ({
      ...s,
      correctAnswers: s.correctAnswers + 1,
      score: s.score + questionScore,
    }));

    // Update combo
    quizComboCount.update((c) => c + 1);
    const currentCombo = get(quizComboCount);
    const currentMax = get(quizMaxCombo);
    if (currentCombo > currentMax) {
      quizMaxCombo.set(currentCombo);
    }
  } else {
    quizStats.update((s) => ({
      ...s,
      wrongAnswers: s.wrongAnswers + 1,
    }));

    // Reset combo
    quizComboCount.set(0);
  }

  // Show result state briefly
  quizGameState.set("answered");

  // Move to next question after delay
  setTimeout(() => {
    const newIndex = index + 1;
    const totalQuestions = get(quizQuestions).length;

    if (newIndex >= totalQuestions) {
      // Round complete
      handleRoundComplete();
    } else {
      // Next question
      currentQuestionIndex.set(newIndex);
      quizStats.update((s) => ({
        ...s,
        currentQuestion: newIndex + 1,
      }));
      questionStartTime.set(Date.now());
      quizGameState.set("idle");
    }
  }, config.nextQuestionDelay);
}

/**
 * Handle round completion
 */
async function handleRoundComplete() {
  stopTimer();
  quizGameState.set("completed");

  const stats = get(quizStats);
  const config = get(quizConfig);

  // Calculate round bonus
  const roundBonus = calculateRoundBonus(
    stats.level,
    stats.correctAnswers,
    stats.totalQuestions,
    stats.timeElapsed,
    config
  );

  quizStats.update((s) => ({
    ...s,
    score: s.score + roundBonus,
  }));

  // Save session to backend
  if (isUserLoggedIn() && get(quizSessionId)) {
    await saveQuizSession();
  }
}

/**
 * Progress to next level
 */
export async function nextLevel() {
  const stats = get(quizStats);
  const newLevel = stats.level + 1;
  const newScore = stats.score;

  quizStats.update((s) => ({
    ...s,
    level: newLevel,
  }));

  // Reset combo for new level
  quizComboCount.set(0);
  quizMaxCombo.set(0);

  // Save progress and start new session
  if (isUserLoggedIn()) {
    await saveQuizProgress(newLevel, newScore);
    await startNewQuizSession();
  }

  await startNewRound();
}

/**
 * Save current session to backend
 */
async function saveQuizSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(quizSessionId);
  if (!sessionId) return null;

  const stats = get(quizStats);
  const startTime = get(quizSessionStartTime);
  const currentMaxCombo = get(quizMaxCombo);

  const timeSpentSeconds = startTime
    ? Math.floor((Date.now() - startTime) / 1000)
    : stats.timeElapsed;

  try {
    const api = getScoringApi();
    const result = await api.completeGameSession(fetch, {
      session_id: sessionId,
      score: stats.score,
      correct_answers: stats.correctAnswers,
      wrong_answers: stats.wrongAnswers,
      combo_max: currentMaxCombo,
      time_spent_seconds: timeSpentSeconds,
    });

    quizLastGameResult.set(result);

    if (result.new_achievements && result.new_achievements.length > 0) {
      quizNewAchievements.set(result.new_achievements);
    }

    console.log("[Quiz] Session saved:", result);
    return result;
  } catch (error) {
    console.error("[Quiz] Failed to save session:", error);
    return null;
  }
}

/**
 * Start a new session for next level
 */
async function startNewQuizSession(): Promise<void> {
  quizSessionStartTime.set(Date.now());

  try {
    const api = getScoringApi();
    const flashcardTypeId = get(quizCurrentFlashcardTypeId);
    const sessionId = await api.startGameSession(fetch, {
      game_type_code: "quiz",
      flashcard_type_id: flashcardTypeId,
    });
    quizSessionId.set(sessionId);
    console.log("[Quiz] New session started:", sessionId);
  } catch (error) {
    console.warn("[Quiz] Failed to start new session:", error);
    quizSessionId.set(null);
  }
}

/**
 * End game session
 */
export async function endQuizSession(): Promise<GameCompleteResponse | null> {
  const sessionId = get(quizSessionId);
  if (!sessionId) {
    console.warn("No active quiz session to end");
    return null;
  }

  stopTimer();

  const result = await saveQuizSession();
  quizSessionId.set(null);

  return result;
}

/**
 * Start the timer
 */
function startTimer() {
  stopTimer();

  timerInterval = window.setInterval(() => {
    quizStats.update((stats) => ({
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
 * Reset the quiz
 */
export async function resetQuiz() {
  if (get(quizSessionId)) {
    await endQuizSession();
  }

  stopTimer();

  quizStats.set({
    currentQuestion: 1,
    totalQuestions: 5,
    correctAnswers: 0,
    wrongAnswers: 0,
    timeElapsed: 0,
    score: 0,
    level: 1,
  });

  quizComboCount.set(0);
  quizMaxCombo.set(0);
  quizNewAchievements.set([]);
  quizLastGameResult.set(null);
  currentQuestionIndex.set(0);

  // Start new session
  quizSessionStartTime.set(Date.now());
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      const sessionId = await api.startGameSession(fetch, {
        game_type_code: "quiz",
      });
      quizSessionId.set(sessionId);
    } catch (error) {
      console.warn("[Quiz] Failed to start new session:", error);
      quizSessionId.set(null);
    }
  }

  await startNewRound();
}

/**
 * Cleanup function
 */
export async function cleanupQuiz() {
  stopTimer();
  if (get(quizSessionId)) {
    await endQuizSession();
  }
}

/**
 * Set error state
 */
export function setQuizError() {
  stopTimer();
  quizGameState.set("error");
}

/**
 * Clear achievements notification
 */
export function clearQuizAchievements() {
  quizNewAchievements.set([]);
}

/**
 * Get saved game progress
 */
export async function getQuizProgress(): Promise<GameProgress | null> {
  if (!isUserLoggedIn()) return null;

  try {
    const api = getScoringApi();
    const progress = await api.getGameProgress(fetch, "quiz");
    quizSavedProgress.set(progress);
    return progress;
  } catch (error) {
    console.error("[Quiz] Failed to get progress:", error);
    return null;
  }
}

/**
 * Save game progress
 */
async function saveQuizProgress(level: number, score: number): Promise<void> {
  if (!isUserLoggedIn()) return;

  try {
    const api = getScoringApi();
    const progress = await api.saveGameProgress(fetch, {
      game_type_code: "quiz",
      current_level: level,
      total_score: score,
    });
    quizSavedProgress.set(progress);
    console.log("[Quiz] Progress saved:", progress);
  } catch (error) {
    console.error("[Quiz] Failed to save progress:", error);
  }
}

/**
 * Continue from saved progress
 */
export async function continueQuizFromProgress(
  progress: GameProgress
): Promise<void> {
  quizStats.update((stats) => ({
    ...stats,
    level: progress.current_level,
    score: progress.total_score,
  }));

  quizSavedProgress.set(null);
}

/**
 * Start new game (reset progress)
 */
export async function startNewQuizGame(): Promise<void> {
  if (isUserLoggedIn()) {
    try {
      const api = getScoringApi();
      await api.resetGameProgress(fetch, "quiz");
      console.log("[Quiz] Progress reset");
    } catch (error) {
      console.error("[Quiz] Failed to reset progress:", error);
    }
  }

  quizSavedProgress.set(null);
}
