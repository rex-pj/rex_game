import { redirect, type Actions } from "@sveltejs/kit";
import { UserApi } from "../../../../lib/api/userApi";
import { fail } from "@sveltejs/kit";
import type { ForgotPasswordRequest } from "$lib/models/user";
import { UserServerApiOptions } from "$lib/api/apiOptions";

export const actions: Actions = {
  default: async ({ request, cookies }) => {
    const userApi = new UserApi(new UserServerApiOptions(cookies));
    const formData = await request.formData();

    const dataObject: { [key: string]: any } = {};
    for (const [key, value] of formData.entries()) {
      dataObject[key] = value;
    }

    const data = dataObject as ForgotPasswordRequest;
    const response = await userApi.forgotPassword(fetch, data);
    if (!response || response.error || response.field_errors) {
      const email = data.email;
      return fail(400, {
        message: response?.error || "Request to forgot password failed",
        field_errors: response?.field_errors,
        values: { email },
      });
    }

    if (!data.email) {
      throw redirect(303, `/account/forgot-password-success`);
    }
    throw redirect(303, `/account/forgot-password-success?email=${encodeURIComponent(data.email)}`);
  },
};
