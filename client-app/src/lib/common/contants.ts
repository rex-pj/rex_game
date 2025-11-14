export const SHARED_CONTEXT = {
  CURRENT_USER: "currentUser",
};

export const ADMIN_URLS = {
  LOGIN_URL: "/admin/auth/login",
  LOGOUT_URL: "/admin/auth/logout",
  LOGIN_SUCCESS_URL: "/admin/auth/login-success",
  LOGOUT_SUCCESS_URL: "/admin/auth/logout-success",
  DASHBOARD_URL: "/admin/dashboard",
};

export const ROLE_NAMES = {
  ADMIN: "Admin",
  ROOT_ADMIN: "RootAdmin",
};

export const APP_URLS = {
  HOME: "/",
  LOGIN_URL: "/account/login",
  LOGOUT_URL: "/account/logout",
  SIGNUP_URL: "/account/signup",
};

export enum ACCESS_TOKEN {
  ADMIN_ACCESS_TOKEN = "s_access_token",
  ADMIN_ACCESS_TOKEN_EXP = "s_access_token_exp",
  ADMIN_REFRESH_TOKEN = "s_refresh_token",
  USER_ACCESS_TOKEN = "u_access_token",
  USER_ACCESS_TOKEN_EXP = "u_access_token_exp",
  USER_REFRESH_TOKEN = "u_refresh_token",
}
