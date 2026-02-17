<script>
  const currentYear = new Date().getFullYear();
</script>

<footer class="footer">
  <div class="container">
    <div class="row text-center text-md-start">
      <div class="col-md-2 mb-3">
        <enhanced:img class="footer-logo" src="../../assets/imgs/logo.png" alt="logo" />
      </div>
      <div class="col-md-2 mb-3">
        <h5 class="footer-heading">Liên hệ</h5>
        <p class="footer-text mb-0">Email: contact@qhortus.com</p>
      </div>
      <div class="col-md-2 mb-3">
        <h5 class="footer-heading">Giới thiệu</h5>
        <p class="footer-text mb-0">Tìm hiểu về sứ mệnh của chúng tôi.</p>
      </div>
      <div class="col-md-2 mb-3">
        <h5 class="footer-heading">Cửa hàng</h5>
        <p class="footer-text mb-0">Khám phá bộ sưu tập đồ chơi.</p>
      </div>
      <div class="col-md-2 mb-3">
        <h5 class="footer-heading">Youtube</h5>
        <a href="https://youtube.com" target="_blank" class="footer-link">Truy cập kênh</a>
      </div>
    </div>
    <div class="text-center mt-4">
      <p class="copyright mb-0">&copy; {currentYear} Qhortus. Bản quyền được bảo hộ.</p>
    </div>
  </div>
</footer>

<style>
  .footer {
    background: linear-gradient(135deg, var(--primary-color) 0%, var(--primary-hover-color) 100%);
    color: var(--white);
    padding: 3rem 0 2rem 0;
    box-shadow: 0 -4px 12px var(--box-shadow-color);
  }

  .footer-heading {
    font-family: var(--font-family-primary);
    font-weight: 700;
    font-size: 1.1rem;
    color: var(--white);
    margin-bottom: 1rem;
  }

  .footer-text {
    font-family: var(--font-family-secondary);
    font-size: 0.95rem;
    color: rgba(255, 255, 255, 0.9);
    line-height: 1.6;
  }

  .footer-link {
    font-family: var(--font-family-secondary);
    color: rgba(255, 255, 255, 0.9);
    text-decoration: none;
    transition: all var(--transition-fast);
    display: inline-block;
  }

  .footer-link:hover {
    color: var(--secondary-color);
    transform: translateY(-2px);
  }

  .copyright {
    font-family: var(--font-family-secondary);
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.8);
    padding-top: 1rem;
    border-top: 1px solid rgba(255, 255, 255, 0.2);
  }

  .footer-logo {
    max-width: 80px;
    height: auto;
    transition: all var(--transition-normal);
  }

  .footer-logo:hover {
    transform: scale(1.1) rotate(5deg);
  }
</style>
