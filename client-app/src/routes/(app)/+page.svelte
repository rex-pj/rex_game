<script>
  import FlashcardCategoryTab from "../../components/organisms/flashcards/FlashcardCategoryTab.svelte";
  import FlashcardMatching from "../../components/organisms/flashcards/FlashcardMatching.svelte";
  import FlashcardQuiz from "../../components/organisms/flashcards/FlashcardQuiz.svelte";
  import FlashcardSpeedMatch from "../../components/organisms/flashcards/FlashcardSpeedMatch.svelte";
  import FlashcardSpelling from "../../components/organisms/flashcards/FlashcardSpelling.svelte";
  import OnboardingTour from "../../components/organisms/OnboardingTour.svelte";
</script>

<svelte:head>
  <title>Qhortus — Học flashcard qua game thú vị</title>
  <meta name="description" content="Học flashcard theo cách gamified với Quiz, Ghép đôi, Spelling và Speed Match. Nâng cao kiến thức mỗi ngày cùng Qhortus!" />
  <meta property="og:title" content="Qhortus — Học flashcard qua game thú vị" />
  <meta property="og:description" content="Học flashcard theo cách gamified với Quiz, Ghép đôi, Spelling và Speed Match." />
  <meta property="og:url" content="/" />
</svelte:head>

<div class="home-wrapper">

  <!-- Hero Section -->
  <section class="hero-section text-center py-5">
    <div class="container">
      <h1 class="display-4 fw-bold mb-3">Học mãi mà vẫn quên?<br/>Thử cách não thực sự ghi nhớ.</h1>
      <p class="lead text-muted mx-auto mb-4" style="max-width: 580px;">
        Áp lực thời gian + phần thưởng + cạnh tranh = công thức ghi nhớ thực sự. Qhortus biến mỗi lần ôn bài thành trận đấu — 5 phút đủ để não nhớ cả tuần.
      </p>
      <div class="d-flex gap-3 justify-content-center flex-wrap mb-5">
        <a href="#game-section" class="btn btn-primary btn-lg px-4">
          <i class="fa-solid fa-bolt me-2"></i>Thử ngay — không cần đăng ký
        </a>
        <a href="/account/signup" class="btn btn-outline-secondary btn-lg px-4">
          Đăng ký miễn phí <i class="fa-solid fa-arrow-right ms-2"></i>
        </a>
      </div>

      <!-- Feature badges -->
      <div class="d-flex flex-wrap gap-3 justify-content-center">
        <span class="feature-badge">
          <i class="fa-solid fa-shuffle text-primary me-1"></i> Ghép đôi
        </span>
        <span class="feature-badge">
          <i class="fa-solid fa-list-check text-success me-1"></i> Trắc nghiệm
        </span>
        <span class="feature-badge">
          <i class="fa-solid fa-spell-check text-warning me-1"></i> Spelling
        </span>
        <span class="feature-badge">
          <i class="fa-solid fa-bolt text-danger me-1"></i> Speed Match
        </span>
        <span class="feature-badge">
          <i class="fa-solid fa-trophy text-info me-1"></i> Bảng xếp hạng
        </span>
        <span class="feature-badge">
          <i class="fa-solid fa-medal me-1" style="color:#8b5cf6"></i> Thành tích
        </span>
      </div>
    </div>
  </section>

  <!-- Game Section -->
  <div class="container py-4" id="game-section">
    <h2 class="fw-bold mb-4 text-center">Chọn chủ đề và bắt đầu</h2>
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

<OnboardingTour />

<style>
  .hero-section {
    background: linear-gradient(160deg, rgba(59,130,246,0.06) 0%, rgba(139,92,246,0.06) 100%);
    border-bottom: 1px solid rgba(59,130,246,0.1);
  }

  .hero-section h1 {
    color: var(--primary-color);
    line-height: 1.2;
  }

  .feature-badge {
    background: white;
    border: 1px solid rgba(0,0,0,0.1);
    border-radius: 999px;
    padding: 6px 16px;
    font-size: 0.9rem;
    font-weight: 500;
    box-shadow: 0 1px 4px rgba(0,0,0,0.06);
  }

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

  /* Responsive for tablets */
  @media (max-width: 768px) {
    .welcome-section h1 {
      font-size: 2rem;
    }

    .welcome-section .lead {
      font-size: 1rem;
      padding: 0 15px;
    }
  }

  /* Responsive for mobile */
  @media (max-width: 480px) {
    .home-wrapper {
      min-height: auto;
    }

    .welcome-section {
      margin-bottom: 20px !important;
    }

    .welcome-section h1 {
      font-size: 1.5rem;
      margin-bottom: 10px !important;
    }

    .welcome-section .lead {
      font-size: 0.9rem;
    }

    .tab-content {
      min-height: 300px;
    }
  }
</style>
