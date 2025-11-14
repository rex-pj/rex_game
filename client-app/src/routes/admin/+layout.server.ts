import { AdminServerApiOptions } from "$lib/api/apiOptions";
import { UserApi } from "$lib/api/userApi";
import type { CurrentUser } from "$lib/models/current-user";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
  const userApi = new UserApi(new AdminServerApiOptions(cookies));
  const adminUser = (await userApi.getCurrentUser(fetch)) as CurrentUser;
  return {
    adminUser: adminUser,
  };
};
