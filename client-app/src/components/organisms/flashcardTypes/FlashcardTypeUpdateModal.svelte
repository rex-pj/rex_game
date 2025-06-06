<script lang="ts">
  import type { FlashcardTypeRequest } from "$lib/models/flashcard-type";
  import { writable, type Writable } from "svelte/store";

  const {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    creationError = writable(""),
    initialData = writable({ id: 0, name: "", description: "" }),
  }: {
    showModal: Writable<boolean>;
    closeModal: () => void;
    isSubmitting: Writable<boolean>;
    submit: (data: FlashcardTypeRequest) => Promise<void>;
    creationError: Writable<string>;
    initialData: Writable<FlashcardTypeRequest>;
  } = $props();
  // Submit handler
  async function handleSubmit(event: Event) {
    event.preventDefault();
    creationError.set("");
    submit($initialData);
  }
</script>

{#if $showModal}
  <div class="modal fade show d-block" tabindex="-1" style="background:rgba(0,0,0,0.5)">
    <div class="modal-dialog">
      <div class="modal-content">
        <form onsubmit={handleSubmit}>
          <div class="modal-header">
            <h5 class="modal-title">Create Flashcard Type</h5>
            <button
              type="button"
              class="btn-close"
              aria-label="Close create flashcard type"
              onclick={closeModal}
            ></button>
          </div>
          <div class="modal-body">
            {#if $creationError}
              <div class="alert alert-danger">{$creationError}</div>
            {/if}
            <div class="mb-3">
              <label class="form-label" for="name">Name</label>
              <input class="form-control" bind:value={$initialData.id} required type="hidden" />
              <input class="form-control" bind:value={$initialData.name} required />
            </div>
            <div class="mb-3">
              <label class="form-label" for="description">Description</label>
              <textarea class="form-control" bind:value={$initialData.description}></textarea>
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
              {#if $isSubmitting}
                {#if !$initialData.id}
                  <span>Creating...</span>
                {:else}
                  <span>Updating...</span>
                {/if}
              {:else if !$initialData.id}
                <span>Create</span>
              {:else}
                <span>Update</span>
              {/if}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
