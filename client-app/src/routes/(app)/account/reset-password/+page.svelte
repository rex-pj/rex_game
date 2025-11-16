<script lang="ts">
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  let { form } = $props();

  let token = $derived(page.url.searchParams.get("token") || "");
</script>

<div class="auth-wrapper">
  <div class="vh-100 d-flex align-items-center justify-content-center">
    <div class="reset-card">
      <div class="reset-title">Đặt lại mật khẩu</div>
      <div class="reset-desc">Nhập mật khẩu mới cho tài khoản của bạn.</div>
      {#if form?.message}
        <div class="alert alert-danger" role="alert">
          {form.message}
        </div>
      {/if}
      <form method="POST" autocomplete="off" use:enhance>
        <div class="mb-3">
          <label for="password" class="form-label">Mật khẩu mới</label>
          <input
            type="password"
            class="form-control"
            id="password"
            name="password"
            required
            minlength="6"
            maxlength="16"
            autocomplete="new-password"
            placeholder="Nhập mật khẩu mới"
          />
          <input type="hidden" id="token" name="token" required value={token} />
        </div>
        <div class="mb-3">
          <label for="password_confirm" class="form-label">Xác nhận mật khẩu</label>
          <input
            type="password"
            class="form-control"
            id="password_confirm"
            name="password_confirm"
            required
            minlength="6"
            maxlength="16"
            autocomplete="new-password"
            placeholder="Nhập lại mật khẩu"
          />
        </div>
        <button type="submit" class="btn btn-primary w-100"> Đặt lại mật khẩu </button>
        <div class="mt-3 text-center">
          <a href="/account/login" class="text-decoration-none">Quay lại đăng nhập</a>
        </div>
      </form>
    </div>
  </div>
</div>

<style>
  .auth-wrapper {
    background-color: var(--background-color);
  }

  .reset-card {
    background: white;
    border-radius: 16px;
    box-shadow: 0 4px 12px var(--box-shadow-color);
    padding: 2.5rem 2rem 2rem 2rem;
    margin: 0 auto;
    max-width: 400px;
    position: relative;
    overflow: hidden;
  }

  .reset-card::before {
    content: "";
    position: absolute;
    top: -60px;
    left: -60px;
    width: 160px;
    height: 160px;
    background: var(--primary-color);
    opacity: 0.15;
    border-radius: 50%;
    z-index: 0;
  }

  .reset-title {
    font-weight: 700;
    font-size: 1.5rem;
    text-align: center;
    margin-bottom: 1rem;
    color: var(--primary-color);
    position: relative;
    z-index: 1;
  }

  .reset-desc {
    text-align: center;
    color: #6c757d;
    margin-bottom: 2rem;
    font-size: 1rem;
    position: relative;
    z-index: 1;
  }

  .form-label {
    font-weight: 500;
  }

  .alert {
    font-size: 0.97rem;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border-radius: 8px;
    position: relative;
    z-index: 1;
  }

  .alert-danger {
    background: #ffeaea;
    color: #d32f2f;
    border: 1px solid #ffcdd2;
  }

  form {
    position: relative;
    z-index: 1;
  }
</style>
