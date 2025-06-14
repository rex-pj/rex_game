import { redirect } from "@sveltejs/kit";
import * as authenticationClient from "$lib/helpers/authenticationClient";
import type { LayoutLoad } from "./$types";
import { browser } from "$app/environment";
import { ADMIN_URLS } from "$lib/common/contants";

export const load: LayoutLoad = () => {
  if (browser) {
    const access_token = authenticationClient.getAccessToken();
    if (!access_token || authenticationClient.isTokenExpired(access_token)) {
      redirect(302, ADMIN_URLS.LOGIN_URL);
    }
  }

  return {};
};
