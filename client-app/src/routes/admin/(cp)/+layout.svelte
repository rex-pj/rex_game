<script lang="ts">
  import { page } from "$app/state";
  import * as accessService from "$lib/services/accessService";
  import { ADMIN_URLS } from "$lib/common/contants";
  import { goto } from "$app/navigation";
  import type { LayoutProps } from "./$types";
  const { data, children }: LayoutProps = $props();

  interface MenuItem {
    name: string;
    link: string;
    icon: string;
    canRead: boolean | "" | undefined;
  }

  interface MenuGroup {
    name: string;
    icon: string;
    items: MenuItem[];
  }

  type MenuEntry = MenuItem | MenuGroup;

  function isGroup(entry: MenuEntry): entry is MenuGroup {
    return "items" in entry;
  }

  let menus: MenuEntry[] = [
    { name: "Dashboard", link: "/admin/dashboard", icon: "fa-solid fa-gauge", canRead: true },
    {
      name: "Content",
      icon: "fa-solid fa-folder-open",
      items: [
        {
          name: "Flashcards",
          link: "/admin/flashcards",
          icon: "fa-solid fa-clone",
          canRead: accessService.canReadFlashcards(data.adminUser),
        },
        {
          name: "Flashcard Types",
          link: "/admin/flashcard-types",
          icon: "fa-solid fa-layer-group",
          canRead: accessService.canReadFlashcardTypes(data.adminUser),
        },
      ],
    },
    {
      name: "Games",
      icon: "fa-solid fa-gamepad",
      items: [
        {
          name: "Game Types",
          link: "/admin/game-types",
          icon: "fa-solid fa-gamepad",
          canRead: accessService.canReadGameTypes(data.adminUser),
        },
        {
          name: "Achievements",
          link: "/admin/achievements",
          icon: "fa-solid fa-trophy",
          canRead: accessService.canReadAchievements(data.adminUser),
        },
        {
          name: "Game Sessions",
          link: "/admin/game-sessions",
          icon: "fa-solid fa-clock-rotate-left",
          canRead: accessService.canReadGameSessions(data.adminUser),
        },
        {
          name: "User Stats",
          link: "/admin/user-stats",
          icon: "fa-solid fa-chart-line",
          canRead: accessService.canReadUserStats(data.adminUser),
        },
      ],
    },
    {
      name: "Users & Auth",
      icon: "fa-solid fa-users-gear",
      items: [
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
      ],
    },
    {
      name: "System",
      icon: "fa-solid fa-gear",
      items: [
        {
          name: "Mail Templates",
          link: "/admin/mail-templates",
          icon: "fa-solid fa-envelope-open-text",
          canRead: accessService.canReadAccesses(data.adminUser),
        },
      ],
    },
  ];

  let currentPath = $derived(page.url.pathname);

  function groupHasActiveItem(group: MenuGroup): boolean {
    return group.items.some((item) => currentPath.includes(item.link));
  }

  let expandedGroups: Record<string, boolean> = $state({});

  function initExpanded() {
    for (const entry of menus) {
      if (isGroup(entry)) {
        expandedGroups[entry.name] = groupHasActiveItem(entry);
      }
    }
  }

  initExpanded();

  function toggleGroup(name: string) {
    expandedGroups[name] = !expandedGroups[name];
  }

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
      {#each menus as entry}
        {#if isGroup(entry)}
          <div class="menu-group">
            <button
              class="group-toggle"
              class:active={groupHasActiveItem(entry)}
              onclick={() => toggleGroup(entry.name)}
            >
              <i class={entry.icon} style="margin-right: 8px; width: 16px; text-align: center;"></i>
              <span>{entry.name}</span>
              <i
                class="fa-solid fa-chevron-right chevron-icon ms-auto"
                class:rotated={expandedGroups[entry.name]}
              ></i>
            </button>
            {#if expandedGroups[entry.name]}
              <div class="group-items">
                {#each entry.items as item}
                  {#if item.canRead}
                    <a
                      class="menu-link ps-4"
                      class:actived={currentPath.includes(item.link)}
                      href={item.link}
                    >
                      <i class={item.icon} style="margin-right: 8px; width: 16px; text-align: center;"></i>
                      {item.name}
                    </a>
                  {/if}
                {/each}
              </div>
            {/if}
          </div>
        {:else if entry.canRead}
          <div class="menu-item">
            <a
              class="menu-link ps-2"
              class:actived={currentPath.includes(entry.link)}
              href={entry.link}
            >
              <i class={entry.icon} style="margin-right: 8px; width: 16px; text-align: center;"></i>
              {entry.name}
            </a>
          </div>
        {/if}
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
    padding: 0.5rem;
    border-right: 1px solid #ddd;
    overflow-y: auto;
  }

  .menu-item {
    margin-bottom: 0.1rem;
  }

  .menu-group {
    margin-bottom: 0.1rem;
  }

  .group-toggle {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.45rem 0.5rem;
    border: none;
    background: none;
    color: #555;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.15s;
  }

  .group-toggle:hover {
    background-color: #e9e9e9;
  }

  .group-toggle.active {
    color: rgb(25, 135, 84);
  }

  .chevron-icon {
    font-size: 0.65rem;
    transition: transform 0.2s;
  }

  .chevron-icon.rotated {
    transform: rotate(90deg);
  }

  .group-items {
    padding: 0.1rem 0;
  }

  .menu-link {
    display: flex;
    align-items: center;
    text-decoration: none;
    color: #555;
    font-weight: 500;
    font-size: 0.85rem;
    padding: 0.35rem 0.5rem;
    border-radius: 4px;
    transition: background-color 0.15s;
  }

  .menu-link:hover {
    background-color: #e9e9e9;
    color: #333;
  }

  .menu-link.actived {
    color: rgb(25, 135, 84);
    background-color: rgba(25, 135, 84, 0.08);
    font-weight: 600;
  }

  .content {
    padding: 1rem;
    overflow-y: auto;
  }
</style>
