<script lang="ts">
  import { setImageBase64Url } from "$lib/helpers/imageHelper";
  import type { FlashcardRequest } from "$lib/models/flashcard";
  import type { SelectOption } from "$lib/models/select-option";
  import Select from "svelte-select";
  import { writable, type Writable } from "svelte/store";

  const {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    creationError = writable(""),
    initialData = writable({
      id: 0,
      name: "",
      description: "",
      sub_description: "",
      type_ids: [],
    }),
    flashcardTypeOptions = [],
  }: {
    showModal: Writable<boolean>;
    closeModal: () => void;
    isSubmitting: Writable<boolean>;
    submit: (data: FlashcardRequest) => Promise<void>;
    creationError: Writable<string>;
    initialData: Writable<FlashcardRequest>;
    flashcardTypeOptions: SelectOption[];
  } = $props();

  // Submit handler
  async function handleSubmit(event: Event) {
    event.preventDefault();
    creationError.set("");
    submit($initialData);
  }

  async function onImageChange(e: Event, data: Writable<FlashcardRequest>) {
    const target = e.target as any;
    const fileData = target.files[0];
    if (!fileData) {
      return;
    }
    const imageUrl = await setImageBase64Url(fileData);
    if (!imageUrl) {
      return;
    }

    data.update((currentData) => {
      const updatedData = { ...currentData };
      updatedData.image_data = fileData;
      updatedData.image_url = imageUrl;
      return updatedData;
    });
  }
</script>

{#if $showModal}
  <div class="modal fade show d-block" tabindex="-1" style="background:rgba(0,0,0,0.5)">
    <div class="modal-dialog">
      <div class="modal-content">
        <form onsubmit={handleSubmit}>
          <div class="modal-header">
            <h5 class="modal-title">Create Flashcard</h5>
            <button
              type="button"
              class="btn-close"
              aria-label="Close create flashcard"
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
            <div class="mb-3">
              <label class="form-label" for="description">Sub Description</label>
              <textarea class="form-control" bind:value={$initialData.sub_description}></textarea>
            </div>
            <div class="mb-3">
              <label class="form-label" for="type_ids">Types</label>
              <Select
                id="type_ids"
                class="form-select"
                items={flashcardTypeOptions}
                multiple={true}
                bind:value={$initialData.types}
                bind:justValue={$initialData.type_ids}
                placeholder="Select types"
              ></Select>
            </div>
            <div class="mb-3">
              <label class="form-label" for="image_data">Image</label>
              <input
                accept="image/*"
                type="file"
                id="image_data"
                class="form-control"
                onchange={async (e) => {
                  onImageChange(e, initialData);
                }}
              />
              {#if !$initialData.image_url && $initialData.original_image_url}
                <p>Current Image:</p>
                <div class="mt-2 thumbnail">
                  <img
                    src={$initialData.original_image_url}
                    alt="Flashcard"
                    class="img-fluid"
                    width="50%"
                  />
                </div>
              {:else if $initialData.image_url}
                <p>Updated Image:</p>
                <div class="mt-2 thumbnail">
                  <faUser></faUser>
                  <button
                    type="button"
                    class="btn btn-sm btn-outline-secondary"
                    aria-label="Remove image"
                    onclick={() => {
                      initialData.update((data) => ({
                        ...data,
                        image_data: undefined,
                        image_url: undefined,
                      }));
                    }}
                  >
                    <i class="fa-solid fa-xmark"></i>
                  </button>

                  <img src={$initialData.image_url} alt="Flashcard" class="img-fluid" width="50%" />
                </div>
              {:else}
                <span>No Image Choosen</span>
              {/if}
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

<style>
  .thumbnail {
    position: relative;
    background-color: #f0f0f0;
    text-align: center;
    padding-top: 20px;
    padding-bottom: 20px;
  }

  .thumbnail img {
    max-width: 100%;
    height: auto;
  }

  .thumbnail button {
    position: absolute;
    top: 0;
    right: 0;
    z-index: 1;
  }
</style>
