<script>
  const currentYear = new Date().getFullYear();
</script>

<svelte:head>
  <title>Chính sách bảo mật — Qhortus</title>
  <meta name="description" content="Chính sách bảo mật và điều khoản sử dụng của Qhortus. Tìm hiểu cách chúng tôi bảo vệ dữ liệu của bạn." />
</svelte:head>

<div class="policy-page">
  <div class="container py-5">
    <div class="row justify-content-center">
      <div class="col-lg-8">

        <h1 class="fw-bold mb-2">Chính sách bảo mật</h1>
        <p class="text-muted mb-5">Cập nhật lần cuối: Tháng 3, {currentYear}</p>

        <div class="policy-section mb-5">
          <h2 class="h4 fw-bold mb-3">1. Thông tin chúng tôi thu thập</h2>
          <p>Khi bạn đăng ký và sử dụng Qhortus, chúng tôi thu thập các thông tin sau:</p>
          <ul>
            <li><strong>Thông tin tài khoản:</strong> Tên đăng nhập, địa chỉ email, tên hiển thị.</li>
            <li><strong>Dữ liệu sử dụng:</strong> Lịch sử chơi game, điểm số, thành tích và chuỗi ngày học.</li>
            <li><strong>Thông tin kỹ thuật:</strong> Địa chỉ IP, loại trình duyệt, thời gian truy cập (cho mục đích bảo mật và cải thiện dịch vụ).</li>
          </ul>
          <p>Chúng tôi <strong>không</strong> thu thập thông tin thanh toán, số điện thoại hay địa chỉ thực tế.</p>
        </div>

        <div class="policy-section mb-5">
          <h2 class="h4 fw-bold mb-3">2. Mục đích sử dụng thông tin</h2>
          <p>Thông tin thu thập được sử dụng để:</p>
          <ul>
            <li>Cung cấp và duy trì dịch vụ học tập cho bạn.</li>
            <li>Lưu trữ tiến trình học tập và thành tích.</li>
            <li>Hiển thị bảng xếp hạng và tính năng cộng đồng.</li>
            <li>Gửi email xác nhận tài khoản và thông báo liên quan đến dịch vụ.</li>
            <li>Cải thiện chất lượng nội dung và trải nghiệm người dùng.</li>
          </ul>
        </div>

        <div class="policy-section mb-5">
          <h2 class="h4 fw-bold mb-3">3. Chia sẻ thông tin với bên thứ ba</h2>
          <p>Qhortus <strong>không bán hoặc cho thuê</strong> thông tin cá nhân của bạn cho bất kỳ bên thứ ba nào vì mục đích thương mại.</p>
          <p>Chúng tôi chỉ chia sẻ thông tin trong các trường hợp:</p>
          <ul>
            <li>Với các nhà cung cấp dịch vụ kỹ thuật hỗ trợ vận hành (như dịch vụ email, hosting).</li>
            <li>Khi có yêu cầu từ cơ quan pháp luật có thẩm quyền.</li>
            <li>Khi bạn đồng ý chia sẻ thông tin công khai (như tên trên bảng xếp hạng).</li>
          </ul>
        </div>

        <div class="policy-section mb-5">
          <h2 class="h4 fw-bold mb-3">4. Bảo mật dữ liệu</h2>
          <p>Chúng tôi áp dụng các biện pháp bảo mật kỹ thuật tiêu chuẩn ngành:</p>
          <ul>
            <li>Mật khẩu được mã hóa (hashed) — chúng tôi không lưu mật khẩu dạng văn bản thuần.</li>
            <li>Kết nối HTTPS được mã hóa toàn bộ.</li>
            <li>Token xác thực JWT với thời hạn có giới hạn.</li>
            <li>Giới hạn số lần đăng nhập để chống brute force.</li>
          </ul>
        </div>

        <div class="policy-section mb-5">
          <h2 class="h4 fw-bold mb-3">5. Quyền của bạn</h2>
          <p>Bạn có quyền:</p>
          <ul>
            <li><strong>Truy cập:</strong> Xem thông tin tài khoản và lịch sử sử dụng của mình.</li>
            <li><strong>Chỉnh sửa:</strong> Cập nhật tên hiển thị và thông tin hồ sơ.</li>
            <li><strong>Xóa tài khoản:</strong> Yêu cầu xóa toàn bộ dữ liệu của bạn bằng cách liên hệ qua email.</li>
          </ul>
          <p>Để thực hiện các quyền trên, vui lòng liên hệ: <a href="mailto:contact@qhortus.com">contact@qhortus.com</a></p>
        </div>

        <div class="policy-section mb-5">
          <h2 class="h4 fw-bold mb-3">6. Cookie</h2>
          <p>Qhortus sử dụng cookie cần thiết cho hoạt động đăng nhập và duy trì phiên làm việc. Chúng tôi không sử dụng cookie theo dõi quảng cáo của bên thứ ba.</p>
        </div>

        <div class="policy-section mb-5" id="terms">
          <h2 class="h4 fw-bold mb-3">7. Điều khoản sử dụng</h2>
          <p>Khi sử dụng Qhortus, bạn đồng ý:</p>
          <ul>
            <li>Không sử dụng dịch vụ cho mục đích vi phạm pháp luật.</li>
            <li>Không tạo tài khoản giả mạo hoặc đăng nội dung sai lệch.</li>
            <li>Không cố tình tấn công, phá hoại hệ thống.</li>
            <li>Chịu trách nhiệm về bảo mật thông tin đăng nhập của mình.</li>
          </ul>
          <p>Qhortus có quyền tạm khóa hoặc xóa tài khoản vi phạm các điều khoản trên mà không cần thông báo trước.</p>
        </div>

        <div class="policy-section mb-5">
          <h2 class="h4 fw-bold mb-3">8. Thay đổi chính sách</h2>
          <p>Chúng tôi có thể cập nhật chính sách này theo thời gian. Khi có thay đổi quan trọng, chúng tôi sẽ thông báo qua email đã đăng ký. Ngày cập nhật luôn được hiển thị ở đầu trang.</p>
        </div>

        <div class="policy-section">
          <h2 class="h4 fw-bold mb-3">9. Liên hệ</h2>
          <p>Nếu có bất kỳ câu hỏi nào về chính sách bảo mật, vui lòng liên hệ:</p>
          <ul>
            <li>Email: <a href="mailto:contact@qhortus.com">contact@qhortus.com</a></li>
          </ul>
        </div>

      </div>
    </div>
  </div>
</div>

<style>
  .policy-page {
    background-color: var(--background-color);
    min-height: calc(100vh - 200px);
  }

  .policy-section {
    border-bottom: 1px solid rgba(0, 0, 0, 0.07);
    padding-bottom: 2rem;
  }

  .policy-section:last-child {
    border-bottom: none;
  }

  .policy-section ul {
    padding-left: 1.5rem;
    line-height: 2;
    color: #555;
  }

  .policy-section p {
    color: #444;
    line-height: 1.8;
  }

  a {
    color: var(--primary-color);
  }
</style>
