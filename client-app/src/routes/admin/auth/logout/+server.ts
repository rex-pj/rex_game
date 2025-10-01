import { AuthenticateApi } from "$lib/api/authenticateApi";
import { json, type RequestHandler } from "@sveltejs/kit";

export const DELETE: RequestHandler = async ({ cookies, fetch }) => {
  const authenticateApi = new AuthenticateApi(cookies);
  authenticateApi.removeRefreshToken(cookies);

  try {
    await authenticateApi.logout(fetch);
    return json({ message: "ok" });
  } catch {
    return json({ message: "failed" });
  }
};
