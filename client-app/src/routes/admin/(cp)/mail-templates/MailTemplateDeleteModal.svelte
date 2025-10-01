<script lang="ts">
  import { writable, type Writable } from "svelte/store";

  const {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    deletionError = writable(""),
    initialData = writable({ id: 0, name: "" }),
  }: {
    showModal: Writable<boolean>;
    closeModal: () => void;
    isSubmitting: Writable<boolean>;
    submit: (id: number) => Promise<void>;
    deletionError: Writable<string>;
    initialData: Writable<{ id: number; name: string }>;
  } = $props();
  // Submit handler
  async function handleSubmit(event: Event) {
    event.preventDefault();
    deletionError.set("");
    submit($initialData.id);
  }
</script>

{#if $showModal}
  <div class="modal fade show d-block" tabindex="-1" style="background:rgba(0,0,0,0.5)">
    <div class="modal-dialog">
      <div class="modal-content">
        <form onsubmit={handleSubmit}>
          <div class="modal-header">
            <h5 class="modal-title">Delete Email Template</h5>
            <button
              type="button"
              class="btn-close"
              aria-label="Close delete email template"
              onclick={closeModal}
            ></button>
          </div>
          <div class="modal-body">
            {#if $deletionError}
              <div class="alert alert-danger">{$deletionError}</div>
            {/if}
            <div class="mb-3">
              <label class="form-label" for="name">Name</label>
              <input class="form-control" bind:value={$initialData.id} required type="hidden" />
              <span>Do you want to delete the email template: {$initialData.name}</span>
            </div>
          </div>
          <div class="modal-footer">
            <button
              type="button"
              class="btn btn-secondary"
              onclick={closeModal}
              disabled={$isSubmitting}>Cancel</button
            >
            <button type="submit" class="btn btn-primary" disabled={$isSubmitting}>
              {$isSubmitting ? "Deleting..." : "Delete"}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
