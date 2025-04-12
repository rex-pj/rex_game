<script>
  let { title, className = "primary-heading" } = $props();
</script>

<h1 class={className}>{title}</h1>

<style>
  h1.primary-heading {
    font-size: 1.8rem;
  }
</style>
