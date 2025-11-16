<script lang="ts">
  import { enhance } from "$app/forms";
  let { form } = $props();
</script>

<div class="auth-wrapper">
  <div class="d-flex justify-content-center align-items-center vh-100">
    <div class="auth-card card shadow p-4" style="min-width: 350px; max-width: 400px; width: 100%;">
      <h3 class="mb-3 text-center">Quên mật khẩu</h3>
      <p class="text-center text-muted mb-4">
        Nhập email của bạn để nhận liên kết đặt lại mật khẩu
      </p>
      {#if form?.message}
        <div class="alert alert-danger" role="alert">
          {form.message}
        </div>
      {/if}
      <form method="POST" use:enhance>
        <div class="mb-3">
          <label for="email" class="form-label">Email</label>
          <input
            id="email"
            type="email"
            name="email"
            class="form-control"
            class:is-invalid={form?.field_errors && form.field_errors["email"]}
            value={form?.values?.email ?? ""}
            required
            placeholder="Nhập email của bạn"
          />
          {#if form?.field_errors && form.field_errors["email"]}
            <p class="invalid-feedback">{form.field_errors["email"]}</p>
          {/if}
        </div>
        <button type="submit" class="btn btn-primary w-100">Gửi yêu cầu</button>
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

  .auth-card {
    background: white;
    border: none;
    box-shadow: 0 4px 12px var(--box-shadow-color) !important;
  }

  .auth-card h3 {
    color: var(--primary-color);
    font-weight: 700;
  }
</style>
