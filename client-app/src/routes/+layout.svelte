<script>
  import { onMount } from "svelte";
  import "bootstrap/dist/css/bootstrap.min.css";
  import "../assets/css/base.css";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  onMount(async () => {
    await import("bootstrap");
  });
</script>

<slot />
