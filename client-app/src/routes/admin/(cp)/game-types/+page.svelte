<script lang="ts">
  import { onMount } from "svelte";
  import {
    items,
    fetchItems,
    changePage,
    pager,
    showCreationModal,
    isSubmitting,
    creationError,
    toggleCreationModal,
    openEditingModal,
    edittingData,
    submit,
    deleteById,
    showDeletionModal,
    isDeletionSubmitting,
    openDeletingModal,
    deletionError,
    toggleDeletionModal,
    deletingData,
    canUpdate,
    canDelete,
    canCreate,
    toggleActive,
    showFlashcardModal,
    flashcardModalGameType,
    assignedFlashcards,
    allFlashcards,
    flashcardError,
    isFlashcardLoading,
    openFlashcardModal,
    assignFlashcards,
    removeFlashcard,
    closeFlashcardModal,
  } from "./store";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import GameTypeUpdateModal from "./GameTypeUpdateModal.svelte";
  import GameTypeDeleteModal from "./GameTypeDeleteModal.svelte";
  import GameTypeFlashcardsModal from "./GameTypeFlashcardsModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";
  import { canReadGameTypes } from "$lib/services/accessService";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();

  onMount(() => {
    fetchItems($pager.currentPage);
  });
</script>

{#if canReadGameTypes(data.adminUser)}
  <div class="container mt-4">
    <div class="row">
      <div class="col col-auto">
        <h3 class="mb-4">Game Type Manager</h3>
      </div>
      <div class="col">
        {#if canCreate(data.adminUser)}
          <button
            class="btn btn-primary mb-3"
            onclick={() => toggleCreationModal(true)}>Add</button
          >
        {/if}
      </div>
    </div>

    <table class="table table-striped">
      <thead>
        <tr>
          <th>#</th>
          <th>Code</th>
          <th>Name</th>
          <th>Description</th>
          <th>Icon</th>
          <th>Status</th>
          <th>Created On</th>
          <th>Modified On</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each $items as item}
          <tr>
            <td>{item.id}</td>
            <td>{item.code}</td>
            <td>{item.name}</td>
            <td>{item.description}</td>
            <td>{item.icon}</td>
            <td>
              <div class="form-check form-switch">
                <input
                  class="form-check-input"
                  type="checkbox"
                  role="switch"
                  checked={item.is_actived}
                  onchange={() => toggleActive(item.id)}
                  disabled={!canUpdate(data.adminUser)}
                />
                <span class="form-check-label">
                  {item.is_actived ? "Active" : "Inactive"}
                </span>
              </div>
            </td>
            <td>{standardizeDate(item.created_on)}</td>
            <td>{standardizeDate(item.updated_on)}</td>
            <td>
              <div class="dropdown">
                <button
                  class="btn btn-link p-0"
                  type="button"
                  id="dropdownMenuButton-{item.id}"
                  data-bs-toggle="dropdown"
                  aria-expanded="false"
                  aria-label="Actions"
                >
                  <i class="fa-solid fa-ellipsis"></i>
                </button>
                <ul
                  class="dropdown-menu dropdown-menu-end"
                  aria-labelledby="dropdownMenuButton-{item.id}"
                >
                  {#if canUpdate(data.adminUser) || canCreate(data.adminUser)}
                    <li>
                      <button
                        class="dropdown-item"
                        type="button"
                        onclick={() => {
                          openEditingModal(item.id);
                        }}>Edit</button
                      >
                    </li>
                  {/if}
                  {#if canUpdate(data.adminUser)}
                    <li>
                      <button
                        class="dropdown-item"
                        type="button"
                        onclick={() => {
                          openFlashcardModal(item.id);
                        }}>
                        <i class="fa-solid fa-clone me-1"></i> Flashcards
                      </button>
                    </li>
                  {/if}
                  {#if canDelete(data.adminUser)}
                    <li>
                      <button
                        class="dropdown-item text-danger"
                        type="button"
                        onclick={() => {
                          openDeletingModal(item.id);
                        }}>Delete</button
                      >
                    </li>
                  {/if}
                </ul>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>

    <div class="d-flex justify-content-center">
      <Pagination pager={$pager} {changePage} />
    </div>
    {#if canUpdate(data.adminUser)}
      <GameTypeUpdateModal
        initialData={edittingData}
        showModal={showCreationModal}
        closeModal={() => toggleCreationModal(false)}
        {submit}
        {isSubmitting}
        {creationError}
      ></GameTypeUpdateModal>
    {/if}
    {#if canDelete(data.adminUser)}
      <GameTypeDeleteModal
        showModal={showDeletionModal}
        closeModal={() => toggleDeletionModal(false)}
        isSubmitting={isDeletionSubmitting}
        submit={deleteById}
        {deletionError}
        initialData={deletingData}
      ></GameTypeDeleteModal>
    {/if}
    {#if canUpdate(data.adminUser)}
      <GameTypeFlashcardsModal
        showModal={showFlashcardModal}
        gameType={flashcardModalGameType}
        {assignedFlashcards}
        {allFlashcards}
        error={flashcardError}
        loading={isFlashcardLoading}
        onAssign={assignFlashcards}
        onRemove={removeFlashcard}
        onClose={closeFlashcardModal}
      />
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
