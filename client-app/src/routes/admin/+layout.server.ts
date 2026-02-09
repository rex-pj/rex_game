import { AdminServerApiOptions } from "$lib/api/apiOptions";
import { UserApi } from "$lib/api/userApi";
import type { CurrentUser } from "$lib/models/current-user";
import type { LayoutServerLoad } from "./$types";
import { tryRefreshToken, ADMIN_TOKEN_CONFIG } from "$lib/services/tokenRefreshService";

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
  let userApi = new UserApi(new AdminServerApiOptions(cookies));
  let adminUser = (await userApi.getCurrentUser(fetch)) as CurrentUser;

  // If getCurrentUser failed (likely expired token), try refreshing
  if (!adminUser) {
    const refreshed = await tryRefreshToken(cookies, fetch, ADMIN_TOKEN_CONFIG);
    if (refreshed) {
      userApi = new UserApi(new AdminServerApiOptions(cookies));
      adminUser = (await userApi.getCurrentUser(fetch)) as CurrentUser;
    }
  }

  return {
    adminUser: adminUser,
  };
};
