<script lang="ts">
  import { onMount } from "svelte";
  import {
    items,
    pager,
    fetchItems,
    changePage,
    showCreationModal,
    isSubmitting,
    creationError,
    toggleCreationModal,
    edittingData,
    submit,
    flashcardTypeSuggestions,
    deleteById,
    showDeletionModal,
    isDeletionSubmitting,
    openDeletingModal,
    deletionError,
    toggleDeletionModal,
    deletingData,
    openEditingModal,
    canCreate,
    canUpdate,
    canDelete,
    toggleActive,
  } from "./store";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import FlashcardUpdateModal from "./FlashcardUpdateModal.svelte";
  import type { SelectOption } from "$lib/models/select-option";
  import FlashcardDeleteModal from "./FlashcardDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";
  import { canReadFlashcards } from "$lib/services/accessService";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();

  onMount(() => {
    fetchItems($pager.currentPage);
  });

  let flashcardTypeOptions: SelectOption[] = $state([]);
  $effect(() => {
    if ($flashcardTypeSuggestions) {
      flashcardTypeOptions = $flashcardTypeSuggestions.map((type) => ({
        value: type.id,
        label: type.name,
      }));
    }
  });
</script>

{#if canReadFlashcards(data.adminUser)}
  <div class="container mt-4">
    <div class="row">
      <div class="col col-auto">
        <h3 class="mb-4">Flashcard Manager</h3>
      </div>
      <div class="col">
        <!-- Add button -->
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
          <th colspan="2">Name</th>
          <th>Description</th>
          <th>Types</th>
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
            <td width="50">
              {#if item.image_url}
                <img
                  src={item.image_url}
                  alt="Flashcard"
                  class="img-fluid"
                  width="50"
                  height="50"
                />
              {/if}
            </td>
            <td>{item.name}</td>
            <td>
              <p class="mb-1">{item.description}</p>
              <p class="mb-0">Sub: <i>{item.sub_description}</i></p>
            </td>
            <td>
              {#if item.flashcard_type_names?.length}
                {item.flashcard_type_names.join(", ")}
              {:else}
                <span class="text-muted">—</span>
              {/if}
            </td>
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
                  {#if canUpdate(data.adminUser)}
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
    {#if canUpdate(data.adminUser) || canCreate(data.adminUser)}
      <FlashcardUpdateModal
        initialData={edittingData}
        showModal={showCreationModal}
        closeModal={() => toggleCreationModal(false)}
        {submit}
        {isSubmitting}
        {creationError}
        {flashcardTypeOptions}
      ></FlashcardUpdateModal>
    {/if}
    {#if canDelete(data.adminUser)}
      <FlashcardDeleteModal
        showModal={showDeletionModal}
        closeModal={() => toggleDeletionModal(false)}
        isSubmitting={isDeletionSubmitting}
        submit={deleteById}
        {deletionError}
        initialData={deletingData}
      ></FlashcardDeleteModal>
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
