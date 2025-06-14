import { json, redirect } from "@sveltejs/kit";
import { AuthenticateService } from "../../../lib/services/authenticateService";
import { ADMIN_URLS } from "$lib/common/contants";

export async function DELETE({ request, cookies, fetch }) {
  const authenticateService = new AuthenticateService(cookies);
  authenticateService.removeRefreshToken(cookies);
  return await authenticateService
    .logout(fetch)
    .then(() => {
      return json({ message: "ok" });
    })
    .catch(() => {
      return json({ message: "failed" });
    });
}

export async function POST({ request, cookies, fetch }) {
  const authenticateService = new AuthenticateService(cookies);
  const data = await request.formData();
  const email = data.get("email") as string;
  const password = data.get("password") as string;

  const response = await authenticateService.login(fetch, email, password).catch((error) => {
    console.error("Error during login:", error);
  });

  if (!response || response.status !== 200) {
    console.error(
      "Login failed with status:",
      response?.status,
      "and message:",
      await response?.text()
    );
    redirect(302, ADMIN_URLS.LOGIN_URL);
  }

  authenticateService.setRefreshToken(response, cookies);
  const loginResult = await response.json();
  authenticateService.setAccessToken(loginResult, cookies);
  redirect(302, ADMIN_URLS.DASHBOARD_URL);
}

export async function PATCH({ request, cookies, fetch }) {
  const authenticateService = new AuthenticateService(cookies);
  const refresh_token = cookies.get("refresh_token");
  if (!refresh_token) {
    console.error("No refresh token found");
    return Promise.reject(new Error("No refresh token found"));
  }
  const response = await authenticateService.refreshToken(fetch).catch((error: any) => {
    console.error("Error during token refresh:", error);
  });
  if (!response || response.status !== 200) {
    console.error(
      "Token refresh failed with status:",
      response?.status,
      "and message:",
      await response?.text()
    );
    return Promise.reject(new Error("Token refresh failed with status:"));
  }
  const loginResult = await response.json();
  authenticateService.setAccessToken(loginResult, cookies);
  authenticateService.setRefreshToken(response, cookies);
  return Promise.resolve(response);
}
