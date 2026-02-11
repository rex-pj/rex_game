import { get, writable, type Writable } from "svelte/store";
import { AchievementAdminApi } from "$lib/api/achievementAdminApi";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { Achievement, AchievementRequest } from "$lib/models/achievement";
import * as accessService from "$lib/services/accessService";
import type { CurrentUser } from "$lib/models/current-user";
import { PermissionCodes } from "$lib/common/permissions";
import { AdminClientApiOptions } from "$lib/api/apiOptions";

const achievementService: AchievementAdminApi = new AchievementAdminApi(
  new AdminClientApiOptions(Cookies)
);
export const items: Writable<Achievement[]> = writable([]);
export const pager: Writable<Pager> = writable({ currentPage: 1, totalPages: 0 });
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const edittingData: Writable<AchievementRequest> = writable({
  id: 0,
  code: "",
  name: "",
  description: "",
  icon: "",
  points: 0,
  category: "",
});

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

export const fetchItems = async (page: number) => {
  const response = await achievementService.getList(fetch, page, itemsPerPage);
  items.set(response.items);
  const totalPages = Math.ceil(response.total_count / itemsPerPage);
  pager.update((current) => ({
    ...current,
    totalPages: totalPages,
  }));
};

export const submit = async (data: AchievementRequest) => {
  if (data.id) {
    return await update(data.id, data);
  }
  return await create(data);
};

export const create = async (data: AchievementRequest) => {
  isSubmitting.set(true);
  await achievementService
    .create(fetch, {
      code: data.code,
      name: data.name,
      description: data.description,
      icon: data.icon,
      points: data.points,
      category: data.category,
    })
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

export const update = async (id: number, data: AchievementRequest) => {
  isSubmitting.set(true);
  await achievementService
    .update(fetch, id, {
      code: data.code,
      name: data.name,
      description: data.description,
      icon: data.icon,
      points: data.points,
      category: data.category,
    })
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
  return await achievementService
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
  edittingData.set({ id: 0, code: "", name: "", description: "", icon: "", points: 0, category: "" });
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
        points: response.points,
        category: response.category,
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
  await achievementService
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
    const newStatus = await achievementService.toggleActive(fetch, id);
    items.update((currentItems) =>
      currentItems.map((item) =>
        item.id === id ? { ...item, is_actived: newStatus } : item
      )
    );
  } catch (error) {
    console.error("Failed to toggle achievement status:", error);
  }
};

export const canUpdate = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.AchievementUpdate])
  );
};

export const canCreate = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.AchievementCreate])
  );
};

export const canDelete = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.AchievementDelete])
  );
};
