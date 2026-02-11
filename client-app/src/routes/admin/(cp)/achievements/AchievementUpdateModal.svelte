<script lang="ts">
  import type { AchievementRequest } from "$lib/models/achievement";
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
    submit: (data: AchievementRequest) => Promise<void>;
    creationError?: Writable<string>;
    initialData?: Writable<AchievementRequest>;
  }

  let {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    creationError = writable(""),
    initialData = writable({ id: 0, code: "", name: "", description: "", icon: "", points: 0, category: "" }),
  }: Props = $props();

  const modalTitle = $derived($initialData.id ? "Update Achievement" : "Create Achievement");
  const submitText = $derived($initialData.id ? "Update" : "Create");
  const loadingText = $derived($initialData.id ? "Updating..." : "Creating...");

  async function handleSubmit(event: Event) {
    event.preventDefault();
    creationError.set("");
    await submit($initialData);
  }
</script>

<Modal show={$showModal} title={modalTitle} onclose={closeModal}>
  <form id="achievement-form" onsubmit={handleSubmit}>
    {#if $creationError}
      <Alert variant="danger">{$creationError}</Alert>
    {/if}

    <input type="hidden" bind:value={$initialData.id} />

    <FormField
      id="achievement-code"
      label="Code"
      bind:value={$initialData.code}
      required
    />

    <FormField
      id="achievement-name"
      label="Name"
      bind:value={$initialData.name}
      required
    />

    <TextArea
      id="achievement-description"
      label="Description"
      bind:value={$initialData.description}
    />

    <FormField
      id="achievement-icon"
      label="Icon"
      bind:value={$initialData.icon}
    />

    <FormField
      id="achievement-points"
      label="Points"
      type="number"
      bind:value={$initialData.points}
      required
    />

    <FormField
      id="achievement-category"
      label="Category"
      bind:value={$initialData.category}
    />
  </form>

  {#snippet footer()}
    <Button variant="secondary" onclick={closeModal} disabled={$isSubmitting}>
      Cancel
    </Button>
    <Button
      type="submit"
      variant="primary"
      loading={$isSubmitting}
      loadingText={loadingText}
      onclick={handleSubmit}
    >
      {submitText}
    </Button>
  {/snippet}
</Modal>
