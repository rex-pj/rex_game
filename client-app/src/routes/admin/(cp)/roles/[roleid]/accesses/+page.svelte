<script lang="ts">
  import { onMount } from "svelte";
  import type { PageProps } from "./$types";
  import { permissions, initPermission, togglePermission } from "./store";
  const { data }: PageProps = $props();
  onMount(() => {
    if (!data.currentUser || !data.currentUser.id) {
      return;
    }

    initPermission(data.permissions, data.rolePermissions);
  });
</script>

<div class="container mt-4">
  <div class="row">
    <div class="col col-auto">
      <h3 class="mb-4">Edit Role Accesses</h3>
    </div>
  </div>

  <div class="row">
    <div class="col-12 mt-4">
      <form method="POST" action="?/permissions">
        <div class="card border-danger">
          <div class="card-header">
            <h5>
              Role Permissions <span class="badge rounded-pill text-bg-success"
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
                          <input
                            type="checkbox"
                            checked={permission.assigned}
                            value={permission.code}
                            onchange={(e) => togglePermission($permissions, permission.code, e)}
                            name="permission_codes"
                          />
                        </td>
                      </tr>
                    {/each}
                  {/if}
                </tbody>
              </table>
            </div>
          </div>
          <div class="card-footer">
            <button
              type="button"
              class="btn btn-secondary"
              onclick={(e) => {
                initPermission(data.permissions, data.rolePermissions);
              }}
            >
              Reset
            </button>
            <button class="btn btn-primary" type="submit"> Save </button>
          </div>
        </div>
      </form>
    </div>
  </div>
</div>

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
