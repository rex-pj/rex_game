import { redirect } from "@sveltejs/kit";
import * as authenticationClient from "$lib/helpers/authenticationClient";
import type { LayoutLoad } from "./$types";
import { browser } from "$app/environment";

export const load: LayoutLoad = ({ params }) => {
  if (browser) {
    const access_token = authenticationClient.getAccessToken();
    if (!access_token || authenticationClient.isTokenExpired(access_token)) {
      redirect(302, "/admin/authentication");
    }
  }

  return {};
};
