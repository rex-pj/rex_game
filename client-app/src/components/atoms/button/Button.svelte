<script lang="ts">
  import type { Snippet } from "svelte";

  type ButtonVariant = "primary" | "secondary" | "danger" | "warning" | "success" | "info"
    | "outline-primary" | "outline-secondary" | "outline-danger" | "outline-warning";
  type ButtonSize = "sm" | "md" | "lg";

  interface Props {
    type?: "button" | "submit" | "reset";
    variant?: ButtonVariant;
    size?: ButtonSize;
    disabled?: boolean;
    loading?: boolean;
    loadingText?: string;
    class?: string;
    onclick?: (e: MouseEvent) => void;
    children: Snippet;
  }

  let {
    type = "button",
    variant = "primary",
    size = "md",
    disabled = false,
    loading = false,
    loadingText = "",
    class: className = "",
    onclick,
    children,
  }: Props = $props();

  const sizeClasses: Record<ButtonSize, string> = {
    sm: "btn-sm",
    md: "",
    lg: "btn-lg",
  };
</script>

<button
  {type}
  class="btn btn-{variant} {sizeClasses[size]} {className}"
  disabled={disabled || loading}
  {onclick}
>
  {#if loading}
    <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
    {#if loadingText}
      {loadingText}
    {:else}
      {@render children()}
    {/if}
  {:else}
    {@render children()}
  {/if}
</button>
