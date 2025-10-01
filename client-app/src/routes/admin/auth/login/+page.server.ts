import { redirect, type Actions } from "@sveltejs/kit";
import { AuthenticateApi } from "../../../../lib/api/authenticateApi";
import { ADMIN_URLS, ROLE_NAMES } from "$lib/common/contants";

export const load = async ({ parent }) => {
  const parentData = await parent();
  const currentUser = parentData?.currentUser;

  if (!currentUser) {
    return {};
  }

  if (!currentUser?.roles?.some((r) => r === ROLE_NAMES.ADMIN || ROLE_NAMES.ROOT_ADMIN)) {
    return {};
  }

  throw redirect(302, ADMIN_URLS.DASHBOARD_URL);
};

export const actions: Actions = {
  login: async ({ request, cookies, fetch }) => {
    const authenticateApi = new AuthenticateApi(cookies);
    const data = await request.formData();
    const email = data.get("email") as string;
    const password = data.get("password") as string;

    let response: Response | undefined;
    try {
      response = await authenticateApi.login(fetch, email, password);
    } catch (error) {
      console.error("Error during login:", error);
    }

    if (!response || response.status !== 200) {
      console.error(
        "Login failed with status:",
        response?.status,
        "and message:",
        await response?.text()
      );
      throw redirect(302, ADMIN_URLS.LOGIN_URL);
    }

    authenticateApi.setRefreshToken(response, cookies);
    const loginResult = await response.json();
    authenticateApi.setAccessToken(loginResult, cookies);

    throw redirect(302, ADMIN_URLS.DASHBOARD_URL);
  },
};
