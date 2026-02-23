<script lang="ts">
  import type { LeaderboardEntry, UserStats } from "$lib/api/scoringApi";
  import Badge from "../../../components/atoms/badge/Badge.svelte";
  import Card from "../../../components/atoms/card/Card.svelte";
  import Spinner from "../../../components/atoms/spinner/Spinner.svelte";

  interface Props {
    data: {
      leaderboard: LeaderboardEntry[];
      myStats: UserStats | null;
    };
  }

  let { data }: Props = $props();

  function formatTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  }

  function getRankBadgeVariant(rank: number): "warning" | "secondary" | "info" | "primary" {
    if (rank === 1) return "warning";
    if (rank === 2) return "secondary";
    if (rank === 3) return "info";
    return "primary";
  }

  function getRankIcon(rank: number): { type: "icon" | "text"; value: string; colorClass: string } {
    if (rank === 1) return { type: "icon", value: "fa-medal", colorClass: "rank-gold" };
    if (rank === 2) return { type: "icon", value: "fa-medal", colorClass: "rank-silver" };
    if (rank === 3) return { type: "icon", value: "fa-medal", colorClass: "rank-bronze" };
    return { type: "text", value: `#${rank}`, colorClass: "" };
  }
</script>

<svelte:head>
  <title>Leaderboard — Xếp hạng người chơi — Qhortus</title>
  <meta name="description" content="Xem bảng xếp hạng toàn cầu trên Qhortus. Ai đang đứng đầu về điểm số, chuỗi ngày liên tiếp và độ chính xác?" />
  <meta property="og:title" content="Leaderboard — Xếp hạng người chơi — Qhortus" />
  <meta property="og:description" content="Xem bảng xếp hạng toàn cầu. Cạnh tranh, leo hạng và khẳng định vị trí của bạn!" />
  <meta property="og:url" content="/leaderboard" />
</svelte:head>

<div class="container py-4">
  <h1 class="text-center mb-4">
    <i class="fa-solid fa-trophy text-warning me-2"></i>
    Leaderboard
  </h1>

  <!-- My Stats Summary -->
  {#if data.myStats}
    <Card class="mb-4 bg-primary bg-opacity-10 border-primary">
      {#snippet header()}
        <h5 class="mb-0">
          <i class="fa-solid fa-user me-2"></i>
          Your Stats
        </h5>
      {/snippet}
      <div class="row text-center">
        <div class="col-6 col-md-3 mb-3">
          <div class="fs-3 fw-bold text-primary">{data.myStats.rank ?? "-"}</div>
          <small class="text-muted">Your Rank</small>
        </div>
        <div class="col-6 col-md-3 mb-3">
          <div class="fs-3 fw-bold text-success">{data.myStats.total_score.toLocaleString()}</div>
          <small class="text-muted">Total Score</small>
        </div>
        <div class="col-6 col-md-3 mb-3">
          <div class="fs-3 fw-bold text-info">{data.myStats.total_games_played}</div>
          <small class="text-muted">Games Played</small>
        </div>
        <div class="col-6 col-md-3 mb-3">
          <div class="fs-3 fw-bold text-warning">{data.myStats.current_streak}</div>
          <small class="text-muted">Day Streak <i class="fa-solid fa-fire text-orange"></i></small>
        </div>
      </div>
    </Card>
  {/if}

  <!-- Leaderboard Table -->
  <Card>
    {#snippet header()}
      <h5 class="mb-0">
        <i class="fa-solid fa-ranking-star me-2"></i>
        Top Players
      </h5>
    {/snippet}

    {#if data.leaderboard.length === 0}
      <div class="text-center py-5">
        <i class="fa-solid fa-gamepad fa-3x text-muted mb-3"></i>
        <p class="text-muted">No players yet. Be the first to play!</p>
      </div>
    {:else}
      <div class="table-responsive">
        <table class="table table-hover align-middle mb-0">
          <thead class="table-light">
            <tr>
              <th class="text-center" style="width: 80px;">Rank</th>
              <th>Player</th>
              <th class="text-end">Score</th>
              <th class="text-center d-none d-md-table-cell">Games</th>
              <th class="text-center d-none d-md-table-cell">Best</th>
              <th class="text-center d-none d-lg-table-cell">Accuracy</th>
            </tr>
          </thead>
          <tbody>
            {#each data.leaderboard as entry}
              <tr class:table-warning={entry.rank === 1} class:table-secondary={entry.rank === 2} class:table-info={entry.rank === 3}>
                <td class="text-center">
                  {#if getRankIcon(entry.rank).type === "icon"}
                    <span class="fs-5 {getRankIcon(entry.rank).colorClass}">
                      <i class="fa-solid {getRankIcon(entry.rank).value}"></i>
                    </span>
                  {:else}
                    <span class="fs-5 fw-bold text-muted">{getRankIcon(entry.rank).value}</span>
                  {/if}
                </td>
                <td>
                  <div class="d-flex align-items-center">
                    <div class="avatar-circle me-2">
                      {(entry.user_display_name || entry.user_name).charAt(0).toUpperCase()}
                    </div>
                    <div>
                      <div class="fw-semibold">{entry.user_display_name || entry.user_name}</div>
                      {#if entry.user_display_name}
                        <small class="text-muted">@{entry.user_name}</small>
                      {/if}
                    </div>
                  </div>
                </td>
                <td class="text-end">
                  <span class="fw-bold text-primary">{entry.total_score.toLocaleString()}</span>
                </td>
                <td class="text-center d-none d-md-table-cell">
                  <Badge variant="info">{entry.total_games_played}</Badge>
                </td>
                <td class="text-center d-none d-md-table-cell">
                  <Badge variant="success">{entry.best_score.toLocaleString()}</Badge>
                </td>
                <td class="text-center d-none d-lg-table-cell">
                  <Badge variant={Number(entry.average_accuracy) >= 80 ? "success" : Number(entry.average_accuracy) >= 50 ? "warning" : "danger"}>
                    {Number(entry.average_accuracy || 0).toFixed(1)}%
                  </Badge>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </Card>
</div>

<style>
  .avatar-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 1.1rem;
  }

  .table-warning .avatar-circle {
    background: linear-gradient(135deg, #f6d365 0%, #fda085 100%);
  }

  .table-secondary .avatar-circle {
    background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%);
    color: #333;
  }

  .table-info .avatar-circle {
    background: linear-gradient(135deg, #89f7fe 0%, #66a6ff 100%);
  }

  /* Rank medal colors */
  .rank-gold {
    color: #fbbf24;
    text-shadow: 0 2px 4px rgba(251, 191, 36, 0.4);
  }

  .rank-silver {
    color: #9ca3af;
    text-shadow: 0 2px 4px rgba(156, 163, 175, 0.4);
  }

  .rank-bronze {
    color: #cd7f32;
    text-shadow: 0 2px 4px rgba(205, 127, 50, 0.4);
  }

  /* Icon colors */
  .text-orange {
    color: #f97316;
  }

  :global(.fa-trophy.text-warning) {
    color: #fbbf24 !important;
  }

  :global(.fa-ranking-star) {
    color: #8b5cf6;
  }

  :global(.fa-user) {
    color: #3b82f6;
  }

  :global(.fa-gamepad.text-muted) {
    color: #9ca3af !important;
  }
</style>
