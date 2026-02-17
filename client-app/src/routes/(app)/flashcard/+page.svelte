<script>
  import FlashcardCategoryTab from "../../../components/organisms/flashcards/FlashcardCategoryTab.svelte";
  import PrimaryHeading from "../../../components/atoms/heading/PrimaryHeading.svelte";
  import FlashcardMatching from "../../../components/organisms/flashcards/FlashcardMatching.svelte";
  import FlashcardQuiz from "../../../components/organisms/flashcards/FlashcardQuiz.svelte";
  import FlashcardSpeedMatch from "../../../components/organisms/flashcards/FlashcardSpeedMatch.svelte";
  import FlashcardSpelling from "../../../components/organisms/flashcards/FlashcardSpelling.svelte";
</script>

<div class="flashcard-page">
  <div class="container py-4">
    <div class="welcome-section text-center mb-5">
      <h1 class="display-4 mb-3">Chào mừng đến với Qhortus</h1>
      <p class="lead text-muted">
        Khám phá và học tập qua các flashcard thú vị. Nâng cao kiến thức của bạn
        mỗi ngày!
      </p>
    </div>
    <FlashcardCategoryTab></FlashcardCategoryTab>
    <div class="tab-content" id="pills-tabContent">
      <div
        class="tab-pane show active"
        id="pills-matching"
        role="tabpanel"
        aria-labelledby="pills-matching-tab"
        tabindex="0"
      >
        <FlashcardMatching></FlashcardMatching>
      </div>
      <div
        class="tab-pane"
        id="pills-quiz"
        role="tabpanel"
        aria-labelledby="pills-quiz-tab"
        tabindex="0"
      >
        <FlashcardQuiz></FlashcardQuiz>
      </div>
      <div
        class="tab-pane"
        id="pills-spelling"
        role="tabpanel"
        aria-labelledby="pills-spelling-tab"
        tabindex="0"
      >
        <FlashcardSpelling></FlashcardSpelling>
      </div>
      <div
        class="tab-pane"
        id="pills-speed-match"
        role="tabpanel"
        aria-labelledby="pills-speed-match-tab"
        tabindex="0"
      >
        <FlashcardSpeedMatch></FlashcardSpeedMatch>
      </div>
    </div>
  </div>
</div>

<style>
  .flashcard-page {
    min-height: calc(100vh - 200px);
    background-color: var(--background-color);
  }

  .tab-content {
    padding: 5px 0px;
    min-height: 400px;
    border-radius: 0 0 8px 8px;
  }

  .tab-pane {
    padding: 5px 0px;
    border-radius: 3px;
    background-color: var(--background-color);
  }

  :global(.primary-heading) {
    margin-bottom: 20px;
    margin-top: 30px;
  }
</style>
