import { json, type RequestHandler } from "@sveltejs/kit";
import {
  tryRefreshToken,
  ADMIN_TOKEN_CONFIG,
  USER_TOKEN_CONFIG,
} from "$lib/services/tokenRefreshService";

export const POST: RequestHandler = async ({ cookies, fetch, request }) => {
  const body = await request.json().catch(() => ({}));
  const tokenType: string = body.tokenType || "admin";

  const config = tokenType === "user" ? USER_TOKEN_CONFIG : ADMIN_TOKEN_CONFIG;

  const refreshed = await tryRefreshToken(cookies, fetch, config);

  if (refreshed) {
    const newToken = cookies.get(config.accessTokenKey);
    const newExp = cookies.get(config.accessTokenExpKey);
    return json({ access_token: newToken, expiration: newExp });
  }

  return json({ error: "Failed to refresh token" }, { status: 401 });
};
