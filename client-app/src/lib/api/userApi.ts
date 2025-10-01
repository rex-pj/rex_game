import { type Cookies } from "@sveltejs/kit";
import { BaseApi } from "./baseApi";
import type JsCookies from "js-cookie";
import type {
  ConfirmUserRequest,
  ForgotPasswordRequest,
  ResetPasswordRequest,
  UserRequest,
} from "$lib/models/user";
import type { UserPermission } from "$lib/models/user-permission";
import type { UserRole } from "$lib/models/user-role";

class UserApi extends BaseApi {
  private readonly baseUrl = "/users";
  constructor(cookies?: Cookies | typeof JsCookies) {
    super(cookies);
  }

  async getCurrentUser(fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) {
    try {
      const response = await this.get(fetch, `${this.baseUrl}/me`, new URLSearchParams(), {
        observe: true,
      });
      if (response.status !== 200) {
        return null;
      }
      return await response.json();
    } catch {
      return null;
    }
  }

  async getList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number = 1,
    page_size: number = 10
  ) {
    const response = await this.get(
      fetch,
      this.baseUrl,
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
    data: UserRequest
  ) {
    const response = await this.post(fetch, this.baseUrl, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
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
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to update user");
    }
    return await response.json();
  }

  async confirm(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: ConfirmUserRequest
  ) {
    const response = await this.post(fetch, `${this.baseUrl}/confirmations`, data, {
      observe: true,
    });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to confirm user");
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

  async getRoleList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    page?: number,
    page_size?: number
  ): Promise<UserRole[]> {
    const params = new URLSearchParams();
    if (page) {
      params.append("page", page.toString());
    }

    if (page_size) {
      params.append("page_size", page_size.toString());
    }

    const response = await this.get(fetch, `${this.baseUrl}/${id}/roles`, params, {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to fetch user roles");
    }
    return await response.json();
  }

  async assignRoles(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    data: { role_ids: number[] }
  ): Promise<number> {
    const response = await this.post(fetch, `${this.baseUrl}/${id}/roles`, data, {
      observe: true,
    });
    if (response.status !== 200) {
      return Promise.reject(new Error("Failed to assign role to user"));
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
      return Promise.reject(new Error("Failed to assign permission to user"));
    }
    return await response.json();
  }

  async getPermissionList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    page: number | null = null,
    page_size: number | null = null
  ): Promise<UserPermission[]> {
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
      throw new Error("Failed to fetch user permissions");
    }
    return await response.json();
  }

  async forgotPassword(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: ForgotPasswordRequest
  ) {
    const response = await this.post(fetch, this.baseUrl + "/password", data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to request to reset password");
    }

    return await response.json();
  }

  async resetPassword(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: ResetPasswordRequest
  ) {
    const response = await this.patch(fetch, this.baseUrl + "/password", data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw new Error(error.error);
      }
      throw new Error("Failed to reset password");
    }

    return await response.json();
  }
}

export { UserApi };
