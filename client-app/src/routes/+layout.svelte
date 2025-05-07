<script>
  import { onMount } from "svelte";
  import "bootstrap/dist/css/bootstrap.min.css";
  import "../assets/css/base.css";
  onMount(async () => {
    await import("bootstrap");
  });
</script>

<slot />
