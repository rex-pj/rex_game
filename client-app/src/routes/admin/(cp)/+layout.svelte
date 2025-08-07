<script lang="ts">
  import * as authenticationClient from "$lib/helpers/authenticationClient";
  import { getContext, onMount } from "svelte";
  import { SHARED_CONTEXT, ADMIN_URLS, ROLE_NAMES } from "$lib/common/contants";
  import { type CurrentUser } from "$lib/models/current-user";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  let menus = [
    { name: "Dashboard", link: "/admin/dashboard" },
    { name: "Falshcards", link: "/admin/flashcards" },
    { name: "Falshcard Types", link: "/admin/flashcard-types" },
    { name: "Users", link: "/admin/users" },
    { name: "Security" },
    { name: "Roles", link: "/admin/roles" },
    { name: "Permissions", link: "/admin/permissions" },
    { name: "User Accesses", link: "/admin/user-accesses" },
  ];

  async function logout() {
    await authenticationClient.logout();
    goto(ADMIN_URLS.LOGIN_URL);
  }

  const currentUser = getContext<CurrentUser | null>(SHARED_CONTEXT.CURRENT_USER);
  onMount(() => {
    if (
      !currentUser ||
      !currentUser.roles?.some((r) => r === ROLE_NAMES.ADMIN || ROLE_NAMES.ROOT_ADMIN)
    ) {
      goto(ADMIN_URLS.LOGIN_URL);
    }
  });

  $: currentPath = $page.url.pathname;
</script>

<div class="layout">
  {#if currentUser && currentUser.roles?.some((r) => r === ROLE_NAMES.ADMIN || ROLE_NAMES.ROOT_ADMIN)}
    <div class="header">
      <div class="logo">Admin Panel</div>
      <button class="logout" onclick={logout}>Logout</button>
    </div>
    <div class="sidebar">
      {#each menus as menu}
        <div class="menu-item">
          {#if !menu.link}
            <span class="text-muted"
              ><i class="fa-solid fa-minus fa-2xs"></i>
              {menu.name}
            </span>
          {:else}
            <a
              class:actived={currentPath.includes(menu.link)}
              class="py-1 ps-2 pe-1"
              href={menu.link}>{menu.name}</a
            >
          {/if}
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

  .menu-item a.actived {
    color: rgb(25, 135, 84);
  }

  .content {
    padding: 1rem;
    overflow-y: auto;
  }
</style>
