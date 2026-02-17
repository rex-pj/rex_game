import { get, writable, type Writable } from "svelte/store";
import { FlashcardApi } from "$lib/api/flashcardApi";
import { FlashcardTypeApi } from "$lib/api/flashcardTypeApi";
import { GameTypeAdminApi } from "$lib/api/gameTypeAdminApi";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { Flashcard, FlashcardDetail, FlashcardRequest } from "$lib/models/flashcard";
import type { FlashcardType } from "$lib/models/flashcard-type";
import type { GameType } from "$lib/models/game-type";
import { getImageBase64Url } from "$lib/helpers/imageHelper";
import * as accessService from "$lib/services/accessService";
import type { CurrentUser } from "$lib/models/current-user";
import { PermissionCodes } from "$lib/common/permissions";
import { AdminClientApiOptions } from "$lib/api/apiOptions";

const flashcardService: FlashcardApi = new FlashcardApi(new AdminClientApiOptions(Cookies));
const flashcardTypeService: FlashcardTypeApi = new FlashcardTypeApi(
  new AdminClientApiOptions(Cookies)
);
const gameTypeService: GameTypeAdminApi = new GameTypeAdminApi(
  new AdminClientApiOptions(Cookies)
);
export const items: Writable<Flashcard[]> = writable([]);
export const pager: Writable<Pager> = writable({ currentPage: 1, totalPages: 0 });
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const flashcardTypeSuggestions: Writable<FlashcardType[]> = writable([]);
export const gameTypeSuggestions: Writable<GameType[]> = writable([]);
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
export const fetchItems = async (page: number) => {
  // Replace this with your API call
  const response = await flashcardService.getList(fetch, page, itemsPerPage);

  for (const item of response.items) {
    if (item.image_id) {
      item.image_url = await getImageBase64Url(item.image_id);
    }
  }
  items.set(response.items);
  const totalPages = Math.ceil(response.total_count / itemsPerPage);
  pager.update((current) => ({
    ...current,
    totalPages: totalPages,
  }));
};

export const submit = async (data: FlashcardRequest) => {
  if (data.id) {
    return await update(data.id, data);
  }
  return await create(data);
};

export const create = async (data: FlashcardRequest) => {
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
  if (data.game_type_ids) {
    data.game_type_ids.forEach((item, index) => {
      formData.append(`game_type_ids[${index}]`, item.toString());
    });
  }

  if (data.image_data !== undefined && data.image_data !== null) {
    formData.append("image_data", data.image_data);
  }

  await flashcardService
    .create(fetch, formData)
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

export const update = async (id: number, data: FlashcardRequest) => {
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
  if (data.game_type_ids) {
    data.game_type_ids.forEach((item, index) => {
      formData.append(`game_type_ids[${index}]`, item.toString());
    });
  }

  if (data.image_data !== undefined && data.image_data !== null) {
    formData.append("image_data", data.image_data);
  }

  await flashcardService
    .update(fetch, id, formData)
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

export const openDeletingModal = (id: number) => {
  getById(id).then((response) => {
    if (response) {
      deletingData.set({ id: response.id, name: response.name });
      showDeletionModal.set(true);
    }
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

export const getById = async (id: number) => {
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
      const [typeData, gameTypeData] = await Promise.all([
        flashcardTypeService.getList(fetch),
        gameTypeService.getList(fetch, 1, 100),
      ]);
      flashcardTypeSuggestions.set(typeData.items);
      gameTypeSuggestions.set(gameTypeData.items);
  } else {
    flashcardTypeSuggestions.set([]);
    gameTypeSuggestions.set([]);
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
  const [typeData, gameTypeData] = await Promise.all([
    flashcardTypeService.getList(fetch),
    gameTypeService.getList(fetch, 1, 100),
  ]);
  flashcardTypeSuggestions.set(typeData.items);
  gameTypeSuggestions.set(gameTypeData.items);

  getById(id).then(async (response: FlashcardDetail) => {
    if (response) {
      const imageBase64Url = await getImageBase64Url(response.image_id);
      const data: FlashcardRequest = {
        id: response.id,
        name: response.name,
        description: response.description,
        sub_description: response.sub_description ?? null,
        original_image_url: imageBase64Url,
        type_ids: response.flashcard_types
          ? response.flashcard_types.map((type: FlashcardType) => type.id)
          : [],
        types: response.flashcard_types
          ? response.flashcard_types.map((type: FlashcardType) => ({
              value: type.id,
              label: type.name,
            }))
          : [],
        game_type_ids: response.game_types
          ? response.game_types.map((gt) => gt.id)
          : [],
        game_types: response.game_types
          ? response.game_types.map((gt) => ({
              value: gt.id,
              label: gt.name,
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

export const canUpdate = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.FlashcardUpdate])
  );
};

export const canCreate = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.FlashcardCreate])
  );
};

export const canDelete = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.FlashcardDelete])
  );
};

export const toggleActive = async (id: number) => {
  try {
    const newStatus = await flashcardService.toggleActive(fetch, id);
    items.update((currentItems) =>
      currentItems.map((item) =>
        item.id === id ? { ...item, is_actived: newStatus } : item
      )
    );
  } catch (error) {
    console.error("Failed to toggle flashcard status:", error);
  }
};
