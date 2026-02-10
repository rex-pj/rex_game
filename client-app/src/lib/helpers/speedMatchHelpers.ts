import type { Flashcard } from "$lib/models/flashcard";
import type {
  SpeedMatchPair,
  SpeedMatchConfig,
} from "$lib/models/speed-match-card";
import { DEFAULT_SPEED_MATCH_CONFIG } from "$lib/models/speed-match-card";
import { getImageBase64Url } from "$lib/helpers/imageHelper";
import { shuffle } from "$lib/helpers/quizHelpers";

/**
 * Get number of pairs for a given level
 */
export function getPairsForLevel(level: number): number {
  if (level === 1) return 15;
  if (level === 2) return 20;
  return 25;
}

/**
 * Generate speed match pairs from flashcards
 * Creates a mix of matching (same image both sides) and non-matching pairs
 */
export async function generateSpeedMatchPairs(
  flashcards: Flashcard[],
  count: number
): Promise<SpeedMatchPair[]> {
  if (flashcards.length < 2) {
    throw new Error("Need at least 2 flashcards");
  }

  const shuffled = shuffle(flashcards);
  const pairs: SpeedMatchPair[] = [];

  // Pre-load images for all flashcards we'll use
  const imageCache = new Map<number, string>();
  const toLoad = shuffled.slice(0, Math.min(count, shuffled.length));
  await Promise.all(
    toLoad.map(async (fc) => {
      try {
        const url = await getImageBase64Url(fc.image_id);
        imageCache.set(fc.id, url);
      } catch {
        imageCache.set(fc.id, "");
      }
    })
  );

  for (let i = 0; i < count; i++) {
    const isMatch = Math.random() < 0.5;
    const leftIndex = i % shuffled.length;
    const leftFc = shuffled[leftIndex];
    const leftImageUrl = imageCache.get(leftFc.id) || "";

    if (isMatch) {
      pairs.push({
        id: `sp-${i}`,
        leftFlashcardId: leftFc.id,
        rightFlashcardId: leftFc.id,
        leftImageUrl,
        rightImageUrl: leftImageUrl,
        leftName: leftFc.name,
        rightName: leftFc.name,
        isMatch: true,
      });
    } else {
      // Pick a different flashcard for the right side
      let rightIndex = (leftIndex + 1 + Math.floor(Math.random() * (shuffled.length - 1))) % shuffled.length;
      if (rightIndex === leftIndex) rightIndex = (leftIndex + 1) % shuffled.length;
      const rightFc = shuffled[rightIndex];
      const rightImageUrl = imageCache.get(rightFc.id) || leftImageUrl;

      pairs.push({
        id: `sp-${i}`,
        leftFlashcardId: leftFc.id,
        rightFlashcardId: rightFc.id,
        leftImageUrl,
        rightImageUrl,
        leftName: leftFc.name,
        rightName: rightFc.name,
        isMatch: false,
      });
    }
  }

  return shuffle(pairs);
}

/**
 * Calculate score for a speed match answer
 */
export function calculateSpeedMatchScore(
  isCorrect: boolean,
  timeRemaining: number,
  streak: number,
  level: number,
  config: SpeedMatchConfig = DEFAULT_SPEED_MATCH_CONFIG
): number {
  if (!isCorrect) return 0;

  const baseScore = config.basePoints * level;
  const timeBonus = Math.floor(timeRemaining * 2);
  const streakBonus = streak * config.streakMultiplier;

  return baseScore + timeBonus + streakBonus;
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
export function hasEnoughFlashcardsForSpeedMatch(
  flashcards: Flashcard[],
  minRequired: number = 3
): boolean {
  return flashcards.length >= minRequired;
}
