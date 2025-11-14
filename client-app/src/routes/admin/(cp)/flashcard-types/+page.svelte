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
  } from "./store";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import FlashcardTypeUpdateModal from "./FlashcardTypeUpdateModal.svelte";
  import FlashcardTypeDeleteModal from "./FlashcardTypeDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";
  import { canReadFlashcardTypes } from "$lib/services/accessService";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();

  onMount(() => {
    fetchItems($pager.currentPage);
  });
</script>

{#if canReadFlashcardTypes(data.adminUser)}
  <div class="container mt-4">
    <div class="row">
      <div class="col col-auto">
        <h3 class="mb-4">Flashcard Type Manager</h3>
      </div>
      <div class="col">
        <!-- Add button -->
        {#if canCreate(data.adminUser)}
          <button class="btn btn-primary mb-3" onclick={() => toggleCreationModal(true)}>Add</button
          >
        {/if}
      </div>
    </div>

    <table class="table table-striped">
      <thead>
        <tr>
          <th>#</th>
          <th>Name</th>
          <th>Description</th>
          <th>Created On</th>
          <th>Modified On</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each $items as item}
          <tr>
            <td>{item.id}</td>
            <td>{item.name}</td>
            <td>{item.description}</td>
            <td>{standardizeDate(item.created_date)}</td>
            <td>{standardizeDate(item.updated_date)}</td>
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
      <FlashcardTypeUpdateModal
        initialData={edittingData}
        showModal={showCreationModal}
        closeModal={() => toggleCreationModal(false)}
        {submit}
        {isSubmitting}
        {creationError}
      ></FlashcardTypeUpdateModal>
    {/if}
    {#if canDelete(data.adminUser)}
      <FlashcardTypeDeleteModal
        showModal={showDeletionModal}
        closeModal={() => toggleDeletionModal(false)}
        isSubmitting={isDeletionSubmitting}
        submit={deleteById}
        {deletionError}
        initialData={deletingData}
      ></FlashcardTypeDeleteModal>
    {/if}
  </div>
{:else}
  <div class="alert alert-danger">You do not have permission to view this page.</div>
{/if}

<style>
  .table {
    margin-top: 20px;
  }
</style>
