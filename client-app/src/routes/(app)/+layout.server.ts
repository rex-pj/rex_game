import type { CurrentUser } from "$lib/models/current-user";
import { UserApi } from "$lib/api/userApi";
import type { LayoutServerLoad } from "./$types";
import { UserServerApiOptions } from "$lib/api/apiOptions";

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
  const userApi = new UserApi(new UserServerApiOptions(cookies));
  const currentUser = (await userApi.getCurrentUser(fetch)) as CurrentUser;
  return { currentUser };
};
