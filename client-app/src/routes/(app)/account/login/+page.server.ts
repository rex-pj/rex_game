import { redirect, type Actions } from "@sveltejs/kit";
import { ACCESS_TOKEN, APP_URLS } from "$lib/common/contants";
import { AuthenticateApi } from "$lib/api/authenticateApi";
import { UserServerApiOptions } from "$lib/api/apiOptions";
import { container } from "$lib/di";

export const load = async ({ parent }) => {
  const parentData = await parent();
  const currentUser = parentData?.currentUser;

  if (!currentUser) {
    return {};
  }

  if (!currentUser) {
    return {};
  }

  throw redirect(302, APP_URLS.HOME);
};

export const actions: Actions = {
  login: async ({ request, cookies, fetch }) => {
    const authenticateApi = new AuthenticateApi(new UserServerApiOptions(cookies));
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
      throw redirect(302, APP_URLS.LOGIN_URL);
    }

    const cookieHeaders = response.headers.getSetCookie();
    const refresh_token_data = container.cookieHelper.parseSetCookie(
      cookieHeaders,
      "refresh_token"
    );
    if (!refresh_token_data) {
      console.error("No Set-Cookie header found in the response.");
      return;
    }

    cookies.set(
      ACCESS_TOKEN.USER_REFRESH_TOKEN,
      refresh_token_data.value,
      refresh_token_data.options
    );
    const loginResponse = await response.json();
    const { access_token, expiration } = loginResponse;
    cookies.set(ACCESS_TOKEN.USER_ACCESS_TOKEN, access_token, {
      path: "/",
      httpOnly: false,
      sameSite: "strict",
      secure: true,
      expires: new Date(expiration),
    });
    cookies.set(ACCESS_TOKEN.USER_ACCESS_TOKEN_EXP, expiration, {
      path: "/",
      httpOnly: false,
      sameSite: "strict",
      secure: true,
      expires: new Date(expiration),
    });

    throw redirect(302, APP_URLS.HOME);
  },
};
