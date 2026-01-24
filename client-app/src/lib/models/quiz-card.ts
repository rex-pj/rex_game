/**
 * Quiz Card Model for Flashcard Quiz Game
 * Represents a question in the quiz game
 */

/**
 * Represents a single question in the quiz
 */
export interface QuizQuestion {
  /** Unique ID for this question */
  id: string;

  /** The flashcard this question is based on */
  flashcardId: number;

  /** The correct answer (flashcard name) */
  correctAnswer: string;

  /** Image URL to display */
  imageUrl: string;

  /** All answer options (including correct answer) */
  options: string[];

  /** Whether this question has been answered */
  answered: boolean;

  /** Whether the answer was correct (null if not answered yet) */
  isCorrect: boolean | null;

  /** The user's selected answer (null if not answered yet) */
  selectedAnswer: string | null;
}

/**
 * Quiz Game State
 */
export type QuizGameState =
  | "loading" // Loading flashcards from API
  | "idle" // Ready to answer
  | "answered" // Just answered, showing result
  | "completed" // Quiz completed
  | "error"; // Error loading flashcards

/**
 * Quiz Game Statistics
 */
export interface QuizStats {
  /** Current question number (1-indexed) */
  currentQuestion: number;

  /** Total questions in this round */
  totalQuestions: number;

  /** Number of correct answers */
  correctAnswers: number;

  /** Number of wrong answers */
  wrongAnswers: number;

  /** Time elapsed in seconds */
  timeElapsed: number;

  /** Current score */
  score: number;

  /** Current level */
  level: number;
}

/**
 * Quiz Game Configuration
 */
export interface QuizConfig {
  /** Number of questions per round */
  questionsPerRound: number;

  /** Number of answer options per question */
  optionsPerQuestion: number;

  /** Time limit per question in seconds (0 = no limit) */
  timeLimitPerQuestion: number;

  /** Base points for correct answer */
  basePointsPerCorrect: number;

  /** Time bonus multiplier */
  timeBonusMultiplier: number;

  /** Delay before showing next question (ms) */
  nextQuestionDelay: number;

  /** Delay before showing results (ms) */
  resultsDelay: number;
}

/**
 * Default quiz configuration
 */
export const DEFAULT_QUIZ_CONFIG: QuizConfig = {
  questionsPerRound: 10,
  optionsPerQuestion: 4,
  timeLimitPerQuestion: 0, // No time limit
  basePointsPerCorrect: 100,
  timeBonusMultiplier: 10,
  nextQuestionDelay: 1500,
  resultsDelay: 2000,
};
