<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { FlashcardApi } from "$lib/api/flashcardApi";
  import { ACCESS_TOKEN, GAME_FLASHCARD_TYPES } from "$lib/common/contants";
  import type { Flashcard } from "$lib/models/flashcard";
  import {
    speedMatchState,
    speedMatchStats,
    speedMatchPairs,
    currentPair,
    currentPairIndex,
    speedMatchNewAchievements,
    speedMatchProgress,
    initializeSpeedMatch,
    answerSpeedMatch,
    resetSpeedMatch,
    cleanupSpeedMatch,
    setSpeedMatchError,
    clearSpeedMatchAchievements,
    getSpeedMatchProgress,
    continueSpeedMatchFromProgress,
    startNewSpeedMatchGame,
    nextSpeedMatchLevel,
  } from "$lib/stores/speed-match-game.store";
  import type { GameProgress } from "$lib/api/scoringApi";
  import { getAccuracy } from "$lib/helpers/speedMatchHelpers";
  import Cookies from "js-cookie";

  // Props
  interface Props {
    initialLevel?: number;
    flashcardTypeId?: number;
  }

  let { initialLevel = 1, flashcardTypeId = undefined }: Props = $props();

  // Local state
  let errorMessage = $state("");
  let showContinueDialog = $state(false);
  let pendingProgress: GameProgress | null = $state(null);
  let lastAnswerCorrect: boolean | null = $state(null);

  /**
   * Check for saved progress
   */
  async function checkSavedProgress() {
    const progress = await getSpeedMatchProgress();
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
      await loadFlashcards(
        pendingProgress.current_level,
        pendingProgress.total_score,
      );
    }
  }

  /**
   * Handle start new game
   */
  async function handleStartNew() {
    showContinueDialog = false;
    await startNewSpeedMatchGame();
    await loadFlashcards(1);
  }

  /**
   * Load flashcards from API
   */
  async function loadFlashcards(
    startLevel: number = 1,
    startScore: number = 0,
  ) {
    try {
      const api = new FlashcardApi({
        cookies: Cookies,
        tokenKey: ACCESS_TOKEN.USER_ACCESS_TOKEN,
      });

      const response = await api.getList(
        fetch,
        1,
        50,
        GAME_FLASHCARD_TYPES.SPEED_MATCH,
      );
      const flashcards: Flashcard[] = response.items || response.data || [];

      if (!flashcards || flashcards.length < 2) {
        throw new Error("Cần ít nhất 2 flashcards để chơi Speed Match");
      }

      const filteredFlashcards = flashcardTypeId
        ? flashcards.filter((f) => f.flashcard_type_id === flashcardTypeId)
        : flashcards;

      if (filteredFlashcards.length < 2) {
        throw new Error("Không đủ flashcards cho loại này");
      }

      await initializeSpeedMatch(filteredFlashcards, {
        flashcardTypeId,
      });

      if (startScore > 0) {
        await continueSpeedMatchFromProgress({
          current_level: startLevel,
          total_score: startScore,
        } as GameProgress);
      }
    } catch (error) {
      console.error("Failed to load flashcards:", error);
      errorMessage =
        error instanceof Error ? error.message : "Không thể tải flashcards";
      setSpeedMatchError();
    }
  }

  /**
   * Handle answer (same or different)
   */
  function handleAnswer(userSaysMatch: boolean) {
    if ($speedMatchState !== "idle") return;

    const pair = $currentPair;
    if (!pair) return;

    const isCorrect = userSaysMatch === pair.isMatch;
    lastAnswerCorrect = isCorrect;

    answerSpeedMatch(userSaysMatch);

    // Clear feedback after delay
    setTimeout(() => {
      lastAnswerCorrect = null;
    }, 600);
  }

  /**
   * Handle next level
   */
  async function handleNextLevel() {
    await nextSpeedMatchLevel();
  }

  /**
   * Handle reset
   */
  async function handleReset() {
    await resetSpeedMatch();
  }

  /**
   * Format time for display
   */
  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  // Lifecycle
  onMount(() => {
    checkSavedProgress();
  });

  onDestroy(() => {
    cleanupSpeedMatch();
  });
</script>

<!-- Continue Dialog -->
{#if showContinueDialog && pendingProgress}
  <div class="continue-dialog-overlay">
    <div class="continue-dialog">
      <h2><i class="fa-solid fa-clipboard-question"></i> Tiếp tục chơi?</h2>
      <p>Bạn đang ở <strong>Màn {pendingProgress.current_level}</strong></p>
      <p class="score-info">
        Điểm: <strong>{pendingProgress.total_score.toLocaleString()}</strong>
      </p>
      <p class="highest-info">
        Màn cao nhất: <strong>{pendingProgress.highest_level}</strong>
      </p>
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

<div class="speed-match-container">
  {#if $speedMatchState === "loading"}
    <!-- Loading State -->
    <div class="loading-container">
      <div class="spinner"></div>
      <p>Đang tải trò chơi...</p>
    </div>
  {:else if $speedMatchState === "error"}
    <!-- Error State -->
    <div class="error-container">
      <div class="error-icon">
        <i class="fa-solid fa-triangle-exclamation"></i>
      </div>
      <h3>Không thể tải game</h3>
      <p>{errorMessage}</p>
      <button class="btn btn-primary" onclick={() => loadFlashcards()}
        >Thử lại</button
      >
    </div>
  {:else if $speedMatchState === "completed"}
    <!-- Results State -->
    <div class="results-container">
      <div class="results-card">
        <h2>
          <i class="fa-solid fa-trophy"></i> Hoàn thành Màn {$speedMatchStats.level}!
        </h2>

        <div class="results-stats">
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-star"></i></span>
            <span class="result-label">Điểm</span>
            <span class="result-value"
              >{$speedMatchStats.score.toLocaleString()}</span
            >
          </div>
          <div class="result-item">
            <span class="result-icon"
              ><i class="fa-solid fa-circle-check"></i></span
            >
            <span class="result-label">Đúng</span>
            <span class="result-value correct"
              >{$speedMatchStats.correct}/{$speedMatchStats.correct +
                $speedMatchStats.wrong}</span
            >
          </div>
          <div class="result-item">
            <span class="result-icon"
              ><i class="fa-solid fa-chart-simple"></i></span
            >
            <span class="result-label">Độ chính xác</span>
            <span class="result-value"
              >{getAccuracy(
                $speedMatchStats.correct,
                $speedMatchStats.correct + $speedMatchStats.wrong,
              )}%</span
            >
          </div>
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-fire"></i></span>
            <span class="result-label">Streak cao nhất</span>
            <span class="result-value streak">{$speedMatchStats.streak}</span>
          </div>
        </div>

        <div class="results-actions">
          <button class="btn-next-level" onclick={handleNextLevel}>
            Màn tiếp theo →
          </button>
          <button class="btn-restart" onclick={handleReset}> Chơi lại </button>
        </div>
      </div>
    </div>
  {:else}
    <!-- Game Header -->
    <div class="game-header">
      <div class="level-display">
        <span class="level-label">Màn</span>
        <span class="level-number">{$speedMatchStats.level}</span>
      </div>

      <div
        class="timer-display"
        class:timer-warning={$speedMatchStats.timeRemaining <= 10}
      >
        <span class="timer-icon"><i class="fa-solid fa-clock"></i></span>
        <span class="timer-value"
          >{formatTime($speedMatchStats.timeRemaining)}</span
        >
      </div>

      <div class="stats-display">
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-star"></i></span>
          <span class="stat-value">{$speedMatchStats.score}</span>
        </div>
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-fire"></i></span>
          <span class="stat-value">{$speedMatchStats.streak}</span>
        </div>
      </div>
    </div>

    <!-- Progress Bar -->
    <div class="progress-section">
      <span class="progress-text">
        {$currentPairIndex + 1}/{$speedMatchPairs.length}
      </span>
      <div class="progress-bar">
        <div class="progress-fill" style="width: {$speedMatchProgress}%"></div>
      </div>
    </div>

    <!-- Card Comparison Area -->
    {#if $currentPair}
      <div
        class="comparison-area"
        class:feedback-correct={lastAnswerCorrect === true}
        class:feedback-incorrect={lastAnswerCorrect === false}
      >
        <div class="card-pair">
          <!-- Left Card -->
          <div class="match-card">
            <div
              class="card-image"
              style="background-image: url({$currentPair.leftImageUrl});"
            ></div>
            <div class="card-label">{$currentPair.leftName}</div>
          </div>

          <!-- VS Divider -->
          <div class="vs-divider">
            <span>VS</span>
          </div>

          <!-- Right Card -->
          <div class="match-card">
            <div
              class="card-image"
              style="background-image: url({$currentPair.rightImageUrl});"
            ></div>
            <div class="card-label">{$currentPair.rightName}</div>
          </div>
        </div>

        <!-- Answer Buttons -->
        <div class="answer-buttons">
          <button
            class="btn-same"
            onclick={() => handleAnswer(true)}
            disabled={$speedMatchState !== "idle"}
          >
            <i class="fa-solid fa-equals"></i>
            Giống nhau
          </button>
          <button
            class="btn-different"
            onclick={() => handleAnswer(false)}
            disabled={$speedMatchState !== "idle"}
          >
            <i class="fa-solid fa-not-equal"></i>
            Khác nhau
          </button>
        </div>

        <!-- Feedback Indicator -->
        {#if lastAnswerCorrect !== null}
          <div
            class="answer-feedback {lastAnswerCorrect
              ? 'correct'
              : 'incorrect'}"
          >
            {#if lastAnswerCorrect}
              <i class="fa-solid fa-circle-check"></i> Chính xác!
            {:else}
              <i class="fa-solid fa-circle-xmark"></i> Sai rồi!
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  {/if}

  <!-- Achievement Toast -->
  {#if $speedMatchNewAchievements.length > 0}
    <div class="achievements-toast">
      <h4><i class="fa-solid fa-trophy"></i> Thành tựu mới!</h4>
      {#each $speedMatchNewAchievements as achievement}
        <div class="achievement-item">
          <span class="achievement-icon">
            {#if achievement.icon}
              <i class={achievement.icon}></i>
            {:else}
              <i class="fa-solid fa-medal"></i>
            {/if}
          </span>
          <span class="achievement-name">{achievement.name}</span>
        </div>
      {/each}
      <button
        class="btn-dismiss"
        onclick={() => clearSpeedMatchAchievements()}
        aria-label="Đóng"><i class="fa-solid fa-xmark"></i></button
      >
    </div>
  {/if}
</div>

<style>
  .speed-match-container {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 30px;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.95) 0%,
      rgba(240, 249, 255, 0.95) 100%
    );
    border-radius: 24px;
    position: relative;
    box-shadow:
      0 10px 40px rgba(59, 130, 246, 0.15),
      0 4px 12px rgba(0, 0, 0, 0.08);
  }

  .speed-match-container::before {
    content: "";
    position: absolute;
    top: -3px;
    left: -3px;
    right: -3px;
    bottom: -3px;
    background: linear-gradient(
      45deg,
      #f97316,
      #ef4444,
      #8b5cf6,
      #3b82f6,
      #f97316
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

  /* Loading */
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
    border-top: 5px solid #f97316;
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

  /* Error */
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

  /* Header */
  .game-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 12px;
    margin-bottom: 16px;
    padding-bottom: 16px;
    border-bottom: 2px solid rgba(249, 115, 22, 0.2);
  }

  .level-display {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
  }

  .level-label {
    font-size: 14px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    text-transform: uppercase;
  }

  .level-number {
    font-size: 24px;
    font-weight: 700;
    color: white;
  }

  .timer-display {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 18px;
    background: rgba(59, 130, 246, 0.1);
    border-radius: 12px;
    border: 2px solid rgba(59, 130, 246, 0.2);
    transition: all 0.3s ease;
  }

  .timer-display.timer-warning {
    background: rgba(239, 68, 68, 0.15);
    border-color: rgba(239, 68, 68, 0.4);
    animation: timerPulse 1s ease infinite;
  }

  .timer-icon {
    font-size: 18px;
  }

  .timer-display .timer-icon .fa-clock {
    color: #3b82f6;
  }

  .timer-display.timer-warning .timer-icon .fa-clock {
    color: #ef4444;
  }

  .timer-value {
    font-size: 22px;
    font-weight: 700;
    color: #1e293b;
    font-variant-numeric: tabular-nums;
  }

  .timer-display.timer-warning .timer-value {
    color: #ef4444;
  }

  @keyframes timerPulse {
    0%,
    100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.05);
    }
  }

  .stats-display {
    display: flex;
    gap: 10px;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: rgba(249, 115, 22, 0.1);
    border-radius: 10px;
  }

  .stat-icon {
    font-size: 16px;
  }

  .stat-icon .fa-star {
    color: #fbbf24;
  }

  .stat-icon .fa-fire {
    color: #f97316;
  }

  .stat-value {
    font-size: 14px;
    font-weight: 600;
    color: #ea580c;
  }

  /* Progress */
  .progress-section {
    margin-bottom: 20px;
  }

  .progress-text {
    display: block;
    text-align: center;
    font-size: 13px;
    font-weight: 600;
    color: #64748b;
    margin-bottom: 6px;
  }

  .progress-bar {
    height: 6px;
    background: #e2e8f0;
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #f97316, #ef4444);
    border-radius: 3px;
    transition: width 0.3s ease;
  }

  /* Card Comparison */
  .comparison-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
    padding: 20px 0;
    transition: all 0.3s ease;
  }

  .comparison-area.feedback-correct {
    animation: correctFlash 0.5s ease;
  }

  .comparison-area.feedback-incorrect {
    animation: shake 0.5s ease;
  }

  @keyframes correctFlash {
    0%,
    100% {
      background-color: transparent;
    }
    50% {
      background-color: rgba(16, 185, 129, 0.1);
    }
  }

  @keyframes shake {
    0%,
    100% {
      transform: translateX(0);
    }
    20%,
    60% {
      transform: translateX(-8px);
    }
    40%,
    80% {
      transform: translateX(8px);
    }
  }

  .card-pair {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;
    width: 100%;
  }

  .match-card {
    flex: 1;
    max-width: 260px;
    background: white;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.12);
    border: 3px solid #e2e8f0;
    transition: all 0.3s ease;
  }

  .card-image {
    width: 100%;
    aspect-ratio: 1;
    background-size: cover;
    background-position: center;
    background-color: #f8fafc;
  }

  .card-label {
    padding: 10px 12px;
    text-align: center;
    font-size: 14px;
    font-weight: 600;
    color: #1e293b;
    background: #f8fafc;
    border-top: 1px solid #e2e8f0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .vs-divider {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    background: linear-gradient(135deg, #f97316, #ef4444);
    border-radius: 50%;
    flex-shrink: 0;
    box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
  }

  .vs-divider span {
    font-size: 16px;
    font-weight: 800;
    color: white;
    letter-spacing: 1px;
  }

  /* Answer Buttons */
  .answer-buttons {
    display: flex;
    gap: 16px;
    width: 100%;
    max-width: 500px;
  }

  .btn-same,
  .btn-different {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 18px 24px;
    font-size: 18px;
    font-weight: 700;
    border: none;
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.2s ease;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .btn-same {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
    box-shadow: 0 4px 14px rgba(16, 185, 129, 0.35);
  }

  .btn-same:hover:not(:disabled) {
    transform: translateY(-3px);
    box-shadow: 0 8px 24px rgba(16, 185, 129, 0.45);
  }

  .btn-different {
    background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
    color: white;
    box-shadow: 0 4px 14px rgba(239, 68, 68, 0.35);
  }

  .btn-different:hover:not(:disabled) {
    transform: translateY(-3px);
    box-shadow: 0 8px 24px rgba(239, 68, 68, 0.45);
  }

  .btn-same:disabled,
  .btn-different:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
  }

  .btn-same:active:not(:disabled),
  .btn-different:active:not(:disabled) {
    transform: translateY(0);
  }

  /* Feedback */
  .answer-feedback {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 24px;
    border-radius: 12px;
    font-size: 16px;
    font-weight: 600;
    animation: fadeInUp 0.3s ease;
  }

  .answer-feedback.correct {
    background: rgba(16, 185, 129, 0.15);
    color: #059669;
    border: 2px solid #10b981;
  }

  .answer-feedback.incorrect {
    background: rgba(239, 68, 68, 0.15);
    color: #dc2626;
    border: 2px solid #ef4444;
  }

  .answer-feedback .fa-circle-check {
    color: #10b981;
  }

  .answer-feedback .fa-circle-xmark {
    color: #ef4444;
  }

  @keyframes fadeInUp {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Results */
  .results-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 400px;
  }

  .results-card {
    text-align: center;
    padding: 32px;
    background: white;
    border-radius: 20px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
    max-width: 500px;
    width: 90%;
    box-sizing: border-box;
  }

  .results-card h2 {
    margin: 0 0 24px 0;
    font-size: 28px;
    color: #f97316;
  }

  .results-card h2 .fa-trophy {
    color: #fbbf24;
  }

  .results-stats {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
    margin-bottom: 24px;
  }

  .result-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 16px;
    background: #f8fafc;
    border-radius: 12px;
  }

  .result-icon {
    font-size: 24px;
  }

  .result-icon .fa-star {
    color: #fbbf24;
  }

  .result-icon .fa-circle-check {
    color: #10b981;
  }

  .result-icon .fa-chart-simple {
    color: #8b5cf6;
  }

  .result-icon .fa-fire {
    color: #f97316;
  }

  .result-label {
    font-size: 12px;
    color: #64748b;
    text-transform: uppercase;
  }

  .result-value {
    font-size: 20px;
    font-weight: 700;
    color: #1e293b;
  }

  .result-value.correct {
    color: #10b981;
  }

  .result-value.streak {
    color: #f97316;
  }

  .results-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .btn-next-level {
    padding: 14px 28px;
    background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
    color: white;
    border: none;
    border-radius: 12px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .btn-next-level:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(249, 115, 22, 0.4);
  }

  .btn-restart {
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

  .btn-restart:hover {
    background: #f8fafc;
    border-color: #cbd5e1;
  }

  /* Icon colors */
  .error-icon .fa-triangle-exclamation {
    color: #f59e0b;
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

  .achievements-toast h4 .fa-trophy {
    color: white;
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
  }

  .continue-dialog {
    background: white;
    border-radius: 24px;
    padding: 32px 40px;
    text-align: center;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    max-width: 400px;
    width: 90%;
  }

  .continue-dialog h2 {
    margin: 0 0 16px 0;
    font-size: 24px;
    color: #ea580c;
  }

  .continue-dialog h2 .fa-clipboard-question {
    color: #f97316;
  }

  .continue-dialog p {
    margin: 8px 0;
    color: #64748b;
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
    background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
    color: white;
    border: none;
    border-radius: 12px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
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
  }

  /* Responsive */
  @media (max-width: 768px) {
    .speed-match-container {
      padding: 20px;
    }

    .game-header {
      flex-direction: column;
      align-items: center;
    }

    .stats-display {
      width: 100%;
      justify-content: center;
    }

    .card-pair {
      gap: 12px;
    }

    .match-card {
      max-width: 200px;
    }

    .vs-divider {
      width: 40px;
      height: 40px;
    }

    .vs-divider span {
      font-size: 14px;
    }

    .answer-buttons {
      gap: 12px;
    }

    .btn-same,
    .btn-different {
      padding: 14px 18px;
      font-size: 16px;
    }

    .results-card {
      padding: 28px 24px;
    }

    .results-card h2 {
      font-size: 24px;
    }

    .continue-dialog {
      padding: 28px 24px;
    }
  }

  @media (max-width: 480px) {
    .speed-match-container {
      padding: 15px;
      border-radius: 16px;
    }

    .card-pair {
      flex-direction: column;
      gap: 10px;
    }

    .match-card {
      max-width: 220px;
      width: 100%;
    }

    .vs-divider {
      width: 36px;
      height: 36px;
    }

    .vs-divider span {
      font-size: 12px;
    }

    .answer-buttons {
      flex-direction: row;
      gap: 10px;
    }

    .btn-same,
    .btn-different {
      padding: 14px 12px;
      font-size: 14px;
      gap: 6px;
    }

    .timer-value {
      font-size: 18px;
    }

    .results-card {
      padding: 24px 16px;
    }

    .results-card h2 {
      font-size: 20px;
    }

    .results-stats {
      grid-template-columns: 1fr 1fr;
      gap: 10px;
    }

    .result-item {
      padding: 12px;
    }

    .result-value {
      font-size: 18px;
    }

    .results-actions {
      flex-direction: column;
    }

    .btn-next-level,
    .btn-restart {
      width: 100%;
    }

    .continue-dialog {
      padding: 24px 16px;
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

    .achievement-icon {
      font-size: 16px;
    }

    .achievement-name {
      font-size: 14px;
    }
  }
</style>
