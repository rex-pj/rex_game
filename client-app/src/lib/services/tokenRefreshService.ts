import type { Cookies } from "@sveltejs/kit";
import { PUBLIC_API_URL } from "$env/static/public";
import { ACCESS_TOKEN } from "$lib/common/contants";
import { container } from "$lib/di";

interface TokenRefreshConfig {
  accessTokenKey: ACCESS_TOKEN;
  accessTokenExpKey: ACCESS_TOKEN;
  refreshTokenKey: ACCESS_TOKEN;
}

export const ADMIN_TOKEN_CONFIG: TokenRefreshConfig = {
  accessTokenKey: ACCESS_TOKEN.ADMIN_ACCESS_TOKEN,
  accessTokenExpKey: ACCESS_TOKEN.ADMIN_ACCESS_TOKEN_EXP,
  refreshTokenKey: ACCESS_TOKEN.ADMIN_REFRESH_TOKEN,
};

export const USER_TOKEN_CONFIG: TokenRefreshConfig = {
  accessTokenKey: ACCESS_TOKEN.USER_ACCESS_TOKEN,
  accessTokenExpKey: ACCESS_TOKEN.USER_ACCESS_TOKEN_EXP,
  refreshTokenKey: ACCESS_TOKEN.USER_REFRESH_TOKEN,
};

/**
 * Attempt to refresh the access token using the refresh token.
 * This runs server-side only (in SvelteKit load functions).
 *
 * The backend expects:
 * - Authorization header with the (possibly expired) access token
 * - A cookie named "refresh_token" with the refresh token value
 *
 * Since the frontend stores the refresh token under a different cookie name
 * (s_refresh_token / u_refresh_token), we manually set the Cookie header.
 */
export async function tryRefreshToken(
  cookies: Cookies,
  fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
  config: TokenRefreshConfig
): Promise<boolean> {
  const accessToken = cookies.get(config.accessTokenKey);
  const refreshToken = cookies.get(config.refreshTokenKey);

  if (!accessToken || !refreshToken) {
    return false;
  }

  try {
    const response = await fetch(`${PUBLIC_API_URL}/auth/refresh`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${accessToken}`,
        Cookie: `refresh_token=${refreshToken}`,
      },
      body: JSON.stringify({}),
    });

    if (!response.ok) {
      return false;
    }

    // Parse new refresh_token from Set-Cookie header
    const cookieHeaders = response.headers.getSetCookie();
    const refreshTokenData = container.cookieHelper.parseSetCookie(cookieHeaders, "refresh_token");
    if (refreshTokenData) {
      cookies.set(config.refreshTokenKey, refreshTokenData.value, refreshTokenData.options);
    }

    // Parse new access_token + expiration from JSON body
    const loginResponse = await response.json();
    const { access_token, expiration } = loginResponse;

    cookies.set(config.accessTokenKey, access_token, {
      path: "/",
      httpOnly: false,
      sameSite: "strict",
      secure: true,
      expires: new Date(expiration),
    });

    cookies.set(config.accessTokenExpKey, expiration, {
      path: "/",
      httpOnly: false,
      sameSite: "strict",
      secure: true,
      expires: new Date(expiration),
    });

    return true;
  } catch {
    return false;
  }
}
