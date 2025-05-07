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

    const { access_token, refresh_token } = response;
    cookies.set("access_token", access_token, {
      path: "/",
      httpOnly: false,
      sameSite: "strict",
    });
    cookies.set("refresh_token", refresh_token, {
      path: "/",
      httpOnly: true,
      sameSite: "lax",
    });
    redirect(302, "/admin/dashboard");
  },
} satisfies Actions;
