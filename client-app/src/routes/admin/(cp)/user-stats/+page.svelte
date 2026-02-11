<script lang="ts">
  import { onMount } from "svelte";
  import {
    items,
    fetchItems,
    changePage,
    pager,
    showResetModal,
    isResetSubmitting,
    openResetModal,
    resetError,
    toggleResetModal,
    resettingData,
    resetStats,
    canReset,
  } from "./store";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import UserStatsResetModal from "./UserStatsResetModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";
  import { canReadUserStats } from "$lib/services/accessService";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();

  onMount(() => {
    fetchItems($pager.currentPage);
  });
</script>

{#if canReadUserStats(data.adminUser)}
  <div class="container mt-4">
    <div class="row">
      <div class="col col-auto">
        <h3 class="mb-4">User Stats Manager</h3>
      </div>
    </div>

    <table class="table table-striped">
      <thead>
        <tr>
          <th>#</th>
          <th>User</th>
          <th>Total Score</th>
          <th>Games Played</th>
          <th>Best Score</th>
          <th>Best Combo</th>
          <th>Avg Accuracy</th>
          <th>Current Streak</th>
          <th>Best Streak</th>
          <th>Last Played</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each $items as item}
          <tr>
            <td>{item.id}</td>
            <td>{item.user_display_name || item.user_name}</td>
            <td>{item.total_score}</td>
            <td>{item.total_games_played}</td>
            <td>{item.best_score}</td>
            <td>{item.best_combo}</td>
            <td>{item.average_accuracy}%</td>
            <td>{item.current_streak}</td>
            <td>{item.best_streak}</td>
            <td>{item.last_played_at ? standardizeDate(item.last_played_at) : "Never"}</td>
            <td>
              {#if canReset(data.adminUser)}
                <button
                  class="btn btn-link text-warning p-0"
                  type="button"
                  title="Reset stats"
                  onclick={() => {
                    openResetModal(item.user_id, item.user_display_name || item.user_name);
                  }}
                >
                  <i class="fa-solid fa-rotate-left"></i>
                </button>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>

    <div class="d-flex justify-content-center">
      <Pagination pager={$pager} {changePage} />
    </div>
    {#if canReset(data.adminUser)}
      <UserStatsResetModal
        showModal={showResetModal}
        closeModal={() => toggleResetModal(false)}
        isSubmitting={isResetSubmitting}
        submit={resetStats}
        error={resetError}
        initialData={resettingData}
      ></UserStatsResetModal>
    {/if}
  </div>
{:else}
  <div class="alert alert-danger">
    You do not have permission to view this page.
  </div>
{/if}

<style>
  .table {
    margin-top: 20px;
  }
</style>
