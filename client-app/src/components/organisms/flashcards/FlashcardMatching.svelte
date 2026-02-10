<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { FlashcardApi } from "$lib/api/flashcardApi";
  import { ACCESS_TOKEN, GAME_FLASHCARD_TYPES } from "$lib/common/contants";
  import type { Flashcard } from "$lib/models/flashcard";
  import type { GameCard } from "$lib/models/game-card";
  import {
    gameState,
    gameCards,
    gameStats,
    initializeGame,
    flipCard as storeFlipCard,
    resetGame,
    cleanup,
    setError,
    newAchievements,
    comboCount,
    clearNewAchievements,
    getGameProgress,
    continueFromProgress,
    startNewGame,
  } from "$lib/stores/flashcard-game.store";
  import type { GameProgress } from "$lib/api/scoringApi";
  import Cookies from "js-cookie";

  // Props (Svelte 5 runes)
  interface Props {
    initialLevel?: number;
    flashcardTypeId?: number;
    onLevelComplete?: (stats: any) => void;
    onGameEnd?: (result: any) => void;
  }

  let {
    initialLevel = 1,
    flashcardTypeId = undefined,
    onLevelComplete = undefined,
    onGameEnd = undefined,
  }: Props = $props();

  // Local state (Svelte 5 runes)
  let errorMessage = $state("");
  let showContinueDialog = $state(false);
  let pendingProgress: GameProgress | null = $state(null);
  let innerWidth = $state(typeof window !== "undefined" ? window.innerWidth : 1200);

  // Cap grid columns based on screen width to prevent horizontal overflow
  let gridColumns = $derived.by(() => {
    const baseCols = $gameStats.level + 2;
    if (innerWidth <= 360) return Math.min(baseCols, 3);
    if (innerWidth <= 480) return Math.min(baseCols, 4);
    if (innerWidth <= 768) return Math.min(baseCols, 5);
    if (innerWidth <= 992) return Math.min(baseCols, 6);
    return Math.min(baseCols, 8);
  });

  /**
   * Check for saved progress and show dialog if exists
   */
  async function checkSavedProgress() {
    const progress = await getGameProgress();
    if (progress && progress.current_level > 1) {
      pendingProgress = progress;
      showContinueDialog = true;
    } else {
      await loadFlashcards(initialLevel);
    }
  }

  /**
   * Handle continue from saved progress
   */
  async function handleContinue() {
    if (pendingProgress) {
      showContinueDialog = false;
      await loadFlashcards(pendingProgress.current_level, pendingProgress.total_score);
    }
  }

  /**
   * Handle start new game
   */
  async function handleStartNew() {
    showContinueDialog = false;
    await startNewGame();
    await loadFlashcards(1);
  }

  /**
   * Load flashcards from API
   */
  async function loadFlashcards(startLevel: number = 1, startScore: number = 0) {
    try {
      const api = new FlashcardApi({
        cookies: Cookies,
        tokenKey: ACCESS_TOKEN.USER_ACCESS_TOKEN,
      });

      // Fetch flashcards with pagination to get enough for multiple levels
      const response = await api.getList(fetch, 1, 50, GAME_FLASHCARD_TYPES.MATCHING);

      // Handle different response formats
      const flashcards: Flashcard[] = response.items || response.data || [];

      if (!flashcards || flashcards.length < 3) {
        throw new Error("Cần ít nhất 3 flashcards để chơi game");
      }

      // Filter by type if specified
      const filteredFlashcards = flashcardTypeId
        ? flashcards.filter((f) => f.flashcard_type_id === flashcardTypeId)
        : flashcards;

      if (filteredFlashcards.length < 3) {
        throw new Error("Không đủ flashcards cho loại này");
      }

      // Initialize game with loaded flashcards
      await initializeGame(filteredFlashcards, {
        initialLevel: startLevel,
        flashcardTypeId,
      });

      // If continuing, restore the score
      if (startScore > 0) {
        await continueFromProgress({ current_level: startLevel, total_score: startScore } as GameProgress);
      }
    } catch (error) {
      console.error("Failed to load flashcards:", error);
      errorMessage =
        error instanceof Error ? error.message : "Không thể tải flashcards";
      setError();
    }
  }

  /**
   * Handle card click
   */
  function handleCardClick(card: GameCard) {
    storeFlipCard(card);
  }

  /**
   * Handle reset button
   */
  async function handleReset() {
    await resetGame();
  }

  // Lifecycle
  function handleResize() {
    innerWidth = window.innerWidth;
  }

  onMount(() => {
    window.addEventListener("resize", handleResize);
    checkSavedProgress();
  });

  onDestroy(() => {
    if (typeof window !== "undefined") {
      window.removeEventListener("resize", handleResize);
    }
    cleanup();
  });

  // Effect for level complete callback (Svelte 5)
  $effect(() => {
    if ($gameState === "completed" && onLevelComplete) {
      onLevelComplete($gameStats);
    }
  });
</script>

<!-- Continue Dialog -->
{#if showContinueDialog && pendingProgress}
  <div class="continue-dialog-overlay">
    <div class="continue-dialog">
      <h2><i class="fa-solid fa-gamepad"></i> Tiếp tục chơi?</h2>
      <p>Bạn đang ở <strong>Màn {pendingProgress.current_level}</strong></p>
      <p class="score-info">Điểm: <strong>{pendingProgress.total_score.toLocaleString()}</strong></p>
      <p class="highest-info">Màn cao nhất: <strong>{pendingProgress.highest_level}</strong></p>
      <div class="dialog-actions">
        <button class="btn-continue" onclick={handleContinue}>
          Tiếp tục
        </button>
        <button class="btn-new-game" onclick={handleStartNew}>
          Chơi mới
        </button>
      </div>
    </div>
  </div>
{/if}

<div class="game-container">
  {#if $gameState === "loading"}
    <!-- Loading State -->
    <div class="loading-container">
      <div class="spinner"></div>
      <p>Đang tải flashcards...</p>
    </div>
  {:else if $gameState === "error"}
    <!-- Error State -->
    <div class="error-container">
      <div class="error-icon"><i class="fa-solid fa-triangle-exclamation"></i></div>
      <h3>Không thể tải game</h3>
      <p>{errorMessage}</p>
      <button class="btn btn-primary" onclick={() => loadFlashcards()}>Thử lại</button>
    </div>
  {:else}
    <!-- Game Header with Stats -->
    <div class="game-header">
      <!-- Level Display -->
      <div class="level-display">
        <span class="level-label">Màn</span>
        <span class="level-number">{$gameStats.level}</span>
        <span class="level-grid-info"
          >({$gameStats.level + 2}x{$gameStats.level + 2})</span
        >
      </div>

      <!-- Stats Display -->
      <div class="stats-display">
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-star"></i></span>
          <span class="stat-value">{$gameStats.score}</span>
        </div>
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-fire"></i></span>
          <span class="stat-value">{$comboCount}</span>
        </div>
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-stopwatch"></i></span>
          <span class="stat-value">{$gameStats.timeElapsed}s</span>
        </div>
      </div>
    </div>

    <!-- Game Grid -->
    <div class="container-sm card-container px-0">
      <div
        class="grid"
        style="grid-template-columns: repeat({gridColumns}, minmax(0, 110px));"
      >
        {#each $gameCards as card (card.id)}
          <div
            class="flip-box card-holder {card.flipped
              ? 'flipped'
              : ''} {card.matched ? 'matched' : ''}"
          >
            <div class="flip-box-inner">
              <!-- Card Back (door with question mark) -->
              <button
                type="button"
                class="card flip-box-front {card.flipped ? 'hidden' : ''}"
                onclick={() => handleCardClick(card)}
                disabled={card.matched ||
                  card.flipped ||
                  $gameState === "checking"}
                aria-label="Flip card {card.name}"
              >
                <span class="question-mark">?</span>
              </button>

              <!-- Card Front (flashcard image) -->
              <div
                class="card-image flip-box-back {!card.flipped ? 'hidden' : ''}"
                style="background-image: url({card.imageUrl}); background-size: cover; background-position: center;"
                role="img"
                aria-label={card.name}
              ></div>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Fireworks on Level Complete -->
    {#if $gameState === "completed"}
      <div class="fireworks">
        <div class="explosion"></div>
        <div class="explosion"></div>
        <div class="explosion"></div>
      </div>
      <div class="level-complete-message">
        <h2><i class="fa-solid fa-trophy"></i> Hoàn thành Màn {$gameStats.level}!</h2>
        <p>Chuẩn bị màn tiếp theo...</p>
      </div>
    {/if}

    <!-- New Achievements Toast -->
    {#if $newAchievements.length > 0}
      <div class="achievements-toast">
        <h4><i class="fa-solid fa-trophy"></i> Thành tựu mới!</h4>
        {#each $newAchievements as achievement}
          <div class="achievement-item">
            <span class="achievement-icon">
              {#if achievement.icon}
                <i class="{achievement.icon}"></i>
              {:else}
                <i class="fa-solid fa-medal"></i>
              {/if}
            </span>
            <span class="achievement-name">{achievement.name}</span>
          </div>
        {/each}
        <button class="btn-dismiss" onclick={() => clearNewAchievements()} aria-label="Đóng">
          <i class="fa-solid fa-xmark"></i>
        </button>
      </div>
    {/if}
  {/if}
</div>

<style>
  .game-container {
    width: 100%;
    margin: 0 auto;
    padding: 30px;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.95) 0%,
      rgba(240, 249, 255, 0.95) 100%
    );
    border-radius: 24px;
    border: 3px solid transparent;
    background-clip: padding-box;
    position: relative;
    box-shadow:
      0 10px 40px rgba(59, 130, 246, 0.15),
      0 4px 12px rgba(0, 0, 0, 0.08);
  }

  /* Animated gradient border */
  .game-container::before {
    content: "";
    position: absolute;
    top: -3px;
    left: -3px;
    right: -3px;
    bottom: -3px;
    background: linear-gradient(
      45deg,
      #3b82f6,
      #f59e0b,
      #10b981,
      #a855f7,
      #ef4444,
      #3b82f6
    );
    border-radius: 24px;
    z-index: -1;
    animation: rotateBorder 6s linear infinite;
    background-size: 300% 300%;
  }

  @keyframes rotateBorder {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }

  /* Decorative corner accents */
  .game-container::after {
    content: "";
    position: absolute;
    top: 15px;
    left: 15px;
    right: 15px;
    bottom: 15px;
    border: 2px dashed rgba(59, 130, 246, 0.2);
    border-radius: 16px;
    pointer-events: none;
  }

  /* Loading State */
  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    gap: 20px;
  }

  .spinner {
    width: 50px;
    height: 50px;
    border: 5px solid #f3f3f3;
    border-top: 5px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  /* Error State */
  .error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    gap: 15px;
    text-align: center;
  }

  .error-icon {
    font-size: 64px;
  }

  .error-container h3 {
    color: #e74c3c;
    margin: 0;
  }

  .error-container p {
    color: #666;
    margin: 0;
  }

  /* Card Container */
  .card-container {
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    padding: 10px;
  }

  .grid {
    display: grid;
    gap: 6px;
    perspective: 1500px;
    padding: 0;
    justify-content: center;
  }

  .card-holder {
    width: 100%;
    min-width: 0;
    max-width: 120px;
    aspect-ratio: 3 / 4;
    position: relative;
    cursor: pointer;
    font-size: 24px;
    font-weight: bold;
    user-select: none;
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  /* Decorative frame around card */
  .card-holder::before {
    content: "";
    position: absolute;
    top: -8px;
    left: -8px;
    right: -8px;
    bottom: -8px;
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.3) 0%,
      rgba(245, 158, 11, 0.3) 50%,
      rgba(16, 185, 129, 0.3) 100%
    );
    border-radius: 20px;
    opacity: 0;
    transition: opacity 0.3s ease;
    z-index: -1;
    filter: blur(8px);
  }

  .card-holder:hover::before {
    opacity: 1;
    animation: rotateGradient 3s linear infinite;
  }

  /* Animated gradient rotation */
  @keyframes rotateGradient {
    0% {
      filter: blur(8px) hue-rotate(0deg);
    }
    100% {
      filter: blur(8px) hue-rotate(360deg);
    }
  }

  /* Floating animation for unmatched cards */
  .card-holder:not(.matched) {
    animation: gentleFloat 4s ease-in-out infinite;
  }

  .card-holder:nth-child(odd) {
    animation-delay: 0s;
  }

  .card-holder:nth-child(even) {
    animation-delay: 1s;
  }

  @keyframes gentleFloat {
    0%,
    100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-8px);
    }
  }

  .card {
    width: 100%;
    background: linear-gradient(135deg, #e74c3c 0%, #f59e0b 95%, #fbbf24 100%);
    border: 4px solid rgba(251, 191, 36, 0.3);
    height: 100%;
    padding: 0;
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    box-shadow:
      0 8px 16px rgba(251, 191, 36, 0.25),
      0 2px 4px rgba(0, 0, 0, 0.1),
      inset 0 -2px 8px rgba(0, 0, 0, 0.08);
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Decorative pattern on card back */
  .card::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-image: repeating-linear-gradient(
        45deg,
        transparent,
        transparent 35px,
        rgba(255, 255, 255, 0.05) 35px,
        rgba(255, 255, 255, 0.05) 70px
      ),
      repeating-linear-gradient(
        -45deg,
        transparent,
        transparent 35px,
        rgba(255, 255, 255, 0.05) 35px,
        rgba(255, 255, 255, 0.05) 70px
      );
    z-index: 0;
  }

  /* Question mark icon on card back */
  .question-mark {
    font-size: 4rem;
    font-weight: bold;
    color: rgba(255, 255, 255, 0.9);
    text-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    z-index: 1;
    position: relative;
  }

  /* Card image when flipped */
  .card-image {
    width: 100%;
    height: 100%;
    border-radius: 16px;
    position: absolute;
    top: 0;
    left: 0;
  }

  .card:hover:not(:disabled) {
    transform: scale(1.08) translateY(-10px);
    box-shadow:
      0 20px 40px rgba(251, 191, 36, 0.35),
      0 10px 20px rgba(0, 0, 0, 0.15),
      inset 0 -3px 12px rgba(0, 0, 0, 0.12);
    border-color: rgba(251, 191, 36, 0.5);
  }

  .card:active:not(:disabled) {
    transform: scale(1.02) translateY(-2px);
    box-shadow:
      0 5px 15px rgba(251, 191, 36, 0.25),
      inset 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  .card:disabled {
    cursor: not-allowed;
    opacity: 0.5;
    filter: grayscale(0.3);
  }

  /* Matched card style with celebration effect */
  .card-holder.matched .card {
    background: linear-gradient(135deg, #22c55e 0%, #10b981 100%);
    color: white;
    cursor: default;
    border-color: #10b981;
    box-shadow:
      0 12px 24px rgba(16, 185, 129, 0.4),
      0 0 30px rgba(16, 185, 129, 0.3),
      inset 0 1px 0 rgba(255, 255, 255, 0.3);
    animation: matchSuccess 0.6s ease-out;
  }

  .card-holder.matched::before {
    opacity: 1;
    background: radial-gradient(
      circle,
      rgba(16, 185, 129, 0.6) 0%,
      rgba(34, 197, 94, 0.4) 50%,
      transparent 100%
    );
    filter: blur(12px);
  }

  @keyframes matchSuccess {
    0% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.15) rotate(5deg);
    }
    100% {
      transform: scale(1) rotate(0deg);
    }
  }

  .card-holder.matched {
    animation: celebrateBounce 0.8s ease-out;
  }

  @keyframes celebrateBounce {
    0%,
    100% {
      transform: translateY(0);
    }
    25% {
      transform: translateY(-20px) scale(1.05);
    }
    50% {
      transform: translateY(-10px);
    }
    75% {
      transform: translateY(-15px);
    }
  }

  /* Flip Animation */
  .flip-box-inner {
    position: relative;
    width: 100%;
    height: 100%;
    text-align: center;
    transition: transform 0.7s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    transform-style: preserve-3d;
  }

  .flip-box.flipped .flip-box-inner {
    transform: rotateY(180deg);
  }

  .flip-box-front,
  .flip-box-back {
    position: absolute;
    width: 100%;
    height: 100%;
    -webkit-backface-visibility: hidden;
    backface-visibility: hidden;
    border-radius: 16px;
  }

  .flip-box-front {
    z-index: 2;
  }

  .flip-box-back {
    background-color: #fff;
    transform: rotateY(180deg);
    border: 4px solid #3b82f6;
    box-shadow:
      0 8px 16px rgba(59, 130, 246, 0.3),
      inset 0 2px 8px rgba(59, 130, 246, 0.1);
  }

  .hidden {
    display: none;
  }

  /* Game Header */
  .game-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 16px;
    margin-bottom: 20px;
  }

  /* Stats Display */
  .stats-display {
    display: flex;
    gap: 16px;
    align-items: center;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: rgba(59, 130, 246, 0.1);
    border-radius: 12px;
  }

  .stat-icon {
    font-size: 18px;
  }

  /* Icon colors */
  .stat-icon .fa-star {
    color: #fbbf24;
  }

  .stat-icon .fa-fire {
    color: #f97316;
  }

  .stat-icon .fa-stopwatch {
    color: #3b82f6;
  }

  .error-icon .fa-triangle-exclamation {
    color: #f59e0b;
  }

  .level-complete-message h2 .fa-trophy {
    color: #fbbf24;
  }

  .continue-dialog h2 .fa-gamepad {
    color: #3b82f6;
  }

  .achievements-toast h4 .fa-trophy {
    color: white;
  }

  .stat-value {
    font-size: 16px;
    font-weight: 600;
    color: #1e40af;
  }

  /* Level Display */
  .level-display {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 12px 24px;
    background: linear-gradient(
      135deg,
      var(--primary-color) 0%,
      var(--primary-hover-color) 100%
    );
    border-radius: 16px;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .level-label {
    font-size: 18px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .level-number {
    font-size: 28px;
    font-weight: 700;
    color: white;
    min-width: 40px;
    text-align: center;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 12px;
    padding: 4px 12px;
    animation: pulse 2s ease-in-out infinite;
  }

  .level-grid-info {
    font-size: 16px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.8);
    font-style: italic;
  }

  @keyframes pulse {
    0%,
    100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.05);
    }
  }

  /* Fireworks Animation */
  .fireworks {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .explosion {
    width: 50px;
    height: 50px;
    background-color: transparent;
    border-radius: 50%;
    position: absolute;
    animation: explode 1s ease-out infinite;
  }

  .explosion:nth-child(1) {
    background-color: red;
    animation-delay: 0s;
    left: 20%;
  }

  .explosion:nth-child(2) {
    background-color: yellow;
    animation-delay: 0.2s;
    left: 50%;
  }

  .explosion:nth-child(3) {
    background-color: blue;
    animation-delay: 0.4s;
    left: 80%;
  }

  @keyframes explode {
    0% {
      transform: scale(0.5);
      opacity: 1;
    }
    100% {
      transform: scale(5);
      opacity: 0;
    }
  }

  /* Level Complete Message */
  .level-complete-message {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: white;
    padding: 40px 60px;
    border-radius: 20px;
    box-shadow: 0 10px 50px rgba(0, 0, 0, 0.3);
    text-align: center;
    z-index: 1001;
    animation: fadeInScale 0.5s ease-out;
    max-width: 500px;
    width: 90%;
    box-sizing: border-box;
  }

  .level-complete-message h2 {
    margin: 0 0 10px 0;
    color: #667eea;
  }

  .level-complete-message p {
    margin: 0;
    color: #666;
  }

  @keyframes fadeInScale {
    from {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.8);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
  }

  /* Large screens - bigger cards */
  @media (min-width: 1200px) {
    .grid {
      gap: 8px;
    }

    .question-mark {
      font-size: 5rem;
    }
  }

  /* Extra large screens */
  @media (min-width: 1600px) {
    .grid {
      gap: 10px;
    }

    .question-mark {
      font-size: 5.5rem;
    }
  }

  /* Responsive - Small desktops / large tablets */
  @media (max-width: 992px) {
    .card-container {
      max-width: 100%;
    }

    .grid {
      gap: 5px;
    }

    .question-mark {
      font-size: 3.5rem;
    }
  }

  /* Responsive - Tablets */
  @media (max-width: 768px) {
    .game-container {
      padding: 20px;
    }

    .card-container {
      max-width: 100%;
      padding: 5px;
    }

    .grid {
      gap: 5px;
    }

    .question-mark {
      font-size: 3rem;
    }

    .card {
      border-radius: 12px;
      border-width: 3px;
    }

    .card-image {
      border-radius: 12px;
    }

    .level-complete-message {
      padding: 30px 36px;
    }

    .level-complete-message h2 {
      font-size: 1.4rem;
    }
  }

  @media (max-width: 480px) {
    .game-container {
      padding: 10px;
      border-radius: 16px;
    }

    .game-container::before {
      border-radius: 16px;
    }

    .game-container::after {
      top: 8px;
      left: 8px;
      right: 8px;
      bottom: 8px;
      border-radius: 12px;
    }

    .card-container {
      max-width: 100%;
      padding: 0;
    }

    .grid {
      gap: 4px;
    }

    .card-holder {
      max-width: 70px;
    }

    .question-mark {
      font-size: 2rem;
    }

    .card {
      border-radius: 10px;
      border-width: 2px;
    }

    .card-image {
      border-radius: 10px;
    }

    /* Reduce animation intensity on mobile */
    .card-holder:hover::before {
      animation: none;
    }

    .card-holder::before {
      display: none;
    }

    .card:hover:not(:disabled) {
      transform: scale(1.03) translateY(-3px);
    }

    /* Disable floating animation on mobile for performance */
    .card-holder:not(.matched) {
      animation: none;
    }

    .game-header {
      flex-direction: column;
      align-items: center;
      gap: 10px;
      margin-bottom: 15px;
    }

    .level-display {
      padding: 6px 12px;
      gap: 6px;
    }

    .level-label {
      font-size: 12px;
    }

    .level-number {
      font-size: 18px;
      padding: 2px 6px;
    }

    .level-grid-info {
      font-size: 11px;
    }

    .stats-display {
      justify-content: center;
      gap: 6px;
      flex-wrap: wrap;
    }

    .stat-item {
      padding: 4px 8px;
    }

    .stat-icon {
      font-size: 12px;
    }

    .stat-value {
      font-size: 12px;
    }

    .level-complete-message {
      padding: 24px 20px;
    }

    .level-complete-message h2 {
      font-size: 1.25rem;
    }

    .level-complete-message p {
      font-size: 0.9rem;
    }
  }

  /* Extra small screens */
  @media (max-width: 360px) {
    .game-container {
      padding: 8px;
    }

    .grid {
      gap: 3px;
    }

    .card-holder {
      max-width: 55px;
    }

    .question-mark {
      font-size: 1.5rem;
    }

    .card {
      border-radius: 8px;
      border-width: 2px;
    }

    .card-image {
      border-radius: 8px;
    }

    .level-complete-message {
      padding: 18px 16px;
      border-radius: 16px;
    }

    .level-complete-message h2 {
      font-size: 1.1rem;
    }

    .level-complete-message p {
      font-size: 0.85rem;
    }
  }

  /* Achievement Toast */
  .achievements-toast {
    position: fixed;
    top: 20px;
    right: 20px;
    background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
    padding: 16px 20px;
    padding-right: 40px;
    border-radius: 16px;
    box-shadow: 0 8px 24px rgba(245, 158, 11, 0.4);
    z-index: 1002;
    animation: slideInRight 0.5s ease-out;
  }

  .achievements-toast h4 {
    margin: 0 0 12px 0;
    color: white;
    font-size: 16px;
  }

  .achievement-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 0;
    color: white;
  }

  .achievement-icon {
    font-size: 20px;
  }

  .achievement-name {
    font-weight: 600;
  }

  .btn-dismiss {
    position: absolute;
    top: 8px;
    right: 8px;
    background: rgba(255, 255, 255, 0.3);
    border: none;
    color: white;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    cursor: pointer;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s;
  }

  .btn-dismiss:hover {
    background: rgba(255, 255, 255, 0.5);
  }

  @keyframes slideInRight {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  /* Achievement toast responsive */
  @media (max-width: 480px) {
    .achievements-toast {
      top: 10px;
      right: 10px;
      left: 10px;
      padding: 12px 15px;
      padding-right: 35px;
      border-radius: 12px;
    }

    .achievements-toast h4 {
      font-size: 14px;
      margin-bottom: 8px;
    }

    .achievement-item {
      padding: 4px 0;
    }

    .achievement-icon {
      font-size: 16px;
    }

    .achievement-name {
      font-size: 14px;
    }
  }

  /* Continue Dialog */
  .continue-dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .continue-dialog {
    background: white;
    border-radius: 24px;
    padding: 32px 40px;
    text-align: center;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    animation: slideUp 0.4s ease-out;
    max-width: 400px;
    width: 90%;
  }

  @keyframes slideUp {
    from {
      transform: translateY(30px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .continue-dialog h2 {
    margin: 0 0 16px 0;
    font-size: 24px;
    color: #1e40af;
  }

  .continue-dialog p {
    margin: 8px 0;
    color: #64748b;
    font-size: 16px;
  }

  .continue-dialog .score-info {
    color: #f59e0b;
    font-size: 18px;
  }

  .continue-dialog .highest-info {
    color: #10b981;
  }

  .dialog-actions {
    display: flex;
    gap: 12px;
    margin-top: 24px;
    justify-content: center;
  }

  .btn-continue {
    padding: 14px 28px;
    background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
    color: white;
    border: none;
    border-radius: 12px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .btn-continue:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(59, 130, 246, 0.4);
  }

  .btn-new-game {
    padding: 14px 28px;
    background: white;
    color: #64748b;
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .btn-new-game:hover {
    background: #f8fafc;
    border-color: #cbd5e1;
  }

  @media (max-width: 480px) {
    .continue-dialog {
      padding: 24px 20px;
    }

    .continue-dialog h2 {
      font-size: 20px;
    }

    .dialog-actions {
      flex-direction: column;
    }

    .btn-continue,
    .btn-new-game {
      width: 100%;
    }
  }
</style>
