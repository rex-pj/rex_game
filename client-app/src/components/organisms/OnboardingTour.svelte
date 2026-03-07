<script lang="ts">
  import { onMount } from "svelte";

  const STORAGE_KEY = "qhortus_tour_seen";

  const steps = [
    {
      icon: "fa-solid fa-bolt",
      iconColor: "#3b82f6",
      title: "Học vẹt? Thôi đi. 🎮",
      description:
        "Qhortus biến ôn bài thành trận đấu thực sự — áp lực thời gian, phần thưởng, cạnh tranh. Chính xác những gì não cần để nhớ lâu. Tour nhanh 30 giây thôi!",
      cta: "Xem tiếp →",
    },
    {
      icon: "fa-solid fa-layer-group",
      iconColor: "#10b981",
      title: "Chọn đúng chủ đề cần ôn",
      description:
        "Từ vựng tiếng Anh trước kỳ thi? Kiến thức khoa học? Chọn chủ đề ngay — có hàng trăm bộ flashcard đang chờ bạn chinh phục.",
      cta: "Tiếp theo →",
    },
    {
      icon: "fa-solid fa-gamepad",
      iconColor: "#f59e0b",
      title: "4 chế độ, 4 cách ghi nhớ",
      description:
        "Speed Match rèn phản xạ. Spelling rèn chính tả. Quiz rèn lý luận. Ghép đôi rèn liên kết. Chơi đủ 4 để não ghi nhớ từ nhiều góc độ.",
      cta: "Tiếp theo →",
    },
    {
      icon: "fa-solid fa-trophy",
      iconColor: "#8b5cf6",
      title: "Đăng ký để không mất tiến trình 🏆",
      description:
        "Streak, điểm cao, thành tích — tất cả biến mất nếu không có tài khoản. Đăng ký miễn phí ngay để giữ mọi thứ bạn đã chinh phục.",
      cta: "Đăng ký miễn phí",
      ctaHref: "/account/signup",
      skipLabel: "Chơi thử đã",
    },
  ];

  let visible = $state(false);
  let currentStep = $state(0);

  onMount(() => {
    const seen = localStorage.getItem(STORAGE_KEY);
    if (!seen) {
      setTimeout(() => { visible = true; }, 800);
    }
  });

  function next() {
    if (currentStep < steps.length - 1) {
      currentStep++;
    } else {
      close();
    }
  }

  function close() {
    visible = false;
    localStorage.setItem(STORAGE_KEY, "1");
  }

  const step = $derived(steps[currentStep]);
</script>

{#if visible}
  <!-- Backdrop -->
  <div class="tour-backdrop" onclick={close} role="presentation"></div>

  <!-- Tour card -->
  <div class="tour-card" role="dialog" aria-modal="true" aria-label="Hướng dẫn sử dụng">
    <!-- Progress dots -->
    <div class="tour-dots">
      {#each steps as _, i}
        <span class="tour-dot" class:active={i === currentStep} class:done={i < currentStep}></span>
      {/each}
    </div>

    <!-- Close button -->
    <button class="tour-close" onclick={close} aria-label="Đóng hướng dẫn">
      <i class="fa-solid fa-xmark"></i>
    </button>

    <!-- Content -->
    <div class="tour-icon">
      <i class="{steps[currentStep].icon}" style="color: {steps[currentStep].iconColor}"></i>
    </div>
    <h2 class="tour-title">{steps[currentStep].title}</h2>
    <p class="tour-description">{steps[currentStep].description}</p>

    <!-- Actions -->
    <div class="tour-actions">
      {#if steps[currentStep].ctaHref}
        <a href={steps[currentStep].ctaHref} class="btn btn-primary tour-btn" onclick={close}>
          <i class="fa-solid fa-rocket me-2"></i>{steps[currentStep].cta}
        </a>
        {#if steps[currentStep].skipLabel}
          <button class="tour-skip" onclick={close}>{steps[currentStep].skipLabel}</button>
        {/if}
      {:else}
        <button class="btn btn-primary tour-btn" onclick={next}>
          {steps[currentStep].cta}
        </button>
        {#if currentStep === 0}
          <button class="tour-skip" onclick={close}>Bỏ qua</button>
        {/if}
      {/if}
    </div>

    <!-- Step counter -->
    <p class="tour-counter">{currentStep + 1} / {steps.length}</p>
  </div>
{/if}

<style>
  .tour-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    z-index: 1040;
    animation: fadeIn 0.25s ease;
  }

  .tour-card {
    position: fixed;
    bottom: 40px;
    right: 40px;
    width: 360px;
    background: white;
    border-radius: 20px;
    padding: 32px 28px 24px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
    z-index: 1050;
    animation: slideUp 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @media (max-width: 480px) {
    .tour-card {
      bottom: 0;
      right: 0;
      left: 0;
      width: 100%;
      border-radius: 20px 20px 0 0;
    }
  }

  .tour-dots {
    display: flex;
    gap: 6px;
    margin-bottom: 16px;
  }

  .tour-dot {
    width: 8px;
    height: 8px;
    border-radius: 999px;
    background: #e2e8f0;
    transition: all 0.3s;
  }

  .tour-dot.active {
    width: 24px;
    background: #3b82f6;
  }

  .tour-dot.done {
    background: #93c5fd;
  }

  .tour-close {
    position: absolute;
    top: 16px;
    right: 16px;
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 1rem;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 6px;
    transition: all 0.2s;
  }

  .tour-close:hover {
    color: #475569;
    background: #f1f5f9;
  }

  .tour-icon {
    font-size: 2.5rem;
    margin-bottom: 12px;
  }

  .tour-title {
    font-size: 1.2rem;
    font-weight: 700;
    color: #1e293b;
    margin-bottom: 10px;
    line-height: 1.3;
  }

  .tour-description {
    font-size: 0.95rem;
    color: #64748b;
    line-height: 1.7;
    margin-bottom: 20px;
  }

  .tour-actions {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .tour-btn {
    width: 100%;
    padding: 10px;
    font-weight: 600;
    border-radius: 12px;
  }

  .tour-skip {
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 0.875rem;
    cursor: pointer;
    padding: 4px;
    transition: color 0.2s;
    text-align: center;
  }

  .tour-skip:hover {
    color: #64748b;
  }

  .tour-counter {
    text-align: center;
    font-size: 0.8rem;
    color: #cbd5e1;
    margin: 12px 0 0;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to   { opacity: 1; }
  }

  @keyframes slideUp {
    from { opacity: 0; transform: translateY(24px) scale(0.97); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
  }
</style>
