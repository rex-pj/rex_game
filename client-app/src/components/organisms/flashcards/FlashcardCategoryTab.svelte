<script>
  import FlashcardMatching from "./FlashcardMatching.svelte";
</script>

<ul class="nav nav-pills nav-fill" id="pills-tab" role="tablist">
  <li class="nav-item" role="presentation">
    <span
      class="nav-link cloud-button active"
      id="pills-matching-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-matching"
      role="tab"
      aria-controls="pills-matching"
      aria-selected="false"><span>Thẻ ghép cặp</span></span
    >
  </li>
  <li class="nav-item" role="presentation">
    <span
      class="nav-link cloud-button"
      id="pills-animals-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-animals"
      role="tab"
      aria-controls="pills-animals"
      aria-selected="true"><span>Động vật</span></span
    >
  </li>
  <li class="nav-item" role="presentation">
    <span
      class="nav-link cloud-button"
      id="pills-numbers-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-numbers"
      role="tab"
      aria-controls="pills-numbers"
      aria-selected="false"><span>Những con số</span></span
    >
  </li>
  <li class="nav-item" role="presentation">
    <span
      class="nav-link cloud-button"
      id="pills-alphabet-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-alphabet"
      role="tab"
      aria-controls="pills-alphabet"
      aria-selected="false"><span>Chữ cái</span></span
    >
  </li>
</ul>
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
    id="pills-animals"
    role="tabpanel"
    aria-labelledby="pills-animals-tab"
    tabindex="0"
  >
    Animals
  </div>
  <div
    class="tab-pane"
    id="pills-numbers"
    role="tabpanel"
    aria-labelledby="pills-numbers-tab"
    tabindex="0"
  >
    Numbers
  </div>
  <div
    class="tab-pane"
    id="pills-alphabet"
    role="tabpanel"
    aria-labelledby="pills-alphabet-tab"
    tabindex="0"
  >
    Alphabet
  </div>
</div>

<style>
  .nav-pills {
    margin: 54px 0 10px 0;
  }

  .nav-pills .nav-item {
    margin: 0 15px;
    position: relative;
  }

  .nav-pills .nav-item .nav-link span {
    position: relative;
    z-index: 2;
    font-weight: bold;
    font-size: 1.5rem;
  }

  .tab-content {
    padding: 5px 0px;
    box-shadow: 0 4px 10px var(--box-shadow-color);
    min-height: 400px;
    border-radius: 0 0 8px 8px;
  }

  .tab-pane {
    padding: 5px 0px;
    border-radius: 3px;
    background-color: var(--background-color);
  }

  .cloud-button {
    position: relative;
    background: var(--primary-light);
    border: none;
    padding: 20px 50px;
    font-size: 1.2rem;
    border-radius: 50px;
    height: 80px;
    box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
    color: var(--text-primary);
    cursor: pointer;
    transition: background 0.8s ease;
  }

  .nav-pills .nav-link.active {
    background: var(--secondary-color);
    color: var(--white);
  }

  .cloud-button::before,
  .cloud-button::after {
    content: "";
    position: absolute;
    background: var(--primary-light);
    border-radius: 50%;
    z-index: 2;
    transition:
      background 0.8s ease,
      transform 0.8s ease;
  }

  /* Small cloud parts */
  .cloud-button::before {
    width: 80px;
    height: 80px;
    top: -20px;
    left: 50px;
    box-shadow: 49px -30px 0 0 var(--primary-light);
  }

  .cloud-button::after {
    width: 70px;
    height: 70px;
    top: -24px;
    left: 158px;
    z-index: -1;
  }

  .nav-item:hover .cloud-button,
  .nav-item .nav-link.active.cloud-button {
    background: var(--primary-hover-color);
    color: var(--white);
    transform: translateY(-5px);
    box-shadow: 0 10px 20px var(--primary-light);
  }

  .nav-item:hover .cloud-button::before,
  .nav-item .nav-link.active.cloud-button::before {
    background: var(--primary-hover-color);
    box-shadow: 49px -30px 0 0 var(--secondary-color);
  }

  .nav-item:hover .cloud-button::after,
  .nav-item .nav-link.active.cloud-button::after {
    background: var(--primary-hover-color);
  }

  .nav-pills .nav-item:hover .nav-link span {
    color: var(--white);
  }

  .cloud-button:active {
    transform: translateY(2px);
    box-shadow: 0 5px 10px rgba(0, 0, 0, 0.15);
  }

  /* Gentle floating animation */
  @keyframes float {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-4px);
    }
  }

  .cloud-button {
    animation: float 3s ease-in-out infinite;
  }
</style>
