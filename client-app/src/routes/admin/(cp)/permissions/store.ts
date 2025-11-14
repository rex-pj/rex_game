import { get, writable, type Writable } from "svelte/store";
import { PermissionApi } from "$lib/api/permissionApi";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { Permission, PermissionRequest } from "$lib/models/permission";
import type { CurrentUser } from "$lib/models/current-user";
import * as accessService from "$lib/services/accessService";
import { PermissionCodes } from "$lib/common/permissions";
import { AdminClientApiOptions } from "$lib/api/apiOptions";

const permissionService: PermissionApi = new PermissionApi(new AdminClientApiOptions(Cookies));
export const items: Writable<Permission[]> = writable([]);
export const pager: Writable<Pager> = writable({ currentPage: 1, totalPages: 0 });
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const edittingData: Writable<PermissionRequest> = writable({
  id: 0,
  name: "",
  code: "",
  module: "",
  description: "",
});

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

// Fetch flashcards data (mocked for now)
export const fetchItems = async (page: number) => {
  // Replace this with your API call
  const response = await permissionService.getList(fetch, page, itemsPerPage);

  items.set(response.items);
  const totalPages = Math.ceil(response.total_count / itemsPerPage);
  pager.update((current) => ({
    ...current,
    totalPages: totalPages,
  }));
};

export const submit = async (data: PermissionRequest) => {
  if (data.id) {
    return await update(data.id, data);
  }
  return await create(data);
};

export const create = async (data: PermissionRequest) => {
  isSubmitting.set(true);
  await permissionService
    .create(fetch, data)
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

export const update = async (id: number, data: PermissionRequest) => {
  isSubmitting.set(true);
  await permissionService
    .update(fetch, id, { name: data.name, description: data.description })
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
  return await permissionService
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
  edittingData.set({ id: 0, code: "", name: "", module: "", description: "" });
};

export const openEditingModal = (id: number) => {
  getById(id).then((response) => {
    if (response) {
      edittingData.set(response);
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
  await permissionService
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
    accessService.isInPermissions(currentUser, [PermissionCodes.PermissionUpdate])
  );
};

export const canCreate = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.PermissionCreate])
  );
};

export const canDelete = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.PermissionDelete])
  );
};
