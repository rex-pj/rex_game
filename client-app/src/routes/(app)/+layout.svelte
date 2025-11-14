<script lang="ts">
  import { setContext } from "svelte";
  import type { LayoutProps } from "./$types";
  import { SHARED_CONTEXT } from "$lib/common/contants";
  import type { CurrentUser } from "$lib/models/current-user";
  import Footer from "../../components/organisms/Footer.svelte";
  import Navigation from "../../components/organisms/Navigation.svelte";

  const { children, data }: LayoutProps = $props();
  if (data?.currentUser) {
    setContext<CurrentUser>(SHARED_CONTEXT.CURRENT_USER, data.currentUser);
  }
</script>

<Navigation currentUser={data.currentUser}></Navigation>
{@render children()}
<Footer></Footer>
