<script lang="ts">
  import type { Achievement } from "$lib/api/scoringApi";
  import Card from "../../../components/atoms/card/Card.svelte";
  import Badge from "../../../components/atoms/badge/Badge.svelte";

  interface Props {
    data: {
      achievements: Achievement[];
    };
  }

  let { data }: Props = $props();

  const categoryLabels: Record<string, string> = {
    beginner: "Getting Started",
    accuracy: "Accuracy",
    speed: "Speed",
    combo: "Combo",
    streak: "Daily Streak",
    games: "Games Played",
    score: "Score Milestones",
  };

  const categoryIcons: Record<string, string> = {
    beginner: "fa-baby",
    accuracy: "fa-bullseye",
    speed: "fa-bolt",
    combo: "fa-fire",
    streak: "fa-calendar-check",
    games: "fa-gamepad",
    score: "fa-trophy",
  };

  // Group achievements by category
  const groupedAchievements = $derived.by(() => {
    const groups: Record<string, Achievement[]> = {};
    for (const achievement of data.achievements) {
      const category = achievement.category || "other";
      if (!groups[category]) {
        groups[category] = [];
      }
      groups[category].push(achievement);
    }
    return groups;
  });

  const unlockedCount = $derived(data.achievements.filter((a) => a.unlocked).length);
  const totalPoints = $derived(
    data.achievements.filter((a) => a.unlocked).reduce((sum, a) => sum + a.points, 0)
  );
</script>

<svelte:head>
  <title>Achievements - Rex Game</title>
</svelte:head>

<div class="container py-4">
  <h1 class="text-center mb-4">
    <i class="fa-solid fa-medal text-warning me-2"></i>
    Achievements
  </h1>

  <!-- Summary Card -->
  <Card class="mb-4 bg-gradient-primary text-white">
    <div class="row text-center">
      <div class="col-6">
        <div class="fs-2 fw-bold">{unlockedCount} / {data.achievements.length}</div>
        <small>Achievements Unlocked</small>
      </div>
      <div class="col-6">
        <div class="fs-2 fw-bold">{totalPoints}</div>
        <small>Total Points</small>
      </div>
    </div>
  </Card>

  <!-- Achievements by Category -->
  {#each Object.entries(groupedAchievements) as [category, achievements]}
    <Card class="mb-4">
      {#snippet header()}
        <h5 class="mb-0">
          <i class="fa-solid {categoryIcons[category] || 'fa-star'} me-2"></i>
          {categoryLabels[category] || category}
        </h5>
      {/snippet}

      <div class="row g-3">
        {#each achievements as achievement}
          <div class="col-12 col-md-6 col-lg-4">
            <div
              class="achievement-card p-3 rounded-3 h-100"
              class:unlocked={achievement.unlocked}
              class:locked={!achievement.unlocked}
            >
              <div class="d-flex align-items-start">
                <div class="achievement-icon me-3">
                  <i class="fa-solid {achievement.icon || 'fa-star'} fa-2x"></i>
                </div>
                <div class="flex-grow-1">
                  <div class="d-flex justify-content-between align-items-start">
                    <h6 class="mb-1">{achievement.name}</h6>
                    <Badge variant={achievement.unlocked ? "success" : "secondary"}>
                      {achievement.points} pts
                    </Badge>
                  </div>
                  <p class="text-muted small mb-0">{achievement.description}</p>
                  {#if achievement.unlocked && achievement.unlocked_at}
                    <small class="text-success">
                      <i class="fa-solid fa-check-circle me-1"></i>
                      Unlocked {new Date(achievement.unlocked_at).toLocaleDateString()}
                    </small>
                  {/if}
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </Card>
  {/each}
</div>

<style>
  .bg-gradient-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }

  .achievement-card {
    border: 2px solid transparent;
    transition: all 0.3s ease;
  }

  .achievement-card.unlocked {
    background: linear-gradient(135deg, #d4fc79 0%, #96e6a1 100%);
    border-color: #4caf50;
  }

  .achievement-card.unlocked .achievement-icon {
    color: #2e7d32;
  }

  .achievement-card.locked {
    background: #f5f5f5;
    border-color: #e0e0e0;
    opacity: 0.7;
  }

  .achievement-card.locked .achievement-icon {
    color: #9e9e9e;
  }

  .achievement-icon {
    width: 50px;
    height: 50px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.5);
  }

  .achievement-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }
</style>
