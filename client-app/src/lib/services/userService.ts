import { type Cookies } from "@sveltejs/kit";
import { BaseService } from "./baseService";

class UserService extends BaseService {
  private readonly userUrl = "/users";
  constructor(cookies?: Cookies) {
    super(cookies);
  }

  async getCurrentUser(fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) {
    const response = await this.get(fetch, `${this.userUrl}/me`, new URLSearchParams(), {
      observe: true,
    });
    if (response.status !== 200) {
      return null;
    }
    return await response.json();
  }
}

export { UserService };
