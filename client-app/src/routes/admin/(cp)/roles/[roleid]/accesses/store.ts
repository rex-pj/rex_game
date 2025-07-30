import { writable, type Writable } from "svelte/store";
import type { Permission } from "$lib/models/permission";
import type { RolePermission } from "$lib/models/role-permission";

export const permissions: Writable<Permission[]> = writable([]);

export const initPermission = (
  permissionList: Permission[],
  rolePermissionList: RolePermission[]
) => {
  permissionList.forEach((permission: Permission) => {
    permission.assigned = rolePermissionList?.some(
      (rolePermission) => rolePermission.permission_id === permission.id
    );
  });

  permissions.set(permissionList);
};

export const togglePermission = (
  permissionList: Permission[],
  permissionCode: string,
  e: Event
) => {
  const checked = (e.target as HTMLInputElement).checked;
  permissionList.forEach((permission: Permission) => {
    if (permission.code === permissionCode) {
      permission.assigned = checked;
    }
  });

  permissions.set(permissionList);
};
