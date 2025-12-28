<script lang="ts">
  import type { MailTemplateRequest } from "$lib/models/mail-template";
  import { writable, type Writable } from "svelte/store";
  import Editor from "../../../../components/molecules/editor/editor.svelte";
  import Modal from "../../../../components/molecules/modal/Modal.svelte";
  import Button from "../../../../components/atoms/button/Button.svelte";
  import Alert from "../../../../components/atoms/alert/Alert.svelte";
  import FormField from "../../../../components/atoms/form/FormField.svelte";
  import TextArea from "../../../../components/atoms/form/TextArea.svelte";

  interface Props {
    showModal?: Writable<boolean>;
    isSubmitting?: Writable<boolean>;
    closeModal: () => void;
    submit: (data: MailTemplateRequest) => Promise<void>;
    creationError?: Writable<string>;
    initialData?: Writable<MailTemplateRequest>;
  }

  let {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    creationError = writable(""),
    initialData = writable({ id: 0, name: "", subject: "", body: "" }),
  }: Props = $props();

  const modalTitle = $derived($initialData.id ? "Update Mail Template" : "Create Mail Template");
  const submitText = $derived($initialData.id ? "Update" : "Create");
  const loadingText = $derived($initialData.id ? "Updating..." : "Creating...");

  async function handleSubmit(event: Event) {
    event.preventDefault();
    creationError.set("");
    await submit($initialData);
  }
</script>

<Modal show={$showModal} title={modalTitle} size="lg" onclose={closeModal}>
  <form id="mail-template-form" onsubmit={handleSubmit}>
    {#if $creationError}
      <Alert variant="danger">{$creationError}</Alert>
    {/if}

    <input type="hidden" bind:value={$initialData.id} />

    <FormField
      id="mail-template-name"
      label="Name"
      bind:value={$initialData.name}
      required
    />

    <TextArea
      id="mail-template-subject"
      label="Subject"
      bind:value={$initialData.subject}
      required
    />

    <div class="mb-3">
      <label class="form-label" for="mail-template-body">Body</label>
      <Editor bind:value={$initialData.body} />
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
      loadingText={loadingText}
      onclick={handleSubmit}
    >
      {submitText}
    </Button>
  {/snippet}
</Modal>
