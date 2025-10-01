import { PermissionApi } from "$lib/api/permissionApi";
import { RoleApi } from "$lib/api/roleApi";
import { UserApi } from "$lib/api/userApi";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, cookies }) => {
  if (!params.userid) {
    return { userPermissions: [], userRoles: [], roles: [], permissions: [] };
  }

  const userId = Number.parseInt(params.userid);
  if (isNaN(userId)) {
    return { userPermissions: [], userRoles: [], roles: [], permissions: [] };
  }

  const userApi: UserApi = new UserApi(cookies);
  const roleService: RoleApi = new RoleApi(cookies);
  const permissionService: PermissionApi = new PermissionApi(cookies);

  const userPermissions = await userApi.getPermissionList(fetch, userId);
  const userRoles = await userApi.getRoleList(fetch, userId);
  const { items: roles } = await roleService.getList(fetch);
  const { items: permissions } = await permissionService.getList(fetch);

  return {
    userPermissions: userPermissions || [],
    userRoles: userRoles || [],
    roles: roles || [],
    permissions: permissions || [],
  };
};

export const actions = {
  roles: async ({ request, params, cookies }) => {
    const formData = await request.formData();
    if (!params.userid) {
      return { success: false, error: "User ID is required" };
    }

    const userId = params.userid ? Number.parseInt(params.userid) : 0;
    if (userId <= 0) {
      return { success: false, error: "Invalid User ID" };
    }

    if (!formData) {
      return { success: false, error: "Form data is required" };
    }

    if (!formData.has("role_ids")) {
      return { success: false, error: "No roles selected" };
    }

    const roleIds = formData.getAll("role_ids") as string[];
    if (roleIds.length === 0) {
      return { success: false, error: "No roles selected" };
    }

    const userApi = new UserApi(cookies);
    try {
      // Assign each role to the user
      await userApi.assignRoles(fetch, userId, {
        role_ids: roleIds.map(Number),
      });
    } catch (error: any) {
      return {
        success: false,
        error: `Failed to assign role ID ${roleIds}: ${error.message}`,
      };
    }

    // Fetch updated roles after assignment
    const userRoles = await userApi.getRoleList(fetch, userId);
    return { success: true, userRoles: userRoles || [] };
  },

  permissions: async ({ request, params, cookies }) => {
    const formData = await request.formData();
    if (!params.userid) {
      return { success: false, error: "User ID is required" };
    }

    const userId = params.userid ? Number.parseInt(params.userid) : 0;
    if (userId <= 0) {
      return { success: false, error: "Invalid User ID" };
    }

    if (!formData) {
      return { success: false, error: "Form data is required" };
    }

    if (!formData.has("permission_codes")) {
      return { success: false, error: "No permissions selected" };
    }

    const permissionCodes = formData.getAll("permission_codes") as string[];
    if (permissionCodes.length === 0) {
      return { success: false, error: "No permissions selected" };
    }

    const userApi = new UserApi(cookies);
    try {
      // Assign each permission to the user
      await userApi.assignPermissions(fetch, userId, {
        permission_codes: permissionCodes,
      });
    } catch (error: any) {
      return {
        success: false,
        error: `Failed to assign permission Code ${permissionCodes}: ${error.message}`,
      };
    }

    // Fetch updated permissions after assignment
    const userPermissions = await userApi.getPermissionList(fetch, userId);
    return { success: true, userPermissions: userPermissions || [] };
  },
};
