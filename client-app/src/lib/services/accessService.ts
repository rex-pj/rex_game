import { jwtDecode } from "jwt-decode";
import Cookies from "js-cookie";
import type { CurrentUser } from "$lib/models/current-user";
import { ADMIN_URLS, ROLE_NAMES } from "$lib/common/contants";
import { PermissionCodes } from "$lib/common/permissions";

export const logout = async (): Promise<Response> => {
  return fetch(ADMIN_URLS.LOGOUT_URL, {
    method: "DELETE",
  }).then((rs: Response) => {
    Cookies.remove("access_token");
    Cookies.remove("access_token_exp");
    return rs;
  });
};

export const isTokenExpired = (token: string) => {
  try {
    const decoded = jwtDecode(token);
    if (!decoded || !decoded.exp) {
      throw new Error("Invalid token");
    }
    const currentTime = Math.floor(Date.now() / 1000);
    return decoded.exp < currentTime;
  } catch (error) {
    console.error("Error decoding token:", error);
    return true; // Assume expired if there's an error
  }
};

export const getAccessToken = () => {
  const access_token = Cookies.get("access_token");
  return access_token;
};

export const isRootAdmin = (currentUser: CurrentUser | undefined) => {
  return currentUser && currentUser.roles && currentUser.roles.includes(ROLE_NAMES.ROOT_ADMIN);
};

export const isInPermissions = (currentUser: CurrentUser | undefined, permissions: string[]) => {
  return (
    currentUser &&
    currentUser.permissions &&
    currentUser.permissions.some((p) => permissions.includes(p))
  );
};

export const canReadFlashcards = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.FlashcardCreate,
      PermissionCodes.FlashcardDelete,
      PermissionCodes.FlashcardRead,
      PermissionCodes.FlashcardUpdate,
    ])
  );
};

export const canReadFlashcardTypes = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.FlashcardTypeCreate,
      PermissionCodes.FlashcardTypeDelete,
      PermissionCodes.FlashcardTypeRead,
      PermissionCodes.FlashcardTypeUpdate,
    ])
  );
};

export const canReadMailTemplates = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.MailTemplateCreate,
      PermissionCodes.MailTemplateDelete,
      PermissionCodes.MailTemplateRead,
      PermissionCodes.MailTemplateUpdate,
    ])
  );
};

export const canReadPermissions = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.PermissionCreate,
      PermissionCodes.PermissionDelete,
      PermissionCodes.PermissionRead,
      PermissionCodes.PermissionUpdate,
    ])
  );
};

export const canReadRoles = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.RoleCreate,
      PermissionCodes.RoleDelete,
      PermissionCodes.RoleRead,
      PermissionCodes.RoleUpdate,
    ])
  );
};

export const canReadUsers = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserCreate,
      PermissionCodes.UserDelete,
      PermissionCodes.UserRead,
      PermissionCodes.UserUpdate,
    ])
  );
};

export const canReadAccesses = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserRoleRead,
      PermissionCodes.UserRoleCreate,
      PermissionCodes.UserRoleUpdate,
      PermissionCodes.UserRoleDelete,
      PermissionCodes.UserPermissionRead,
      PermissionCodes.UserPermissionCreate,
      PermissionCodes.UserPermissionUpdate,
      PermissionCodes.UserPermissionDelete,
      PermissionCodes.RolePermissionRead,
      PermissionCodes.RolePermissionCreate,
      PermissionCodes.RolePermissionUpdate,
      PermissionCodes.RolePermissionDelete,
      PermissionCodes.MailTemplateRead,
      PermissionCodes.MailTemplateCreate,
      PermissionCodes.MailTemplateUpdate,
      PermissionCodes.MailTemplateDelete,
    ])
  );
};

export const canEditUserAccesses = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserRoleUpdate,
      PermissionCodes.UserRoleCreate,
      PermissionCodes.UserRoleDelete,
      PermissionCodes.UserPermissionUpdate,
      PermissionCodes.UserPermissionCreate,
      PermissionCodes.UserPermissionDelete,
    ])
  );
};

export const canReadUserAccesses = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserRoleRead,
      PermissionCodes.UserPermissionRead,
      PermissionCodes.UserRoleCreate,
      PermissionCodes.UserPermissionCreate,
      PermissionCodes.UserRoleUpdate,
      PermissionCodes.UserPermissionUpdate,
      PermissionCodes.UserRoleDelete,
      PermissionCodes.UserPermissionDelete,
    ])
  );
};

export const canReadUserRoles = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserRoleCreate,
      PermissionCodes.UserRoleRead,
      PermissionCodes.UserRoleUpdate,
      PermissionCodes.UserRoleDelete,
    ])
  );
};

export const canEditUserRoles = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserRoleUpdate,
      PermissionCodes.UserRoleCreate,
      PermissionCodes.UserRoleDelete,
    ])
  );
};

export const canReadUserPermissions = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserPermissionCreate,
      PermissionCodes.UserPermissionRead,
      PermissionCodes.UserPermissionUpdate,
      PermissionCodes.UserPermissionDelete,
    ])
  );
};

export const canEditUserPermissions = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.UserPermissionUpdate,
      PermissionCodes.UserPermissionCreate,
      PermissionCodes.UserPermissionDelete,
    ])
  );
};

export const canEditRolePermissions = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.RolePermissionUpdate,
      PermissionCodes.RolePermissionCreate,
      PermissionCodes.RolePermissionDelete,
    ])
  );
};

export const canReadRolePermissions = (currentUser: CurrentUser | undefined) => {
  return (
    isRootAdmin(currentUser) ||
    isInPermissions(currentUser, [
      PermissionCodes.RolePermissionRead,
      PermissionCodes.RolePermissionCreate,
      PermissionCodes.RolePermissionUpdate,
      PermissionCodes.RolePermissionDelete,
    ])
  );
};
