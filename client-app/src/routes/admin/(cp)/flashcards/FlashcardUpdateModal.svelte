<script lang="ts">
  import { setImageBase64Url } from "$lib/helpers/imageHelper";
  import type { FlashcardRequest } from "$lib/models/flashcard";
  import type { SelectOption } from "$lib/models/select-option";
  import Select from "svelte-select";
  import { writable, type Writable } from "svelte/store";
  import Modal from "../../../../components/molecules/modal/Modal.svelte";
  import Button from "../../../../components/atoms/button/Button.svelte";
  import Alert from "../../../../components/atoms/alert/Alert.svelte";
  import FormField from "../../../../components/atoms/form/FormField.svelte";
  import TextArea from "../../../../components/atoms/form/TextArea.svelte";

  interface Props {
    showModal?: Writable<boolean>;
    isSubmitting?: Writable<boolean>;
    closeModal: () => void;
    submit: (data: FlashcardRequest) => Promise<void>;
    creationError?: Writable<string>;
    initialData?: Writable<FlashcardRequest>;
    flashcardTypeOptions?: SelectOption[];
    gameTypeOptions?: SelectOption[];
  }

  let {
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
    gameTypeOptions = [],
  }: Props = $props();

  const modalTitle = $derived(
    $initialData.id ? "Update Flashcard" : "Create Flashcard",
  );
  const submitText = $derived($initialData.id ? "Update" : "Create");
  const loadingText = $derived($initialData.id ? "Updating..." : "Creating...");

  async function handleSubmit(event: Event) {
    event.preventDefault();
    creationError.set("");
    await submit($initialData);
  }

  function handleTypeChange(e: CustomEvent) {
    const selected = e.detail;
    initialData.update((d) => ({
      ...d,
      types: selected,
      type_ids: selected ? selected.map((i: any) => i.value) : [],
    }));
  }

  function handleGameTypeChange(e: CustomEvent) {
    const selected = e.detail;
    initialData.update((d) => ({
      ...d,
      game_types: selected,
      game_type_ids: selected ? selected.map((i: any) => i.value) : [],
    }));
  }

  async function onImageChange(e: Event) {
    const target = e.target as HTMLInputElement;
    const fileData = target.files?.[0];
    if (!fileData) return;

    const imageUrl = await setImageBase64Url(fileData);
    if (!imageUrl) return;

    initialData.update((data) => ({
      ...data,
      image_data: fileData,
      image_url: imageUrl,
    }));
  }

  function removeImage() {
    initialData.update((data) => ({
      ...data,
      image_data: undefined,
      image_url: undefined,
    }));
  }
</script>

<Modal show={$showModal} title={modalTitle} onclose={closeModal}>
  <form id="flashcard-form" onsubmit={handleSubmit}>
    {#if $creationError}
      <Alert variant="danger">{$creationError}</Alert>
    {/if}

    <input type="hidden" bind:value={$initialData.id} />

    <FormField
      id="flashcard-name"
      label="Name"
      bind:value={$initialData.name}
      required
    />

    <TextArea
      id="flashcard-description"
      label="Description"
      bind:value={$initialData.description}
    />

    <TextArea
      id="flashcard-sub-description"
      label="Sub Description"
      bind:value={$initialData.sub_description}
    />

    <div class="mb-3">
      <label class="form-label" for="type_ids">Types</label>
      <Select
        id="type_ids"
        items={flashcardTypeOptions}
        multiple={true}
        value={$initialData.types}
        on:change={handleTypeChange}
        placeholder="Select types"
      />
    </div>

    <div class="mb-3">
      <label class="form-label" for="game_type_ids">Game Types</label>
      <Select
        id="game_type_ids"
        items={gameTypeOptions}
        multiple={true}
        value={$initialData.game_types}
        on:change={handleGameTypeChange}
        placeholder="Select game types"
      />
    </div>

    <div class="mb-3">
      <label class="form-label" for="image_data">Image</label>
      <input
        accept="image/*"
        type="file"
        id="image_data"
        class="form-control"
        onchange={onImageChange}
      />

      {#if !$initialData.image_url && $initialData.original_image_url}
        <p class="mt-2 mb-1 text-muted small">Current Image:</p>
        <div class="thumbnail">
          <img
            src={$initialData.original_image_url}
            alt="Flashcard"
            class="img-fluid"
          />
        </div>
      {:else if $initialData.image_url}
        <p class="mt-2 mb-1 text-muted small">New Image:</p>
        <div class="thumbnail">
          <button
            type="button"
            aria-label="Remove Image"
            class="btn btn-sm btn-outline-danger remove-btn"
            onclick={removeImage}
          >
            <i class="fa-solid fa-xmark"></i>
          </button>
          <img src={$initialData.image_url} alt="Flashcard" class="img-fluid" />
        </div>
      {:else}
        <p class="text-muted small mt-2">No image selected</p>
      {/if}
    </div>
  </form>

  {#snippet footer()}
    <Button variant="secondary" onclick={closeModal} disabled={$isSubmitting}>
      Cancel
    </Button>
    <Button
      type="submit"
      variant="primary"
      loading={$isSubmitting}
      {loadingText}
      onclick={handleSubmit}
    >
      {submitText}
    </Button>
  {/snippet}
</Modal>

<style>
  .thumbnail {
    position: relative;
    display: inline-block;
    background-color: #f8f9fa;
    padding: 0.5rem;
    border-radius: 0.375rem;
    text-align: center;
  }
  .thumbnail img {
    max-width: 200px;
    max-height: 200px;
    object-fit: contain;
  }
  .remove-btn {
    position: absolute;
    top: 0.25rem;
    right: 0.25rem;
    z-index: 1;
  }
</style>
