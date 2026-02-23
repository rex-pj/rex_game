<script lang="ts">
  import { setContext } from "svelte";
  import type { LayoutProps } from "./$types";
  import { SHARED_CONTEXT } from "$lib/common/contants";
  import type { CurrentUser } from "$lib/models/current-user";

  const { children, data }: LayoutProps = $props();
  if (data?.adminUser) {
    setContext<CurrentUser>(SHARED_CONTEXT.CURRENT_USER, data.adminUser);
  }
</script>

<svelte:head>
  <meta name="robots" content="noindex, nofollow" />
</svelte:head>

{@render children()}
