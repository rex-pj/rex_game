<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation"; // For redirect if needed
  import { APP_URLS } from "$lib/common/contants";

  // State for UI
  let status: "idle" | "loading" | "success" | "error" = "idle";
  let message = "";
  let errorMessage = "";

  // Get token from URL query params
  onMount(async () => {
    const urlParams = new URLSearchParams(window.location.search);
    const token = urlParams.get("token");

    if (!token) {
      status = "error";
      errorMessage = "Invalid or missing token.";
      return;
    }

    status = "loading";

    try {
      const response = await fetch("/account/confirm", {
        method: "POST",
        body: JSON.stringify({ token }),
      });

      if (response.ok) {
        status = "success";
        message = "Your account has been successfully activated!";
        // Optional: Redirect to login after 3s
        setTimeout(() => goto(APP_URLS.LOGIN_URL), 3000);
      } else {
        status = "error";
        const data = await response.json();
        errorMessage = data.error?.message || "Failed to activate account. Please try again.";
      }
    } catch (err) {
      status = "error";
      errorMessage = "An unexpected error occurred. Please try again later.";
    }
  });
</script>

<div class="container d-flex justify-content-center align-items-center min-vh-100">
  <div class="card shadow-sm p-4" style="max-width: 500px; width: 100%;">
    <div class="card-body text-center">
      {#if status === "loading"}
        <div class="spinner-border text-primary" role="status">
          <span class="visually-hidden">Loading...</span>
        </div>
        <h5 class="card-title mt-3">Activating your account...</h5>
      {:else if status === "success"}
        <i class="bi bi-check-circle-fill text-success" style="font-size: 3rem;"></i>
        <h5 class="card-title mt-3">Success!</h5>
        <p class="card-text">{message}</p>
        <p class="text-muted">Redirecting to login in a few seconds...</p>
        <a href={APP_URLS.LOGIN_URL} class="btn btn-primary mt-3">Go to Login</a>
      {:else if status === "error"}
        <i class="bi bi-x-circle-fill text-danger" style="font-size: 3rem;"></i>
        <h5 class="card-title mt-3">Error</h5>
        <p class="card-text">{errorMessage}</p>
        <a href={APP_URLS.SIGNUP_URL} class="btn btn-outline-secondary mt-3"
          >Try Registering Again</a
        >
      {/if}
    </div>
  </div>
</div>

<style>
  /* Optional: Customize Bootstrap if needed */
  .card {
    border-radius: 10px;
  }
  .bi-check-circle-fill,
  .bi-x-circle-fill {
    animation: fadeIn 0.5s ease-in;
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>
