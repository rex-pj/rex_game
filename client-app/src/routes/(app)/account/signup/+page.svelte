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
  <div class="container vh-100 d-flex align-items-center justify-content-center">
    <div class="row justify-content-center">
      <div class="col-lg-12 col-md-8 col-sm-10">
        <div class="row shadow rounded overflow-hidden">
          <div class="col-md-6 bg-primary text-white d-flex flex-column justify-content-center p-4">
            <h1 class="mb-3">Chào mừng bạn!</h1>
            <p class="mb-0">
              Đăng ký tài khoản để trải nghiệm các tính năng tuyệt vời của chúng tôi.
            </p>
          </div>
          <div class="col-md-6 bg-white p-4">
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
                  class={{
                    "is-invalid": form?.field_errors && form.field_errors["name"],
                    "form-control": true,
                  }}
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
                  class={{
                    "is-invalid": form?.field_errors && form.field_errors["email"],
                    "form-control": true,
                  }}
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
                  class={{
                    "is-invalid": form?.field_errors && form.field_errors["display_name"],
                    "form-control": true,
                  }}
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
                  class={{
                    "is-invalid": form?.field_errors && form.field_errors["password"],
                    "form-control": true,
                  }}
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
                  class={{
                    "is-invalid": form?.field_errors && form.field_errors["password_confirm"],
                    "form-control": true,
                  }}
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
  </div>
{/if}
