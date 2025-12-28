import { BaseApi } from "./baseApi";
import type { BaseApiOptions } from "./apiOptions";
import type { Role } from "$lib/models/role";
import type { RolePermission } from "$lib/models/role-permission";

export class RoleApi extends BaseApi {
  private readonly baseUrl = "/roles";
  constructor(options: BaseApiOptions) {
    super(options);
  }

  async getList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number | null = null,
    page_size: number | null = null
  ): Promise<{ items: Role[]; total_count: number }> {
    const params = new URLSearchParams();
    if (page !== null) {
      params.append("page", page.toString());
    }

    if (page_size !== null) {
      params.append("page_size", page_size.toString());
    }
    const response = await this.get(fetch, this.baseUrl, new URLSearchParams(params), {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to fetch roles");
    }
    return await response.json();
  }

  async getById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ): Promise<Role> {
    const response = await this.get(fetch, `${this.baseUrl}/${id}`, new URLSearchParams(), {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to fetch role");
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
      throw new Error("Failed to create role");
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
      throw new Error("Failed to update role");
    }
    return await response.json();
  }

  async deleteById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ) {
    const response = await this.delete(fetch, `${this.baseUrl}/${id}`, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to delete role");
    }
    return await response.json();
  }

  async getPermissionList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    page: number | null = null,
    page_size: number | null = null
  ): Promise<RolePermission[]> {
    const params = new URLSearchParams();
    if (page !== null) {
      params.append("page", page.toString());
    }

    if (page_size !== null) {
      params.append("page_size", page_size.toString());
    }
    const response = await this.get(fetch, `${this.baseUrl}/${id}/permissions`, params, {
      observe: true,
    });

    if (response.status !== 200) {
      throw new Error("Failed to fetch role permissions");
    }
    return await response.json();
  }

  async assignPermissions(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    data: { permission_codes: string[] }
  ): Promise<number> {
    const response = await this.post(fetch, `${this.baseUrl}/${id}/permissions`, data, {
      observe: true,
    });
    if (response.status !== 200) {
      return Promise.reject(new Error("Failed to assign permission to role"));
    }
    return await response.json();
  }
}
