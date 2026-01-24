import type { Flashcard } from "$lib/models/flashcard";
import {
  type QuizQuestion,
  type QuizConfig,
  DEFAULT_QUIZ_CONFIG,
} from "$lib/models/quiz-card";
import { getImageBase64Url } from "$lib/helpers/imageHelper";

/**
 * Fisher-Yates shuffle algorithm
 * Randomly shuffles an array in place
 */
export function shuffle<T>(array: T[]): T[] {
  const result = [...array];
  for (let i = result.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [result[i], result[j]] = [result[j], result[i]];
  }
  return result;
}

/**
 * Get number of questions for a given level
 * Level 1: 5 questions
 * Level 2: 7 questions
 * Level 3+: 10 questions
 */
export function getQuestionsForLevel(level: number): number {
  if (level === 1) return 5;
  if (level === 2) return 7;
  return 10;
}

/**
 * Generate quiz questions from flashcards
 * Each question shows an image and asks for the correct name
 *
 * @param flashcards - Array of flashcards from backend
 * @param numQuestions - Number of questions to generate
 * @param optionsCount - Number of options per question (default: 4)
 * @returns Promise<Array of quiz questions>
 */
export async function generateQuizQuestions(
  flashcards: Flashcard[],
  numQuestions: number,
  optionsCount: number = 4
): Promise<QuizQuestion[]> {
  // Shuffle and select flashcards for questions
  const shuffledFlashcards = shuffle(flashcards);
  const selectedFlashcards = shuffledFlashcards.slice(
    0,
    Math.min(numQuestions, shuffledFlashcards.length)
  );

  // Get all unique names for wrong answers
  const allNames = flashcards.map((f) => f.name);

  // Load images and generate questions
  const questions = await Promise.all(
    selectedFlashcards.map(async (flashcard, index) => {
      // Load image
      let imageUrl = "";
      try {
        imageUrl = await getImageBase64Url(flashcard.image_id);
      } catch (error) {
        console.error(
          `Failed to load image for flashcard ${flashcard.id}:`,
          error
        );
      }

      // Generate wrong answers (different from correct answer)
      const wrongAnswers = shuffle(
        allNames.filter((name) => name !== flashcard.name)
      ).slice(0, optionsCount - 1);

      // Combine and shuffle options
      const options = shuffle([flashcard.name, ...wrongAnswers]);

      return {
        id: `q-${flashcard.id}-${index}`,
        flashcardId: flashcard.id,
        correctAnswer: flashcard.name,
        imageUrl,
        options,
        answered: false,
        isCorrect: null,
        selectedAnswer: null,
      };
    })
  );

  return questions;
}

/**
 * Calculate score for a correct answer
 * Formula: basePoints * levelMultiplier + timeBonus
 *
 * @param level - Current level
 * @param timeElapsed - Time taken for this question in seconds
 * @param config - Quiz configuration
 * @returns Score for this answer
 */
export function calculateQuizScore(
  level: number,
  timeElapsed: number,
  config: QuizConfig = DEFAULT_QUIZ_CONFIG
): number {
  const { basePointsPerCorrect, timeBonusMultiplier } = config;

  // Base score increases with level
  const baseScore = basePointsPerCorrect * level;

  // Time bonus: faster answers = higher bonus
  // Max bonus at 0 seconds, no bonus after 10 seconds
  const timeBonus = Math.max(0, (10 - timeElapsed) * timeBonusMultiplier);

  return Math.floor(baseScore + timeBonus);
}

/**
 * Calculate bonus score for completing a round
 *
 * @param level - Completed level
 * @param correctAnswers - Number of correct answers
 * @param totalQuestions - Total questions in round
 * @param timeElapsed - Total time taken in seconds
 * @param config - Quiz configuration
 * @returns Bonus score
 */
export function calculateRoundBonus(
  level: number,
  correctAnswers: number,
  totalQuestions: number,
  timeElapsed: number,
  config: QuizConfig = DEFAULT_QUIZ_CONFIG
): number {
  // Accuracy bonus
  const accuracy = correctAnswers / totalQuestions;
  const accuracyBonus = Math.floor(accuracy * 500 * level);

  // Perfect bonus (all correct)
  const perfectBonus = correctAnswers === totalQuestions ? 1000 * level : 0;

  // Time bonus
  const expectedTime = totalQuestions * 5; // 5 seconds per question expected
  const timeBonus = Math.max(0, (expectedTime - timeElapsed) * 20);

  return Math.floor(accuracyBonus + perfectBonus + timeBonus);
}

/**
 * Format time in MM:SS format
 *
 * @param seconds - Time in seconds
 * @returns Formatted time string
 */
export function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
}

/**
 * Check if enough flashcards are available for quiz
 *
 * @param flashcards - Array of flashcards
 * @param minRequired - Minimum flashcards required (default: 4 for options)
 * @returns True if enough flashcards available
 */
export function hasEnoughFlashcardsForQuiz(
  flashcards: Flashcard[],
  minRequired: number = 4
): boolean {
  return flashcards.length >= minRequired;
}

/**
 * Get accuracy percentage
 *
 * @param correct - Number of correct answers
 * @param total - Total questions answered
 * @returns Accuracy percentage (0-100)
 */
export function getAccuracy(correct: number, total: number): number {
  if (total === 0) return 0;
  return Math.round((correct / total) * 100);
}
