import type { Cookies } from "@sveltejs/kit";
import JsCookies from "js-cookie";
import { BaseService } from "./baseService";
import { jwtDecode } from "jwt-decode";
import { container } from "$lib/di";

class AuthenticateService extends BaseService {
  constructor(cookies?: Cookies | typeof JsCookies) {
    super(cookies);
  }

  async login(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    email: string,
    password: string
  ) {
    const response = await this.post(fetch, "/auth/login", { email, password }, { observe: true });
    return await response;
  }

  async refreshToken(fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) {
    const response = await this.post(fetch, "/auth/refresh_token", { observe: true });
    return await response;
  }

  setRefreshToken(response: Response, cookies: any) {
    const cookieHeaders = response.headers.getSetCookie();
    const cookie_key = "refresh_token";
    const refresh_token_data = container.cookieHelper.parseSetCookie(cookieHeaders, cookie_key);
    if (!refresh_token_data) {
      console.error("No Set-Cookie header found in the response.");
      return;
    }

    cookies.set(cookie_key, refresh_token_data.value, refresh_token_data.options);
  }

  setAccessToken(response: { access_token: string; expiration: number }, cookies: any) {
    const { access_token, expiration } = response;
    cookies.set("access_token", access_token, {
      path: "/",
      httpOnly: false,
      sameSite: "strict",
      secure: true,
      expires: new Date(expiration),
    });

    cookies.set("access_token_exp", expiration, {
      path: "/",
      httpOnly: false,
      sameSite: "strict",
      secure: true,
      expires: new Date(expiration),
    });
  }

  async logout(fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) {
    const response = await this.delete(fetch, "/auth/logout");
    return response;
  }

  isTokenExpired(token: string): boolean {
    try {
      const decoded = jwtDecode(token);
      if (!decoded || !decoded.exp) {
        throw new Error("Invalid token");
      }
      const currentTime = Math.floor(Date.now() / 1000);
      return decoded.exp < currentTime;
    } catch (error) {
      console.error("Error decoding token:", error);
      return true; // Assume expired if there's an error
    }
  }
}

export { AuthenticateService };
