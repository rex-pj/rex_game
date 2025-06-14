<script lang="ts">
  import { onMount } from "svelte";
  import "bootstrap/dist/css/bootstrap.min.css";
  import "../assets/css/base.css";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import { setContext } from "svelte";
  import type { LayoutProps } from "./$types";
  import { SHARED_CONTEXT } from "$lib/common/contants";
  import type { CurrentUser } from "$lib/models/current-user";

  const { children, data }: LayoutProps = $props();
  if (data?.currentUser) {
    setContext<CurrentUser>(SHARED_CONTEXT.CURRENT_USER, data.currentUser);
  }

  onMount(async () => {
    await import("bootstrap");
  });
</script>

{@render children()}
