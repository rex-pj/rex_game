<script lang="ts">
  let level = 3; // Bắt đầu từ level 3x3
  let cards: any[] = [];
  let firstCard: { id: number; text: string; matched: boolean } | null = null;
  let secondCard: { id: number; text: string; matched: boolean } | null = null;
  let showFireworks = false;

  function generateCards(level: number) {
    const totalCards = level * level;
    const uniquePairs = totalCards / 2;
    const cardTexts = Array.from({ length: uniquePairs }, (_, i) => String.fromCharCode(65 + i));
    const allCards = [...cardTexts, ...cardTexts].map((text, index) => ({
      id: index + 1,
      text,
      matched: false,
    }));
    shuffle(allCards);
    return allCards;
  }

  function shuffle(array: any[]) {
    for (let i = array.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [array[i], array[j]] = [array[j], array[i]];
    }
  }

  function resetGame() {
    cards = generateCards(level);
    firstCard = null;
    secondCard = null;
    showFireworks = false;
  }

  function flipCard(card: { id: number; text: string; matched: boolean } | null) {
    if (!card) return;
    if (card.matched || card === firstCard || card === secondCard) return;

    if (!firstCard) {
      firstCard = card;
    } else if (!secondCard) {
      secondCard = card;

      if (firstCard.text === secondCard.text) {
        firstCard.matched = true;
        secondCard.matched = true;
        firstCard = null;
        secondCard = null;

        // Kiểm tra nếu tất cả các thẻ đã được ghép
        if (cards.every((c) => c.matched)) {
          showFireworks = true;
          setTimeout(() => {
            level++;
            resetGame();
          }, 3000); // Đợi 3 giây để hiển thị pháo bông
        }
      } else {
        setTimeout(() => {
          firstCard = null;
          secondCard = null;
        }, 1000);
      }
    }
  }

  // Khởi tạo game
  resetGame();
</script>

<div class="grid" style="grid-template-columns: repeat({level}, 1fr);">
  {#each cards as card}
    <button
      type="button"
      class="card {card.matched ? 'matched' : ''} {card !== firstCard &&
      card !== secondCard &&
      !card.matched
        ? 'hidden'
        : ''}"
      on:click={() => flipCard(card)}
      aria-label="Flip card"
    >
      {card.text}
    </button>
  {/each}
</div>

{#if showFireworks}
  <div class="fireworks">
    <div class="explosion"></div>
    <div class="explosion"></div>
    <div class="explosion"></div>
  </div>
{/if}

<style>
  .grid {
    display: grid;
    gap: 10px;
  }

  .card {
    width: 100px;
    height: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 5px;
    cursor: pointer;
    font-size: 24px;
    font-weight: bold;
    user-select: none;
  }

  .card.matched {
    background-color: #28a745;
    color: white;
    cursor: default;
  }

  .card.hidden {
    background-color: #6c757d;
    color: transparent;
  }

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
  }

  .explosion:nth-child(2) {
    background-color: yellow;
    animation-delay: 0.2s;
  }

  .explosion:nth-child(3) {
    background-color: blue;
    animation-delay: 0.4s;
  }

  @keyframes explode {
    0% {
      transform: scale(0.5);
      opacity: 1;
    }
    100% {
      transform: scale(3);
      opacity: 0;
    }
  }
</style>
