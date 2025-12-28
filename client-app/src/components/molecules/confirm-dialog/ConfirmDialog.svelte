<script lang="ts">
  import Modal from "../modal/Modal.svelte";
  import Button from "../../atoms/button/Button.svelte";
  import Alert from "../../atoms/alert/Alert.svelte";

  type ConfirmVariant = "danger" | "warning" | "primary";

  interface Props {
    show: boolean;
    title?: string;
    message?: string;
    itemName?: string;
    confirmText?: string;
    cancelText?: string;
    variant?: ConfirmVariant;
    loading?: boolean;
    error?: string;
    onconfirm: () => void;
    oncancel: () => void;
  }

  let {
    show = false,
    title = "Confirm",
    message = "",
    itemName = "",
    confirmText = "Confirm",
    cancelText = "Cancel",
    variant = "danger",
    loading = false,
    error = "",
    onconfirm,
    oncancel,
  }: Props = $props();

  const defaultMessages: Record<ConfirmVariant, string> = {
    danger: "Are you sure you want to delete",
    warning: "Are you sure you want to proceed with",
    primary: "Do you want to confirm",
  };

  const displayMessage = $derived(message || `${defaultMessages[variant]} "${itemName}"?`);
</script>

<Modal {show} {title} onclose={oncancel}>
  {#if error}
    <Alert variant="danger">{error}</Alert>
  {/if}

  <p>{displayMessage}</p>

  {#snippet footer()}
    <Button variant="secondary" onclick={oncancel} disabled={loading}>
      {cancelText}
    </Button>
    <Button {variant} onclick={onconfirm} {loading} loadingText="Processing...">
      {confirmText}
    </Button>
  {/snippet}
</Modal>
