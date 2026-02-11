<script lang="ts">
  import { writable, type Writable } from "svelte/store";
  import ConfirmDialog from "../../../../components/molecules/confirm-dialog/ConfirmDialog.svelte";

  interface Props {
    showModal?: Writable<boolean>;
    isSubmitting?: Writable<boolean>;
    closeModal: () => void;
    submit: (userId: number) => Promise<void>;
    error?: Writable<string>;
    initialData?: Writable<{ userId: number; name: string }>;
  }

  let {
    showModal = writable(false),
    isSubmitting = writable(false),
    closeModal,
    submit,
    error = writable(""),
    initialData = writable({ userId: 0, name: "" }),
  }: Props = $props();

  async function handleConfirm() {
    error.set("");
    await submit($initialData.userId);
  }
</script>

<ConfirmDialog
  show={$showModal}
  title="Reset User Stats"
  itemName={$initialData.name}
  confirmText="Reset"
  variant="warning"
  loading={$isSubmitting}
  error={$error}
  onconfirm={handleConfirm}
  oncancel={closeModal}
/>
