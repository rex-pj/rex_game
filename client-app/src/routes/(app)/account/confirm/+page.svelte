<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation"; // For redirect if needed
  import { APP_URLS } from "$lib/common/contants";

  // State for UI
  let status: "idle" | "loading" | "success" | "error" = "idle";
  let message = "";
  let errorMessage = "";

  // Get token from URL query params
  onMount(async () => {
    const urlParams = new URLSearchParams(window.location.search);
    const token = urlParams.get("token");

    if (!token) {
      status = "error";
      errorMessage = "Invalid or missing token.";
      return;
    }

    status = "loading";

    try {
      const response = await fetch("/account/confirm", {
        method: "POST",
        body: JSON.stringify({ token }),
      });

      if (response.ok) {
        status = "success";
        message = "Your account has been successfully activated!";
        // Optional: Redirect to login after 3s
        setTimeout(() => goto(APP_URLS.LOGIN_URL), 3000);
      } else {
        status = "error";
        const data = await response.json();
        errorMessage = data.error?.message || "Failed to activate account. Please try again.";
      }
    } catch (err) {
      status = "error";
      errorMessage = "An unexpected error occurred. Please try again later.";
    }
  });
</script>

<div class="confirm-wrapper">
  <div class="container d-flex justify-content-center align-items-center min-vh-100">
    <div class="confirm-card card shadow-sm p-4" style="max-width: 500px; width: 100%;">
      <div class="card-body text-center">
        {#if status === "loading"}
          <div class="spinner-border mb-3" role="status">
            <span class="visually-hidden">Loading...</span>
          </div>
          <h5 class="card-title">Đang kích hoạt tài khoản...</h5>
        {:else if status === "success"}
          <div class="success-icon mb-3">
            <i class="bi bi-check-circle-fill"></i>
          </div>
          <h5 class="card-title mb-3">Thành công!</h5>
          <p class="card-text mb-2">{message}</p>
          <p class="text-muted mb-4">Đang chuyển hướng đến trang đăng nhập...</p>
          <a href={APP_URLS.LOGIN_URL} class="btn btn-primary">Đi đến trang đăng nhập</a>
        {:else if status === "error"}
          <div class="error-icon mb-3">
            <i class="bi bi-x-circle-fill"></i>
          </div>
          <h5 class="card-title mb-3">Lỗi</h5>
          <p class="card-text mb-4">{errorMessage}</p>
          <a href={APP_URLS.SIGNUP_URL} class="btn btn-outline-secondary">Thử đăng ký lại</a>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .confirm-wrapper {
    background-color: var(--background-color);
  }

  .confirm-card {
    border: none;
    border-radius: 16px;
    box-shadow: 0 4px 12px var(--box-shadow-color) !important;
  }

  .spinner-border {
    color: var(--primary-color);
    width: 3rem;
    height: 3rem;
  }

  .success-icon i {
    font-size: 4rem;
    color: #28a745;
  }

  .error-icon i {
    font-size: 4rem;
    color: #dc3545;
  }

  .card-title {
    color: var(--primary-color);
    font-weight: 700;
    font-size: 1.5rem;
  }

  .card-text {
    font-size: 1.1rem;
    line-height: 1.6;
  }

  .success-icon i,
  .error-icon i {
    animation: fadeIn 0.5s ease-in;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.8);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
</style>
