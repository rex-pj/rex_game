<script lang="ts">
  import type { GameCompleteResponse, Achievement } from "$lib/api/scoringApi";
  import Button from "../../atoms/button/Button.svelte";
  import Badge from "../../atoms/badge/Badge.svelte";
  import Card from "../../atoms/card/Card.svelte";

  interface Props {
    result: GameCompleteResponse;
    onPlayAgain?: () => void;
    onViewLeaderboard?: () => void;
  }

  let { result, onPlayAgain, onViewLeaderboard }: Props = $props();

  const session = $derived(result.session);
  const stats = $derived(result.updated_stats);
  const newAchievements = $derived(result.new_achievements);

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  function getAccuracyColor(accuracy: number): "success" | "warning" | "danger" {
    if (accuracy >= 80) return "success";
    if (accuracy >= 50) return "warning";
    return "danger";
  }
</script>

<div class="game-results">
  <!-- Score Display -->
  <div class="text-center mb-4">
    <div class="score-circle mx-auto mb-3">
      <div class="score-value">{session.score}</div>
      <div class="score-label">Points</div>
    </div>

    {#if session.accuracy}
      <Badge variant={getAccuracyColor(session.accuracy)} class="fs-5 px-3 py-2">
        {session.accuracy.toFixed(1)}% Accuracy
      </Badge>
    {/if}
  </div>

  <!-- Stats Grid -->
  <div class="row g-3 mb-4">
    <div class="col-6 col-md-3">
      <div class="stat-card text-center p-3 rounded-3">
        <i class="fa-solid fa-check-circle text-success fa-2x mb-2"></i>
        <div class="fs-4 fw-bold">{session.correct_answers}</div>
        <small class="text-muted">Correct</small>
      </div>
    </div>
    <div class="col-6 col-md-3">
      <div class="stat-card text-center p-3 rounded-3">
        <i class="fa-solid fa-times-circle text-danger fa-2x mb-2"></i>
        <div class="fs-4 fw-bold">{session.wrong_answers}</div>
        <small class="text-muted">Wrong</small>
      </div>
    </div>
    <div class="col-6 col-md-3">
      <div class="stat-card text-center p-3 rounded-3">
        <i class="fa-solid fa-fire text-warning fa-2x mb-2"></i>
        <div class="fs-4 fw-bold">{session.combo_max}x</div>
        <small class="text-muted">Best Combo</small>
      </div>
    </div>
    <div class="col-6 col-md-3">
      <div class="stat-card text-center p-3 rounded-3">
        <i class="fa-solid fa-clock text-info fa-2x mb-2"></i>
        <div class="fs-4 fw-bold">{formatTime(session.time_spent_seconds)}</div>
        <small class="text-muted">Time</small>
      </div>
    </div>
  </div>

  <!-- New Achievements -->
  {#if newAchievements.length > 0}
    <Card class="mb-4 border-warning bg-warning bg-opacity-10">
      {#snippet header()}
        <h5 class="mb-0 text-warning">
          <i class="fa-solid fa-trophy me-2"></i>
          New Achievements Unlocked!
        </h5>
      {/snippet}
      <div class="row g-2">
        {#each newAchievements as achievement}
          <div class="col-12">
            <div class="d-flex align-items-center p-2 bg-white rounded-2">
              <div class="achievement-icon me-3">
                <i class="fa-solid {achievement.icon || 'fa-star'} fa-lg text-warning"></i>
              </div>
              <div class="flex-grow-1">
                <div class="fw-semibold">{achievement.name}</div>
                <small class="text-muted">{achievement.description}</small>
              </div>
              <Badge variant="warning">+{achievement.points} pts</Badge>
            </div>
          </div>
        {/each}
      </div>
    </Card>
  {/if}

  <!-- Updated Stats -->
  <Card class="mb-4">
    {#snippet header()}
      <h6 class="mb-0">
        <i class="fa-solid fa-chart-line me-2"></i>
        Your Progress
      </h6>
    {/snippet}
    <div class="row text-center">
      <div class="col-4">
        <div class="fs-5 fw-bold text-primary">{stats.total_score.toLocaleString()}</div>
        <small class="text-muted">Total Score</small>
      </div>
      <div class="col-4">
        <div class="fs-5 fw-bold text-info">{stats.total_games_played}</div>
        <small class="text-muted">Games</small>
      </div>
      <div class="col-4">
        <div class="fs-5 fw-bold text-success">#{stats.rank ?? "-"}</div>
        <small class="text-muted">Rank</small>
      </div>
    </div>
  </Card>

  <!-- Actions -->
  <div class="d-flex gap-2 justify-content-center">
    {#if onPlayAgain}
      <Button variant="primary" size="lg" onclick={onPlayAgain}>
        <i class="fa-solid fa-redo me-2"></i>
        Play Again
      </Button>
    {/if}
    {#if onViewLeaderboard}
      <Button variant="outline-primary" size="lg" onclick={onViewLeaderboard}>
        <i class="fa-solid fa-trophy me-2"></i>
        Leaderboard
      </Button>
    {/if}
  </div>
</div>

<style>
  .score-circle {
    width: 150px;
    height: 150px;
    border-radius: 50%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow: 0 10px 40px rgba(102, 126, 234, 0.4);
  }

  .score-value {
    font-size: 3rem;
    font-weight: bold;
    line-height: 1;
  }

  .score-label {
    font-size: 0.9rem;
    opacity: 0.9;
  }

  .stat-card {
    background: #f8f9fa;
    border: 1px solid #e9ecef;
    transition: transform 0.2s ease;
  }

  .stat-card:hover {
    transform: translateY(-2px);
  }

  .achievement-icon {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #fff3cd;
    border-radius: 50%;
  }
</style>
