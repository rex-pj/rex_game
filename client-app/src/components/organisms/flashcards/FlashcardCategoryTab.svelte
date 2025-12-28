<ul class="nav nav-pills nav-fill" id="pills-tab" role="tablist">
  <li class="nav-item" role="presentation">
    <span
      class="nav-link cloud-button active"
      id="pills-matching-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-matching"
      role="tab"
      aria-controls="pills-matching"
      aria-selected="false"><span>Ghép cặp</span></span
    >
  </li>
  <li class="nav-item" role="presentation">
    <span
      class="nav-link cloud-button"
      id="pills-quiz-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-quiz"
      role="tab"
      aria-controls="pills-quiz"
      aria-selected="true"><span>Trắc nghiệm</span></span
    >
  </li>
  <li class="nav-item" role="presentation">
    <span
      class="nav-link cloud-button"
      id="pills-spelling-tab"
      data-bs-toggle="pill"
      data-bs-target="#pills-spelling"
      role="tab"
      aria-controls="pills-spelling"
      aria-selected="false"><span>Đánh Vần</span></span
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

  /* Responsive for tablets */
  @media (max-width: 992px) {
    .nav-pills {
      margin: 30px 0 10px 0;
      flex-wrap: wrap;
      gap: 15px;
    }

    .nav-pills .nav-item {
      margin: 0 5px;
    }

    .cloud-button {
      padding: 15px 30px;
      height: 60px;
    }

    .nav-pills .nav-item .nav-link span {
      font-size: 1.2rem;
    }

    .cloud-button::before {
      width: 50px;
      height: 50px;
      top: -12px;
      left: 30px;
      box-shadow: 30px -18px 0 0 var(--primary-light);
    }

    .cloud-button::after {
      width: 45px;
      height: 45px;
      top: -15px;
      left: 100px;
    }
  }

  /* Responsive for mobile */
  @media (max-width: 576px) {
    .nav-pills {
      margin: 20px 0 10px 0;
      gap: 10px;
      justify-content: center;
    }

    .nav-pills .nav-item {
      margin: 0;
      flex: 0 0 calc(50% - 10px);
    }

    .cloud-button {
      padding: 12px 15px;
      height: 50px;
      font-size: 1rem;
      border-radius: 30px;
    }

    .nav-pills .nav-item .nav-link span {
      font-size: 0.9rem;
    }

    /* Hide cloud decorations on mobile */
    .cloud-button::before,
    .cloud-button::after {
      display: none;
    }

    .cloud-button {
      animation: none;
    }
  }
</style>
