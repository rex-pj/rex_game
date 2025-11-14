<script lang="ts">
  import { onMount } from "svelte";
  import {
    items,
    pager,
    getList,
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
    redirectToAccesses,
    canDelete,
    canUpdate,
  } from "./store";
  import Pagination from "../../../../components/molecules/pagination/pagination.svelte";
  import UserUpdateModal from "./UserUpdateModal.svelte";
  import UserDeleteModal from "./UserDeleteModal.svelte";
  import { standardizeDate } from "$lib/helpers/dateTimeHelper";
  import { canReadUserAccesses, canReadUsers } from "$lib/services/accessService";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();

  onMount(() => {
    getList($pager.currentPage);
  });
</script>

{#if canReadUsers(data.adminUser)}
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
        {#each $items as item}
          <tr>
            <td>{item.id}</td>
            <td>{item.email}</td>
            <td>{item.display_name}</td>
            <td>{item.name}</td>

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
                  {#if canReadUserAccesses(data.adminUser)}
                    <li>
                      <button
                        class="dropdown-item text-success"
                        type="button"
                        onclick={() => {
                          redirectToAccesses(item.id);
                        }}>User accesses</button
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
      <UserUpdateModal
        initialData={edittingData}
        showModal={showCreationModal}
        closeModal={() => toggleCreationModal(false)}
        {submit}
        {isSubmitting}
        {creationError}
      ></UserUpdateModal>
    {/if}

    {#if canDelete(data.adminUser)}
      <UserDeleteModal
        showModal={showDeletionModal}
        closeModal={() => toggleDeletionModal(false)}
        isSubmitting={isDeletionSubmitting}
        submit={deleteById}
        {deletionError}
        initialData={deletingData}
      ></UserDeleteModal>
    {/if}
  </div>
{:else}
  <div class="container mt-4">
    <div class="alert alert-danger" role="alert">You do not have permission to view this page.</div>
  </div>
{/if}

<style>
  .table {
    margin-top: 20px;
  }
</style>
