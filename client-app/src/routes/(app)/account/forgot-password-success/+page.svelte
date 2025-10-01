<script lang="ts">
  let { data } = $props();
</script>

<div class="container vh-100 d-flex align-items-center justify-content-center">
  <div class="row justify-content-center">
    <div class="col-md-12">
      <div class="card shadow-sm">
        <div class="card-body text-center">
          <h1 class="card-title text-success mb-4">Đã thông báo quên mật khẩu!</h1>
          <p class="card-text">
            Một email hướng dẫn đã được gửi đến
            <strong>{data.email || "email của bạn"}</strong>.
          </p>
          <p class="card-text">
            Vui lòng kiểm tra hộp thư đến (hoặc thư mục spam) để xem chi tiết.
          </p>
          <a href="/account/login" class="btn btn-primary mt-3">Đi đến trang đăng nhập</a>
        </div>
      </div>
    </div>
  </div>
</div>
