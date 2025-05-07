<script>
  import FlashcardMatching from "./FlashcardMatching.svelte";
</script>

<ul class="nav nav-pills nav-fill" id="pills-tab" role="tablist">
  <li class="nav-item" role="presentation">
    <span
      class="nav-link active"
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
      class="nav-link"
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
      class="nav-link"
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
      class="nav-link"
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
    margin: 30px 0 10px 0;
  }

  .nav-pills .nav-item {
    margin: 0;
    position: relative;
  }

  .nav-pills .nav-item .nav-link span {
    position: relative;
    z-index: 2;
  }

  .nav-pills .nav-item .nav-link {
    padding: 1rem 0.5rem;
    color: var(--primary-color);
    background: var(--nav-hover-color);
    border-radius: 80px 80px 80px 80px;
    font-weight: 600;
    font-size: 1rem;
    letter-spacing: 1px;
    max-width: 200px;
    position: relative;
    margin: auto;
  }

  .nav-pills .nav-item .nav-link:before,
  .nav-pills .nav-item .nav-link::after {
    content: "";
    width: 70px;
    height: 70px;
    border-radius: 50px;
    background: var(--nav-hover-color);
    position: absolute;
    z-index: 0;
    left: 30%;
  }

  .nav-pills .nav-item .nav-link:before {
    bottom: 7px;
    width: 60px;
    height: 60px;
  }

  .nav-pills .nav-item .nav-link::after {
    left: 47%;
    bottom: 10px;
  }

  .nav-pills .nav-item .nav-link.active {
    color: var(--background-color);
    background: var(--primary-color);
  }

  .nav-pills .nav-item .nav-link.active::before,
  .nav-pills .nav-item .nav-link.active::after {
    background: var(--primary-color);
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
</style>
