import { BaseApi } from "./baseApi";
import type { Permission } from "$lib/models/permission";
import type { BaseApiOptions } from "./apiOptions";

export class PermissionApi extends BaseApi {
  private readonly baseUrl = "/permissions";
  constructor(options: BaseApiOptions) {
    super(options);
  }

  async getList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page?: number,
    page_size?: number
  ): Promise<{ items: Permission[]; total_count: number }> {
    const params = new URLSearchParams();
    if (page) {
      params.append("page", page.toString());
    }

    if (page_size) {
      params.append("page_size", page_size.toString());
    }
    const response = await this.get(fetch, this.baseUrl, params, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to fetch permissions");
    }
    return await response.json();
  }

  async getById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ): Promise<Permission> {
    const response = await this.get(fetch, `${this.baseUrl}/${id}`, new URLSearchParams(), {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to fetch permission");
    }
    return await response.json();
  }

  async create(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: { name: string; description: string }
  ) {
    const response = await this.post(fetch, this.baseUrl, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to create permission");
    }
    return await response.json();
  }

  async update(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    data: { name: string; description: string }
  ) {
    const response = await this.patch(fetch, `${this.baseUrl}/${id}`, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to update permission");
    }
    return await response.json();
  }

  async deleteById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ) {
    const response = await this.delete(fetch, `${this.baseUrl}/${id}`, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to delete permission");
    }
    return await response.json();
  }
}
