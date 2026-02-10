/**
 * Speed Match Card Model
 * Represents a pair comparison in the speed match game
 */

export interface SpeedMatchPair {
  id: string;
  leftFlashcardId: number;
  rightFlashcardId: number;
  leftImageUrl: string;
  rightImageUrl: string;
  leftName: string;
  rightName: string;
  isMatch: boolean;
}

export type SpeedMatchState =
  | "loading"
  | "idle"
  | "answered"
  | "completed"
  | "error";

export interface SpeedMatchStats {
  level: number;
  correct: number;
  wrong: number;
  timeRemaining: number;
  score: number;
  streak: number;
}

export interface SpeedMatchConfig {
  timePerRound: number;
  pairsPerRound: number;
  basePoints: number;
  streakMultiplier: number;
  answerFeedbackDelay: number;
  levelCompleteDelay: number;
}

export const DEFAULT_SPEED_MATCH_CONFIG: SpeedMatchConfig = {
  timePerRound: 60,
  pairsPerRound: 15,
  basePoints: 100,
  streakMultiplier: 10,
  answerFeedbackDelay: 600,
  levelCompleteDelay: 2000,
};
