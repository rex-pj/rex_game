import { BaseApi } from "./baseApi";
import type { UserRole } from "$lib/models/user-role";
import type { UserPermission } from "$lib/models/user-permission";
import type { RolePermission } from "$lib/models/role-permission";
import type { BaseApiOptions } from "./apiOptions";

export class AccessApi extends BaseApi {
  private readonly userRoleUrl = "/user-roles";
  private readonly userPermissionUrl = "/user-permissions";
  private readonly rolePermissionUrl = "/role-permissions";
  constructor(options: BaseApiOptions) {
    super(options);
  }

  async getUserRoleList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ): Promise<UserRole[]> {
    const params = new URLSearchParams();
    const response = await this.get(fetch, this.userRoleUrl, params, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to fetch the user roles");
    }
    return await response.json();
  }

  async getUserPermissionList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ): Promise<UserPermission[]> {
    const params = new URLSearchParams();
    const response = await this.get(fetch, this.userPermissionUrl, params, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to fetch the user permissions");
    }
    return await response.json();
  }

  async getRolePermissionList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ): Promise<RolePermission[]> {
    const params = new URLSearchParams();
    const response = await this.get(fetch, this.rolePermissionUrl, params, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to fetch the user permissions");
    }
    return await response.json();
  }
}
