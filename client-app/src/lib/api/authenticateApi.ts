import { type Cookies } from "@sveltejs/kit";
import { BaseApi } from "./baseApi";
import type { BaseApiOptions } from "./apiOptions";
import { ACCESS_TOKEN } from "$lib/common/contants";

class AuthenticateApi extends BaseApi {
  constructor(options: BaseApiOptions) {
    super(options);
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

  removeRefreshToken(cookies: Cookies) {
    const cookie_key = "refresh_token";
    cookies.delete(cookie_key, { path: "/" });
    cookies.delete(ACCESS_TOKEN.ADMIN_REFRESH_TOKEN, { path: "/" });
    cookies.delete(ACCESS_TOKEN.USER_REFRESH_TOKEN, { path: "/" });
  }
}

export { AuthenticateApi };
