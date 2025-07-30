<script lang="ts">
  import { onMount } from "svelte";
  import {
    userRoles,
    rolePermissions,
    userPermissions,
    fetchUserRoleItems,
    fetchRolePermissionItems,
    fetchUserPermissionItems,
  } from "./store";
  import type { LayoutProps } from "../$types";
  const { data }: LayoutProps = $props();
  onMount(() => {
    if (!data.currentUser || !data.currentUser.id) {
      return;
    }
    fetchUserRoleItems();
    fetchRolePermissionItems();
    fetchUserPermissionItems();
  });
</script>

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">User Accesses Manager</h3>
    </div>
  </div>

  <div class="row">
    <div class="col-12">
      <h5>User Roles</h5>
      <table class="table table-bordered">
        <thead>
          <tr>
            <th>User</th>
            <th>Roles</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each $userRoles as userRole}
            <tr>
              <td>{userRole.user_name}</td>
              <td><div>{userRole.role_name}</div> </td>
              <td>
                <input
                  type="checkbox"
                  checked={$rolePermissions?.some((x) => x.role_id === userRole.id)}
                />
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <div class="col-12">
      <h5>Role Permissions</h5>
      <table class="table table-bordered">
        <thead>
          <tr>
            <th>Role</th>
            <th>Assigned Permissions</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each $rolePermissions as permissions}
            <tr>
              <td>{permissions.role_name}</td>
              <td
                ><div>{permissions.permission_name}</div>
                <p>{permissions.permission_code}</p>
                <p>{permissions.permission_module}</p>
              </td>
              <td>
                <input
                  type="checkbox"
                  checked={$rolePermissions?.some((x) => x.role_id === permissions.id)}
                />
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <div class="col-12 mt-4">
      <h5>User Permissions</h5>
      <table class="table table-bordered">
        <thead>
          <tr>
            <th>User</th>
            <th>Assigned Permissions</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each $userPermissions as permission}
            <tr>
              <td>{permission.user_name}</td>
              <td
                ><div>{permission.permission_name}</div>
                <p>{permission.permission_code}</p>
                <p>{permission.permission_module}</p>
              </td>
              <td>
                <input
                  type="checkbox"
                  checked={$userPermissions?.some((x) => x.permission_id === permission.id)}
                />
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>

<style>
  .table {
    margin-top: 20px;
  }
</style>
