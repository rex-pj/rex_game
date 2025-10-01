<script lang="ts">
  import { onMount } from "svelte";
  import {
    userAccessess,
    rolePermissions,
    fetchUserAccessItems,
    fetchRolePermissionItems,
    redirectToRoleAccesses,
    redirectToUserAccesses,
  } from "./store";
  import {
    canReadUserAccesses,
    canReadRolePermissions,
    canReadUserRoles,
    canReadUserPermissions,
    canEditUserAccesses,
    canEditRolePermissions,
  } from "$lib/services/accessService";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();
  onMount(() => {
    if (!data.currentUser || !data.currentUser.id) {
      return;
    }

    if (!canReadUserAccesses(data.currentUser) && !canReadRolePermissions(data.currentUser)) {
      return;
    }
    fetchUserAccessItems();
    fetchRolePermissionItems();
  });
</script>

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">User Accesses Manager</h3>
    </div>
  </div>

  <div class="row">
    {#if canReadUserAccesses(data.currentUser)}
      <div class="col-12">
        <h5>User Accesses</h5>
        <table class="table table-bordered">
          <thead>
            <tr>
              <th>User</th>
              {#if canReadUserRoles(data.currentUser)}
                <th>Assigned roles</th>
              {/if}
              {#if canReadUserPermissions(data.currentUser)}
                <th>Assigned permissions</th>
              {/if}
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each $userAccessess as userAccess}
              <tr>
                <td>{userAccess.user.name}</td>
                {#if canReadUserRoles(data.currentUser)}
                  <td
                    ><div>
                      {#each userAccess.roles as role}
                        <span class="badge bg-secondary me-1">{role.name}</span>
                      {/each}
                    </div>
                  </td>
                {/if}
                {#if canReadUserPermissions(data.currentUser)}
                  <td>
                    <div>
                      {#each userAccess.permissions as permission}
                        <span class="badge bg-secondary me-1">{permission.code}</span>
                      {/each}
                    </div>
                  </td>
                {/if}
                <td>
                  <button
                    class="btn btn-link p-0"
                    type="button"
                    id="dropdownUserRole-{userAccess.user.id}"
                    data-bs-toggle="dropdown"
                    aria-expanded="false"
                    aria-label="Actions"
                  >
                    <i class="fa-solid fa-ellipsis"></i>
                  </button>
                  <ul
                    class="dropdown-menu dropdown-menu-end"
                    aria-labelledby="dropdownUserRole-{userAccess.user.id}"
                  >
                    {#if canEditUserAccesses(data.currentUser)}
                      <li>
                        <button
                          class="dropdown-item text-success"
                          type="button"
                          onclick={() => {
                            redirectToUserAccesses(userAccess.user.id);
                          }}>User accesses</button
                        >
                      </li>
                    {/if}
                  </ul>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}

    {#if canReadRolePermissions(data.currentUser)}
      <div class="col-12">
        <h5>Role Permissions</h5>
        <table class="table table-bordered">
          <thead>
            <tr>
              <th>Role</th>
              {#if canReadRolePermissions(data.currentUser)}
                <th>Assigned permissions</th>
              {/if}
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each $rolePermissions as rolePermission}
              <tr>
                <td>{rolePermission.role.name}</td>
                <td>
                  {#if canReadRolePermissions(data.currentUser)}
                    <div>
                      {#each rolePermission.permissions as permission}
                        <span class="badge bg-secondary me-1">{permission.name}</span>
                      {/each}
                    </div>
                  {/if}
                </td>
                <td>
                  <button
                    class="btn btn-link p-0"
                    type="button"
                    id="dropdownRolePermission-{rolePermission.role.id}"
                    data-bs-toggle="dropdown"
                    aria-expanded="false"
                    aria-label="Actions"
                  >
                    <i class="fa-solid fa-ellipsis"></i>
                  </button>
                  <ul
                    class="dropdown-menu dropdown-menu-end"
                    aria-labelledby="dropdownRolePermission-{rolePermission.role.id}"
                  >
                    {#if canEditRolePermissions(data.currentUser)}
                      <li>
                        <button
                          class="dropdown-item text-success"
                          type="button"
                          onclick={() => {
                            redirectToRoleAccesses(rolePermission.role.id);
                          }}>Role accesses</button
                        >
                      </li>
                    {/if}
                  </ul>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>

<style>
  .table {
    margin-top: 20px;
  }
</style>
