<script lang="ts">
  import { onMount } from "svelte";
  import "bootstrap/dist/css/bootstrap.min.css";
  import "../assets/css/base.css";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import type { LayoutProps } from "./$types";

  const { children }: LayoutProps = $props();
  onMount(async () => {
    await import("bootstrap");
  });
</script>

{@render children()}
