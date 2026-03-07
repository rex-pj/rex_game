<svelte:head>
  <title>Giới thiệu — Qhortus</title>
  <meta name="description" content="Qhortus là nền tảng học flashcard gamified giúp bạn học hiệu quả hơn qua Quiz, Ghép đôi, Spelling và Speed Match." />
</svelte:head>

<div class="about-page">
  <!-- Hero -->
  <section class="about-hero text-center py-5">
    <div class="container">
      <h1 class="display-5 fw-bold mb-3">Học vẹt đã chết rồi.</h1>
      <p class="lead text-muted mx-auto" style="max-width: 640px;">
        Qhortus sinh ra để chứng minh một điều: học mà không vui thì học mãi cũng quên. Chúng tôi biến ôn bài thành trận đấu thực sự — để bạn nhớ lâu hơn mà không cần cố.
      </p>
    </div>
  </section>

  <!-- Sứ mệnh -->
  <section class="py-5 bg-light">
    <div class="container">
      <div class="row align-items-center g-5">
        <div class="col-md-6">
          <h2 class="fw-bold mb-3">Tạo ra vì học vẹt không bao giờ đủ</h2>
          <p class="text-muted">
            Đọc đi đọc lại mà vẫn quên trước khi thi? Đó không phải lỗi của bạn — đó là cách học sai. Não người ghi nhớ tốt nhất khi có áp lực thời gian, cạnh tranh và phần thưởng tức thì.
          </p>
          <p class="text-muted">
            Qhortus được xây dựng đúng theo cơ chế đó. Mỗi chế độ chơi là một bài tập phản xạ thực sự — không phải đọc lại, mà là chiến đấu để nhớ.
          </p>
        </div>
        <div class="col-md-6">
          <div class="row g-3">
            <div class="col-6">
              <div class="stat-card text-center p-4 rounded-3 bg-white shadow-sm">
                <div class="stat-icon mb-2"><i class="fa-solid fa-gamepad fa-2x text-primary"></i></div>
                <div class="fw-bold fs-4">4</div>
                <div class="text-muted small">Chế độ chơi</div>
              </div>
            </div>
            <div class="col-6">
              <div class="stat-card text-center p-4 rounded-3 bg-white shadow-sm">
                <div class="stat-icon mb-2"><i class="fa-solid fa-medal fa-2x text-warning"></i></div>
                <div class="fw-bold fs-4">9+</div>
                <div class="text-muted small">Thành tích</div>
              </div>
            </div>
            <div class="col-6">
              <div class="stat-card text-center p-4 rounded-3 bg-white shadow-sm">
                <div class="stat-icon mb-2"><i class="fa-solid fa-layer-group fa-2x text-success"></i></div>
                <div class="fw-bold fs-4">∞</div>
                <div class="text-muted small">Bộ flashcard</div>
              </div>
            </div>
            <div class="col-6">
              <div class="stat-card text-center p-4 rounded-3 bg-white shadow-sm">
                <div class="stat-icon mb-2"><i class="fa-solid fa-trophy fa-2x text-info"></i></div>
                <div class="fw-bold fs-4">Top</div>
                <div class="text-muted small">Bảng xếp hạng</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>

  <!-- Tính năng -->
  <section class="py-5">
    <div class="container">
      <h2 class="fw-bold text-center mb-5">Cách Qhortus hoạt động</h2>
      <div class="row g-4">
        <div class="col-md-3 text-center">
          <div class="feature-icon mb-3">
            <i class="fa-solid fa-cards-blank fa-3x text-primary"></i>
          </div>
          <h5 class="fw-bold">Chọn chủ đề</h5>
          <p class="text-muted small">Hàng trăm bộ flashcard đa dạng từ tiếng Anh đến khoa học, lịch sử và nhiều hơn nữa.</p>
        </div>
        <div class="col-md-3 text-center">
          <div class="feature-icon mb-3">
            <i class="fa-solid fa-gamepad fa-3x text-success"></i>
          </div>
          <h5 class="fw-bold">Chọn chế độ chơi</h5>
          <p class="text-muted small">Quiz, Ghép đôi, Spelling hay Speed Match — mỗi chế độ rèn luyện một kỹ năng ghi nhớ khác nhau.</p>
        </div>
        <div class="col-md-3 text-center">
          <div class="feature-icon mb-3">
            <i class="fa-solid fa-chart-line fa-3x text-warning"></i>
          </div>
          <h5 class="fw-bold">Theo dõi tiến trình</h5>
          <p class="text-muted small">Thống kê chi tiết, chuỗi ngày liên tiếp và hệ thống điểm giúp bạn thấy rõ sự tiến bộ của mình.</p>
        </div>
        <div class="col-md-3 text-center">
          <div class="feature-icon mb-3">
            <i class="fa-solid fa-ranking-star fa-3x text-danger"></i>
          </div>
          <h5 class="fw-bold">Cạnh tranh & Thành tích</h5>
          <p class="text-muted small">Leo hạng trên bảng xếp hạng toàn cầu và mở khóa những thành tích đặc biệt.</p>
        </div>
      </div>
    </div>
  </section>

  <!-- CTA -->
  <section class="py-5 bg-primary text-white text-center">
    <div class="container">
      <h2 class="fw-bold mb-3">Thử ngay — mất gì đâu mà ngại?</h2>
      <p class="lead mb-4 opacity-90">Không cần thẻ tín dụng. Không cần cam kết. Chơi thử 5 phút xem não bạn nhớ được bao nhiêu.</p>
      <div class="d-flex gap-3 justify-content-center flex-wrap">
        <a href="/account/signup" class="btn btn-light btn-lg fw-bold">
          <i class="fa-solid fa-rocket me-2"></i>Đăng ký miễn phí
        </a>
        <a href="/flashcard" class="btn btn-outline-light btn-lg">Thử ngay không cần đăng ký</a>
      </div>
    </div>
  </section>
</div>

<style>
  .about-hero {
    background: linear-gradient(160deg, rgba(59, 130, 246, 0.06) 0%, rgba(139, 92, 246, 0.06) 100%);
    border-bottom: 1px solid rgba(59, 130, 246, 0.1);
  }

  .stat-card {
    border: 1px solid rgba(0,0,0,0.06);
    transition: transform 0.2s;
  }

  .stat-card:hover {
    transform: translateY(-4px);
  }

  .feature-icon {
    height: 70px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
