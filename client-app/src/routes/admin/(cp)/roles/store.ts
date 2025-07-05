import { get, writable, type Writable } from "svelte/store";
import { RoleService } from "$lib/services/roleService";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { Role, RoleRequest } from "$lib/models/role";

const roleService: RoleService = new RoleService(Cookies);
export const items: Writable<Role[]> = writable([]);
export const pager: Writable<Pager> = writable({ currentPage: 1, totalPages: 0 });
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const edittingData: Writable<RoleRequest> = writable({
  id: 0,
  name: "",
  description: "",
});

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

// Fetch flashcards data (mocked for now)
export const getList = async (page: number) => {
  // Replace this with your API call
  const response = await roleService.getList(fetch, page, itemsPerPage);

  items.set(response.items);
  const totalPages = Math.ceil(response.total_count / itemsPerPage);
  pager.update((current) => ({
    ...current,
    totalPages: totalPages,
  }));
};

export const submit = async (data: RoleRequest) => {
  if (data.id) {
    return await update(data.id, data);
  }
  return await create(data);
};

export const create = async (data: RoleRequest) => {
  isSubmitting.set(true);
  await roleService
    .create(fetch, data)
    .then(async () => {
      await getList(1);
      toggleCreationModal(false);
    })
    .catch((error) => {
      creationError.set(error.message);
    })
    .finally(() => {
      isSubmitting.set(false);
    });
};

export const update = async (id: number, data: RoleRequest) => {
  isSubmitting.set(true);
  await roleService
    .update(fetch, id, { name: data.name, description: data.description })
    .then(async () => {
      await getList(1);
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
  return await roleService
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
    getList(page);
  }
};

export const toggleCreationModal = (isShown: boolean = false) => {
  showCreationModal.set(isShown);
  edittingData.set({ id: 0, name: "", description: "" });
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
  await roleService
    .deleteById(fetch, id)
    .then(async () => {
      await getList(1);
      toggleDeletionModal(false);
    })
    .catch((error) => {
      deletionError.set(error.message);
    })
    .finally(() => {
      isDeletionSubmitting.set(false);
    });
};
