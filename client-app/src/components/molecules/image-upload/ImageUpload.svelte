<script lang="ts">
  import { setImageBase64Url } from "$lib/helpers/imageHelper";

  interface Props {
    id: string;
    label?: string;
    imageUrl?: string | undefined;
    imageFile?: File | undefined;
    originalImageUrl?: string;
    accept?: string;
    required?: boolean;
    disabled?: boolean;
    error?: string;
    class?: string;
  }

  let {
    id,
    label = "Image",
    imageUrl = $bindable<string | undefined>(undefined),
    imageFile = $bindable<File | undefined>(undefined),
    originalImageUrl = "",
    accept = "image/*",
    required = false,
    disabled = false,
    error = "",
    class: className = "",
  }: Props = $props();

  async function handleImageChange(e: Event) {
    const target = e.target as HTMLInputElement;
    const file = target.files?.[0];

    if (!file) return;

    const base64Url = await setImageBase64Url(file);
    if (base64Url) {
      imageUrl = base64Url;
      imageFile = file;
    }
  }

  function removeImage() {
    imageUrl = undefined;
    imageFile = undefined;
  }
</script>

<div class="mb-3 {className}">
  <label class="form-label" for={id}>
    {label}
    {#if required}
      <span class="text-danger">*</span>
    {/if}
  </label>

  <input
    {id}
    type="file"
    class="form-control"
    class:is-invalid={!!error}
    {accept}
    {disabled}
    onchange={handleImageChange}
  />

  {#if error}
    <div class="invalid-feedback">{error}</div>
  {/if}

  {#if !imageUrl && originalImageUrl}
    <div class="mt-2">
      <p class="mb-1 text-muted small">Current Image:</p>
      <div class="thumbnail">
        <img src={originalImageUrl} alt="Current" class="img-thumbnail" />
      </div>
    </div>
  {:else if imageUrl}
    <div class="mt-2">
      <p class="mb-1 text-muted small">New Image:</p>
      <div class="thumbnail">
        <button
          type="button"
          class="btn btn-sm btn-outline-danger remove-btn"
          aria-label="Remove image"
          onclick={removeImage}
        >
          <i class="fa-solid fa-xmark"></i>
        </button>
        <img src={imageUrl} alt="Preview" class="img-thumbnail" />
      </div>
    </div>
  {:else}
    <p class="text-muted small mt-2">No image selected</p>
  {/if}
</div>

<style>
  .thumbnail {
    position: relative;
    display: inline-block;
    background-color: #f8f9fa;
    padding: 0.5rem;
    border-radius: 0.375rem;
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
    padding: 0.25rem 0.5rem;
  }
</style>
