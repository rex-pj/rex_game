<script lang="ts">
  import { writable, type Writable } from "svelte/store";
  import type { Flashcard } from "$lib/models/flashcard";
  import Modal from "../../../../components/molecules/modal/Modal.svelte";
  import Button from "../../../../components/atoms/button/Button.svelte";
  import Alert from "../../../../components/atoms/alert/Alert.svelte";

  interface Props {
    showModal?: Writable<boolean>;
    gameType?: Writable<{ id: number; name: string }>;
    assignedFlashcards?: Writable<Flashcard[]>;
    allFlashcards?: Writable<Flashcard[]>;
    error?: Writable<string>;
    loading?: Writable<boolean>;
    onAssign: (flashcardIds: number[]) => Promise<void>;
    onRemove: (flashcardId: number) => Promise<void>;
    onClose: () => void;
  }

  let {
    showModal = writable(false),
    gameType = writable({ id: 0, name: "" }),
    assignedFlashcards = writable([]),
    allFlashcards = writable([]),
    error = writable(""),
    loading = writable(false),
    onAssign,
    onRemove,
    onClose,
  }: Props = $props();

  let selectedIds: number[] = $state([]);
  let searchTerm: string = $state("");

  const assignedIds = $derived(new Set($assignedFlashcards.map((f) => f.id)));

  const availableFlashcards = $derived(
    $allFlashcards
      .filter((f) => !assignedIds.has(f.id))
      .filter(
        (f) =>
          !searchTerm ||
          f.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
          f.description.toLowerCase().includes(searchTerm.toLowerCase())
      )
  );

  function toggleSelect(id: number) {
    if (selectedIds.includes(id)) {
      selectedIds = selectedIds.filter((i) => i !== id);
    } else {
      selectedIds = [...selectedIds, id];
    }
  }

  async function handleAssign() {
    if (selectedIds.length === 0) return;
    await onAssign(selectedIds);
    selectedIds = [];
  }
</script>

<Modal show={$showModal} title="Manage Flashcards - {$gameType.name}" size="lg" onclose={onClose}>
  {#if $error}
    <Alert variant="danger">{$error}</Alert>
  {/if}

  <div class="mb-4">
    <h6>Assigned Flashcards ({$assignedFlashcards.length})</h6>
    {#if $loading}
      <div class="text-center py-3">
        <div class="spinner-border spinner-border-sm" role="status"></div>
        <span class="ms-2">Loading...</span>
      </div>
    {:else if $assignedFlashcards.length === 0}
      <p class="text-muted">No flashcards assigned yet.</p>
    {:else}
      <div class="table-responsive" style="max-height: 200px; overflow-y: auto;">
        <table class="table table-sm table-striped mb-0">
          <thead>
            <tr>
              <th>#</th>
              <th>Name</th>
              <th>Description</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {#each $assignedFlashcards as flashcard}
              <tr>
                <td>{flashcard.id}</td>
                <td>{flashcard.name}</td>
                <td class="text-truncate" style="max-width: 200px;">{flashcard.description}</td>
                <td>
                  <button
                    class="btn btn-outline-danger btn-sm"
                    onclick={() => onRemove(flashcard.id)}
                    disabled={$loading}
                  >
                    <i class="fa-solid fa-xmark"></i>
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>

  <hr />

  <div>
    <h6>Add Flashcards</h6>
    <div class="mb-2">
      <input
        type="text"
        class="form-control form-control-sm"
        placeholder="Search flashcards..."
        bind:value={searchTerm}
      />
    </div>
    {#if availableFlashcards.length === 0}
      <p class="text-muted">No available flashcards to add.</p>
    {:else}
      <div class="table-responsive" style="max-height: 250px; overflow-y: auto;">
        <table class="table table-sm table-hover mb-0">
          <thead>
            <tr>
              <th style="width: 40px;">
                <input
                  type="checkbox"
                  class="form-check-input"
                  checked={selectedIds.length > 0 && selectedIds.length === availableFlashcards.length}
                  onchange={() => {
                    if (selectedIds.length === availableFlashcards.length) {
                      selectedIds = [];
                    } else {
                      selectedIds = availableFlashcards.map((f) => f.id);
                    }
                  }}
                />
              </th>
              <th>#</th>
              <th>Name</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            {#each availableFlashcards as flashcard}
              <tr
                style="cursor: pointer;"
                onclick={() => toggleSelect(flashcard.id)}
              >
                <td>
                  <input
                    type="checkbox"
                    class="form-check-input"
                    checked={selectedIds.includes(flashcard.id)}
                    onchange={() => toggleSelect(flashcard.id)}
                  />
                </td>
                <td>{flashcard.id}</td>
                <td>{flashcard.name}</td>
                <td class="text-truncate" style="max-width: 200px;">{flashcard.description}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" onclick={onClose} disabled={$loading}>
      Close
    </Button>
    {#if selectedIds.length > 0}
      <Button
        variant="primary"
        loading={$loading}
        loadingText="Assigning..."
        onclick={handleAssign}
      >
        Assign ({selectedIds.length})
      </Button>
    {/if}
  {/snippet}
</Modal>
