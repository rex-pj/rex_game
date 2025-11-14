<script lang="ts">
  import { page } from "$app/state";
  import * as accessService from "$lib/services/accessService";
  import { ADMIN_URLS } from "$lib/common/contants";
  import { goto } from "$app/navigation";
  import type { LayoutProps } from "./$types";
  const { data, children }: LayoutProps = $props();
  let menus = [
    { name: "Dashboard", link: "/admin/dashboard", icon: "fa-solid fa-gauge", canRead: true },
    {
      name: "Falshcards",
      link: "/admin/flashcards",
      icon: "fa-solid fa-clone",
      canRead: accessService.canReadFlashcards(data.adminUser),
    },
    {
      name: "Falshcard Types",
      link: "/admin/flashcard-types",
      icon: "fa-solid fa-layer-group",
      canRead: accessService.canReadFlashcardTypes(data.adminUser),
    },
    {
      name: "Users",
      link: "/admin/users",
      icon: "fa-solid fa-users",
      canRead: accessService.canReadUsers(data.adminUser),
    },
    {
      name: "Roles",
      link: "/admin/roles",
      icon: "fa-solid fa-user-shield",
      canRead: accessService.canReadRoles(data.adminUser),
    },
    {
      name: "Permissions",
      link: "/admin/permissions",
      icon: "fa-solid fa-key",
      canRead: accessService.canReadPermissions(data.adminUser),
    },
    {
      name: "Accesses",
      link: "/admin/accesses",
      icon: "fa-solid fa-user-lock",
      canRead: accessService.canReadAccesses(data.adminUser),
    },
    {
      name: "Mail Templates",
      link: "/admin/mail-templates",
      icon: "fa-solid fa-envelope-open-text",
      canRead: accessService.canReadAccesses(data.adminUser),
    },
  ];

  let currentPath = $derived(page.url.pathname);
  async function logout() {
    await accessService.logout().then(() => {
      goto(ADMIN_URLS.LOGOUT_SUCCESS_URL);
    });
  }

  const currentUser = data.adminUser;
</script>

<div class="layout">
  {#if currentUser}
    <div class="header">
      <div class="d-flex align-items-center w-100 justify-content-between">
        <div class="d-flex align-items-center">
          <span class="logo me-4">Admin Panel</span>
        </div>
        <div class="d-flex align-items-center">
          <span class="me-3 text-white">Welcome, {currentUser.display_name}</span>
        </div>
        <nav class="d-flex align-items-center">
          <a href="/" class="btn btn-outline-light btn-sm me-1">
            <i class="fa fa-home me-1"></i> Home
          </a>
          <button class="btn btn-outline-danger btn-sm logout" onclick={logout}>
            <i class="fa fa-sign-out-alt me-1"></i> Logout
          </button>
        </nav>
      </div>
    </div>
    <div class="sidebar">
      {#each menus as menu}
        <div class="menu-item">
          {#if !menu.link}
            <span class="text-muted"
              ><i class="fa-solid fa-minus fa-2xs"></i>
              {menu.name}
              <i class="fa-solid fa-minus fa-2xs"></i>
            </span>
          {:else}
            <a
              class:actived={currentPath.includes(menu.link)}
              class="py-1 ps-2 pe-1"
              href={menu.link}
            >
              {#if menu.icon}
                <i class={menu.icon} style="margin-right: 8px;"></i>
              {/if}
              {menu.name}
            </a>
          {/if}
        </div>
      {/each}
    </div>
    <div class="content">
      {@render children()}
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
    display: flex;
    align-items: center;
  }

  .menu-item a.actived {
    color: rgb(25, 135, 84);
  }

  .content {
    padding: 1rem;
    overflow-y: auto;
  }
</style>
