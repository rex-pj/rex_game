<script lang="ts">
  import type { RoleRequest } from "$lib/models/role";
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
    submit: (data: RoleRequest) => Promise<void>;
    creationError?: Writable<string>;
    initialData?: Writable<RoleRequest>;
  }

  let {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    creationError = writable(""),
    initialData = writable({ id: 0, name: "", description: "" }),
  }: Props = $props();

  // Derived values
  const modalTitle = $derived($initialData.id ? "Update Role" : "Create Role");
  const submitText = $derived($initialData.id ? "Update" : "Create");
  const loadingText = $derived($initialData.id ? "Updating..." : "Creating...");

  async function handleSubmit(event: Event) {
    event.preventDefault();
    creationError.set("");
    await submit($initialData);
  }
</script>

<Modal show={$showModal} title={modalTitle} onclose={closeModal}>
  <form id="role-form" onsubmit={handleSubmit}>
    {#if $creationError}
      <Alert variant="danger">{$creationError}</Alert>
    {/if}

    <input type="hidden" bind:value={$initialData.id} />

    <FormField
      id="role-name"
      label="Name"
      bind:value={$initialData.name}
      required
    />

    <TextArea
      id="role-description"
      label="Description"
      bind:value={$initialData.description}
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
