<script lang="ts">
  let { data } = $props();
</script>

<div class="success-wrapper">
  <div class="container vh-100 d-flex align-items-center justify-content-center">
    <div class="row justify-content-center w-100">
      <div class="col-md-6">
        <div class="success-card card shadow-sm">
          <div class="card-body text-center p-5">
            <div class="success-icon mb-4">
              <i class="bi bi-envelope-check-fill"></i>
            </div>
            <h1 class="card-title mb-4">Đã gửi email!</h1>
            <p class="card-text mb-3">
              Một email hướng dẫn đặt lại mật khẩu đã được gửi đến
              <strong>{data.email || "email của bạn"}</strong>.
            </p>
            <p class="card-text text-muted mb-4">
              Vui lòng kiểm tra hộp thư đến (hoặc thư mục spam) để xem chi tiết.
            </p>
            <a href="/account/login" class="btn btn-primary">Đi đến trang đăng nhập</a>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .success-wrapper {
    background-color: var(--background-color);
  }

  .success-card {
    border: none;
    border-radius: 16px;
    box-shadow: 0 4px 12px var(--box-shadow-color) !important;
  }

  .success-icon i {
    font-size: 4rem;
    color: var(--secondary-color);
  }

  .success-card .card-title {
    color: var(--primary-color);
    font-weight: 700;
    font-size: 1.8rem;
  }

  .success-card .card-text {
    font-size: 1.1rem;
    line-height: 1.6;
  }
</style>
