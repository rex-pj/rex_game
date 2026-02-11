import { get, writable, type Writable } from "svelte/store";
import { GameSessionAdminApi } from "$lib/api/gameSessionAdminApi";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { AdminGameSession } from "$lib/models/admin-game-session";
import * as accessService from "$lib/services/accessService";
import type { CurrentUser } from "$lib/models/current-user";
import { PermissionCodes } from "$lib/common/permissions";
import { AdminClientApiOptions } from "$lib/api/apiOptions";

const gameSessionService: GameSessionAdminApi = new GameSessionAdminApi(
  new AdminClientApiOptions(Cookies)
);
export const items: Writable<AdminGameSession[]> = writable([]);
export const pager: Writable<Pager> = writable({ currentPage: 1, totalPages: 0 });
const itemsPerPage = 10;

export const showDeletionModal = writable(false);
export const deletionError = writable("");
export const isDeletionSubmitting = writable(false);
export const deletingData = writable({ id: 0, name: "" });

export const fetchItems = async (page: number) => {
  const response = await gameSessionService.getList(fetch, page, itemsPerPage);
  items.set(response.items);
  const totalPages = Math.ceil(response.total_count / itemsPerPage);
  pager.update((current) => ({
    ...current,
    totalPages: totalPages,
  }));
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

export const openDeletingModal = (id: number, displayName: string) => {
  deletingData.set({ id, name: displayName });
  showDeletionModal.set(true);
};

export const toggleDeletionModal = (isShown: boolean = false) => {
  showDeletionModal.set(isShown);
  deletingData.set({ id: 0, name: "" });
};

export const deleteById = async (id: number) => {
  isDeletionSubmitting.set(true);
  await gameSessionService
    .deleteById(fetch, id)
    .then(async () => {
      await fetchItems(get(pager).currentPage);
      toggleDeletionModal(false);
    })
    .catch((error) => {
      deletionError.set(error.message);
    })
    .finally(() => {
      isDeletionSubmitting.set(false);
    });
};

export const canDelete = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.GameSessionDelete])
  );
};
