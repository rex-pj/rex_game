<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { APP_URLS } from "$lib/common/contants.js";
  let { data, form } = $props();

  if (data.currentUser) {
    goto(APP_URLS.HOME);
  }

  if (form) {
    goto(APP_URLS.LOGIN_URL);
  }
</script>

{#if !data.currentUser}
  <div class="auth-wrapper">
    <div class="container vh-100 d-flex align-items-center justify-content-center">
      <div class="row w-100 auth-card shadow rounded overflow-hidden" style="max-width: 900px;">
        <!-- Khung giới thiệu bên trái -->
        <div class="col-md-6 auth-intro text-white d-flex flex-column justify-content-center p-5">
          <h1 class="mb-3">Chào mừng bạn!</h1>
          <p>
            Đăng ký tài khoản để trải nghiệm các tính năng tuyệt vời và học tập vui vẻ cùng Rex
            Game!
          </p>
        </div>

        <!-- Khung đăng ký bên phải -->
        <div class="col-md-6 bg-white p-5">
          <form method="POST" use:enhance>
            <h2 class="mb-4">Đăng ký</h2>
            <div class="mb-3">
              {#if form?.message}
                <div class="alert alert-danger" role="alert">
                  {form.message}
                </div>
              {/if}
              <label for="name" class="form-label">Tên đăng nhập</label>
              <input
                id="name"
                type="text"
                name="name"
                class="form-control"
                class:is-invalid={form?.field_errors && form.field_errors["name"]}
                required
                value={form?.values?.name ?? ""}
              />
              {#if form?.field_errors && form.field_errors["name"]}
                <p class="invalid-feedback">{form.field_errors["name"]}</p>
              {/if}
            </div>
            <div class="mb-3">
              <label for="email" class="form-label">Email</label>
              <input
                id="email"
                type="email"
                name="email"
                class="form-control"
                class:is-invalid={form?.field_errors && form.field_errors["email"]}
                required
                value={form?.values?.email ?? ""}
              />
              {#if form?.field_errors && form.field_errors["email"]}
                <p class="invalid-feedback">{form.field_errors["email"]}</p>
              {/if}
            </div>
            <div class="mb-3">
              <label for="display_name" class="form-label">Tên hiển thị</label>
              <input
                id="display_name"
                type="text"
                name="display_name"
                class="form-control"
                class:is-invalid={form?.field_errors && form.field_errors["display_name"]}
                required
                value={form?.values?.display_name ?? ""}
              />
              {#if form?.field_errors && form.field_errors["display_name"]}
                <p class="invalid-feedback">{form.field_errors["display_name"]}</p>
              {/if}
            </div>
            <div class="mb-3">
              <label for="password" class="form-label">Mật khẩu</label>
              <input
                id="password"
                type="password"
                name="password"
                class="form-control"
                class:is-invalid={form?.field_errors && form.field_errors["password"]}
                required
              />
              {#if form?.field_errors && form.field_errors["password"]}
                <p class="invalid-feedback">{form.field_errors["password"]}</p>
              {/if}
            </div>
            <div class="mb-3">
              <label for="password_confirm" class="form-label">Xác nhận mật khẩu</label>
              <input
                id="password_confirm"
                type="password"
                name="password_confirm"
                class="form-control"
                class:is-invalid={form?.field_errors && form.field_errors["password_confirm"]}
                required
              />
              {#if form?.field_errors && form.field_errors["password_confirm"]}
                <p class="invalid-feedback">{form.field_errors["password_confirm"]}</p>
              {/if}
            </div>
            <button type="submit" class="btn btn-primary w-100">Đăng ký</button>
            <div class="mt-3">
              <div class="mt-3 text-end">
                Đã có tài khoản?
                <a href="/account/login" class="text-decoration-none ms-1">Đăng nhập</a>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Main wrapper for signup page */
  .auth-wrapper {
    background-color: var(--background-color);
  }

  /* Card container for signup form */
  .auth-card {
    box-shadow: 0 4px 12px var(--box-shadow-color) !important;
  }

  /* Left intro section - blue color */
  .auth-intro {
    background-color: var(--primary-color) !important;
  }

  /* Heading in intro section */
  .auth-intro h1 {
    font-weight: 700;
  }

  /* Text in intro section */
  .auth-intro p {
    font-size: 1.1rem;
    line-height: 1.6;
  }
</style>
