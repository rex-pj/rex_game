// +layout.server.ts
import { ADMIN_URLS, ROLE_NAMES } from "$lib/common/contants.js";
import { redirect } from "@sveltejs/kit";

export const load = async ({ parent }) => {
  const parentData = await parent();
  const currentUser = parentData?.currentUser;
  if (!currentUser) {
    throw redirect(302, ADMIN_URLS.LOGIN_URL);
  }

  if (!currentUser?.roles?.some((r) => r === ROLE_NAMES.ADMIN || ROLE_NAMES.ROOT_ADMIN)) {
    throw redirect(302, ADMIN_URLS.LOGIN_URL);
  }
  return { currentUser: parentData.currentUser };
};
