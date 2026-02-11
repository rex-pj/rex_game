<script lang="ts">
  import { onMount } from "svelte";
  import {
    items,
    fetchItems,
    changePage,
    pager,
    deleteById,
    showDeletionModal,
    isDeletionSubmitting,
    openDeletingModal,
    deletionError,
    toggleDeletionModal,
    deletingData,
    canDelete,
  } from "./store";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import GameSessionDeleteModal from "./GameSessionDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";
  import { canReadGameSessions } from "$lib/services/accessService";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();

  onMount(() => {
    fetchItems($pager.currentPage);
  });
</script>

{#if canReadGameSessions(data.adminUser)}
  <div class="container mt-4">
    <div class="row">
      <div class="col col-auto">
        <h3 class="mb-4">Game Session Manager</h3>
      </div>
    </div>

    <table class="table table-striped">
      <thead>
        <tr>
          <th>#</th>
          <th>User</th>
          <th>Game Type</th>
          <th>Score</th>
          <th>Accuracy</th>
          <th>Time (s)</th>
          <th>Correct</th>
          <th>Wrong</th>
          <th>Max Combo</th>
          <th>Completed At</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each $items as item}
          <tr>
            <td>{item.id}</td>
            <td>{item.user_display_name || item.user_name}</td>
            <td>{item.game_type_name || item.game_type_code}</td>
            <td>{item.score}{item.max_score ? `/${item.max_score}` : ""}</td>
            <td>{item.accuracy ? `${item.accuracy}%` : "N/A"}</td>
            <td>{item.time_spent_seconds}</td>
            <td>{item.correct_answers}</td>
            <td>{item.wrong_answers}</td>
            <td>{item.combo_max}</td>
            <td>{item.completed_at ? standardizeDate(item.completed_at) : "In progress"}</td>
            <td>
              {#if canDelete(data.adminUser)}
                <button
                  class="btn btn-link text-danger p-0"
                  type="button"
                  onclick={() => {
                    openDeletingModal(item.id, `Session #${item.id} - ${item.user_display_name || item.user_name}`);
                  }}
                >
                  <i class="fa-solid fa-trash"></i>
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
    {#if canDelete(data.adminUser)}
      <GameSessionDeleteModal
        showModal={showDeletionModal}
        closeModal={() => toggleDeletionModal(false)}
        isSubmitting={isDeletionSubmitting}
        submit={deleteById}
        {deletionError}
        initialData={deletingData}
      ></GameSessionDeleteModal>
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
