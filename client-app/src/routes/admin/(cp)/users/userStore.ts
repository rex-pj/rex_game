import { get, writable, type Writable } from "svelte/store";
import { UserService } from "$lib/services/userService";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { UserDto, UserRequest } from "$lib/models/user";

const userService: UserService = new UserService(Cookies);
export const users: Writable<UserDto[]> = writable([]);
export const currentPage = writable(1);
export const totalPages = writable(1);
export const pager = { currentPage: 1, totalPages: 0 } as Pager;
const itemsPerPage = 10;
export const showCreationModal = writable(false);
export const creationError = writable("");
export const isSubmitting = writable(false);
export const edittingData: Writable<UserRequest> = writable({
  id: 0,
  name: "",
  display_name: "",
  role_name: "",
  email: "",
});

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

// Fetch users data (mocked for now)
export const fetchUsers = async (page: number) => {
  // Replace this with your API call
  const response = await userService.getList(fetch, page, itemsPerPage);

  const start = (page - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  const items = response.items;
  users.set(items.slice(start, end));
  pager.totalPages = Math.ceil(response.total_count / itemsPerPage);
};

export const submit = async (data: UserRequest) => {
  return await updateUser(data.id, data);
};

export const updateUser = async (id: number, data: UserRequest) => {
  isSubmitting.set(true);

  await userService
    .update(fetch, id, data)
    .then(async () => {
      await fetchUsers(pager.currentPage);
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
  getUser(id).then((response) => {
    if (response) {
      deletingData.set({ id: response.id, name: response.name });
      showDeletionModal.set(true);
    }
  });
};

export const changePage = (page: number) => {
  if (page >= 1 && page <= get(totalPages)) {
    currentPage.set(page);
    fetchUsers(page);
  }
};

export const getUser = async (id: number) => {
  isSubmitting.set(true);
  return await userService
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
  edittingData.set({
    id: 0,
    name: "",
    display_name: "",
    email: "",
  });
};

export const openEditingModal = async (id: number) => {
  getUser(id).then(async (response: UserDto) => {
    if (response) {
      const data: UserRequest = {
        id: response.id,
        name: response.name,
        display_name: response.display_name,
        email: response.email,
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
  await userService
    .deleteById(fetch, id)
    .then(async () => {
      await fetchUsers(1);
      toggleDeletionModal(false);
    })
    .catch((error) => {
      deletionError.set(error.message);
    })
    .finally(() => {
      isDeletionSubmitting.set(false);
    });
};
