<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto, invalidateAll } from "$app/navigation";
  import { APP_URLS } from "$lib/common/contants.js";
  import type { SubmitFunction } from "@sveltejs/kit";
  let { data } = $props();
  let isSubmitting = $state(false);

  if (data.currentUser) {
    goto(APP_URLS.HOME);
  }

  const handleEnhance: SubmitFunction = () => {
    isSubmitting = true;

    return async ({ result, update }) => {
      isSubmitting = false;

      if (result.type === "failure") {
        await update();
      } else if (result.type === "redirect") {
        await invalidateAll();
        goto(result.location);
      }
    };
  };
</script>

<svelte:head>
  <title>Đăng nhập — Qhortus</title>
  <meta name="description" content="Đăng nhập vào Qhortus để tiếp tục học flashcard và theo dõi tiến trình của bạn." />
  <meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="auth-wrapper">
  <div class="container vh-100 d-flex align-items-center justify-content-center">
    <div class="row w-100 auth-card shadow rounded overflow-hidden" style="max-width: 900px;">
      <!-- Intro bên trái -->
      <div class="col-md-6 auth-intro text-white d-flex flex-column justify-content-center p-5">
        <h2 class="mb-3">Chào mừng trở lại!</h2>
        <p>
          Đăng nhập để tiếp tục truy cập vào tài khoản của bạn và khám phá nhiều tính năng hấp dẫn.
        </p>
      </div>
      <!-- Khung đăng nhập bên phải -->
      <div class="col-md-6 bg-white p-5">
        <h3 class="mb-4">Đăng nhập</h3>
        <form method="POST" action="?/login" use:enhance={handleEnhance}>
          <div class="mb-3">
            <label for="email" class="form-label">Email</label>
            <input
              id="email"
              type="email"
              name="email"
              class="form-control"
              required
              autocomplete="username"
            />
          </div>
          <div class="mb-3">
            <label for="password" class="form-label">Mật khẩu</label>
            <input
              id="password"
              type="password"
              class="form-control"
              name="password"
              required
              autocomplete="current-password"
            />
          </div>
          <div class="mb-3 form-check">
            <input id="remember" type="checkbox" class="form-check-input" />
            <div class="d-flex align-items-between justify-content-between">
              <label class="form-check-label" for="remember">Ghi nhớ đăng nhập</label>
              <a href="/account/forgot-password" class="text-decoration-none">Quên mật khẩu?</a>
            </div>
          </div>

          <button type="submit" class="btn btn-primary w-100" disabled={isSubmitting}>
            {#if isSubmitting}
              <span class="spinner-border spinner-border-sm me-2"></span> Đang đăng nhập...
            {:else}
              Đăng nhập
            {/if}
          </button>
          <div class="mt-3">
            <div>
              Chưa có tài khoản?
              <a href="/account/signup" class="text-decoration-none ms-1">Đăng ký</a>
            </div>
          </div>
        </form>
      </div>
    </div>
  </div>
</div>

<style>
  /* Main wrapper for login page */
  .auth-wrapper {
    background-color: var(--background-color);
  }

  /* Card container for login form */
  .auth-card {
    box-shadow: 0 4px 12px var(--box-shadow-color) !important;
  }

  /* Left intro section - blue color */
  .auth-intro {
    background-color: var(--primary-color) !important;
  }

  /* Heading in intro section */
  .auth-intro h2 {
    font-weight: 700;
  }

  /* Text in intro section */
  .auth-intro p {
    font-size: 1.1rem;
    line-height: 1.6;
  }
</style>
