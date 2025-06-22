<script>
  import { onMount } from "svelte";
  import {
    roles,
    fetchRoles,
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
  } from "./roleStore";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import RoleUpdateModal from "../../../../components/organisms/roles/RoleUpdateModal.svelte";
  import RoleDeleteModal from "../../../../components/organisms/roles/RoleDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";

  onMount(() => {
    fetchRoles(pager.currentPage);
  });
</script>

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">Role Manager</h3>
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
      {#each $roles as role}
        <tr>
          <td>{role.id}</td>
          <td>{role.name}</td>
          <td>{role.description}</td>
          <td>{standardizeDate(role.created_date)}</td>
          <td>{standardizeDate(role.updated_date)}</td>
          <td>
            <div class="dropdown">
              <button
                class="btn btn-link p-0"
                type="button"
                id="dropdownMenuButton-{role.id}"
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
                aria-labelledby="dropdownMenuButton-{role.id}"
              >
                <li>
                  <button
                    class="dropdown-item"
                    type="button"
                    onclick={() => {
                      openEditingModal(role.id);
                    }}>Edit</button
                  >
                </li>
                <li>
                  <button
                    class="dropdown-item text-danger"
                    type="button"
                    onclick={() => {
                      openDeletingModal(role.id);
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
  <RoleUpdateModal
    initialData={edittingData}
    showModal={showCreationModal}
    closeModal={() => toggleCreationModal(false)}
    {submit}
    {isSubmitting}
    {creationError}
  ></RoleUpdateModal>
  <RoleDeleteModal
    showModal={showDeletionModal}
    closeModal={() => toggleDeletionModal(false)}
    isSubmitting={isDeletionSubmitting}
    submit={deleteById}
    {deletionError}
    initialData={deletingData}
  ></RoleDeleteModal>
</div>

<style>
  .table {
    margin-top: 20px;
  }
</style>
