import { redirect } from "@sveltejs/kit";
import * as accessService from "$lib/services/accessService";
import type { LayoutLoad } from "./$types";
import { browser } from "$app/environment";
import { ACCESS_TOKEN, ADMIN_URLS } from "$lib/common/contants";

export const load: LayoutLoad = () => {
  if (!browser) {
    return {};
  }

  const access_token = accessService.getAccessToken(ACCESS_TOKEN.ADMIN_ACCESS_TOKEN);
  if (!access_token || accessService.isTokenExpired(access_token)) {
    redirect(302, ADMIN_URLS.LOGIN_URL);
  }
};
