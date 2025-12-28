<script lang="ts">
  import type { Snippet } from "svelte";

  type ModalSize = "sm" | "md" | "lg" | "xl";

  interface Props {
    show: boolean;
    title?: string;
    size?: ModalSize;
    closeOnBackdrop?: boolean;
    onclose: () => void;
    header?: Snippet;
    children: Snippet;
    footer?: Snippet;
  }

  let {
    show = false,
    title = "",
    size = "md",
    closeOnBackdrop = true,
    onclose,
    header,
    children,
    footer,
  }: Props = $props();

  const sizeClasses: Record<ModalSize, string> = {
    sm: "modal-sm",
    md: "",
    lg: "modal-lg",
    xl: "modal-xl",
  };

  function handleBackdropClick(e: MouseEvent) {
    if (closeOnBackdrop && e.target === e.currentTarget) {
      onclose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="modal fade show d-block"
    tabindex="-1"
    role="dialog"
    aria-modal="true"
    onclick={handleBackdropClick}
  >
    <div class="modal-dialog {sizeClasses[size]}" role="document">
      <div class="modal-content">
        <div class="modal-header">
          {#if header}
            {@render header()}
          {:else}
            <h5 class="modal-title">{title}</h5>
          {/if}
          <button
            type="button"
            class="btn-close"
            aria-label="Close"
            onclick={onclose}
          ></button>
        </div>
        <div class="modal-body">
          {@render children()}
        </div>
        {#if footer}
          <div class="modal-footer">
            {@render footer()}
          </div>
        {/if}
      </div>
    </div>
  </div>
  <div class="modal-backdrop fade show"></div>
{/if}

<style>
  .modal {
    background: rgba(0, 0, 0, 0.5);
  }
</style>
