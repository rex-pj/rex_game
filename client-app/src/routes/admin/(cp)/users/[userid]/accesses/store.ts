import { writable, type Writable } from "svelte/store";
import type { Role } from "$lib/models/role";
import type { UserRole } from "$lib/models/user-role";
import type { UserPermission } from "$lib/models/user-permission";
import type { Permission } from "$lib/models/permission";

export const roles: Writable<Role[]> = writable([]);
export const permissions: Writable<Permission[]> = writable([]);
export const initRole = (roleList: Role[], userRoleList: UserRole[]) => {
  roleList.forEach((role: Role) => {
    role.assigned = userRoleList?.some((userRole) => userRole.role_id === role.id);
  });

  roles.set(roleList);
};

export const initPermission = (
  permissionList: Permission[],
  userPermissionList: UserPermission[]
) => {
  permissionList.forEach((permission: Permission) => {
    permission.assigned = userPermissionList?.some(
      (userPermission) => userPermission.permission_id === permission.id
    );
  });

  permissions.set(permissionList);
};

export const toggleRole = (roleList: Role[], roleId: number, e: Event) => {
  const checked = (e.target as HTMLInputElement).checked;
  roleList.forEach((role: Role) => {
    if (role.id === roleId) {
      role.assigned = checked;
    }
  });

  roles.set(roleList);
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
