import { writable, type Writable } from "svelte/store";
import Cookies from "js-cookie";
import type { RolePermission } from "$lib/models/role-permission";
import type { UserPermission } from "$lib/models/user-permission";
import { AccessService } from "$lib/services/accessService";
import type { UserRole } from "$lib/models/user-role";

const accessService: AccessService = new AccessService(Cookies);
export const userRoles: Writable<UserRole[]> = writable([]);
export const rolePermissions: Writable<RolePermission[]> = writable([]);
export const userPermissions: Writable<UserPermission[]> = writable([]);

// Fetch user role data
export const fetchUserRoleItems = async () => {
  const userRoleResponse = await accessService.getUserRoleList(fetch);
  userRoles.set(userRoleResponse);
};

// Fetch role permissions data
export const fetchRolePermissionItems = async () => {
  const rolePermissionResponse = await accessService.getRolePermissionList(fetch);
  rolePermissions.set(rolePermissionResponse);
};

// Fetch user permissions data
export const fetchUserPermissionItems = async () => {
  const userPermissionResponse = await accessService.getUserPermissionList(fetch);
  userPermissions.set(userPermissionResponse);
};
