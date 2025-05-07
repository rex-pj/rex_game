<script lang="ts">
  let level = 3; // Bắt đầu từ level 3x3
  let cards: any[] = [];
  let firstCard: { id: number; text: string; matched: boolean } | null = null;
  let secondCard: { id: number; text: string; matched: boolean } | null = null;
  let showFireworks = false;

  function generateCards(level: number) {
    const allCards = [
      {
        id: 1,
        text: "Allosaurus",
        image_url: "https://kids-flashcards.com/images/en/43/cards/picture-flashcard/new.webp",
        matched: false,
      },
      {
        id: 2,
        text: "Ankylosaurus",
        image_url: "../../../assets/imgs/flascards/Ankylosaurus-flashcard.png",
        matched: false,
      },
      {
        id: 3,
        text: "Brachiosaurus",
        image_url: "../../../assets/imgs/flascards/Brachiosaurus-flashcard.png",
        matched: false,
      },
      {
        id: 4,
        text: "Allosaurus",
        image_url: "https://kids-flashcards.com/images/en/43/cards/picture-flashcard/new.webp",
        matched: false,
      },
      {
        id: 5,
        text: "Ankylosaurus",
        image_url: "../../../assets/imgs/flascards/Ankylosaurus-flashcard.png",
        matched: false,
      },
      {
        id: 6,
        text: "Brachiosaurus",
        image_url: "../../../assets/imgs/flascards/Brachiosaurus-flashcard.png",
        matched: false,
      },
    ];
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
    if (!card || card.matched || card === firstCard || card === secondCard) {
      return;
    }

    if (!firstCard) {
      firstCard = card;
    } else if (!secondCard) {
      secondCard = card;

      if (firstCard.text === secondCard.text) {
        firstCard.matched = true;
        secondCard.matched = true;
        firstCard = null;
        secondCard = null;

        if (cards.every((c) => c.matched)) {
          showFireworks = true;
          setTimeout(() => {
            level++;
            resetGame();
          }, 3000);
        }
      } else {
        setTimeout(() => {
          firstCard = null;
          secondCard = null;
        }, 1000);
      }
    }
  }

  resetGame();
</script>

<div class="container-sm card-container px-0">
  <div class="grid" style="grid-template-columns: repeat({level}, minmax(112px, 1fr));">
    {#each cards as card}
      <div
        class="flip-box card-holder {card === firstCard || card === secondCard
          ? 'flipped'
          : ''} {card.matched ? 'matched' : ''}"
      >
        <div class="flip-box-inner">
          <button
            type="button"
            class="card flip-box-front {card === firstCard || card === secondCard || card.matched
              ? 'hidden'
              : ''}"
            on:click={() => flipCard(card)}
            aria-label="Flip front card"
          >
          </button>
          <button
            type="button"
            class="card flip-box-back {card !== firstCard && card !== secondCard && !card.matched
              ? 'hidden'
              : ''}"
            on:click={() => flipCard(card)}
            aria-label="Flip back card"
            style="background-image: url({card.image_url}); background-size: cover; background-position: center;"
          >
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>

{#if showFireworks}
  <div class="fireworks">
    <div class="explosion"></div>
    <div class="explosion"></div>
    <div class="explosion"></div>
  </div>
{/if}

<style>
  .card-container {
    max-width: 600px;
    width: auto;
  }

  .grid {
    display: grid;
    gap: 10px;
    perspective: 1000px;
  }

  .card-holder {
    width: 100%;
    height: 240px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 5px;
    cursor: pointer;
    font-size: 24px;
    font-weight: bold;
    user-select: none;
    max-width: 300px;
    margin: auto;
  }

  .card {
    width: 100%;
    background-color: #f8f9fa;
    border: 1px solid #dee2e6;
    height: 100%;
    padding: 0;
    background-image: url("../../../assets/imgs/door.png");
    background-size: cover;
  }

  .card-holder.matched .card {
    background-color: #28a745;
    color: white;
    cursor: default;
  }

  .flip-box-inner {
    position: relative;
    width: 100%;
    height: 100%;
    text-align: center;
    transition: transform 0.8s;
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
  }

  .flip-box-front {
    background-color: #bbb;
    color: black;
  }

  .flip-box-back {
    background-color: #555;
    color: white;
    transform: rotateY(180deg);
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
