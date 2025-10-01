<script lang="ts">
  import { goto } from "$app/navigation";
  import { APP_URLS } from "$lib/common/contants.js";

  let { data } = $props();

  if (data.currentUser) {
    goto(APP_URLS.HOME);
  }

  function handleSubmit(event: any) {
    event.preventDefault();
  }
</script>

<div class="container vh-100 d-flex align-items-center justify-content-center">
  <div class="row w-100 shadow rounded overflow-hidden" style="max-width: 800px;">
    <!-- Intro bên trái -->
    <div class="col-md-6 bg-primary text-white d-flex flex-column justify-content-center p-5">
      <h2 class="mb-3">Chào mừng trở lại!</h2>
      <p>
        Đăng nhập để tiếp tục truy cập vào tài khoản của bạn và khám phá nhiều tính năng hấp dẫn.
      </p>
    </div>
    <!-- Khung đăng nhập bên phải -->
    <div class="col-md-6 bg-white p-5">
      <h3 class="mb-4">Đăng nhập</h3>
      <form onsubmit={handleSubmit}>
        <div class="mb-3">
          <label for="email" class="form-label">Email</label>
          <input id="email" type="email" class="form-control" required autocomplete="username" />
        </div>
        <div class="mb-3">
          <label for="password" class="form-label">Mật khẩu</label>
          <input
            id="password"
            type="password"
            class="form-control"
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

        <button type="submit" class="btn btn-primary w-100">Đăng nhập</button>
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
