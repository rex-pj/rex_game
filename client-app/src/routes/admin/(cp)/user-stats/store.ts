import { get, writable, type Writable } from "svelte/store";
import { UserStatsAdminApi } from "$lib/api/userStatsAdminApi";
import Cookies from "js-cookie";
import type { Pager } from "../../../../components/molecules/pagination/pager";
import type { AdminUserStats } from "$lib/models/admin-user-stats";
import * as accessService from "$lib/services/accessService";
import type { CurrentUser } from "$lib/models/current-user";
import { PermissionCodes } from "$lib/common/permissions";
import { AdminClientApiOptions } from "$lib/api/apiOptions";

const userStatsService: UserStatsAdminApi = new UserStatsAdminApi(
  new AdminClientApiOptions(Cookies)
);
export const items: Writable<AdminUserStats[]> = writable([]);
export const pager: Writable<Pager> = writable({ currentPage: 1, totalPages: 0 });
const itemsPerPage = 10;

export const showResetModal = writable(false);
export const resetError = writable("");
export const isResetSubmitting = writable(false);
export const resettingData = writable({ userId: 0, name: "" });

export const fetchItems = async (page: number) => {
  const response = await userStatsService.getList(fetch, page, itemsPerPage);
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

export const openResetModal = (userId: number, displayName: string) => {
  resettingData.set({ userId, name: displayName });
  showResetModal.set(true);
};

export const toggleResetModal = (isShown: boolean = false) => {
  showResetModal.set(isShown);
  resettingData.set({ userId: 0, name: "" });
};

export const resetStats = async (userId: number) => {
  isResetSubmitting.set(true);
  await userStatsService
    .resetStats(fetch, userId)
    .then(async () => {
      await fetchItems(get(pager).currentPage);
      toggleResetModal(false);
    })
    .catch((error) => {
      resetError.set(error.message);
    })
    .finally(() => {
      isResetSubmitting.set(false);
    });
};

export const canReset = (currentUser: CurrentUser | undefined) => {
  return (
    (currentUser && accessService.isRootAdmin(currentUser)) ||
    accessService.isInPermissions(currentUser, [PermissionCodes.UserStatsUpdate])
  );
};
