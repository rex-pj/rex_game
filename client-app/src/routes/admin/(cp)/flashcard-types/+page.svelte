<script>
  import { onMount } from "svelte";
  import {
    flashcardTypes,
    fetchFlashcardTypes,
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
  } from "./flashcardTypeStore";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import FlashcardTypeUpdateModal from "../../../../components/organisms/flashcardTypes/FlashcardTypeUpdateModal.svelte";
  import FlashcardTypeDeleteModal from "../../../../components/organisms/flashcardTypes/FlashcardTypeDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";

  onMount(() => {
    fetchFlashcardTypes(pager.currentPage);
  });
</script>

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">Flashcard Type Manager</h3>
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
        <th>Name</th>
        <th>Description</th>
        <th>Created On</th>
        <th>Modified On</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each $flashcardTypes as flashcardType}
        <tr>
          <td>{flashcardType.id}</td>
          <td>{flashcardType.name}</td>
          <td>{flashcardType.description}</td>
          <td>{standardizeDate(flashcardType.created_date)}</td>
          <td>{standardizeDate(flashcardType.updated_date)}</td>
          <td>
            <div class="dropdown">
              <button
                class="btn btn-link p-0"
                type="button"
                id="dropdownMenuButton-{flashcardType.id}"
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
                aria-labelledby="dropdownMenuButton-{flashcardType.id}"
              >
                <li>
                  <button
                    class="dropdown-item"
                    type="button"
                    onclick={() => {
                      openEditingModal(flashcardType.id);
                    }}>Edit</button
                  >
                </li>
                <li>
                  <button
                    class="dropdown-item text-danger"
                    type="button"
                    onclick={() => {
                      openDeletingModal(flashcardType.id);
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
  <FlashcardTypeUpdateModal
    initialData={edittingData}
    showModal={showCreationModal}
    closeModal={() => toggleCreationModal(false)}
    {submit}
    {isSubmitting}
    {creationError}
  ></FlashcardTypeUpdateModal>
  <FlashcardTypeDeleteModal
    showModal={showDeletionModal}
    closeModal={() => toggleDeletionModal(false)}
    isSubmitting={isDeletionSubmitting}
    submit={deleteById}
    {deletionError}
    initialData={deletingData}
  ></FlashcardTypeDeleteModal>
</div>

<style>
  .table {
    margin-top: 20px;
  }
</style>
