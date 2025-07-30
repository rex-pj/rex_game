import type { Cookies } from "@sveltejs/kit";
import { BaseService } from "./baseService";
import type JsCookies from "js-cookie";

export class AccessService extends BaseService {
  private readonly userRoleUrl = "/user-roles";
  private readonly userPermissionUrl = "/user-permissions";
  private readonly rolePermissionUrl = "/role-permissions";
  constructor(cookies?: Cookies | typeof JsCookies) {
    super(cookies);
  }

  async getUserRoleList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ) {
    const params = new URLSearchParams();
    const response = await this.get(fetch, this.userRoleUrl, params, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to fetch the user roles");
    }
    return await response.json();
  }

  async getUserPermissionList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ) {
    const params = new URLSearchParams();
    const response = await this.get(fetch, this.userPermissionUrl, params, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to fetch the user permissions");
    }
    return await response.json();
  }

  async getRolePermissionList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ) {
    const params = new URLSearchParams();
    const response = await this.get(fetch, this.rolePermissionUrl, params, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to fetch the user permissions");
    }
    return await response.json();
  }
}
