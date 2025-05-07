import { redirect } from "@sveltejs/kit";
import { AuthenticateService } from "../../../lib/services/authenticateService";
let authenticateService: AuthenticateService;

export function load({ cookies }) {
  const access_token = cookies.get("access_token");
  authenticateService = new AuthenticateService();
  if (!access_token || authenticateService.isTokenExpired(access_token)) {
    redirect(302, "/admin/authentication");
  }

  return {};
}
