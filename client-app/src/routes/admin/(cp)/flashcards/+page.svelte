<script lang="ts">
  import { onMount } from "svelte";
  import {
    flashcards,
    pager,
    fetchFlashcards,
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
  } from "./flashcardStore";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import FlashcardUpdateModal from "../../../../components/organisms/flashcards/FlashcardUpdateModal.svelte";
  import type { SelectOption } from "$lib/models/select-option";
  import FlashcardDeleteModal from "../../../../components/organisms/flashcards/FlashcardDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";

  onMount(() => {
    fetchFlashcards(pager.currentPage);
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

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">Flashcard Manager</h3>
    </div>
    <div class="col">
      <!-- Add button -->
      <button class="btn btn-primary mb-3" onclick={() => toggleCreationModal(true)}>Add</button>
    </div>
  </div>

  <table class="table table-striped">
    <thead>
      <tr>
        <th>#</th>
        <th colspan="2">Name</th>
        <th>Description</th>
        <th>Created On</th>
        <th>Modified On</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each $flashcards as flashcard}
        <tr>
          <td>{flashcard.id}</td>
          <td width="50">
            {#if flashcard.image_url}
              <img
                src={flashcard.image_url}
                alt="Flashcard"
                class="img-fluid"
                width="50"
                height="50"
              />
            {/if}
          </td>
          <td>{flashcard.name}</td>
          <td>
            <p>{flashcard.description}</p>
            <p>Sub: <i>{flashcard.sub_description}</i></p>
          </td>
          <td>{standardizeDate(flashcard.created_date)}</td>
          <td>{standardizeDate(flashcard.updated_date)}</td>
          <td>
            <div class="dropdown">
              <button
                class="btn btn-link p-0"
                type="button"
                id="dropdownMenuButton-{flashcard.id}"
                data-bs-toggle="dropdown"
                aria-expanded="false"
                aria-label="Actions"
              >
                <svg width="20" height="20" fill="currentColor" viewBox="0 0 16 16">
                  <circle cx="2" cy="8" r="1.5" />
                  <circle cx="8" cy="8" r="1.5" />
                  <circle cx="14" cy="8" r="1.5" />
                </svg>
              </button>
              <ul
                class="dropdown-menu dropdown-menu-end"
                aria-labelledby="dropdownMenuButton-{flashcard.id}"
              >
                <li>
                  <button
                    class="dropdown-item"
                    type="button"
                    onclick={() => {
                      openEditingModal(flashcard.id);
                    }}>Edit</button
                  >
                </li>
                <li>
                  <button
                    class="dropdown-item text-danger"
                    type="button"
                    onclick={() => {
                      openDeletingModal(flashcard.id);
                    }}>Delete</button
                  >
                </li>
              </ul>
            </div>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  <div class="d-flex justify-content-center">
    <Pagination {pager} {changePage} />
  </div>
  <FlashcardUpdateModal
    initialData={edittingData}
    showModal={showCreationModal}
    closeModal={() => toggleCreationModal(false)}
    {submit}
    {isSubmitting}
    {creationError}
    {flashcardTypeOptions}
  ></FlashcardUpdateModal>
  <FlashcardDeleteModal
    showModal={showDeletionModal}
    closeModal={() => toggleDeletionModal(false)}
    isSubmitting={isDeletionSubmitting}
    submit={deleteById}
    {deletionError}
    initialData={deletingData}
  ></FlashcardDeleteModal>
</div>

<style>
  .table {
    margin-top: 20px;
  }
</style>
