import type { CurrentUser } from "$lib/models/current-user";
import { UserApi } from "$lib/api/userApi";
import type { LayoutServerLoad } from "./$types";
import { UserServerApiOptions } from "$lib/api/apiOptions";
import { tryRefreshToken, USER_TOKEN_CONFIG } from "$lib/services/tokenRefreshService";

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
  const userApi = new UserApi(new UserServerApiOptions(cookies));
  let currentUser = (await userApi.getCurrentUser(fetch)) as CurrentUser;

  // If getCurrentUser failed (likely expired token), try refreshing
  if (!currentUser) {
    const refreshed = await tryRefreshToken(cookies, fetch, USER_TOKEN_CONFIG);
    if (refreshed) {
      const freshUserApi = new UserApi(new UserServerApiOptions(cookies));
      currentUser = (await freshUserApi.getCurrentUser(fetch)) as CurrentUser;
    }
  }

  return { currentUser };
};
