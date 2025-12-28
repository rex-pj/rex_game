<script lang="ts">
  type SpinnerSize = "sm" | "md" | "lg";
  type SpinnerVariant = "primary" | "secondary" | "success" | "danger" | "warning" | "info" | "light" | "dark";

  interface Props {
    size?: SpinnerSize;
    variant?: SpinnerVariant;
    text?: string;
    class?: string;
  }

  let {
    size = "md",
    variant = "primary",
    text = "",
    class: className = "",
  }: Props = $props();

  const sizeClasses: Record<SpinnerSize, string> = {
    sm: "spinner-border-sm",
    md: "",
    lg: "spinner-lg",
  };
</script>

<div class="d-flex align-items-center gap-2 {className}">
  <div
    class="spinner-border text-{variant} {sizeClasses[size]}"
    role="status"
  >
    <span class="visually-hidden">Loading...</span>
  </div>
  {#if text}
    <span class="text-{variant}">{text}</span>
  {/if}
</div>

<style>
  .spinner-lg {
    width: 3rem;
    height: 3rem;
  }
</style>
