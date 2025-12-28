<script lang="ts">
  import type { Snippet } from "svelte";

  type BadgeVariant = "primary" | "secondary" | "success" | "danger" | "warning" | "info" | "light" | "dark";

  interface Props {
    variant?: BadgeVariant;
    pill?: boolean;
    class?: string;
    children: Snippet;
  }

  let {
    variant = "primary",
    pill = false,
    class: className = "",
    children,
  }: Props = $props();
</script>

<span class="badge bg-{variant} {className}" class:rounded-pill={pill}>
  {@render children()}
</span>
