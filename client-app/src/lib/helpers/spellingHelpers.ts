import type { Flashcard } from "$lib/models/flashcard";
import type {
  SpellingQuestion,
  SpellingConfig,
} from "$lib/models/spelling-card";
import { DEFAULT_SPELLING_CONFIG } from "$lib/models/spelling-card";
import { getImageBase64Url } from "$lib/helpers/imageHelper";
import { shuffle } from "$lib/helpers/quizHelpers";

/**
 * Get number of questions for a given level
 */
export function getQuestionsForLevel(level: number): number {
  if (level === 1) return 5;
  if (level === 2) return 7;
  return 10;
}

/**
 * Generate spelling questions from flashcards
 */
export async function generateSpellingQuestions(
  flashcards: Flashcard[],
  count: number
): Promise<SpellingQuestion[]> {
  if (flashcards.length < 1) {
    throw new Error("Need at least 1 flashcard");
  }

  const shuffled = shuffle(flashcards);
  const selected = shuffled.slice(0, Math.min(count, shuffled.length));
  const questions: SpellingQuestion[] = [];

  // Pre-load images
  const imageCache = new Map<number, string>();
  await Promise.all(
    selected.map(async (fc) => {
      try {
        const url = await getImageBase64Url(fc.image_id);
        imageCache.set(fc.id, url);
      } catch {
        imageCache.set(fc.id, "");
      }
    })
  );

  for (let i = 0; i < count; i++) {
    const fc = selected[i % selected.length];
    const imageUrl = imageCache.get(fc.id) || "";
    const correctAnswer = fc.name;
    const hint = getHint(correctAnswer, 1);

    questions.push({
      id: `spell-${i}`,
      flashcardId: fc.id,
      correctAnswer,
      imageUrl,
      hint,
    });
  }

  return shuffle(questions);
}

/**
 * Scramble letters of a word and add distractor letters
 */
export function scrambleLetters(word: string): string[] {
  const letters = word.split("");
  const distractors = getDistractorLetters(word, Math.max(2, Math.floor(word.length * 0.4)));
  const allLetters = [...letters, ...distractors];
  return shuffle(allLetters);
}

/**
 * Get random distractor letters not in the word
 */
function getDistractorLetters(word: string, count: number): string[] {
  const vietnamese = "abcdefghiklmnopqrstuvxy";
  const wordLower = word.toLowerCase();
  const available = vietnamese.split("").filter((c) => !wordLower.includes(c));
  const shuffledAvailable = shuffle(available);
  return shuffledAvailable.slice(0, Math.min(count, shuffledAvailable.length));
}

/**
 * Normalize and compare answers
 * Case-insensitive comparison, trims whitespace
 */
export function normalizeAnswer(input: string, correct: string): boolean {
  const normalizedInput = input.trim().toLowerCase();
  const normalizedCorrect = correct.trim().toLowerCase();
  return normalizedInput === normalizedCorrect;
}

/**
 * Calculate spelling score
 */
export function calculateSpellingScore(
  isCorrect: boolean,
  level: number,
  timeElapsed: number,
  hintsUsed: number,
  config: SpellingConfig = DEFAULT_SPELLING_CONFIG
): number {
  if (!isCorrect) return 0;

  const baseScore = config.basePoints * level;
  const timeBonus = Math.max(0, 30 - timeElapsed) * config.timeBonusMultiplier;
  const hintPenalty = hintsUsed * config.hintPenalty;

  return Math.max(0, baseScore + Math.floor(timeBonus) - hintPenalty);
}

/**
 * Get hint - reveals first N letters
 */
export function getHint(correctAnswer: string, revealCount: number): string {
  if (revealCount >= correctAnswer.length) {
    return correctAnswer;
  }

  const revealed = correctAnswer.slice(0, revealCount);
  const hidden = "_".repeat(correctAnswer.length - revealCount);
  return `${revealed}${hidden} (${correctAnswer.length} chữ cái)`;
}

/**
 * Get accuracy percentage
 */
export function getAccuracy(correct: number, total: number): number {
  if (total === 0) return 0;
  return Math.round((correct / total) * 100);
}

/**
 * Check if enough flashcards are available
 */
export function hasEnoughFlashcardsForSpelling(
  flashcards: Flashcard[],
  minRequired: number = 1
): boolean {
  return flashcards.length >= minRequired;
}
