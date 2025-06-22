<script lang="ts">
  import { onMount } from "svelte";
  import {
    users,
    pager,
    fetchUsers,
    changePage,
    showCreationModal,
    isSubmitting,
    creationError,
    toggleCreationModal,
    edittingData,
    submit,
    deleteById,
    showDeletionModal,
    isDeletionSubmitting,
    openDeletingModal,
    deletionError,
    toggleDeletionModal,
    deletingData,
    openEditingModal,
  } from "./userStore";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import UserUpdateModal from "../../../../components/organisms/users/UserUpdateModal.svelte";
  import type { SelectOption } from "$lib/models/select-option";
  import UserDeleteModal from "../../../../components/organisms/users/UserDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";

  onMount(() => {
    fetchUsers(pager.currentPage);
  });
</script>

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">User Manager</h3>
    </div>
  </div>

  <table class="table table-striped">
    <thead>
      <tr>
        <th>#</th>
        <th colspan="2">Name</th>
        <th>Email</th>
        <th>Display Name</th>
        <th>Created On</th>
        <th>Modified On</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each $users as user}
        <tr>
          <td>{user.id}</td>
          <td>{user.email}</td>
          <td>{user.display_name}</td>
          <td>{user.name}</td>

          <td>{standardizeDate(user.created_date)}</td>
          <td>{standardizeDate(user.updated_date)}</td>
          <td>
            <div class="dropdown">
              <button
                class="btn btn-link p-0"
                type="button"
                id="dropdownMenuButton-{user.id}"
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
                aria-labelledby="dropdownMenuButton-{user.id}"
              >
                <li>
                  <button
                    class="dropdown-item"
                    type="button"
                    onclick={() => {
                      openEditingModal(user.id);
                    }}>Edit</button
                  >
                </li>
                <li>
                  <button
                    class="dropdown-item text-danger"
                    type="button"
                    onclick={() => {
                      openDeletingModal(user.id);
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
  <UserUpdateModal
    initialData={edittingData}
    showModal={showCreationModal}
    closeModal={() => toggleCreationModal(false)}
    {submit}
    {isSubmitting}
    {creationError}
  ></UserUpdateModal>
  <UserDeleteModal
    showModal={showDeletionModal}
    closeModal={() => toggleDeletionModal(false)}
    isSubmitting={isDeletionSubmitting}
    submit={deleteById}
    {deletionError}
    initialData={deletingData}
  ></UserDeleteModal>
</div>

<style>
  .table {
    margin-top: 20px;
  }
</style>
