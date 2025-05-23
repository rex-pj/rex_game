<script>
  import { onMount } from "svelte";
  import {
    flashcards,
    currentPage,
    totalPages,
    fetchFlashcards,
    changePage,
  } from "./flashcardStore";

  onMount(() => {
    fetchFlashcards($currentPage);
  });
</script>

<div class="container mt-4">
  <h1 class="mb-4">Flashcard Manager</h1>

  <table class="table table-striped">
    <thead>
      <tr>
        <th>#</th>
        <th>Question</th>
        <th>Answer</th>
      </tr>
    </thead>
    <tbody>
      {#each $flashcards as flashcard}
        <tr>
          <td>{flashcard.id}</td>
          <td>{flashcard.question}</td>
          <td>{flashcard.answer}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <nav>
    <ul class="pagination">
      <li class="page-item {$currentPage === 1 ? 'disabled' : ''}">
        <button class="page-link" on:click={() => changePage($currentPage - 1)}>Previous</button>
      </li>
      {#each Array($totalPages) as _, i}
        <li class="page-item {$currentPage === i + 1 ? 'active' : ''}">
          <button class="page-link" on:click={() => changePage(i + 1)}>{i + 1}</button>
        </li>
      {/each}
      <li class="page-item {$currentPage === $totalPages ? 'disabled' : ''}">
        <button class="page-link" on:click={() => changePage($currentPage + 1)}>Next</button>
      </li>
    </ul>
  </nav>
</div>

<style>
  .table {
    margin-top: 20px;
  }
  .pagination {
    justify-content: center;
    margin-top: 20px;
  }
</style>
