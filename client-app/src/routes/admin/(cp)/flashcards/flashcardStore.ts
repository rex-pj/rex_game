import { get, writable, type Writable } from "svelte/store";
import { FlashcardService } from "$lib/services/flashcardService";
import { FlashcardTypeService } from "$lib/services/flashcardTypeService";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { Flashcard, FlashcardDetail, FlashcardRequest } from "$lib/models/flashcard";
import type { FlashcardType } from "$lib/models/flashcard-type";
import { getImageBase64Url, setImageBase64Url } from "$lib/helpers/imageHelper";

const flashcardService: FlashcardService = new FlashcardService(Cookies);
const flashcardTypeService: FlashcardTypeService = new FlashcardTypeService(Cookies);
export const flashcards: Writable<Flashcard[]> = writable([]);
export const currentPage = writable(1);
export const totalPages = writable(1);
export const pager = { currentPage: 1, totalPages: 0 } as Pager;
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const flashcardTypeSuggestions: Writable<FlashcardType[]> = writable([]);
export const edittingData: Writable<FlashcardRequest> = writable({
  id: 0,
  name: "",
  description: "",
  sub_description: "",
  image_data: undefined,
});

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

// Fetch flashcards data (mocked for now)
export const fetchFlashcards = async (page: number) => {
  // Replace this with your API call
  const response = await flashcardService.getList(fetch, page, itemsPerPage);

  const start = (page - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  const items = response.items;
  for (const item of items) {
    if (item.image_id) {
      item.image_url = await getImageBase64Url(item.image_id);
    }
  }
  flashcards.set(items.slice(start, end));
  pager.totalPages = Math.ceil(response.total_count / itemsPerPage);
};

export const submit = async (data: FlashcardRequest) => {
  if (data.id) {
    return await updateFlashcard(data.id, data);
  }
  return await createFlashcard(data);
};

export const createFlashcard = async (data: FlashcardRequest) => {
  isSubmitting.set(true);

  const formData = new FormData();
  formData.append("name", data.name);
  formData.append("description", data.description);
  formData.append("sub_description", data.sub_description);
  if (data.type_ids) {
    data.type_ids.forEach((item, index) => {
      formData.append(`type_ids[${index}]`, item.toString());
    });
  }

  if (data.image_data !== undefined && data.image_data !== null) {
    formData.append("image_data", data.image_data);
  }

  await flashcardService
    .create(fetch, formData)
    .then(async () => {
      await fetchFlashcards(1);
      toggleCreationModal(false);
    })
    .catch((error) => {
      creationError.set(error.message);
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const updateFlashcard = async (id: number, data: FlashcardRequest) => {
  isSubmitting.set(true);

  const formData = new FormData();
  formData.append("name", data.name);
  formData.append("description", data.description);
  formData.append("sub_description", data.sub_description);
  if (data.type_ids) {
    data.type_ids.forEach((item, index) => {
      formData.append(`type_ids[${index}]`, item.toString());
    });
  }

  if (data.image_data !== undefined && data.image_data !== null) {
    formData.append("image_data", data.image_data);
  }

  await flashcardService
    .update(fetch, id, formData)
    .then(async () => {
      await fetchFlashcards(pager.currentPage);
      toggleCreationModal(false);
    })
    .catch((error) => {
      creationError.set(error.message);
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const openDeletingModal = (id: number) => {
  getFlashcard(id).then((response) => {
    if (response) {
      deletingData.set({ id: response.id, name: response.name });
      showDeletionModal.set(true);
    }
  });
};

export const changePage = (page: number) => {
  if (page >= 1 && page <= get(totalPages)) {
    currentPage.set(page);
    fetchFlashcards(page);
  }
};

export const getFlashcard = async (id: number) => {
  isSubmitting.set(true);
  return await flashcardService
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

export const toggleCreationModal = async (isShown: boolean = false) => {
  showCreationModal.set(isShown);
  if (isShown) {
    await flashcardTypeService.getList(fetch).then((data) => {
      flashcardTypeSuggestions.set(data.items);
    });
  } else {
    flashcardTypeSuggestions.set([]);
  }
  edittingData.set({
    id: 0,
    name: "",
    description: "",
    sub_description: "",
    image_data: undefined,
  });
};

export const openEditingModal = async (id: number) => {
  await flashcardTypeService.getList(fetch).then((data) => {
    flashcardTypeSuggestions.set(data.items);
  });
  getFlashcard(id).then(async (response: FlashcardDetail) => {
    if (response) {
      const imageBase64Url = await getImageBase64Url(response.image_id);
      const data: FlashcardRequest = {
        id: response.id,
        name: response.name,
        description: response.description,
        sub_description: response.sub_description ?? null,
        original_image_url: imageBase64Url,
        types: response.flashcard_types
          ? response.flashcard_types.map((type: FlashcardType) => ({
              value: type.id,
              label: type.name,
            }))
          : [],
      };
      edittingData.set(data);
      showCreationModal.set(true);
    }
  });
};

export const toggleDeletionModal = (isShown: boolean = false) => {
  showDeletionModal.set(isShown);
  deletingData.set({ id: 0, name: "" });
};

export const deleteById = async (id: number) => {
  isDeletionSubmitting.set(true);
  await flashcardService
    .deleteById(fetch, id)
    .then(async () => {
      await fetchFlashcards(1);
      toggleDeletionModal(false);
    })
    .catch((error) => {
      deletionError.set(error.message);
    })
    .finally(() => {
      isDeletionSubmitting.set(false);
    });
};
