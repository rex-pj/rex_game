<script lang="ts">
  import { enhance } from "$app/forms";
  let { form } = $props();
</script>

<div class="d-flex justify-content-center align-items-center vh-100 bg-light">
  <div class="card shadow p-4" style="min-width: 350px; max-width: 400px; width: 100%;">
    <h3 class="mb-3 text-center">Quên mật khẩu</h3>
    {#if form?.message}
      <div class="alert alert-danger" role="alert">
        {form.message}
      </div>
    {/if}
    <form method="POST" use:enhance>
      <div class="mb-3">
        <label for="email" class="form-label">Email</label>
        <input
          id="email"
          type="email"
          name="email"
          class={{
            "is-invalid": form?.field_errors && form.field_errors["email"],
            "form-control": true,
          }}
          value={form?.values?.email ?? ""}
          required
          placeholder="Nhập email của bạn"
        />
        {#if form?.field_errors && form.field_errors["email"]}
          <p class="invalid-feedback">{form.field_errors["email"]}</p>
        {/if}
      </div>
      <button type="submit" class="btn btn-primary w-100">Gửi yêu cầu</button>
    </form>
  </div>
</div>
