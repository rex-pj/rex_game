import type { Cookies } from "@sveltejs/kit";
import JsCookies from "js-cookie";
import { BaseService } from "./baseService";
import { jwtDecode } from "jwt-decode";

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
    return await response.json();
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
