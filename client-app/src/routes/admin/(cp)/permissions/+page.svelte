<script>
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
  } from "./store";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import PermissionUpdateModal from "../../../../components/organisms/permissions/PermissionUpdateModal.svelte";
  import PermissionDeleteModal from "../../../../components/organisms/permissions/PermissionDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";

  onMount(() => {
    fetchItems($pager.currentPage);
  });
</script>

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">Permission Manager</h3>
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
        <th>Code</th>
        <th>Module</th>
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
          <td>{item.code}</td>
          <td>{item.module}</td>
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
                <svg width="20" height="20" fill="currentColor" viewBox="0 0 16 16">
                  <circle cx="2" cy="8" r="1.5" />
                  <circle cx="8" cy="8" r="1.5" />
                  <circle cx="14" cy="8" r="1.5" />
                </svg>
              </button>
              <ul
                class="dropdown-menu dropdown-menu-end"
                aria-labelledby="dropdownMenuButton-{item.id}"
              >
                <li>
                  <button
                    class="dropdown-item"
                    type="button"
                    onclick={() => {
                      openEditingModal(item.id);
                    }}>Edit</button
                  >
                </li>
                <li>
                  <button
                    class="dropdown-item text-danger"
                    type="button"
                    onclick={() => {
                      openDeletingModal(item.id);
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
    <Pagination pager={$pager} {changePage} />
  </div>
  <PermissionUpdateModal
    initialData={edittingData}
    showModal={showCreationModal}
    closeModal={() => toggleCreationModal(false)}
    {submit}
    {isSubmitting}
    {creationError}
  ></PermissionUpdateModal>
  <PermissionDeleteModal
    showModal={showDeletionModal}
    closeModal={() => toggleDeletionModal(false)}
    isSubmitting={isDeletionSubmitting}
    submit={deleteById}
    {deletionError}
    initialData={deletingData}
  ></PermissionDeleteModal>
</div>

<style>
  .table {
    margin-top: 20px;
  }
</style>
