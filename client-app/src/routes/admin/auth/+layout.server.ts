import { ADMIN_URLS, ROLE_NAMES } from "$lib/common/contants";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ parent }) => {
  const parentData = await parent();
  const adminUser = parentData?.adminUser;
  if (adminUser && adminUser?.roles?.some((r) => r === ROLE_NAMES.ADMIN || ROLE_NAMES.ROOT_ADMIN)) {
    throw redirect(302, ADMIN_URLS.DASHBOARD_URL);
  }

  return {
    adminUser: adminUser,
  };
};
