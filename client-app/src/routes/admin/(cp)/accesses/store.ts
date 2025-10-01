import { writable, type Writable } from "svelte/store";
import Cookies from "js-cookie";
import { AccessApi } from "$lib/api/accessApi";
import { goto } from "$app/navigation";
import type { PermissionItem } from "$lib/models/permission";
import type { UserItem } from "$lib/models/user";
import type { RoleItem } from "$lib/models/role";

const accessApi: AccessApi = new AccessApi(Cookies);
export const userAccessess: Writable<
  { user: UserItem; roles: RoleItem[]; permissions: PermissionItem[] }[]
> = writable([]);
export const rolePermissions: Writable<{ role: RoleItem; permissions: PermissionItem[] }[]> =
  writable([]);

// Fetch user role data
export const fetchUserAccessItems = async () => {
  const userRoleResponse = await accessApi.getUserRoleList(fetch);
  const userPermissionResponse = await accessApi.getUserPermissionList(fetch);
  const groupedUserAccesses: {
    user: UserItem;
    roles: RoleItem[];
    permissions: PermissionItem[];
  }[] = [];
  const userMap = new Map<
    number,
    { user: UserItem; roles: RoleItem[]; permissions: PermissionItem[] }
  >();

  userRoleResponse.forEach((item) => {
    const userId = item.user_id;
    if (!userMap.has(userId)) {
      userMap.set(userId, {
        user: {
          id: userId,
          name: item.user_name,
          user_name: item.user_name,
        },
        roles: [],
        permissions: [],
      });
    }
    userMap.get(userId)!.roles.push({
      id: item.role_id,
      name: item.role_name,
    });
  });

  // Merge permissions into userMap
  userPermissionResponse.forEach((item) => {
    const userId = item.user_id;
    if (!userMap.has(userId)) {
      userMap.set(userId, {
        user: {
          id: userId,
          name: item.user_name,
          user_name: item.user_name,
        },
        roles: [],
        permissions: [],
      });
    }
    userMap.get(userId)!.permissions.push({
      id: item.permission_id,
      name: item.permission_name,
      code: item.permission_code,
      module: item.permission_module,
    });
  });

  groupedUserAccesses.push(...userMap.values());
  userAccessess.set(groupedUserAccesses);
};

// Fetch role permissions data
export const fetchRolePermissionItems = async () => {
  const rolePermissionResponse = await accessApi.getRolePermissionList(fetch);
  const groupedRolePermissions: { role: RoleItem; permissions: PermissionItem[] }[] = [];
  const roleMap = new Map<number, { role: RoleItem; permissions: PermissionItem[] }>();

  rolePermissionResponse.forEach((item) => {
    const userId = item.role_id;
    if (!roleMap.has(userId)) {
      roleMap.set(userId, {
        role: {
          id: userId,
          name: item.role_name,
        },
        permissions: [],
      });
    }
    roleMap.get(userId)!.permissions.push({
      id: item.permission_id,
      name: item.permission_name,
      code: item.permission_code,
      module: item.permission_module,
    });
  });

  groupedRolePermissions.push(...roleMap.values());
  rolePermissions.set(groupedRolePermissions);
};

export const redirectToUserAccesses = (id: number) => {
  goto(`/admin/users/${id}/accesses`);
};

export const redirectToRoleAccesses = (id: number) => {
  goto(`/admin/roles/${id}/accesses`);
};
