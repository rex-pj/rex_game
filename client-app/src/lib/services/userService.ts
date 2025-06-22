import { type Cookies } from "@sveltejs/kit";
import { BaseService } from "./baseService";
import type JsCookies from "js-cookie";
import type { UserRequest } from "$lib/models/user";

class UserService extends BaseService {
  private readonly baseUrl = "/users";
  constructor(cookies?: Cookies | typeof JsCookies) {
    super(cookies);
  }

  async getCurrentUser(fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) {
    const response = await this.get(fetch, `${this.baseUrl}/me`, new URLSearchParams(), {
      observe: true,
    });
    if (response.status !== 200) {
      return null;
    }
    return await response.json();
  }

  async getList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number = 1,
    page_size: number = 10
  ) {
    const response = await this.get(
      fetch,
      "/users",
      new URLSearchParams({ page: page.toString(), page_size: page_size.toString() }),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch users");
    }
    return await response.json();
  }

  async getById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ) {
    const response = await this.get(fetch, `${this.baseUrl}/${id}`, new URLSearchParams(), {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to fetch user");
    }
    return await response.json();
  }

  async create(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: FormData
  ) {
    const response = await this.post(fetch, this.baseUrl, data, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to create user");
    }
    return await response.json();
  }

  async update(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    data: UserRequest
  ) {
    const response = await this.patch(
      fetch,
      `${this.baseUrl}/${id}`,
      {
        name: data.name,
        display_name: data.display_name,
      },
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to update user");
    }
    return await response.json();
  }

  async deleteById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ) {
    const response = await this.delete(fetch, `${this.baseUrl}/${id}`, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to update user");
    }
    return await response.json();
  }
}

export { UserService };
