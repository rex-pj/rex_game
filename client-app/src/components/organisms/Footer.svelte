<script>
  const currentYear = new Date().getFullYear();
</script>

<footer class="bg-dark text-white py-4">
  <div class="container">
    <div class="row text-center text-md-start">
      <div class="col-md-2 mb-3">
        <enhanced:img class="footer-logo" src="../../assets/imgs/footer-logo.png" alt="logo" />
      </div>
      <div class="col-md-2 mb-3">
        <h5>Contact Us</h5>
        <p class="mb-0">Email: contact@example.com</p>
      </div>
      <div class="col-md-2 mb-3">
        <h5>About</h5>
        <p class="mb-0">Learn more about our mission and values.</p>
      </div>
      <div class="col-md-2 mb-3">
        <h5>Shop</h5>
        <p class="mb-0">Explore our collection of toys and games.</p>
      </div>
      <div class="col-md-2 mb-3">
        <h5>Youtube</h5>
        <a href="https://youtube.com" target="_blank" class="text-white text-decoration-none"
          >Visit our channel</a
        >
      </div>
    </div>
    <div class="text-center mt-3">
      <p class="mb-0">&copy; {currentYear} Your Company. All rights reserved.</p>
    </div>
  </div>
</footer>

<style>
  footer a:hover {
    text-decoration: underline;
  }

  footer img.footer-logo {
    max-width: 60px;
    height: auto;
  }
</style>
