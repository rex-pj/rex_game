import { redirect, type Actions } from "@sveltejs/kit";
import { UserApi } from "../../../../lib/api/userApi";
import { fail } from "@sveltejs/kit";
import type { UserRequest } from "$lib/models/user";
import { APP_URLS } from "$lib/common/contants";
import { UserServerApiOptions } from "$lib/api/apiOptions";

export const load = async ({ parent }) => {
  const parentData = await parent();
  const currentUser = parentData?.currentUser;

  if (!currentUser) {
    return {};
  }

  if (!currentUser) {
    return {};
  }

  throw redirect(302, APP_URLS.HOME);
};

export const actions: Actions = {
  default: async ({ request, cookies }) => {
    const userApi = new UserApi(new UserServerApiOptions(cookies));
    const formData = await request.formData();

    const dataObject: { [key: string]: any } = {};
    for (const [key, value] of formData.entries()) {
      dataObject[key] = value;
    }

    const data = dataObject as UserRequest;
    const response = await userApi.create(fetch, data);
    if (!response || response.error || response.field_errors) {
      const email = data.email;
      const name = data.name;
      const display_name = data.display_name;
      return fail(400, {
        message: response?.error || "Signup failed",
        field_errors: response?.field_errors,
        values: { email, name, display_name },
      });
    }

    if (!data.email) {
      throw redirect(303, `/account/signup-success`);
    }
    throw redirect(303, `/account/signup-success?email=${encodeURIComponent(data.email)}`);
  },
};
