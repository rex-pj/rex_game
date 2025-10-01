<script lang="ts">
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  let { form } = $props();

  let token = $derived(page.url.searchParams.get("token") || "");
</script>

<div class="vh-100">
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
    </form>
  </div>
</div>

<style>
  .reset-card {
    background: #fff;
    border-radius: 16px;
    box-shadow:
      0 4px 24px rgba(0, 0, 0, 0.08),
      0 1.5px 6px rgba(0, 0, 0, 0.04);
    padding: 2.5rem 2rem 2rem 2rem;
    margin: 0 auto;
    max-width: 400px;
    margin-top: 60px;
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
    background: linear-gradient(135deg, #4f8cff 0%, #6fd6ff 100%);
    opacity: 0.15;
    border-radius: 50%;
    z-index: 0;
  }
  .reset-title {
    font-weight: 700;
    font-size: 1.5rem;
    text-align: center;
    margin-bottom: 1rem;
    letter-spacing: 0.5px;
  }
  .reset-desc {
    text-align: center;
    color: #6c757d;
    margin-bottom: 2rem;
    font-size: 1rem;
  }
  .form-label {
    font-weight: 500;
  }
  .form-control:focus {
    border-color: #4f8cff;
    box-shadow: 0 0 0 0.2rem rgba(79, 140, 255, 0.15);
  }
  .btn-primary {
    background: linear-gradient(90deg, #4f8cff 0%, #6fd6ff 100%);
    border: none;
    font-weight: 600;
    letter-spacing: 0.5px;
    transition: background 0.2s;
  }
  .btn-primary:disabled {
    background: #b3d4fc;
    color: #fff;
  }
  .alert {
    font-size: 0.97rem;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border-radius: 8px;
  }
  .alert-danger {
    background: #ffeaea;
    color: #d32f2f;
    border: 1px solid #ffcdd2;
  }
</style>
