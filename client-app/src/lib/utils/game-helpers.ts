import type { Flashcard } from "$lib/models/flashcard";
import { type GameCard, type GameConfig, DEFAULT_GAME_CONFIG } from "$lib/models/game-card";
import { getImageBase64Url } from "$lib/helpers/imageHelper";

/**
 * Fisher-Yates shuffle algorithm
 * Randomly shuffles an array in place
 * @param array - Array to shuffle
 */
export function shuffle<T>(array: T[]): void {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
}

/**
 * Calculate the number of card pairs needed for a given level
 * Formula: ceil(((level + 2) * (level + 2)) / 2)
 * Examples:
 *   Level 1 (3x3 grid = 9 cards) -> 4 pairs (8 cards, 1 unused slot)
 *   Level 2 (4x4 grid = 16 cards) -> 8 pairs
 *   Level 3 (5x5 grid = 25 cards) -> 12 pairs (24 cards, 1 unused slot)
 *
 * @param level - Current game level (starts from 1)
 * @returns Number of pairs needed
 */
export function getPairsForLevel(level: number): number {
  const gridSize = level + 2; // Level 1 -> 3x3, Level 2 -> 4x4, etc.
  const totalSlots = gridSize * gridSize;
  return Math.floor(totalSlots / 2);
}

/**
 * Generate game cards from flashcards
 * Creates two cards for each flashcard (a matching pair)
 * Loads images from API using image_id
 *
 * @param flashcards - Array of flashcards from backend
 * @param pairsNeeded - Number of pairs to generate
 * @returns Promise<Array of shuffled game cards>
 */
export async function generateGameCards(flashcards: Flashcard[], pairsNeeded: number): Promise<GameCard[]> {
  // Select required number of flashcards
  const selectedFlashcards = flashcards.slice(0, Math.min(pairsNeeded, flashcards.length));

  // Load images for all flashcards
  const flashcardsWithImages = await Promise.all(
    selectedFlashcards.map(async (flashcard) => {
      try {
        const imageUrl = await getImageBase64Url(flashcard.image_id);
        return { ...flashcard, imageUrl };
      } catch (error) {
        console.error(`Failed to load image for flashcard ${flashcard.id}:`, error);
        return { ...flashcard, imageUrl: "" };
      }
    })
  );

  // Create two cards for each flashcard
  const gameCards: GameCard[] = flashcardsWithImages.flatMap((flashcard) => [
    {
      id: `${flashcard.id}-1`,
      flashcardId: flashcard.id,
      name: flashcard.name,
      imageUrl: flashcard.imageUrl,
      matched: false,
      flipped: false,
    },
    {
      id: `${flashcard.id}-2`,
      flashcardId: flashcard.id,
      name: flashcard.name,
      imageUrl: flashcard.imageUrl,
      matched: false,
      flipped: false,
    },
  ]);

  // Shuffle the cards
  shuffle(gameCards);

  return gameCards;
}

/**
 * Calculate score for a successful match
 * Formula: basePoints * levelMultiplier - timePenalty
 *
 * @param level - Current level
 * @param timeElapsed - Time taken in seconds
 * @param config - Game configuration
 * @returns Score for this match
 */
export function calculateScore(
  level: number,
  timeElapsed: number,
  config: GameConfig = DEFAULT_GAME_CONFIG
): number {
  const { basePointsPerMatch, timeBonusMultiplier } = config;

  // Base score increases with level
  const baseScore = basePointsPerMatch * level;

  // Time bonus: faster completion = higher bonus
  // After 30 seconds, no time bonus
  const timeBonus = Math.max(0, (30 - timeElapsed) * timeBonusMultiplier);

  return Math.floor(baseScore + timeBonus);
}

/**
 * Calculate total score for completing a level
 *
 * @param level - Completed level
 * @param moves - Total moves made
 * @param timeElapsed - Total time taken in seconds
 * @param config - Game configuration
 * @returns Total level score
 */
export function calculateLevelScore(
  level: number,
  moves: number,
  timeElapsed: number,
  config: GameConfig = DEFAULT_GAME_CONFIG
): number {
  const pairsNeeded = getPairsForLevel(level);

  // Perfect score if moves == pairs needed
  const efficiencyBonus = moves === pairsNeeded ? 500 * level : 0;

  // Level completion bonus
  const levelBonus = 1000 * level;

  // Time bonus
  const expectedTime = pairsNeeded * 5; // 5 seconds per pair expected
  const timeBonus = Math.max(0, (expectedTime - timeElapsed) * 20);

  return Math.floor(levelBonus + efficiencyBonus + timeBonus);
}

/**
 * Get image URL for a flashcard
 * Handles both relative and absolute URLs
 *
 * @param flashcard - Flashcard object
 * @param apiBaseUrl - Base URL for API (if needed)
 * @returns Complete image URL
 */
export function getFlashcardImageUrl(flashcard: Flashcard, apiBaseUrl?: string): string {
  if (!flashcard.image_url) {
    return "";
  }

  // If already absolute URL, return as is
  if (flashcard.image_url.startsWith("http")) {
    return flashcard.image_url;
  }

  // If relative URL and we have base URL, combine them
  if (apiBaseUrl) {
    return `${apiBaseUrl}${flashcard.image_url}`;
  }

  return flashcard.image_url;
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
 * Check if minimum flashcards are available
 *
 * @param flashcards - Array of flashcards
 * @param level - Target level
 * @returns True if enough flashcards available
 */
export function hasEnoughFlashcards(flashcards: Flashcard[], level: number): boolean {
  const pairsNeeded = getPairsForLevel(level);
  return flashcards.length >= pairsNeeded;
}
