<script>
  import FlashcardCategoryTab from "../../components/organisms/flashcards/FlashcardCategoryTab.svelte";
  import PrimaryHeading from "../../components/atoms/heading/PrimaryHeading.svelte";
</script>

<div class="container mb-3 mt-3">
  <PrimaryHeading title="Chọn kiểu flashcard bạn thích"></PrimaryHeading>
  <FlashcardCategoryTab></FlashcardCategoryTab>
</div>

<style>
  :global(.primary-heading) {
    margin-bottom: 20px;
    margin-top: 30px;
  }
</style>
