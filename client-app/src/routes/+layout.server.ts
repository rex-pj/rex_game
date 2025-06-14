import type { CurrentUser } from "$lib/models/current-user";
import { UserService } from "$lib/services/userService";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
  const userService = new UserService(cookies);
  const currentUser = (await userService.getCurrentUser(fetch)) as CurrentUser;
  return { currentUser };
};
