/**
 * Spelling Card Model
 * Represents a spelling question in the spelling game
 */

export interface SpellingQuestion {
  id: string;
  flashcardId: number;
  correctAnswer: string;
  imageUrl: string;
  hint: string;
}

export type SpellingState =
  | "loading"
  | "idle"
  | "answered"
  | "completed"
  | "error";

export interface SpellingStats {
  currentQuestion: number;
  totalQuestions: number;
  correct: number;
  wrong: number;
  timeElapsed: number;
  score: number;
  level: number;
  hintsUsed: number;
}

export interface SpellingConfig {
  questionsPerRound: number;
  basePoints: number;
  hintPenalty: number;
  timeBonusMultiplier: number;
  answerFeedbackDelay: number;
  levelCompleteDelay: number;
}

export const DEFAULT_SPELLING_CONFIG: SpellingConfig = {
  questionsPerRound: 5,
  basePoints: 150,
  hintPenalty: 50,
  timeBonusMultiplier: 2,
  answerFeedbackDelay: 1500,
  levelCompleteDelay: 2000,
};
