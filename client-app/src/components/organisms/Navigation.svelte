<script lang="ts">
  import { goto, invalidateAll } from "$app/navigation";
  import { APP_URLS } from "$lib/common/contants";
  import type { CurrentUser } from "$lib/models/current-user";
  import * as accessService from "$lib/services/accessService";

  let navItems = [
    { name: "Flashcard", href: "/flashcard", actived: true },
    { name: "Vật phẩm", href: "/items" },
  ];
  const { currentUser }: { currentUser: CurrentUser } = $props();
  async function logout() {
    await accessService.logout();
    await invalidateAll();
    goto(APP_URLS.HOME);
  }
</script>

<nav class="navbar navbar-expand-lg navbar-light topbar">
  <div class="container">
    <a class="navbar-brand" href="/"
      ><enhanced:img src="../../assets/imgs/logo.png" alt="logo" class="navbar-logo" /> Rex Game</a
    >
    <button
      class="navbar-toggler"
      type="button"
      data-bs-toggle="collapse"
      data-bs-target="#navbarNav"
      aria-controls="navbarNav"
      aria-expanded="false"
      aria-label="Toggle navigation"
    >
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarNav">
      <ul class="navbar-nav ms-auto mb-2 mb-lg-0">
        {#each navItems as item}
          <li class="nav-item {item.actived ? 'active' : ''}">
            <a class="nav-link" href={item.href}>{item.name}</a>
          </li>
        {/each}
      </ul>
      <ul class="navbar-nav mb-2 mb-lg-0 align-items-center">
        <!-- Profile Dropdown -->
        {#if currentUser}
          <li class="nav-item dropdown">
            <button
              type="button"
              class="nav-link dropdown-toggle d-flex align-items-center"
              id="navbarDropdown"
              data-bs-toggle="dropdown"
              aria-expanded="false"
            >
              <i class="fa-solid fa-user-circle fa-lg me-2"></i>
              <span>{currentUser.display_name || currentUser.name || currentUser.email}</span>
            </button>
            <ul class="dropdown-menu dropdown-menu-end shadow-sm" aria-labelledby="navbarDropdown">
              <li>
                <a class="dropdown-item" href="/leaderboard">
                  <i class="fa-solid fa-trophy me-2 text-warning"></i>Bảng xếp hạng
                </a>
              </li>
              <li>
                <a class="dropdown-item" href="/achievements">
                  <i class="fa-solid fa-medal me-2 text-info"></i>Thành tựu
                </a>
              </li>
              <li><hr class="dropdown-divider" /></li>
              <li><button class="dropdown-item text-danger" onclick={logout}><i class="fa-solid fa-sign-out-alt me-2"></i>Đăng xuất</button></li>
            </ul>
          </li>
        {:else}
          <li class="nav-item dropdown">
            <a class="btn btn-primary me-2" href={APP_URLS.LOGIN_URL}>Đăng nhập</a>
          </li>
          <li class="nav-item dropdown">
            <a class="btn btn-secondary" href={APP_URLS.SIGNUP_URL}>Ghi danh</a>
          </li>
        {/if}
      </ul>
    </div>
  </div>
</nav>

<style>
  .topbar {
    background-color: var(--nav-bg-color);
    box-shadow: 0 0 10px rgb(26 26 26 / 15%);
  }

  .navbar-brand {
    color: #000;
    font-weight: 700;
    text-transform: uppercase;
  }

  .navbar-brand .navbar-logo {
    max-width: 40px;
    height: auto;
  }

  .nav-item.active .nav-link {
    color: var(--primary-color);
    font-weight: 700;
  }

  .nav-item .nav-link {
    color: #000;
  }
</style>
