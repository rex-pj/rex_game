<script lang="ts">
  import { writable, type Writable } from "svelte/store";
  import ConfirmDialog from "../../../../components/molecules/confirm-dialog/ConfirmDialog.svelte";

  interface Props {
    showModal?: Writable<boolean>;
    isSubmitting?: Writable<boolean>;
    closeModal: () => void;
    submit: (id: number) => Promise<void>;
    deletionError?: Writable<string>;
    initialData?: Writable<{ id: number; name: string }>;
  }

  let {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    deletionError = writable(""),
    initialData = writable({ id: 0, name: "" }),
  }: Props = $props();

  async function handleConfirm() {
    deletionError.set("");
    await submit($initialData.id);
  }
</script>

<ConfirmDialog
  show={$showModal}
  title="Delete Achievement"
  itemName={$initialData.name}
  confirmText="Delete"
  variant="danger"
  loading={$isSubmitting}
  error={$deletionError}
  onconfirm={handleConfirm}
  oncancel={closeModal}
/>
