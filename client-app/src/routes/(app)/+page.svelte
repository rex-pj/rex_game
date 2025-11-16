<script>
  import FlashcardCategoryTab from "../../components/organisms/flashcards/FlashcardCategoryTab.svelte";
  import PrimaryHeading from "../../components/atoms/heading/PrimaryHeading.svelte";
</script>

<div class="home-wrapper">
  <div class="container py-5">
    <div class="welcome-section text-center mb-5">
      <h1 class="display-4 mb-3">Chào mừng đến với Rex Game</h1>
      <p class="lead text-muted">
        Khám phá và học tập qua các flashcard thú vị. Nâng cao kiến thức của bạn mỗi ngày!
      </p>
    </div>

    <FlashcardCategoryTab></FlashcardCategoryTab>
  </div>
</div>

<style>
  .home-wrapper {
    min-height: calc(100vh - 200px);
    background-color: var(--background-color);
  }

  .welcome-section h1 {
    color: var(--primary-color);
    font-weight: 700;
  }

  .welcome-section .lead {
    font-size: 1.25rem;
    max-width: 700px;
    margin: 0 auto;
  }

  :global(.primary-heading) {
    margin-bottom: 20px;
    margin-top: 30px;
  }
</style>
