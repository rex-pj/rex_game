import { get, writable, type Writable } from "svelte/store";
import { GameTypeAdminApi } from "$lib/api/gameTypeAdminApi";
import { FlashcardApi } from "$lib/api/flashcardApi";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { GameType, GameTypeRequest } from "$lib/models/game-type";
import type { Flashcard } from "$lib/models/flashcard";
import * as accessService from "$lib/services/accessService";
import type { CurrentUser } from "$lib/models/current-user";
import { PermissionCodes } from "$lib/common/permissions";
import { AdminClientApiOptions } from "$lib/api/apiOptions";

const gameTypeService: GameTypeAdminApi = new GameTypeAdminApi(
  new AdminClientApiOptions(Cookies)
);
export const items: Writable<GameType[]> = writable([]);
export const pager: Writable<Pager> = writable({ currentPage: 1, totalPages: 0 });
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const edittingData: Writable<GameTypeRequest> = writable({
  id: 0,
  code: "",
  name: "",
  description: "",
  icon: "",
});

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

export const fetchItems = async (page: number) => {
  const response = await gameTypeService.getList(fetch, page, itemsPerPage);
  items.set(response.items);
  const totalPages = Math.ceil(response.total_count / itemsPerPage);
  pager.update((current) => ({
    ...current,
    totalPages: totalPages,
  }));
};

export const submit = async (data: GameTypeRequest) => {
  if (data.id) {
    return await update(data.id, data);
  }
  return await create(data);
};

export const create = async (data: GameTypeRequest) => {
  isSubmitting.set(true);
  await gameTypeService
    .create(fetch, { code: data.code, name: data.name, description: data.description, icon: data.icon })
    .then(async () => {
      await fetchItems(1);
      toggleCreationModal(false);
    })
    .catch((error) => {
      creationError.set(error.message);
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const update = async (id: number, data: GameTypeRequest) => {
  isSubmitting.set(true);
  await gameTypeService
    .update(fetch, id, { code: data.code, name: data.name, description: data.description, icon: data.icon })
    .then(async () => {
      await fetchItems(1);
      toggleCreationModal(false);
    })
    .catch((error) => {
      creationError.set(error.message);
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const getById = async (id: number) => {
  isSubmitting.set(true);
  return await gameTypeService
    .getById(fetch, id)
    .then((response) => {
      return response;
    })
    .catch((error) => {
      creationError.set(error.message);
      return null;
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const changePage = (page: number) => {
  if (page >= 1 && page <= get(pager).totalPages) {
    pager.update((current) => ({
      ...current,
      currentPage: page,
    }));
    fetchItems(page);
  }
};

export const toggleCreationModal = (isShown: boolean = false) => {
  showCreationModal.set(isShown);
  edittingData.set({ id: 0, code: "", name: "", description: "", icon: "" });
};

export const openEditingModal = (id: number) => {
  getById(id).then((response) => {
    if (response) {
      edittingData.set({
        id: response.id,
        code: response.code,
        name: response.name,
        description: response.description,
        icon: response.icon,
      });
      showCreationModal.set(true);
    }
  });
};

export const openDeletingModal = (id: number) => {
  getById(id).then((response) => {
    if (response) {
      deletingData.set({ id: response.id, name: response.name });
      showDeletionModal.set(true);
    }
  });
};

export const toggleDeletionModal = (isShown: boolean = false) => {
  showDeletionModal.set(isShown);
  deletingData.set({ id: 0, name: "" });
};

export const deleteById = async (id: number) => {
  isDeletionSubmitting.set(true);
  await gameTypeService
    .deleteById(fetch, id)
    .then(async () => {
      await fetchItems(1);
      toggleDeletionModal(false);
    })
    .catch((error) => {
      deletionError.set(error.message);
    })
    .finally(() => {
      isDeletionSubmitting.set(false);
    });
};

export const toggleActive = async (id: number) => {
  try {
    const newStatus = await gameTypeService.toggleActive(fetch, id);
    items.update((currentItems) =>
      currentItems.map((item) =>
        item.id === id ? { ...item, is_actived: newStatus } : item
      )
    );
  } catch (error) {
    console.error("Failed to toggle game type status:", error);
  }
};

export const canUpdate = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.GameTypeUpdate])
  );
};

export const canCreate = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.GameTypeCreate])
  );
};

export const canDelete = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.GameTypeDelete])
  );
};

// ---- Flashcard Management ----

const flashcardService: FlashcardApi = new FlashcardApi(
  new AdminClientApiOptions(Cookies)
);

export const showFlashcardModal = writable(false);
export const flashcardModalGameType = writable({ id: 0, name: "" });
export const assignedFlashcards: Writable<Flashcard[]> = writable([]);
export const allFlashcards: Writable<Flashcard[]> = writable([]);
export const flashcardError = writable("");
export const isFlashcardLoading = writable(false);

export const openFlashcardModal = async (id: number) => {
  const gameType = await getById(id);
  if (!gameType) return;

  flashcardModalGameType.set({ id: gameType.id, name: gameType.name });
  flashcardError.set("");
  showFlashcardModal.set(true);
  await loadFlashcardData(id);
};

export const loadFlashcardData = async (gameTypeId: number) => {
  isFlashcardLoading.set(true);
  try {
    const [assigned, all] = await Promise.all([
      gameTypeService.getFlashcards(fetch, gameTypeId),
      flashcardService.getList(fetch, 1, 1000),
    ]);
    assignedFlashcards.set(assigned);
    allFlashcards.set(all.items);
  } catch (error: any) {
    flashcardError.set(error.message);
  } finally {
    isFlashcardLoading.set(false);
  }
};

export const assignFlashcards = async (flashcardIds: number[]) => {
  const gameType = get(flashcardModalGameType);
  if (!gameType.id || flashcardIds.length === 0) return;

  isFlashcardLoading.set(true);
  try {
    await gameTypeService.assignFlashcards(fetch, gameType.id, flashcardIds);
    await loadFlashcardData(gameType.id);
  } catch (error: any) {
    flashcardError.set(error.message);
  } finally {
    isFlashcardLoading.set(false);
  }
};

export const removeFlashcard = async (flashcardId: number) => {
  const gameType = get(flashcardModalGameType);
  if (!gameType.id) return;

  isFlashcardLoading.set(true);
  try {
    await gameTypeService.removeFlashcard(fetch, gameType.id, flashcardId);
    await loadFlashcardData(gameType.id);
  } catch (error: any) {
    flashcardError.set(error.message);
  } finally {
    isFlashcardLoading.set(false);
  }
};

export const closeFlashcardModal = () => {
  showFlashcardModal.set(false);
  flashcardModalGameType.set({ id: 0, name: "" });
  assignedFlashcards.set([]);
  allFlashcards.set([]);
  flashcardError.set("");
};
