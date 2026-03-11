<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import FlashcardCategoryTab from "../../../components/organisms/flashcards/FlashcardCategoryTab.svelte";
  import FlashcardMatching from "../../../components/organisms/flashcards/FlashcardMatching.svelte";
  import FlashcardQuiz from "../../../components/organisms/flashcards/FlashcardQuiz.svelte";
  import FlashcardSpeedMatch from "../../../components/organisms/flashcards/FlashcardSpeedMatch.svelte";
  import FlashcardSpelling from "../../../components/organisms/flashcards/FlashcardSpelling.svelte";
  import { toggleMute, initSound } from "$lib/utils/sound";

  const VALID_MODES = ["matching", "quiz", "spelling", "speed-match"];

  let muted = $state(false);
  let activeMode = $state("matching");

  onMount(() => {
    muted = initSound();
    const mode = page.url.searchParams.get("mode");
    if (mode && VALID_MODES.includes(mode)) {
      activeMode = mode;
    }
  });

  function handleModeChange(mode: string) {
    activeMode = mode;
    goto(`?mode=${mode}`, { replaceState: true });
  }

  function handleToggleMute() {
    muted = toggleMute();
  }
</script>

<svelte:head>
  <title>Chơi Flashcard — Qhortus</title>
  <meta
    name="description"
    content="Chọn chủ đề và bắt đầu học với các bộ flashcard đa dạng. Luyện tập qua Quiz, Ghép đôi, Spelling và Speed Match."
  />
  <meta property="og:title" content="Chơi Flashcard — Qhortus" />
  <meta
    property="og:description"
    content="Chọn chủ đề và bắt đầu học với các bộ flashcard đa dạng."
  />
  <meta property="og:url" content="/flashcard" />
</svelte:head>

<div class="flashcard-page">
  <div class="container py-4">
    <div class="welcome-section text-center mb-5 position-relative">
      <h1 class="display-4 mb-3">Hôm nay chinh phục bộ nào? 🎮</h1>
      <p class="lead text-muted">
        Chọn chủ đề, chọn chế độ — rồi thử beat điểm cao nhất của chính mình.
        Mỗi lần chơi là một lần não ghi nhớ sâu hơn.
      </p>
      <button
        class="btn-mute"
        onclick={handleToggleMute}
        title={muted ? "Bật âm thanh" : "Tắt âm thanh"}
        aria-label={muted ? "Bật âm thanh" : "Tắt âm thanh"}
      >
        <i class="fa-solid {muted ? 'fa-volume-xmark' : 'fa-volume-high'}"></i>
      </button>
    </div>

    <FlashcardCategoryTab {activeMode} onTabChange={handleModeChange} />

    <div class="tab-content">
      <!-- Game components stay mounted to preserve state when switching tabs -->
      <div
        class="tab-pane {activeMode === 'matching' ? 'show active' : ''}"
        role="tabpanel"
      >
        <FlashcardMatching />
      </div>
      <div
        class="tab-pane {activeMode === 'quiz' ? 'show active' : ''}"
        role="tabpanel"
      >
        <FlashcardQuiz />
      </div>
      <div
        class="tab-pane {activeMode === 'spelling' ? 'show active' : ''}"
        role="tabpanel"
      >
        <FlashcardSpelling />
      </div>
      <div
        class="tab-pane {activeMode === 'speed-match' ? 'show active' : ''}"
        role="tabpanel"
      >
        <FlashcardSpeedMatch />
      </div>
    </div>
  </div>
</div>

<style>
  .flashcard-page {
    min-height: calc(100vh - 200px);
    background-color: var(--background-color);
  }

  .btn-mute {
    position: absolute;
    top: 0;
    right: 0;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    border: 2px solid var(--border-color);
    background: var(--card-bg-color);
    color: var(--primary-color);
    font-size: 1rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    box-shadow: 0 2px 8px var(--box-shadow-color);
  }

  .btn-mute:hover {
    background: var(--background-secondary);
    transform: scale(1.1);
  }

  @media (max-width: 480px) {
    .btn-mute {
      top: auto;
      bottom: -10px;
      right: 0;
    }
  }

  .tab-content {
    padding: 5px 0;
    min-height: 400px;
    border-radius: 0 0 8px 8px;
  }

  /* Hide inactive tab panes — components stay mounted to preserve game state */
  .tab-pane {
    display: none;
    padding: 5px 0;
    border-radius: 3px;
    background-color: var(--background-color);
  }

  .tab-pane.show.active {
    display: block;
  }
</style>
