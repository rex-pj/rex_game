<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { FlashcardApi } from "$lib/api/flashcardApi";
  import { ACCESS_TOKEN } from "$lib/common/contants";
  import type { Flashcard } from "$lib/models/flashcard";
  import {
    spellingState,
    spellingStats,
    spellingQuestions,
    currentSpellingQuestion,
    currentSpellingIndex,
    spellingComboCount,
    spellingNewAchievements,
    spellingSavedProgress,
    spellingProgress,
    hintsRevealed,
    initializeSpelling,
    submitSpellingAnswer,
    useHint,
    resetSpelling,
    cleanupSpelling,
    setSpellingError,
    clearSpellingAchievements,
    getSpellingProgress,
    continueSpellingFromProgress,
    startNewSpellingGame,
    nextSpellingLevel,
  } from "$lib/stores/spelling-game.store";
  import type { GameProgress } from "$lib/api/scoringApi";
  import { getHint, getAccuracy, scrambleLetters } from "$lib/helpers/spellingHelpers";
  import { formatTime } from "$lib/helpers/quizHelpers";
  import Cookies from "js-cookie";

  // Props
  interface Props {
    initialLevel?: number;
    flashcardTypeId?: number;
  }

  let {
    initialLevel = 1,
    flashcardTypeId = undefined,
  }: Props = $props();

  // Local state
  let errorMessage = $state("");
  let showContinueDialog = $state(false);
  let pendingProgress: GameProgress | null = $state(null);
  let userInput = $state("");
  let lastAnswerCorrect: boolean | null = $state(null);
  let lastCorrectAnswer = $state("");
  let letterBank: string[] = $state([]);

  // Regenerate letter bank when question changes
  let currentQuestionId = $derived($currentSpellingQuestion?.id ?? "");

  $effect(() => {
    if (currentQuestionId && $currentSpellingQuestion) {
      letterBank = scrambleLetters($currentSpellingQuestion.correctAnswer);
      userInput = "";
      lastAnswerCorrect = null;
      lastCorrectAnswer = "";
    }
  });

  /**
   * Get current hint text
   */
  let currentHint = $derived.by(() => {
    if (!$currentSpellingQuestion) return "";
    return getHint($currentSpellingQuestion.correctAnswer, $hintsRevealed + 1);
  });

  /**
   * Check for saved progress
   */
  async function checkSavedProgress() {
    const progress = await getSpellingProgress();
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
    await startNewSpellingGame();
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

      const response = await api.getList(fetch, 1, 50, "spelling");
      const flashcards: Flashcard[] = response.items || response.data || [];

      if (!flashcards || flashcards.length < 1) {
        throw new Error("Cần ít nhất 1 flashcard để chơi Đánh Vần");
      }

      const filteredFlashcards = flashcardTypeId
        ? flashcards.filter((f) => f.flashcard_type_id === flashcardTypeId)
        : flashcards;

      if (filteredFlashcards.length < 1) {
        throw new Error("Không đủ flashcards cho loại này");
      }

      await initializeSpelling(filteredFlashcards, {
        flashcardTypeId,
      });

      if (startScore > 0) {
        await continueSpellingFromProgress({
          current_level: startLevel,
          total_score: startScore,
        } as GameProgress);
      }
    } catch (error) {
      console.error("Failed to load flashcards:", error);
      errorMessage = error instanceof Error ? error.message : "Không thể tải flashcards";
      setSpellingError();
    }
  }

  /**
   * Handle letter click from bank
   */
  function handleLetterClick(letter: string, index: number) {
    if ($spellingState !== "idle") return;
    userInput += letter;
    // Remove used letter from bank
    letterBank = letterBank.filter((_, i) => i !== index);
  }

  /**
   * Handle backspace / remove last letter
   */
  function handleBackspace() {
    if (userInput.length === 0) return;
    const removedLetter = userInput[userInput.length - 1];
    userInput = userInput.slice(0, -1);
    letterBank = [...letterBank, removedLetter];
  }

  /**
   * Handle submit answer
   */
  function handleSubmit() {
    if ($spellingState !== "idle" || !userInput.trim()) return;

    const question = $currentSpellingQuestion;
    if (!question) return;

    const isCorrect = userInput.trim().toLowerCase() === question.correctAnswer.trim().toLowerCase();
    lastAnswerCorrect = isCorrect;
    lastCorrectAnswer = question.correctAnswer;

    submitSpellingAnswer(userInput);
  }

  /**
   * Handle keyboard enter
   */
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      handleSubmit();
    }
  }

  /**
   * Handle hint usage
   */
  function handleUseHint() {
    useHint();
  }

  /**
   * Handle next level
   */
  async function handleNextLevel() {
    await nextSpellingLevel();
  }

  /**
   * Handle reset
   */
  async function handleReset() {
    await resetSpelling();
  }

  // Lifecycle
  onMount(() => {
    checkSavedProgress();
  });

  onDestroy(() => {
    cleanupSpelling();
  });
</script>

<!-- Continue Dialog -->
{#if showContinueDialog && pendingProgress}
  <div class="continue-dialog-overlay">
    <div class="continue-dialog">
      <h2><i class="fa-solid fa-clipboard-question"></i> Tiếp tục chơi?</h2>
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

<div class="spelling-container">
  {#if $spellingState === "loading"}
    <!-- Loading State -->
    <div class="loading-container">
      <div class="spinner"></div>
      <p>Đang tải câu hỏi...</p>
    </div>
  {:else if $spellingState === "error"}
    <!-- Error State -->
    <div class="error-container">
      <div class="error-icon"><i class="fa-solid fa-triangle-exclamation"></i></div>
      <h3>Không thể tải game</h3>
      <p>{errorMessage}</p>
      <button class="btn btn-primary" onclick={() => loadFlashcards()}>Thử lại</button>
    </div>
  {:else if $spellingState === "completed"}
    <!-- Results State -->
    <div class="results-container">
      <div class="results-card">
        <h2><i class="fa-solid fa-trophy"></i> Hoàn thành Màn {$spellingStats.level}!</h2>

        <div class="results-stats">
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-star"></i></span>
            <span class="result-label">Điểm</span>
            <span class="result-value">{$spellingStats.score.toLocaleString()}</span>
          </div>
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-circle-check"></i></span>
            <span class="result-label">Đúng</span>
            <span class="result-value correct">{$spellingStats.correct}/{$spellingStats.totalQuestions}</span>
          </div>
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-chart-simple"></i></span>
            <span class="result-label">Độ chính xác</span>
            <span class="result-value">{getAccuracy($spellingStats.correct, $spellingStats.totalQuestions)}%</span>
          </div>
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-stopwatch"></i></span>
            <span class="result-label">Thời gian</span>
            <span class="result-value">{formatTime($spellingStats.timeElapsed)}</span>
          </div>
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-lightbulb"></i></span>
            <span class="result-label">Gợi ý đã dùng</span>
            <span class="result-value hints">{$spellingStats.hintsUsed}</span>
          </div>
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-fire"></i></span>
            <span class="result-label">Combo</span>
            <span class="result-value streak">{$spellingComboCount}</span>
          </div>
        </div>

        <div class="results-actions">
          <button class="btn-next-level" onclick={handleNextLevel}>
            Màn tiếp theo →
          </button>
          <button class="btn-restart" onclick={handleReset}>
            Chơi lại
          </button>
        </div>
      </div>
    </div>
  {:else}
    <!-- Game Header -->
    <div class="game-header">
      <div class="level-display">
        <span class="level-label">Màn</span>
        <span class="level-number">{$spellingStats.level}</span>
      </div>

      <div class="progress-display">
        <span class="progress-text">
          Câu {$spellingStats.currentQuestion}/{$spellingStats.totalQuestions}
        </span>
        <div class="progress-bar">
          <div class="progress-fill" style="width: {$spellingProgress}%"></div>
        </div>
      </div>

      <div class="stats-display">
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-star"></i></span>
          <span class="stat-value">{$spellingStats.score}</span>
        </div>
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-fire"></i></span>
          <span class="stat-value">{$spellingComboCount}</span>
        </div>
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-stopwatch"></i></span>
          <span class="stat-value">{formatTime($spellingStats.timeElapsed)}</span>
        </div>
      </div>
    </div>

    <!-- Question Area -->
    {#if $currentSpellingQuestion}
      <div class="question-area">
        <!-- Image -->
        <div class="question-image-container">
          <div
            class="question-image"
            style="background-image: url({$currentSpellingQuestion.imageUrl});"
          ></div>
        </div>

        <!-- Question Text -->
        <div class="question-text">
          <h3>Đánh vần tên hình ảnh này</h3>
        </div>

        <!-- Hint Section -->
        <div class="hint-section">
          <button
            class="btn-hint"
            onclick={handleUseHint}
            disabled={$spellingState !== "idle" || $hintsRevealed >= ($currentSpellingQuestion?.correctAnswer.length ?? 1) - 1}
          >
            <i class="fa-solid fa-lightbulb"></i> Gợi ý (-{$spellingStats.level > 1 ? 50 : 50} điểm)
          </button>
          {#if $hintsRevealed > 0}
            <span class="hint-text">{currentHint}</span>
          {/if}
        </div>

        <!-- Input Area -->
        <div class="input-area">
          <div class="input-display">
            <span class="input-text">{userInput || "\u00A0"}</span>
            {#if userInput.length > 0}
              <button class="btn-backspace" onclick={handleBackspace} disabled={$spellingState !== "idle"} aria-label="Xóa ký tự">
                <i class="fa-solid fa-delete-left"></i>
              </button>
            {/if}
          </div>

          <!-- Letter Bank -->
          <div class="letter-bank">
            {#each letterBank as letter, index}
              <button
                class="letter-btn"
                onclick={() => handleLetterClick(letter, index)}
                disabled={$spellingState !== "idle"}
              >
                {letter}
              </button>
            {/each}
          </div>

          <!-- Text Input (alternative) -->
          <div class="text-input-row">
            <input
              type="text"
              class="text-input"
              placeholder="Hoặc gõ câu trả lời..."
              bind:value={userInput}
              onkeydown={handleKeydown}
              disabled={$spellingState !== "idle"}
            />
            <button
              class="btn-submit"
              onclick={handleSubmit}
              disabled={$spellingState !== "idle" || !userInput.trim()}
              aria-label="Gửi câu trả lời"
            >
              <i class="fa-solid fa-paper-plane"></i>
            </button>
          </div>
        </div>

        <!-- Feedback -->
        {#if $spellingState === "answered" && lastAnswerCorrect !== null}
          <div class="feedback {lastAnswerCorrect ? 'correct' : 'incorrect'}">
            {#if lastAnswerCorrect}
              <span class="feedback-icon"><i class="fa-solid fa-circle-check"></i></span>
              <span>Chính xác!</span>
            {:else}
              <span class="feedback-icon"><i class="fa-solid fa-circle-xmark"></i></span>
              <span>Sai rồi! Đáp án đúng: <strong>{lastCorrectAnswer}</strong></span>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  {/if}

  <!-- Achievement Toast -->
  {#if $spellingNewAchievements.length > 0}
    <div class="achievements-toast">
      <h4><i class="fa-solid fa-trophy"></i> Thành tựu mới!</h4>
      {#each $spellingNewAchievements as achievement}
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
      <button class="btn-dismiss" onclick={() => clearSpellingAchievements()} aria-label="Đóng"><i class="fa-solid fa-xmark"></i></button>
    </div>
  {/if}
</div>

<style>
  .spelling-container {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 30px;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.95) 0%,
      rgba(240, 255, 244, 0.95) 100%
    );
    border-radius: 24px;
    position: relative;
    box-shadow:
      0 10px 40px rgba(16, 185, 129, 0.15),
      0 4px 12px rgba(0, 0, 0, 0.08);
  }

  .spelling-container::before {
    content: "";
    position: absolute;
    top: -3px;
    left: -3px;
    right: -3px;
    bottom: -3px;
    background: linear-gradient(
      45deg,
      #10b981,
      #059669,
      #3b82f6,
      #8b5cf6,
      #10b981
    );
    border-radius: 24px;
    z-index: -1;
    animation: rotateBorder 6s linear infinite;
    background-size: 300% 300%;
  }

  @keyframes rotateBorder {
    0% { background-position: 0% 50%; }
    50% { background-position: 100% 50%; }
    100% { background-position: 0% 50%; }
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
    border-top: 5px solid #10b981;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
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
    gap: 16px;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 2px solid rgba(16, 185, 129, 0.2);
  }

  .level-display {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
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

  .progress-display {
    flex: 1;
    min-width: 150px;
    max-width: 300px;
  }

  .progress-text {
    display: block;
    text-align: center;
    font-size: 14px;
    font-weight: 600;
    color: #64748b;
    margin-bottom: 6px;
  }

  .progress-bar {
    height: 8px;
    background: #e2e8f0;
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #10b981, #059669);
    border-radius: 4px;
    transition: width 0.3s ease;
  }

  .stats-display {
    display: flex;
    gap: 12px;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: rgba(16, 185, 129, 0.1);
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

  .stat-icon .fa-stopwatch {
    color: #3b82f6;
  }

  .stat-value {
    font-size: 14px;
    font-weight: 600;
    color: #059669;
  }

  /* Question Area */
  .question-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
  }

  .question-image-container {
    width: 100%;
    max-width: 250px;
    aspect-ratio: 1;
    border-radius: 20px;
    overflow: hidden;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    border: 4px solid #10b981;
  }

  .question-image {
    width: 100%;
    height: 100%;
    background-size: cover;
    background-position: center;
    background-color: #f8fafc;
  }

  .question-text h3 {
    font-size: 20px;
    font-weight: 600;
    color: #1e293b;
    margin: 0;
  }

  /* Hint Section */
  .hint-section {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .btn-hint {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    background: rgba(251, 191, 36, 0.15);
    color: #d97706;
    border: 2px solid rgba(251, 191, 36, 0.3);
    border-radius: 10px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-hint:hover:not(:disabled) {
    background: rgba(251, 191, 36, 0.25);
    border-color: rgba(251, 191, 36, 0.5);
  }

  .btn-hint:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-hint .fa-lightbulb {
    color: #fbbf24;
  }

  .hint-text {
    font-size: 16px;
    font-weight: 600;
    color: #d97706;
    letter-spacing: 2px;
    font-family: monospace;
  }

  /* Input Area */
  .input-area {
    width: 100%;
    max-width: 500px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .input-display {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 52px;
    padding: 12px 20px;
    background: white;
    border: 3px solid #10b981;
    border-radius: 14px;
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.15);
  }

  .input-text {
    font-size: 24px;
    font-weight: 700;
    color: #1e293b;
    letter-spacing: 4px;
    text-transform: lowercase;
  }

  .btn-backspace {
    background: rgba(239, 68, 68, 0.1);
    border: none;
    color: #ef4444;
    width: 36px;
    height: 36px;
    border-radius: 8px;
    cursor: pointer;
    font-size: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .btn-backspace:hover:not(:disabled) {
    background: rgba(239, 68, 68, 0.2);
  }

  /* Letter Bank */
  .letter-bank {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    justify-content: center;
  }

  .letter-btn {
    width: 44px;
    height: 44px;
    font-size: 18px;
    font-weight: 700;
    text-transform: lowercase;
    background: white;
    border: 2px solid #d1d5db;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.15s ease;
    color: #1e293b;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
  }

  .letter-btn:hover:not(:disabled) {
    border-color: #10b981;
    background: rgba(16, 185, 129, 0.05);
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(16, 185, 129, 0.2);
  }

  .letter-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  .letter-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Text Input Row */
  .text-input-row {
    display: flex;
    gap: 10px;
  }

  .text-input {
    flex: 1;
    padding: 12px 16px;
    font-size: 16px;
    border: 2px solid #d1d5db;
    border-radius: 12px;
    outline: none;
    transition: border-color 0.2s ease;
    color: #1e293b;
  }

  .text-input:focus {
    border-color: #10b981;
    box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.15);
  }

  .text-input:disabled {
    background: #f1f5f9;
    cursor: not-allowed;
  }

  .btn-submit {
    padding: 12px 20px;
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
    border: none;
    border-radius: 12px;
    font-size: 18px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-submit:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.35);
  }

  .btn-submit:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }

  /* Feedback */
  .feedback {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 24px;
    border-radius: 12px;
    font-size: 16px;
    font-weight: 600;
    animation: fadeIn 0.3s ease;
  }

  .feedback.correct {
    background: rgba(16, 185, 129, 0.15);
    color: #059669;
    border: 2px solid #10b981;
  }

  .feedback.incorrect {
    background: rgba(239, 68, 68, 0.15);
    color: #dc2626;
    border: 2px solid #ef4444;
  }

  .feedback-icon {
    font-size: 20px;
  }

  .feedback-icon .fa-circle-check {
    color: #10b981;
  }

  .feedback-icon .fa-circle-xmark {
    color: #ef4444;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
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
    color: #10b981;
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

  .result-icon .fa-stopwatch {
    color: #3b82f6;
  }

  .result-icon .fa-lightbulb {
    color: #fbbf24;
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

  .result-value.hints {
    color: #d97706;
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
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
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
    box-shadow: 0 6px 20px rgba(16, 185, 129, 0.4);
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
    from { transform: translateX(100%); opacity: 0; }
    to { transform: translateX(0); opacity: 1; }
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
    color: #059669;
  }

  .continue-dialog h2 .fa-clipboard-question {
    color: #10b981;
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
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
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
    .spelling-container {
      padding: 20px;
    }

    .game-header {
      flex-direction: column;
      align-items: center;
    }

    .progress-display {
      width: 100%;
      max-width: none;
    }

    .stats-display {
      width: 100%;
      justify-content: center;
    }

    .question-image-container {
      max-width: 200px;
    }

    .input-text {
      font-size: 20px;
      letter-spacing: 3px;
    }

    .letter-btn {
      width: 40px;
      height: 40px;
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
    .spelling-container {
      padding: 15px;
      border-radius: 16px;
    }

    .question-image-container {
      max-width: 180px;
    }

    .question-text h3 {
      font-size: 18px;
    }

    .input-text {
      font-size: 18px;
      letter-spacing: 2px;
    }

    .letter-btn {
      width: 36px;
      height: 36px;
      font-size: 14px;
    }

    .text-input {
      font-size: 14px;
      padding: 10px 12px;
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
