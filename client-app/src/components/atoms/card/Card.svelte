<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string;
    subtitle?: string;
    class?: string;
    headerClass?: string;
    bodyClass?: string;
    header?: Snippet;
    children: Snippet;
    footer?: Snippet;
  }

  let {
    title = "",
    subtitle = "",
    class: className = "",
    headerClass = "",
    bodyClass = "",
    header,
    children,
    footer,
  }: Props = $props();
</script>

<div class="card {className}">
  {#if header || title}
    <div class="card-header {headerClass}">
      {#if header}
        {@render header()}
      {:else}
        <h5 class="card-title mb-0">{title}</h5>
        {#if subtitle}
          <p class="card-subtitle text-muted mb-0">{subtitle}</p>
        {/if}
      {/if}
    </div>
  {/if}

  <div class="card-body {bodyClass}">
    {@render children()}
  </div>

  {#if footer}
    <div class="card-footer">
      {@render footer()}
    </div>
  {/if}
</div>
