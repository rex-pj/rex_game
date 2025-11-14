import * as accessService from "$lib/services/accessService";
import { browser } from "$app/environment";
import { ACCESS_TOKEN } from "$lib/common/contants";

export const load = () => {
  if (!browser) {
    return {};
  }
  const access_token = accessService.getAccessToken(ACCESS_TOKEN.ADMIN_ACCESS_TOKEN);
  if (!access_token) {
    return {};
  }

  if (!accessService.isTokenExpired(access_token)) {
    return { hasToken: true };
  }
  return {};
};
