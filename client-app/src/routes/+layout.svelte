<script>
  import { onMount } from "svelte";
  import "bootstrap/dist/css/bootstrap.min.css";
  import "../assets/css/base.css";
  onMount(async () => {
    await import("bootstrap");
  });
  import Footer from "../components/organisms/Footer.svelte";
</script>

<slot />
<Footer></Footer>
