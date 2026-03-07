<script>
  const currentYear = new Date().getFullYear();
</script>

<footer class="footer">
  <div class="container">
    <div class="row text-center text-md-start">
      <div class="col-md-4 mb-4">
        <a class="footer-brand" href="/">
          <enhanced:img class="footer-logo" src="../../assets/imgs/logo.png" alt="logo" />
          <span class="footer-brand-name">Qhortus</span>
        </a>
        <p class="footer-text mt-3">Nền tảng học flashcard qua game tương tác. Nâng cao kiến thức mỗi ngày!</p>
      </div>
      <div class="col-md-3 mb-4">
        <h5 class="footer-heading">Khám phá</h5>
        <ul class="footer-list">
          <li><a href="/flashcard" class="footer-link">Chơi Flashcard</a></li>
          <li><a href="/leaderboard" class="footer-link">Bảng xếp hạng</a></li>
          <li><a href="/achievements" class="footer-link">Thành tích</a></li>
          <li><a href="/about" class="footer-link">Giới thiệu</a></li>
        </ul>
      </div>
      <div class="col-md-3 mb-4">
        <h5 class="footer-heading">Pháp lý</h5>
        <ul class="footer-list">
          <li><a href="/privacy-policy" class="footer-link">Chính sách bảo mật</a></li>
          <li><a href="/privacy-policy#terms" class="footer-link">Điều khoản sử dụng</a></li>
        </ul>
      </div>
      <div class="col-md-2 mb-4">
        <h5 class="footer-heading">Liên hệ</h5>
        <ul class="footer-list">
          <li>
            <a href="mailto:contact@qhortus.com" class="footer-link">
              <i class="fa-solid fa-envelope me-1"></i>Email
            </a>
          </li>
        </ul>
      </div>
    </div>
    <div class="text-center mt-3 pt-3 border-top border-white border-opacity-25">
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

  .footer-brand {
    display: flex;
    align-items: center;
    gap: 10px;
    text-decoration: none;
  }

  .footer-brand-name {
    font-family: var(--font-family-primary);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--white);
  }

  .footer-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .footer-list li {
    margin-bottom: 0.5rem;
  }

  .footer-link {
    font-family: var(--font-family-secondary);
    color: rgba(255, 255, 255, 0.85);
    text-decoration: none;
    transition: all var(--transition-fast);
    display: inline-block;
    font-size: 0.95rem;
  }

  .footer-link:hover {
    color: var(--white);
    transform: translateX(3px);
  }

  .copyright {
    font-family: var(--font-family-secondary);
    font-size: 0.875rem;
    color: rgba(255, 255, 255, 0.7);
  }

  .footer-logo {
    max-width: 40px;
    height: auto;
  }
</style>
