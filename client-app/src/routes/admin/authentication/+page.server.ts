import type { Actions } from "./$types";
import { redirect } from "@sveltejs/kit";
import { AuthenticateService } from "../../../lib/services/authenticateService";

let authenticateService: AuthenticateService;

export function load({ cookies }) {
  const access_token = cookies.get("access_token");
  authenticateService = new AuthenticateService(cookies);
  if (access_token && !authenticateService.isTokenExpired(access_token)) {
    redirect(302, "/admin/dashboard");
  }

  return {};
}

export const actions = {
  login: async ({ cookies, request, fetch }) => {
    const data = await request.formData();
    const email = data.get("email") as string;
    const password = data.get("password") as string;

    const response = await authenticateService.login(fetch, email, password).catch((error) => {
      console.error("Error during login:", error);
    });

    if (response.status !== 200) {
      console.error(
        "Login failed with status:",
        response.status,
        "and message:",
        await response.text()
      );
      return;
    }

    authenticateService.setRefreshToken(response, cookies);
    const loginResult = await response.json();
    authenticateService.setAccessToken(loginResult, cookies);
    redirect(302, "/admin/dashboard");
  },
  refresh_token: async ({ cookies, fetch }) => {
    const refresh_token = cookies.get("refresh_token");
    if (!refresh_token) {
      console.error("No refresh token found");
      return;
    }
    const response = await authenticateService.refreshToken(fetch).catch((error: any) => {
      console.error("Error during token refresh:", error);
    });
    if (response.status !== 200) {
      console.error(
        "Token refresh failed with status:",
        response.status,
        "and message:",
        await response.text()
      );
      return;
    }
    const loginResult = await response.json();
    authenticateService.setAccessToken(loginResult, cookies);
    authenticateService.setRefreshToken(response, cookies);
  },
} satisfies Actions;
