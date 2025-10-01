import { PermissionApi } from "$lib/api/permissionApi";
import { RoleApi } from "$lib/api/roleApi";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, cookies }) => {
  if (!params.roleid) {
    return { rolePermissions: [], permissions: [] };
  }

  const roleId = Number.parseInt(params.roleid);
  if (isNaN(roleId)) {
    return { rolePermissions: [], permissions: [] };
  }

  const roleService: RoleApi = new RoleApi(cookies);
  const permissionService: PermissionApi = new PermissionApi(cookies);

  const rolePermissions = await roleService.getPermissionList(fetch, roleId);
  const { items: permissions } = await permissionService.getList(fetch);

  return {
    rolePermissions: rolePermissions || [],
    permissions: permissions || [],
  };
};

export const actions = {
  permissions: async ({ request, params, cookies }) => {
    const formData = await request.formData();
    if (!params.roleid) {
      return { success: false, error: "Role ID is required" };
    }

    const roleId = params.roleid ? Number.parseInt(params.roleid) : 0;
    if (roleId <= 0) {
      return { success: false, error: "Invalid Role ID" };
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

    const roleService = new RoleApi(cookies);
    try {
      // Assign each permission to the user
      await roleService.assignPermissions(fetch, roleId, {
        permission_codes: permissionCodes,
      });
    } catch (error: any) {
      return {
        success: false,
        error: `Failed to assign permission Code ${permissionCodes}: ${error.message}`,
      };
    }

    // Fetch updated permissions after assignment
    const rolePermissions = await roleService.getPermissionList(fetch, roleId);
    return { success: true, rolePermissions: rolePermissions || [] };
  },
};
