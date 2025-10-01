<script lang="ts">
  import { onMount } from "svelte";
  import type { PageProps } from "./$types";
  import { ROLE_NAMES } from "$lib/common/contants";
  import {
    roles,
    initRole,
    toggleRole,
    permissions,
    initPermission,
    togglePermission,
  } from "./store";
  import * as accessService from "$lib/services/accessService";
  const { data }: PageProps = $props();
  onMount(() => {
    if (!data.currentUser || !data.currentUser.id) {
      return;
    }

    initRole(data.roles, data.userRoles);
    initPermission(data.permissions, data.userPermissions);
  });
</script>

{#if accessService.canReadUserAccesses(data.currentUser)}
  <div class="container mt-4">
    <div class="row">
      <div class="col col-auto">
        {#if accessService.canEditUserAccesses(data.currentUser)}
          <h3 class="mb-4">Edit User Accesses</h3>
        {:else}
          <h3 class="mb-4">User Accesses</h3>
        {/if}
      </div>
    </div>

    <div class="row">
      <div class="col-12">
        <form method="POST" action="?/roles">
          <div class="card border-primary">
            <div class="card-header">
              <h5>
                User Roles <span class="badge rounded-pill text-bg-success"
                  >{!$roles ? 0 : $roles.length}</span
                >
              </h5>
            </div>
            <div class="card-body">
              <p class="text-muted">
                Assign or unassign roles to the user. The Root Admin role cannot be assigned or
                unassigned.
              </p>
              <div class="table-responsive">
                <table class="table table-bordered m-0">
                  <thead>
                    <tr>
                      <th>Role</th>
                      <th>Assigned</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#if !$roles || $roles.length === 0}
                      <tr>
                        <td colspan="2">No roles available</td>
                      </tr>
                    {:else}
                      {#each $roles as role (role.id)}
                        <tr>
                          <td
                            ><figure class="text-left">
                              <blockquote class="blockquote">
                                <p>{role.name}</p>
                              </blockquote>
                              <figcaption class="blockquote-footer">
                                <cite>{role.description}</cite>
                              </figcaption>
                            </figure></td
                          >
                          <td>
                            {#if role.name === ROLE_NAMES.ROOT_ADMIN}
                              <input type="checkbox" disabled checked={role.assigned} />
                            {:else if accessService.canEditUserRoles(data.currentUser)}
                              <input
                                type="checkbox"
                                checked={role.assigned}
                                value={role.id}
                                onchange={(e) => toggleRole($roles, role.id, e)}
                                name="role_ids"
                              />
                            {:else}
                              <span class="badge {role.assigned ? 'bg-success' : 'bg-secondary'}">
                                {role.assigned ? "Yes" : "No"}
                              </span>
                            {/if}
                          </td>
                        </tr>
                      {/each}
                    {/if}
                  </tbody>
                </table>
              </div>
            </div>
            <div class="card-footer">
              {#if accessService.canEditUserRoles(data.currentUser)}
                <button
                  type="button"
                  class="btn btn-secondary"
                  onclick={(e) => {
                    initRole(data.roles, data.userRoles);
                  }}
                >
                  Reset
                </button>
                <button class="btn btn-primary" type="submit"> Save </button>
              {/if}
            </div>
          </div>
        </form>
      </div>
      <div class="col-12 mt-4">
        <form method="POST" action="?/permissions">
          <div class="card border-danger">
            <div class="card-header">
              <h5>
                User Permissions <span class="badge rounded-pill text-bg-success"
                  >{!$permissions ? 0 : $permissions.length}</span
                >
              </h5>
            </div>
            <div class="card-body">
              <div class="table-responsive">
                <table class="table table-bordered">
                  <thead>
                    <tr>
                      <th>Permission</th>
                      <th>Code</th>
                      <th><input type="checkbox" class="me-1" />Assigned </th>
                    </tr>
                  </thead>
                  <tbody>
                    {#if !$permissions || $permissions.length === 0}
                      <tr>
                        <td colspan="3">No permissions available</td>
                      </tr>
                    {:else}
                      {#each $permissions as permission}
                        <tr>
                          <td>
                            <figure class="text-left">
                              <blockquote class="blockquote">
                                <p>{permission.name}</p>
                              </blockquote>
                              <figcaption class="blockquote-footer">
                                <cite>{permission.description}</cite>
                              </figcaption>
                            </figure></td
                          >
                          <td
                            ><figure class="text-left">
                              <blockquote class="blockquote">
                                <p>{permission.code}</p>
                              </blockquote>
                              <figcaption class="blockquote-footer">
                                Module: <cite>{permission.module}</cite>
                              </figcaption>
                            </figure></td
                          >
                          <td>
                            {#if accessService.canEditUserPermissions(data.currentUser)}
                              <input
                                type="checkbox"
                                checked={permission.assigned}
                                value={permission.code}
                                onchange={(e) => togglePermission($permissions, permission.code, e)}
                                name="permission_codes"
                              />
                            {:else}
                              <span
                                class="badge {permission.assigned ? 'bg-success' : 'bg-secondary'}"
                              >
                                {permission.assigned ? "Yes" : "No"}
                              </span>
                            {/if}
                          </td></tr
                        >
                      {/each}
                    {/if}
                  </tbody>
                </table>
              </div>
            </div>
            <div class="card-footer">
              {#if accessService.canEditUserPermissions(data.currentUser)}
                <button
                  type="button"
                  class="btn btn-secondary"
                  onclick={(e) => {
                    initPermission(data.permissions, data.userPermissions);
                  }}
                >
                  Reset
                </button>
                <button class="btn btn-primary" type="submit"> Save </button>
              {/if}
            </div>
          </div>
        </form>
      </div>
    </div>
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

  .blockquote-footer::before {
    content: "";
  }

  .table td figure {
    margin: 0;
  }

  .table td .blockquote-footer {
    margin-bottom: 0;
  }
</style>
