<script lang="ts">
  interface Props {
    activeMode: string;
    onTabChange: (mode: string) => void;
  }

  let { activeMode, onTabChange }: Props = $props();

  const tabs = [
    { mode: 'matching',    label: 'Ghép cặp' },
    { mode: 'quiz',        label: 'Trắc nghiệm' },
    { mode: 'spelling',    label: 'Đánh Vần' },
    { mode: 'speed-match', label: 'Tốc độ' },
  ];
</script>

<ul class="nav nav-pills nav-fill" role="tablist">
  {#each tabs as tab}
    <li class="nav-item" role="presentation">
      <button
        class="nav-link cloud-button"
        class:active={activeMode === tab.mode}
        role="tab"
        aria-selected={activeMode === tab.mode}
        onclick={() => onTabChange(tab.mode)}
      >
        <span>{tab.label}</span>
      </button>
    </li>
  {/each}
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
    width: 100%;
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

  @keyframes float {
    0%, 100% { transform: translateY(0); }
    50%       { transform: translateY(-4px); }
  }

  .cloud-button {
    animation: float 3s ease-in-out infinite;
  }

  .cloud-button.active {
    animation: none;
  }

  /* Responsive — tablets */
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

  /* Responsive — mobile */
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

    .cloud-button::before,
    .cloud-button::after {
      display: none;
    }

    .cloud-button {
      animation: none;
    }
  }
</style>
