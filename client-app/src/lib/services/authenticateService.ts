import { type Cookies } from "@sveltejs/kit";
import { BaseService } from "./baseService";
import { container } from "$lib/di";

class AuthenticateService extends BaseService {
  constructor(cookies?: Cookies) {
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

  async logout(fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) {
    const response = await this.delete(fetch, "/auth/logout", { observe: true });
    return response;
  }

  setRefreshToken(response: Response, cookies: Cookies) {
    const cookieHeaders = response.headers.getSetCookie();
    const cookie_key = "refresh_token";
    const refresh_token_data = container.cookieHelper.parseSetCookie(cookieHeaders, cookie_key);
    if (!refresh_token_data) {
      console.error("No Set-Cookie header found in the response.");
      return;
    }

    cookies.set(cookie_key, refresh_token_data.value, refresh_token_data.options);
  }

  removeRefreshToken(cookies: Cookies) {
    const cookie_key = "refresh_token";
    cookies.delete(cookie_key, { path: "/" });
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
}

export { AuthenticateService };
