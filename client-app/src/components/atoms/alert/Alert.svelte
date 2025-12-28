<script lang="ts">
  import type { Snippet } from "svelte";

  type AlertVariant =
    | "success"
    | "warning"
    | "danger"
    | "info"
    | "primary"
    | "secondary";

  interface Props {
    variant?: AlertVariant;
    dismissible?: boolean;
    show?: boolean;
    class?: string;
    children: Snippet;
  }

  let {
    variant = "info",
    dismissible = false,
    show = $bindable(true),
    class: className = "",
    children,
  }: Props = $props();
</script>

{#if show}
  <div
    class="alert alert-{variant} {className}"
    class:alert-dismissible={dismissible}
    role="alert"
  >
    {@render children()}
    {#if dismissible}
      <button
        type="button"
        class="btn-close"
        aria-label="Close"
        onclick={() => (show = false)}
      ></button>
    {/if}
  </div>
{/if}
