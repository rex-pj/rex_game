<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { FlashcardApi } from "$lib/api/flashcardApi";
  import { ACCESS_TOKEN } from "$lib/common/contants";
  import type { Flashcard } from "$lib/models/flashcard";
  import {
    quizGameState,
    quizStats,
    currentQuestion,
    quizComboCount,
    quizNewAchievements,
    quizProgress,
    initializeQuiz,
    answerQuestion,
    resetQuiz,
    cleanupQuiz,
    setQuizError,
    clearQuizAchievements,
    getQuizProgress,
    continueQuizFromProgress,
    startNewQuizGame,
    nextLevel,
  } from "$lib/stores/quiz-game.store";
  import type { GameProgress } from "$lib/api/scoringApi";
  import { formatTime, getAccuracy } from "$lib/helpers/quizHelpers";
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
  let selectedOption: string | null = $state(null);

  /**
   * Check for saved progress
   */
  async function checkSavedProgress() {
    const progress = await getQuizProgress();
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
    await startNewQuizGame();
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

      const response = await api.getList(fetch, 1, 50, "quiz");
      const flashcards: Flashcard[] = response.items || response.data || [];

      if (!flashcards || flashcards.length < 4) {
        throw new Error("Cần ít nhất 4 flashcards để chơi quiz");
      }

      const filteredFlashcards = flashcardTypeId
        ? flashcards.filter((f) => f.flashcard_type_id === flashcardTypeId)
        : flashcards;

      if (filteredFlashcards.length < 4) {
        throw new Error("Không đủ flashcards cho loại này");
      }

      await initializeQuiz(filteredFlashcards, {
        flashcardTypeId,
      });

      if (startScore > 0) {
        await continueQuizFromProgress({
          current_level: startLevel,
          total_score: startScore,
        } as GameProgress);
      }
    } catch (error) {
      console.error("Failed to load flashcards:", error);
      errorMessage =
        error instanceof Error ? error.message : "Không thể tải flashcards";
      setQuizError();
    }
  }

  /**
   * Handle option click
   */
  function handleOptionClick(option: string) {
    if ($quizGameState !== "idle" || selectedOption) return;
    selectedOption = option;
    answerQuestion(option);

    // Reset selected option after delay
    setTimeout(() => {
      selectedOption = null;
    }, 1500);
  }

  /**
   * Handle next level
   */
  async function handleNextLevel() {
    await nextLevel();
  }

  /**
   * Handle reset
   */
  async function handleReset() {
    await resetQuiz();
  }

  /**
   * Get option class based on state
   */
  function getOptionClass(option: string): string {
    if ($quizGameState !== "answered" && $quizGameState !== "completed") {
      return selectedOption === option ? "selected" : "";
    }

    const question = $currentQuestion;
    if (!question) return "";

    if (option === question.correctAnswer) {
      return "correct";
    }

    if (option === question.selectedAnswer && !question.isCorrect) {
      return "incorrect";
    }

    return "disabled";
  }

  // Lifecycle
  onMount(() => {
    checkSavedProgress();
  });

  onDestroy(() => {
    cleanupQuiz();
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

<div class="quiz-container">
  {#if $quizGameState === "loading"}
    <!-- Loading State -->
    <div class="loading-container">
      <div class="spinner"></div>
      <p>Đang tải câu hỏi...</p>
    </div>
  {:else if $quizGameState === "error"}
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
  {:else if $quizGameState === "completed"}
    <!-- Results State -->
    <div class="results-container">
      <div class="results-card">
        <h2>
          <i class="fa-solid fa-trophy"></i> Hoàn thành Màn {$quizStats.level}!
        </h2>

        <div class="results-stats">
          <div class="result-item">
            <span class="result-icon"><i class="fa-solid fa-star"></i></span>
            <span class="result-label">Điểm</span>
            <span class="result-value">{$quizStats.score.toLocaleString()}</span
            >
          </div>
          <div class="result-item">
            <span class="result-icon"
              ><i class="fa-solid fa-circle-check"></i></span
            >
            <span class="result-label">Đúng</span>
            <span class="result-value correct"
              >{$quizStats.correctAnswers}/{$quizStats.totalQuestions}</span
            >
          </div>
          <div class="result-item">
            <span class="result-icon"
              ><i class="fa-solid fa-chart-simple"></i></span
            >
            <span class="result-label">Độ chính xác</span>
            <span class="result-value"
              >{getAccuracy(
                $quizStats.correctAnswers,
                $quizStats.totalQuestions,
              )}%</span
            >
          </div>
          <div class="result-item">
            <span class="result-icon"
              ><i class="fa-solid fa-stopwatch"></i></span
            >
            <span class="result-label">Thời gian</span>
            <span class="result-value"
              >{formatTime($quizStats.timeElapsed)}</span
            >
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
    <div class="quiz-header">
      <div class="level-display">
        <span class="level-label">Màn</span>
        <span class="level-number">{$quizStats.level}</span>
      </div>

      <div class="progress-display">
        <span class="progress-text">
          Câu {$quizStats.currentQuestion}/{$quizStats.totalQuestions}
        </span>
        <div class="progress-bar">
          <div class="progress-fill" style="width: {$quizProgress}%"></div>
        </div>
      </div>

      <div class="stats-display">
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-star"></i></span>
          <span class="stat-value">{$quizStats.score}</span>
        </div>
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-fire"></i></span>
          <span class="stat-value">{$quizComboCount}</span>
        </div>
        <div class="stat-item">
          <span class="stat-icon"><i class="fa-solid fa-stopwatch"></i></span>
          <span class="stat-value">{formatTime($quizStats.timeElapsed)}</span>
        </div>
      </div>
    </div>

    <!-- Question Area -->
    {#if $currentQuestion}
      <div class="question-area">
        <!-- Image -->
        <div class="question-image-container">
          <div
            class="question-image"
            style="background-image: url({$currentQuestion.imageUrl});"
          ></div>
        </div>

        <!-- Question Text -->
        <div class="question-text">
          <h3>Đây là hình ảnh gì?</h3>
        </div>

        <!-- Answer Options -->
        <div class="options-grid">
          {#each $currentQuestion.options as option}
            <button
              class="option-btn {getOptionClass(option)}"
              onclick={() => handleOptionClick(option)}
              disabled={$quizGameState !== "idle"}
            >
              {option}
            </button>
          {/each}
        </div>

        <!-- Feedback -->
        {#if $quizGameState === "answered" && $currentQuestion.answered}
          <div
            class="feedback {$currentQuestion.isCorrect
              ? 'correct'
              : 'incorrect'}"
          >
            {#if $currentQuestion.isCorrect}
              <span class="feedback-icon"
                ><i class="fa-solid fa-circle-check"></i></span
              >
              <span>Chính xác!</span>
            {:else}
              <span class="feedback-icon"
                ><i class="fa-solid fa-circle-xmark"></i></span
              >
              <span
                >Sai rồi! Đáp án đúng: <strong
                  >{$currentQuestion.correctAnswer}</strong
                ></span
              >
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  {/if}

  <!-- Achievement Toast -->
  {#if $quizNewAchievements.length > 0}
    <div class="achievements-toast">
      <h4><i class="fa-solid fa-trophy"></i> Thành tựu mới!</h4>
      {#each $quizNewAchievements as achievement}
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
        onclick={() => clearQuizAchievements()}
        aria-label="Đóng"><i class="fa-solid fa-xmark"></i></button
      >
    </div>
  {/if}
</div>

<style>
  .quiz-container {
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

  .quiz-container::before {
    content: "";
    position: absolute;
    top: -3px;
    left: -3px;
    right: -3px;
    bottom: -3px;
    background: linear-gradient(
      45deg,
      #8b5cf6,
      #3b82f6,
      #10b981,
      #f59e0b,
      #8b5cf6
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
    border-top: 5px solid #8b5cf6;
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
  .quiz-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 16px;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 2px solid rgba(139, 92, 246, 0.2);
  }

  .level-display {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3);
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
    background: linear-gradient(90deg, #8b5cf6, #3b82f6);
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
    background: rgba(139, 92, 246, 0.1);
    border-radius: 10px;
  }

  .stat-icon {
    font-size: 16px;
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

  .error-icon .fa-triangle-exclamation {
    color: #f59e0b;
  }

  .feedback-icon .fa-circle-check {
    color: #10b981;
  }

  .feedback-icon .fa-circle-xmark {
    color: #ef4444;
  }

  .results-card h2 .fa-trophy {
    color: #fbbf24;
  }

  .continue-dialog h2 .fa-clipboard-question {
    color: #8b5cf6;
  }

  .achievements-toast h4 .fa-trophy {
    color: white;
  }

  .stat-value {
    font-size: 14px;
    font-weight: 600;
    color: #7c3aed;
  }

  /* Question Area */
  .question-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
  }

  .question-image-container {
    width: 100%;
    max-width: 300px;
    aspect-ratio: 1;
    border-radius: 20px;
    overflow: hidden;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    border: 4px solid #8b5cf6;
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

  /* Options */
  .options-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    width: 100%;
    max-width: 500px;
  }

  .option-btn {
    padding: 16px 20px;
    font-size: 16px;
    font-weight: 600;
    background: white;
    border: 3px solid #e2e8f0;
    border-radius: 14px;
    cursor: pointer;
    transition: all 0.2s ease;
    color: #1e293b;
  }

  .option-btn:hover:not(:disabled) {
    border-color: #8b5cf6;
    background: rgba(139, 92, 246, 0.05);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(139, 92, 246, 0.2);
  }

  .option-btn.selected {
    border-color: #8b5cf6;
    background: rgba(139, 92, 246, 0.1);
  }

  .option-btn.correct {
    border-color: #10b981;
    background: rgba(16, 185, 129, 0.15);
    color: #059669;
    animation: correctPulse 0.5s ease;
  }

  .option-btn.incorrect {
    border-color: #ef4444;
    background: rgba(239, 68, 68, 0.15);
    color: #dc2626;
    animation: shake 0.5s ease;
  }

  .option-btn.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .option-btn:disabled {
    cursor: not-allowed;
  }

  @keyframes correctPulse {
    0%,
    100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.05);
    }
  }

  @keyframes shake {
    0%,
    100% {
      transform: translateX(0);
    }
    20%,
    60% {
      transform: translateX(-5px);
    }
    40%,
    80% {
      transform: translateX(5px);
    }
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

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
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
    color: #8b5cf6;
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

  .results-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .btn-next-level {
    padding: 14px 28px;
    background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
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
    box-shadow: 0 6px 20px rgba(139, 92, 246, 0.4);
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
    color: #7c3aed;
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
    background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
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
    .quiz-container {
      padding: 20px;
    }

    .quiz-header {
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
      max-width: 250px;
    }

    .options-grid {
      grid-template-columns: 1fr;
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
    .quiz-container {
      padding: 15px;
      border-radius: 16px;
    }

    .question-image-container {
      max-width: 200px;
    }

    .question-text h3 {
      font-size: 18px;
    }

    .option-btn {
      padding: 14px 16px;
      font-size: 14px;
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
