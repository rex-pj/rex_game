<script lang="ts">
  import type { LayoutProps } from "../$types";
  import {
    canReadAccesses,
    canReadAchievements,
    canReadFlashcards,
    canReadFlashcardTypes,
    canReadGameSessions,
    canReadGameTypes,
    canReadPermissions,
    canReadRoles,
    canReadUserStats,
    canReadUsers,
  } from "$lib/services/accessService";

  const { data }: LayoutProps = $props();
  const features = [
    {
      href: "/admin/flashcards",
      icon: "fas fa-clone fa-2x text-primary",
      title: "Flashcards",
      desc: "Manage flashcard sets.",
      canRead: canReadFlashcards(data.adminUser),
    },
    {
      href: "/admin/flashcard-types",
      icon: "fas fa-layer-group fa-2x text-success",
      title: "Flashcard Types",
      desc: "Manage flashcard types.",
      canRead: canReadFlashcardTypes(data.adminUser),
    },
    {
      href: "/admin/game-types",
      icon: "fas fa-gamepad fa-2x text-primary",
      title: "Game Types",
      desc: "Manage game types and assign flashcards.",
      canRead: canReadGameTypes(data.adminUser),
    },
    {
      href: "/admin/achievements",
      icon: "fas fa-trophy fa-2x text-warning",
      title: "Achievements",
      desc: "Manage achievements and rewards.",
      canRead: canReadAchievements(data.adminUser),
    },
    {
      href: "/admin/game-sessions",
      icon: "fas fa-clock-rotate-left fa-2x text-info",
      title: "Game Sessions",
      desc: "View and manage game sessions.",
      canRead: canReadGameSessions(data.adminUser),
    },
    {
      href: "/admin/user-stats",
      icon: "fas fa-chart-line fa-2x text-success",
      title: "User Stats",
      desc: "View and reset user statistics.",
      canRead: canReadUserStats(data.adminUser),
    },
    {
      href: "/admin/users",
      icon: "fas fa-users fa-2x text-danger",
      title: "Users",
      desc: "Add, edit, delete, and manage user accounts.",
      canRead: canReadUsers(data.adminUser),
    },
    {
      href: "/admin/roles",
      icon: "fas fa-user-shield fa-2x text-info",
      title: "Roles",
      desc: "Create and assign roles to users.",
      canRead: canReadRoles(data.adminUser),
    },
    {
      href: "/admin/permissions",
      icon: "fas fa-key fa-2x text-warning",
      title: "Permissions",
      desc: "Set up and control access permissions.",
      canRead: canReadPermissions(data.adminUser),
    },
    {
      href: "/admin/accesses",
      icon: "fas fa-user-lock fa-2x text-secondary",
      title: "Accesses",
      desc: "Manage user and role access rights.",
      canRead: canReadAccesses(data.adminUser),
    },
    {
      href: "/admin/mail-templates",
      icon: "fas fa-envelope-open-text fa-2x text-secondary",
      title: "Mail Templates",
      desc: "Manage mail templates.",
      canRead: canReadAccesses(data.adminUser),
    },
  ];
</script>

<div class="container mt-4">
  <div class="row g-4">
    {#each features as feature}
      {#if feature.canRead}
        <div class="col-md-4">
          <a href={feature.href} class="text-decoration-none">
            <div class="card h-100 shadow-sm">
              <div class="card-body text-center">
                <i class="mb-3 {feature.icon}"></i>
                <h5 class="card-title">{feature.title}</h5>
                <p class="card-text">{feature.desc}</p>
              </div>
            </div>
          </a>
        </div>
      {/if}
    {/each}
  </div>
</div>
