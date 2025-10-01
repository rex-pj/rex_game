<script>
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { ADMIN_URLS } from "$lib/common/contants";

  onMount(() => {
    const timer = setTimeout(() => {
      goto(ADMIN_URLS.LOGIN_URL);
    }, 1000);
    return () => {
      clearTimeout(timer);
    };
  });
</script>

<div class="d-flex justify-content-center align-items-center vh-100 bg-light">
  <div class="card shadow-sm p-4 text-center">
    <div class="card-body">
      <div class="mb-3">
        <div class="spinner-border text-primary" role="status">
          <span class="visually-hidden">Loading...</span>
        </div>
      </div>
      <h5 class="card-title mb-3">Đang thoát...</h5>
      <p class="card-text text-muted">Bạn sẽ được chuyển tới trang đăng nhập ngay lập tức.</p>
      <form>
        <button type="button" class="btn btn-primary w-100" disabled>
          Vui lòng chờ <span class="spinner-grow spinner-grow-sm ms-2"></span>
        </button>
      </form>
      <p class="text-muted mb-0">
        Nếu không tự chuyển, <a href={ADMIN_URLS.LOGIN_URL} class="link-primary">bấm vào đây</a>.
      </p>
    </div>
  </div>
</div>

<style>
  .card {
    max-width: 400px;
    width: 100%;
  }
</style>
