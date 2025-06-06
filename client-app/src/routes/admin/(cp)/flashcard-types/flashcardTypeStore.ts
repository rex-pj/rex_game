import { writable, type Writable } from "svelte/store";
import { FlashcardTypeService } from "$lib/services/flashcardTypeService";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { FlashcardType, FlashcardTypeRequest } from "$lib/models/flashcard-type";

const flashcardTypeService: FlashcardTypeService = new FlashcardTypeService(Cookies);
export const flashcardTypes: Writable<FlashcardType[]> = writable([]);
export const pager = { currentPage: 1, totalPages: 0 } as Pager;
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const edittingData: Writable<FlashcardTypeRequest> = writable({
  id: 0,
  name: "",
  description: "",
});

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

// Fetch flashcards data (mocked for now)
export const fetchFlashcardTypes = async (page: number) => {
  // Replace this with your API call
  const response = await flashcardTypeService.getList(fetch, page, itemsPerPage);

  const start = (page - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  flashcardTypes.set(response.items.slice(start, end));
  pager.totalPages = Math.ceil(response.total_count / itemsPerPage);
};

export const submit = async (data: FlashcardTypeRequest) => {
  if (data.id) {
    return await updateFlashcardType(data.id, data);
  }
  return await createFlashcardType(data);
};

export const createFlashcardType = async (data: FlashcardTypeRequest) => {
  isSubmitting.set(true);
  await flashcardTypeService
    .create(fetch, data)
    .then(async () => {
      await fetchFlashcardTypes(1);
      toggleCreationModal(false);
    })
    .catch((error) => {
      creationError.set(error.message);
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const updateFlashcardType = async (id: number, data: FlashcardTypeRequest) => {
  isSubmitting.set(true);
  await flashcardTypeService
    .update(fetch, id, { name: data.name, description: data.description })
    .then(async () => {
      await fetchFlashcardTypes(1);
      toggleCreationModal(false);
    })
    .catch((error) => {
      creationError.set(error.message);
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const getFlashcardType = async (id: number) => {
  isSubmitting.set(true);
  return await flashcardTypeService
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
  if (page >= 1 && page <= pager.totalPages) {
    pager.currentPage = page;
    fetchFlashcardTypes(page);
  }
};

export const toggleCreationModal = (isShown: boolean = false) => {
  showCreationModal.set(isShown);
  edittingData.set({ id: 0, name: "", description: "" });
};

export const openEditingModal = (id: number) => {
  getFlashcardType(id).then((response) => {
    if (response) {
      edittingData.set(response);
      showCreationModal.set(true);
    }
  });
};

export const openDeletingModal = (id: number) => {
  getFlashcardType(id).then((response) => {
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
  await flashcardTypeService
    .deleteById(fetch, id)
    .then(async () => {
      await fetchFlashcardTypes(1);
      toggleDeletionModal(false);
    })
    .catch((error) => {
      deletionError.set(error.message);
    })
    .finally(() => {
      isDeletionSubmitting.set(false);
    });
};
