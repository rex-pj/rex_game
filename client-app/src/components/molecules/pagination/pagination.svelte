<script lang="ts">
  import type { Pager } from "./pager";

  let {
    pager,
    changePage,
  }: {
    pager: Pager;
    changePage: (page: number) => void;
  } = $props();
</script>

<nav>
  <ul class="pagination">
    <li class="page-item {pager.currentPage === 1 ? 'disabled' : ''}">
      <button class="page-link" onclick={() => changePage(pager.currentPage - 1)}>Previous</button>
    </li>
    {#each Array(pager.totalPages) as _, i}
      <li class="page-item {pager.currentPage === i + 1 ? 'active' : ''}">
        <button class="page-link" onclick={() => changePage(i + 1)}>{i + 1}</button>
      </li>
    {/each}
    <li class="page-item {pager.currentPage === pager.totalPages ? 'disabled' : ''}">
      <button class="page-link" onclick={() => changePage(pager.currentPage + 1)}>Next</button>
    </li>
  </ul>
</nav>

<style>
  .pagination {
    justify-content: center;
    margin-top: 20px;
  }
</style>
