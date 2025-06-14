<script lang="ts">
  import * as authenticationClient from "$lib/helpers/authenticationClient";
  import { getContext, onMount } from "svelte";
  import { SHARED_CONTEXT, ADMIN_URLS, ROLE_NAMES } from "$lib/common/contants";
  import { redirect } from "@sveltejs/kit";
  import { type CurrentUser } from "$lib/models/current-user";
  import { goto } from "$app/navigation";
  let menus = [
    { name: "Dashboard", link: "/admin/dashboard" },
    { name: "Falshcards", link: "/admin/flashcards" },
    { name: "Falshcard Types", link: "/admin/flashcard-types" },
    { name: "Settings", link: "/admin/settings" },
  ];

  async function logout() {
    await authenticationClient.logout();
    goto(ADMIN_URLS.LOGIN_URL);
  }

  const currentUser = getContext<CurrentUser | null>(SHARED_CONTEXT.CURRENT_USER);
  onMount(() => {
    if (!currentUser || !currentUser.roles?.some((r) => r.role_name === ROLE_NAMES.ADMIN)) {
      goto(ADMIN_URLS.LOGIN_URL);
    }
  });
</script>

<div class="layout">
  {#if currentUser && currentUser.roles?.some((r) => r.role_name === ROLE_NAMES.ADMIN)}
    <div class="header">
      <div class="logo">Admin Panel</div>
      <button class="logout" onclick={logout}>Logout</button>
    </div>
    <div class="sidebar">
      {#each menus as menu}
        <div class="menu-item">
          <a href={menu.link}>{menu.name}</a>
        </div>
      {/each}
    </div>
    <div class="content">
      <slot />
    </div>
  {:else}
    <div class="content">
      <p>(^_^)</p>
    </div>
  {/if}
</div>

<style>
  .layout {
    display: grid;
    grid-template-rows: auto 1fr;
    grid-template-columns: 250px 1fr;
    height: 100vh;
  }

  .header {
    grid-column: 1 / -1;
    background-color: #333;
    color: white;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
  }

  .logo {
    font-size: 1.5rem;
    font-weight: bold;
  }

  .logout {
    cursor: pointer;
    background: none;
    border: none;
    color: white;
    font-size: 1rem;
  }

  .sidebar {
    background-color: #f4f4f4;
    padding: 1rem;
    border-right: 1px solid #ddd;
  }

  .menu-item {
    margin-bottom: 1rem;
  }

  .menu-item a {
    text-decoration: none;
    color: #333;
    font-weight: bold;
  }

  .content {
    padding: 1rem;
    overflow-y: auto;
  }
</style>
