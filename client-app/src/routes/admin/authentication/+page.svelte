<div class="d-flex justify-content-center align-items-center vh-100">
  <div class="card p-4" style="width: 400px;">
    <h3 class="text-center mb-4">Admin Login</h3>
    <form method="POST" action="/authentication">
      <div class="mb-3">
        <label for="email" class="form-label">Email address</label>
        <input
          type="email"
          class="form-control"
          id="email"
          name="email"
          placeholder="Enter your email"
          required
        />
      </div>
      <div class="mb-3">
        <label for="password" class="form-label">Password</label>
        <input
          type="password"
          class="form-control"
          id="password"
          name="password"
          placeholder="Enter your password"
          required
        />
      </div>
      <button type="submit" class="btn btn-primary w-100">Login</button>
    </form>
  </div>
</div>

<style>
  .vh-100 {
    height: 100vh;
  }
</style>
