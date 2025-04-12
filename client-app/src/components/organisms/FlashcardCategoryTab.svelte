<ul class="nav nav-pills nav-fill" id="pills-tab" role="tablist">
  <li class="nav-item" role="presentation">
    <button
      class="nav-link active"
      id="pills-matching-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-matching"
      type="button"
      role="tab"
      aria-controls="pills-matching"
      aria-selected="false">Thẻ ghép cặp</button
    >
  </li>
  <li class="nav-item" role="presentation">
    <button
      class="nav-link"
      id="pills-animals-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-animals"
      type="button"
      role="tab"
      aria-controls="pills-animals"
      aria-selected="true">Động vật</button
    >
  </li>
  <li class="nav-item" role="presentation">
    <button
      class="nav-link"
      id="pills-numbers-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-numbers"
      type="button"
      role="tab"
      aria-controls="pills-numbers"
      aria-selected="false">Những con số</button
    >
  </li>
  <li class="nav-item" role="presentation">
    <button
      class="nav-link"
      id="pills-alphabet-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-alphabet"
      type="button"
      role="tab"
      aria-controls="pills-alphabet"
      aria-selected="false">Chữ cái</button
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
    Matching
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
    display: flex;
    margin: 0 14px;
  }

  .nav-pills .nav-item {
    margin: 0;
    position: relative;
  }

  .nav-pills .nav-item:hover {
    z-index: 2;
  }

  .nav-pills .nav-item::after,
  .nav-pills .nav-item::before {
    content: "";
    width: 15px;
    height: 15px;
    position: absolute;
    bottom: 0;
  }

  .nav-pills .nav-item::before {
    left: -15px;
  }

  .nav-pills .nav-item::after {
    right: -15px;
  }

  .nav-pills .nav-item:has(.active)::before,
  .nav-pills .nav-item:has(.active)::after {
    z-index: 1;
  }

  .nav-pills .nav-item:has(.active)::after {
    background: -webkit-radial-gradient(
      100% 0,
      circle,
      transparent 15px,
      var(--primary-color) 11px
    );
  }

  .nav-pills .nav-item:has(.active)::before {
    background: -webkit-radial-gradient(0 0, circle, transparent 15px, var(--primary-color) 11px);
  }

  .nav-pills .nav-item:hover::after {
    background: -webkit-radial-gradient(
      100% 0,
      circle,
      transparent 15px,
      var(--primary-hover-color) 11px
    );
  }

  .nav-pills .nav-item:hover::before {
    background: -webkit-radial-gradient(
      0 0,
      circle,
      #cc000000 15px,
      var(--primary-hover-color) 11px
    );
  }

  .nav-pills .nav-link {
    color: var(--secondary-color);
    font-weight: 700;
    font-size: 16px;
    border-bottom: none;
    border-radius: 15px 15px 0 0;
    padding: 10px 20px;
    transition: none;
    height: 100%;
    background-color: var(--nav-bg-color);
  }

  .nav-pills .nav-link.active {
    background-color: var(--primary-color);
    color: #fff;
    z-index: 1;
  }

  .nav-pills .nav-link:hover {
    background-color: var(--primary-hover-color);
    color: #fff;
  }

  .tab-content {
    padding: 10px;
    background-color: #fff;
    box-shadow: 0 4px 10px var(--box-shadow-color);
    min-height: 400px;
    border-radius: 0 0 8px 8px;
  }

  .tab-pane {
    padding: 15px;
    border-radius: 3px;
    background-color: var(--background-color);
  }
</style>
