import * as accessService from "$lib/services/accessService";
import { browser } from "$app/environment";

export const load = () => {
  if (!browser) {
    return {};
  }
  const access_token = accessService.getAccessToken();
  if (!access_token) {
    return {};
  }

  if (!accessService.isTokenExpired(access_token)) {
    return { hasToken: true };
  }
  return {};
};
